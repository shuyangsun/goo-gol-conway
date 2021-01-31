use crate::{
    renderer::{
        board_info::RendererBoardInfo, fps_counter::FPSCounter, keyboard_control::KeyboardControl,
    },
    CellularAutomatonRenderer, ColorMapping,
};
use gfx_hal::{
    adapter::PhysicalDevice,
    command::{ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, Level, SubpassContents},
    device::Device,
    format::{ChannelType, Format},
    image::{Extent, Layout},
    pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDesc},
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, Face, GraphicsPipelineDesc,
        InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Rect, ShaderStageFlags,
        Specialization, Viewport,
    },
    queue::{CommandQueue, QueueFamily, Submission},
    window::{Extent2D, PresentationSurface, Surface, SwapchainConfig},
    Instance, UnsupportedBackend,
};
use gol_core::{util::grid_util::Size2D, BinaryStatesReadOnly, GridPoint2D};
use num_traits::{CheckedSub, FromPrimitive, ToPrimitive};
use rayon::prelude::*;
use shaderc::ShaderKind;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct GraphicalRendererGrid2D<M, CI, T>
where
    CI: Hash,
{
    info: RendererBoardInfo<Size2D>,
    control: Option<KeyboardControl>,
    fps_counter: FPSCounter,
    color_map: M,
    states_read_only: BinaryStatesReadOnly<CI, T>,
    cell_scale: f32,
}

impl<T, U, M> GraphicalRendererGrid2D<M, GridPoint2D<U>, T>
where
    T: 'static + Send + Sync + Clone,
    U: 'static + Send + Sync + Clone + Ord + CheckedSub + ToPrimitive + FromPrimitive + Hash,
    M: 'static + Send + Sync + Clone + ColorMapping<T>,
{
    pub fn new(
        board_width: usize,
        board_height: usize,
        color_map: M,
        states_storage: BinaryStatesReadOnly<GridPoint2D<U>, T>,
    ) -> Result<Self, UnsupportedBackend> {
        let info = RendererBoardInfo::new(Size2D::new(board_width, board_height));
        Ok(Self {
            info,
            control: None,
            fps_counter: FPSCounter::new(240),
            color_map,
            states_read_only: states_storage,
            cell_scale: 0.95,
        })
    }

    pub fn with_title(self, title: String) -> Self {
        let mut res = self;
        res.info.set_title(title);
        res
    }

    pub fn with_color_map(self, cmap: M) -> Self {
        let mut res = self;
        res.color_map = cmap;
        res
    }

    pub fn with_cell_render_scale(self, scale: f32) -> Self {
        let mut res = self;
        res.cell_scale = scale;
        res
    }

    pub fn with_keyboard_control(self, control: KeyboardControl) -> Self {
        let mut res = self;
        res.control = Some(control);
        res
    }
}

impl<T, U, M> CellularAutomatonRenderer for GraphicalRendererGrid2D<M, GridPoint2D<U>, T>
where
    T: 'static + Send + Sync + Clone,
    U: 'static + Send + Sync + Clone + Ord + CheckedSub + ToPrimitive + FromPrimitive + Hash,
    M: 'static + Send + Sync + Clone + ColorMapping<T>,
{
    fn run(&mut self) {
        let event_loop = EventLoop::new();
        let title = self.info.title().clone();
        let color_map = self.color_map.clone();

        let board_size = self.info.board_size().clone();
        let states_read_only = self.states_read_only.clone();
        let cell_scale = self.cell_scale;
        let mut control = self.control.clone();

        let desired_aspect_ratio = 1.0;

        let (logical_window_size, physical_window_size) =
            get_window_size(&event_loop, desired_aspect_ratio, 0.8);

        let mut surface_extent = Extent2D {
            width: physical_window_size.width,
            height: physical_window_size.height,
        };

        let window = winit::window::WindowBuilder::new()
            .with_title(title.as_str())
            .with_inner_size(logical_window_size)
            .build(&event_loop)
            .expect("Failed to create window");

        let (instance, surface, adapter) = {
            let instance =
                backend::Instance::create(title.as_str(), 1).expect("Backend not supported");

            let surface = unsafe {
                instance
                    .create_surface(&window)
                    .expect("Failed to create surface for window")
            };

            let adapter = instance.enumerate_adapters().remove(0);

            (instance, surface, adapter)
        };

        let (device, mut queue_group) = {
            let queue_family = adapter
                .queue_families
                .iter()
                .find(|family| {
                    surface.supports_queue_family(family) && family.queue_type().supports_graphics()
                })
                .expect("No compatible queue family found");

            let mut gpu = unsafe {
                adapter
                    .physical_device
                    .open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
                    .expect("Failed to open device")
            };

            (gpu.device, gpu.queue_groups.pop().unwrap())
        };

        let (command_pool, mut command_buffer) = unsafe {
            let mut command_pool = device
                .create_command_pool(queue_group.family, CommandPoolCreateFlags::empty())
                .expect("Out of memory");

            let command_buffer = command_pool.allocate_one(Level::Primary);

            (command_pool, command_buffer)
        };

        let surface_color_format = {
            let supported_formats = surface
                .supported_formats(&adapter.physical_device)
                .unwrap_or(vec![]);

            let default_format = *supported_formats.get(0).unwrap_or(&Format::Rgba8Srgb);

            supported_formats
                .into_iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .unwrap_or(default_format)
        };

        let render_pass = {
            let color_attachment = Attachment {
                format: Some(surface_color_format),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };

            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            unsafe {
                device
                    .create_render_pass(&[color_attachment], &[subpass], &[])
                    .expect("Out of memory")
            }
        };

        let pipeline_layout = unsafe {
            let push_constant_bytes = std::mem::size_of::<PushConstants>() as u32;

            // The second slice passed here defines the ranges of push constants
            // available to each shader stage. In this example, we're going to give
            // one `PushConstants` struct worth of bytes to the vertex shader.
            //
            // Out data _could_ be offset, which is why we pass a range of bytes,
            // but here we can start at zero since there's no data before our
            // struct.
            device
                .create_pipeline_layout(&[], &[(ShaderStageFlags::VERTEX, 0..push_constant_bytes)])
                .expect("Out of memory")
        };

        let vertex_shader = include_str!("shaders/square.vert");
        let fragment_shader = include_str!("shaders/square.frag");

        let pipeline = unsafe {
            make_pipeline::<backend::Backend>(
                &device,
                &render_pass,
                &pipeline_layout,
                vertex_shader,
                fragment_shader,
            )
        };

        let submission_complete_fence = device.create_fence(true).expect("Out of memory");
        let rendering_complete_semaphore = device.create_semaphore().expect("Out of memory");

        let mut resource_holder: ResourceHolder<backend::Backend> =
            ResourceHolder(ManuallyDrop::new(Resources {
                instance,
                surface,
                device,
                command_pool,
                render_passes: vec![render_pass],
                pipeline_layouts: vec![pipeline_layout],
                pipelines: vec![pipeline],
                submission_complete_fence,
                rendering_complete_semaphore,
            }));

        let mut should_configure_swapchain = true;

        event_loop.run(move |event, _, control_flow| {
            let mut cur_iter: Option<usize> = None;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(dims) => {
                        surface_extent = Extent2D {
                            width: dims.width,
                            height: dims.height,
                        };
                        should_configure_swapchain = true;
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        surface_extent = Extent2D {
                            width: new_inner_size.width,
                            height: new_inner_size.height,
                        };
                        should_configure_swapchain = true;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == ElementState::Released {
                            if let Some(control) = control.as_mut() {
                                match input.virtual_keycode {
                                    Some(ch) => match ch {
                                        VirtualKeyCode::Q => {
                                            control.broadcast('q');
                                        }
                                        VirtualKeyCode::J => {
                                            control.broadcast('j');
                                        }
                                        VirtualKeyCode::K => {
                                            control.broadcast('k');
                                        }
                                        VirtualKeyCode::Space => {
                                            control.broadcast(' ');
                                        }
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                    _ => (),
                },
                Event::MainEventsCleared => window.request_redraw(),
                Event::RedrawRequested(_) => {
                    let res: &mut Resources<_> = &mut resource_holder.0;
                    let render_pass = &res.render_passes[0];
                    let pipeline_layout = &res.pipeline_layouts[0];
                    let pipeline = &res.pipelines[0];

                    let (grid_width, grid_height) =
                        (board_size.width() as u32, board_size.height() as u32);

                    let non_trivial_state = states_read_only.non_trivial_state();
                    let mut indices_clone = HashSet::new();
                    loop {
                        match states_read_only.try_read() {
                            Ok(val) => {
                                if cur_iter.is_none() || cur_iter.unwrap() != val.0 {
                                    indices_clone = val.1.clone();
                                    cur_iter = Some(val.0);
                                    break;
                                }
                            }
                            Err(_) => continue,
                        }
                    }

                    let constants: Vec<((u32, u32), ColorRGBA)> = indices_clone
                        .par_iter()
                        .map(|ele| {
                            let color = color_map.color_representation(&non_trivial_state);
                            let max_color = u16::MAX as f32;
                            let (x_min, y_min) =
                                (board_size.x_idx_min() as i32, board_size.y_idx_min() as i32);
                            let ele_res: ((u32, u32), ColorRGBA) = (
                                (
                                    (ele.x.to_i32().unwrap() - x_min).to_u32().unwrap(),
                                    (ele.y.to_i32().unwrap() - y_min).to_u32().unwrap(),
                                ),
                                ColorRGBA {
                                    r: color.r as f32 / max_color,
                                    g: color.g as f32 / max_color,
                                    b: color.b as f32 / max_color,
                                    a: color.a as f32 / max_color,
                                },
                            );
                            ele_res
                        })
                        .collect::<Vec<((u32, u32), ColorRGBA)>>();

                    unsafe {
                        // We refuse to wait more than a second, to avoid hanging.
                        let render_timeout_ns = 1_000_000_000;

                        res.device
                            .wait_for_fence(&res.submission_complete_fence, render_timeout_ns)
                            .expect("Out of memory or device lost");

                        res.device
                            .reset_fence(&res.submission_complete_fence)
                            .expect("Out of memory");

                        res.command_pool.reset(false);
                    }

                    if should_configure_swapchain {
                        let caps = res.surface.capabilities(&adapter.physical_device);

                        let mut swapchain_config =
                            SwapchainConfig::from_caps(&caps, surface_color_format, surface_extent);

                        // This seems to fix some fullscreen slowdown on macOS.
                        if caps.image_count.contains(&3) {
                            swapchain_config.image_count = 3;
                        }

                        // Uncomment to allow resizing to other aspects:
                        surface_extent = swapchain_config.extent;

                        unsafe {
                            res.surface
                                .configure_swapchain(&res.device, swapchain_config)
                                .expect("Failed to configure swapchain");
                        };

                        should_configure_swapchain = false;
                    }

                    let surface_image = unsafe {
                        // We refuse to wait more than a second, to avoid hanging.
                        let acquire_timeout_ns = 1_000_000_000;

                        match res.surface.acquire_image(acquire_timeout_ns) {
                            Ok((image, _)) => image,
                            Err(_) => {
                                should_configure_swapchain = true;
                                return;
                            }
                        }
                    };

                    let framebuffer = unsafe {
                        res.device
                            .create_framebuffer(
                                render_pass,
                                vec![surface_image.borrow()],
                                Extent {
                                    width: surface_extent.width,
                                    height: surface_extent.height,
                                    depth: 1,
                                },
                            )
                            .unwrap()
                    };

                    let viewport = {
                        Viewport {
                            rect: Rect {
                                x: 0,
                                y: 0,
                                w: surface_extent.width as i16,
                                h: surface_extent.height as i16,
                            },
                            depth: 0.0..1.0,
                        }
                    };

                    // Each `PushConstants` struct in this slice represents the
                    // color, position, and scale of a triangle. This allows us to
                    // efficiently draw the same thing multiple times with varying
                    // parameters.

                    unsafe {
                        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

                        command_buffer.set_viewports(0, &[viewport.clone()]);
                        command_buffer.set_scissors(0, &[viewport.rect]);

                        command_buffer.begin_render_pass(
                            render_pass,
                            &framebuffer,
                            viewport.rect,
                            &[ClearValue {
                                color: ClearColor {
                                    float32: [0.0, 0.0, 0.0, 1.0],
                                },
                            }],
                            SubpassContents::Inline,
                        );

                        command_buffer.bind_graphics_pipeline(pipeline);

                        let squares = create_squares(
                            &surface_extent,
                            grid_width,
                            grid_height,
                            constants,
                            cell_scale,
                        );
                        for square in squares.as_slice() {
                            // This encodes the actual push constants themselves
                            // into the command buffer. The vertex shader will be
                            // able to access these properties.
                            command_buffer.push_graphics_constants(
                                pipeline_layout,
                                ShaderStageFlags::VERTEX,
                                0,
                                push_constant_bytes(square),
                            );

                            command_buffer.draw(0..4, 0..1);
                        }

                        command_buffer.end_render_pass();
                        command_buffer.finish();
                    }

                    unsafe {
                        let submission = Submission {
                            command_buffers: vec![&command_buffer],
                            wait_semaphores: None,
                            signal_semaphores: vec![&res.rendering_complete_semaphore],
                        };

                        queue_group.queues[0]
                            .submit(submission, Some(&res.submission_complete_fence));

                        let result = queue_group.queues[0].present(
                            &mut res.surface,
                            surface_image,
                            Some(&res.rendering_complete_semaphore),
                        );

                        should_configure_swapchain |= result.is_err();

                        res.device.destroy_framebuffer(framebuffer);
                    }
                }
                _ => (),
            }
        });
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct PushConstants {
    color: [f32; 4],
    pos: [f32; 2],
    scale: [f32; 2],
}

fn max_inner_rect_with_ratio<T, U>(width: &T, height: &T, desired_ratio: U, scale: U) -> (T, T)
where
    T: Clone + num_traits::cast::NumCast + std::convert::TryFrom<T>,
    U: Clone
        + num_traits::cast::ToPrimitive
        + num_traits::cast::NumCast
        + std::ops::Div<Output = U>
        + std::ops::Mul<Output = U>
        + PartialOrd,
{
    let (width, height) = (
        U::from(width.clone()).unwrap(),
        U::from(height.clone()).unwrap(),
    );
    let cur_ratio = width.clone() / height.clone();
    if desired_ratio > cur_ratio {
        let res_width = width.clone() * scale;
        let res_height = res_width.clone() / desired_ratio;
        (T::from(res_width).unwrap(), T::from(res_height).unwrap())
    } else {
        let res_height = height.clone() * scale;
        let res_width = res_height.clone() * desired_ratio;
        (T::from(res_width).unwrap(), T::from(res_height).unwrap())
    }
}

fn get_window_size(
    event_loop: &EventLoop<()>,
    aspect_ratio: f32,
    scale: f32,
) -> (LogicalSize<u32>, PhysicalSize<u32>) {
    let primary_monitor = event_loop
        .primary_monitor()
        .expect("Cannot find primary monitor.");
    let screen_physical_size = primary_monitor.size();
    let (width, height) = max_inner_rect_with_ratio(
        &screen_physical_size.width,
        &screen_physical_size.height,
        aspect_ratio,
        scale,
    );
    let physical = PhysicalSize::new(width, height);
    let dpi = primary_monitor.scale_factor();
    let logical: LogicalSize<u32> = physical.to_logical(dpi);

    (logical, physical)
}

#[derive(Clone)]
pub struct ColorRGBA {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

fn create_squares(
    surface_extent: &Extent2D,
    grid_width: u32,
    grid_height: u32,
    states: Vec<((u32, u32), ColorRGBA)>,
    scale: f32,
) -> Vec<PushConstants> {
    assert!(scale >= 0.0 && scale <= 1.0);
    let (mut left_padding, mut top_padding) = (0.0, 0.0);
    let (square_width, square_height) = (
        surface_extent.width as f32 / grid_width as f32,
        surface_extent.height as f32 / grid_height as f32,
    );
    let square_len = square_width.min(square_height);
    let (scale_x, scale_y) = (
        square_len / surface_extent.width as f32 * 2.0,
        square_len / surface_extent.height as f32 * 2.0,
    );

    if square_len < square_width {
        left_padding = 1.0 - scale_x / 2.0 * grid_width as f32;
    } else if square_len < square_height {
        top_padding = 1.0 - scale_y / 2.0 * grid_height as f32;
    }

    let mut res = Vec::new();
    for (idx, color) in states.iter() {
        res.push(PushConstants {
            color: [color.r, color.g, color.b, color.a],
            pos: [
                -1.0 + left_padding + idx.0 as f32 * scale_x + (1.0 - scale) / 2.0 * scale_x,
                -1.0 + top_padding
                    + (grid_height - idx.1 - 1) as f32 * scale_y
                    + (1.0 - scale) / 2.0 * scale_y,
            ],
            scale: [scale_x * scale, scale_y * scale],
        });
    }
    res
}

/// Compile some GLSL shader source to SPIR-V.
///
/// We tend to write shaders in high-level languages, but the GPU doesn't
/// work with that directly. Instead, we can convert it to an intermediate
/// representation: SPIR-V. This is more easily interpreted and optimized
/// by your graphics card. As an added bonus, this allows us to use the
/// same shader code across different backends.
fn compile_shader(glsl: &str, shader_kind: ShaderKind) -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new().unwrap();

    let compiled_shader = compiler
        .compile_into_spirv(glsl, shader_kind, "unnamed", "main", None)
        .expect("Failed to compile shader");

    compiled_shader.as_binary().to_vec()
}

/// Create a pipeline with the given layout and shaders.
///
/// A pipeline contains nearly all the required information for rendering,
/// and is only usable within the render pass it's defined for.
unsafe fn make_pipeline<B: gfx_hal::Backend>(
    device: &B::Device,
    render_pass: &B::RenderPass,
    pipeline_layout: &B::PipelineLayout,
    vertex_shader: &str,
    fragment_shader: &str,
) -> B::GraphicsPipeline {
    let vertex_shader_module = device
        .create_shader_module(&compile_shader(vertex_shader, ShaderKind::Vertex))
        .expect("Failed to create vertex shader module");

    let fragment_shader_module = device
        .create_shader_module(&compile_shader(fragment_shader, ShaderKind::Fragment))
        .expect("Failed to create fragment shader module");

    let (vs_entry, fs_entry) = (
        EntryPoint {
            entry: "main",
            module: &vertex_shader_module,
            specialization: Specialization::default(),
        },
        EntryPoint {
            entry: "main",
            module: &fragment_shader_module,
            specialization: Specialization::default(),
        },
    );
    let primitive_assembler = PrimitiveAssemblerDesc::Vertex {
        buffers: &[],
        attributes: &[],
        input_assembler: InputAssemblerDesc::new(Primitive::TriangleStrip),
        vertex: vs_entry,
        tessellation: None,
        geometry: None,
    };
    let mut pipeline_desc = GraphicsPipelineDesc::new(
        primitive_assembler,
        Rasterizer {
            cull_face: Face::BACK,
            ..Rasterizer::FILL
        },
        Some(fs_entry),
        pipeline_layout,
        Subpass {
            index: 0,
            main_pass: render_pass,
        },
    );

    pipeline_desc.blender.targets.push(ColorBlendDesc {
        mask: ColorMask::ALL,
        blend: Some(BlendState::ALPHA),
    });
    let pipeline = device
        .create_graphics_pipeline(&pipeline_desc, None)
        .expect("Failed to create graphics pipeline");

    device.destroy_shader_module(vertex_shader_module);
    device.destroy_shader_module(fragment_shader_module);

    pipeline
}

/// Returns a view of a struct as a slice of `u32`s.
///
/// Note that this assumes the struct divides evenly into
/// 4-byte chunks. If the contents are all `f32`s, which they
/// often are, then this will always be the case.
unsafe fn push_constant_bytes<T>(push_constants: &T) -> &[u32] {
    let size_in_bytes = std::mem::size_of::<T>();
    let size_in_u32s = size_in_bytes / std::mem::size_of::<u32>();
    let start_ptr = push_constants as *const T as *const u32;
    std::slice::from_raw_parts(start_ptr, size_in_u32s)
}

struct Resources<B: gfx_hal::Backend> {
    instance: B::Instance,
    surface: B::Surface,
    device: B::Device,
    render_passes: Vec<B::RenderPass>,
    pipeline_layouts: Vec<B::PipelineLayout>,
    pipelines: Vec<B::GraphicsPipeline>,
    command_pool: B::CommandPool,
    submission_complete_fence: B::Fence,
    rendering_complete_semaphore: B::Semaphore,
}

struct ResourceHolder<B: gfx_hal::Backend>(ManuallyDrop<Resources<B>>);

impl<B: gfx_hal::Backend> Drop for ResourceHolder<B> {
    fn drop(&mut self) {
        unsafe {
            let Resources {
                instance,
                mut surface,
                device,
                command_pool,
                render_passes,
                pipeline_layouts,
                pipelines,
                submission_complete_fence,
                rendering_complete_semaphore,
            } = ManuallyDrop::take(&mut self.0);

            device.destroy_semaphore(rendering_complete_semaphore);
            device.destroy_fence(submission_complete_fence);
            for pipeline in pipelines {
                device.destroy_graphics_pipeline(pipeline);
            }
            for pipeline_layout in pipeline_layouts {
                device.destroy_pipeline_layout(pipeline_layout);
            }
            for render_pass in render_passes {
                device.destroy_render_pass(render_pass);
            }
            device.destroy_command_pool(command_pool);
            surface.unconfigure_swapchain(&device);
            instance.destroy_surface(surface);
        }
    }
}
