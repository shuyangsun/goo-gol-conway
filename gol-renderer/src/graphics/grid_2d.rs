// Large part of this file was originally copied from https://github.com/mistodon/gfx-hal-tutorials by @mistodon, then refactored, modified then upgraded to newest gfx-hal libraries, which did make breaking changes.
use crate::{
    renderer::{
        board_info::RendererBoardInfo, fps_counter::FPSCounter, keyboard_control::KeyboardControl,
    },
    CellularAutomatonRenderer, StateVisualMapping,
};
use gfx_hal::{
    adapter::PhysicalDevice,
    command::{
        ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, Level, RenderAttachmentInfo,
        SubpassContents,
    },
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
    queue::{CommandQueue, QueueFamily},
    window::{Extent2D, PresentationSurface, Surface, SwapchainConfig},
    Instance, UnsupportedBackend,
};
use gol_core::{util::grid_util::Shape2D, GridPoint2D, StatesReadOnly};
use num_traits::{CheckedSub, FromPrimitive, ToPrimitive};
use rayon::prelude::*;
use rgb::RGBA16;
use shaderc::ShaderKind;
use std::borrow::Borrow;
use std::hash::Hash;
use std::mem::ManuallyDrop;
use std::time::Instant;
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{ElementState, Event, MouseScrollDelta, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub struct GraphicalRendererGrid2D<CI, T>
where
    CI: Hash,
{
    info: RendererBoardInfo<Shape2D>,
    control: Option<KeyboardControl>,
    fps_counter: FPSCounter, // TODO: show FPS
    states_read_only: StatesReadOnly<CI, T>,
    is_triangle: bool,
}

impl<T, U> GraphicalRendererGrid2D<GridPoint2D<U>, T>
where
    T: 'static + Send + Sync + Clone,
    U: 'static + Send + Sync + Clone + Ord + CheckedSub + ToPrimitive + FromPrimitive + Hash,
{
    pub fn new(
        board_width: usize,
        board_height: usize,
        states_storage: StatesReadOnly<GridPoint2D<U>, T>,
    ) -> Result<Self, UnsupportedBackend> {
        let info = RendererBoardInfo::new(Shape2D::new(board_width, board_height));
        Ok(Self {
            info,
            control: None,
            fps_counter: FPSCounter::new(240),
            states_read_only: states_storage,
            is_triangle: false,
        })
    }

    pub fn with_title(self, title: String) -> Self {
        let mut res = self;
        res.info.set_title(title);
        res
    }

    pub fn with_keyboard_control(self, control: KeyboardControl) -> Self {
        let mut res = self;
        res.control = Some(control);
        res
    }

    pub fn with_squares(self) -> Self {
        let mut res = self;
        res.is_triangle = false;
        res
    }

    pub fn with_triangles(self) -> Self {
        let mut res = self;
        res.is_triangle = true;
        res
    }
}

impl<T, U> CellularAutomatonRenderer<T, RGBA16> for GraphicalRendererGrid2D<GridPoint2D<U>, T>
where
    T: 'static + Send + Sync + Clone + Hash,
    U: 'static + Send + Sync + Clone + Ord + CheckedSub + ToPrimitive + FromPrimitive + Hash,
{
    fn need_run_on_main(&self) -> bool {
        true
    }

    fn run(&mut self, visual_mapping: Box<dyn StateVisualMapping<T, RGBA16>>) {
        let event_loop = EventLoop::new();
        let title = self.info.title().clone();

        let board_shape = self.info.board_shape().clone();
        let states_read_only = self.states_read_only.clone();
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
                    .create_render_pass(
                        vec![color_attachment].into_iter(),
                        vec![subpass].into_iter(),
                        vec![].into_iter(),
                    )
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
                .create_pipeline_layout(
                    vec![].into_iter(),
                    vec![(ShaderStageFlags::VERTEX, 0..push_constant_bytes)].into_iter(),
                )
                .expect("Out of memory")
        };

        let is_triangle = self.is_triangle;
        let vertex_shader = if is_triangle {
            include_str!("shaders/triangle.vert")
        } else {
            include_str!("shaders/square.vert")
        };
        let fragment_shader = if is_triangle {
            include_str!("shaders/triangle.frag")
        } else {
            include_str!("shaders/square.frag")
        };

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

        let (mut zoom, mut translate_x, mut translate_y) = (1., 0., 0.);
        let mut click_start_pos: Option<PhysicalPosition<f64>> = None;
        let mut cur_cursor_pos: Option<PhysicalPosition<f64>> = None;
        let mut click_start_translates = None;
        let mut last_mouse_click_time = None;

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
                    WindowEvent::MouseInput { state, .. } => match state {
                        ElementState::Pressed => {
                            click_start_pos = cur_cursor_pos;
                            click_start_translates = Some((translate_x, translate_y));
                        }
                        ElementState::Released => {
                            click_start_pos = None;
                            click_start_translates = None;
                            if last_mouse_click_time.is_some() {
                                let last_click: Instant = last_mouse_click_time.unwrap();
                                let duration = Instant::now() - last_click;
                                if duration.as_millis() < 250 {
                                    zoom = 1.;
                                    translate_x = 0.;
                                    translate_y = 0.;
                                }
                            }
                            last_mouse_click_time = Some(Instant::now());
                        }
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        cur_cursor_pos = Some(position);
                        if click_start_pos.is_some() {
                            let pxl_x = position.x - click_start_pos.unwrap().x;
                            let pxl_y = position.y - click_start_pos.unwrap().y;
                            translate_x = click_start_translates.unwrap().0
                                + pxl_x as f32 / surface_extent.width as f32;
                            translate_y = click_start_translates.unwrap().1
                                + pxl_y as f32 / surface_extent.height as f32;
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        MouseScrollDelta::PixelDelta(position) => {
                            let double_per_position = 512.;
                            let ratio = position.y as f32 / double_per_position;
                            zoom = if ratio >= 0. {
                                zoom * (1. + ratio)
                            } else {
                                zoom / (1. - ratio)
                            };
                        }
                        MouseScrollDelta::LineDelta(_, line_count) => {
                            // Zoom 1.05x everytime scroll 1 line.
                            let line_zoom_factor = 1. + (line_count * 0.05);
                            zoom *= line_zoom_factor;
                        }
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == ElementState::Released {
                            if let Some(control) = control.as_mut() {
                                match input.virtual_keycode {
                                    Some(ch) => match ch {
                                        VirtualKeyCode::Q => {
                                            control.broadcast('q');
                                        }
                                        VirtualKeyCode::H => {
                                            control.broadcast('h');
                                        }
                                        VirtualKeyCode::Left => {
                                            control.broadcast('h');
                                        }
                                        VirtualKeyCode::J => {
                                            control.broadcast('j');
                                        }
                                        VirtualKeyCode::Down => {
                                            control.broadcast('j');
                                        }
                                        VirtualKeyCode::K => {
                                            control.broadcast('k');
                                        }
                                        VirtualKeyCode::Up => {
                                            control.broadcast('k');
                                        }
                                        VirtualKeyCode::L => {
                                            control.broadcast('l');
                                        }
                                        VirtualKeyCode::Right => {
                                            control.broadcast('l');
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
                        (board_shape.width() as u32, board_shape.height() as u32);

                    let lookup;
                    loop {
                        match states_read_only.try_read() {
                            Ok(val) => {
                                if cur_iter.is_none() || cur_iter.unwrap() != val.0 {
                                    lookup = val.1.clone();
                                    cur_iter = Some(val.0); // TODO: show current iteration.
                                    break;
                                }
                            }
                            Err(_) => continue,
                        }
                    }

                    let constants: Vec<((u32, u32), ColorRGBA)> = lookup
                        .par_iter()
                        .map(|(idx, state)| {
                            let color = visual_mapping.to_visual(&state);
                            let max_color = u16::MAX as f32;
                            let (x_min, y_min) = (
                                board_shape.x_idx_min() as i32,
                                board_shape.y_idx_min() as i32,
                            );
                            let ele_res: ((u32, u32), ColorRGBA) = (
                                (
                                    (idx.x.to_i32().unwrap() - x_min).to_u32().unwrap(),
                                    (idx.y.to_i32().unwrap() - y_min).to_u32().unwrap(),
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
                            .reset_fence(&mut res.submission_complete_fence)
                            .expect("Out of memory");

                        res.command_pool.reset(false);
                    }

                    let caps = res.surface.capabilities(&adapter.physical_device);
                    let mut swapchain_config =
                        SwapchainConfig::from_caps(&caps, surface_color_format, surface_extent);
                    let mut framebuffer_attachment = swapchain_config.framebuffer_attachment();
                    if should_configure_swapchain {
                        // This seems to fix some fullscreen slowdown on macOS.
                        if caps.image_count.contains(&3) {
                            swapchain_config.image_count = 3;
                        }

                        // Uncomment to allow resizing to other aspects:
                        surface_extent = swapchain_config.extent;
                        framebuffer_attachment = swapchain_config.framebuffer_attachment();

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
                                vec![framebuffer_attachment].into_iter(),
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

                    let render_attachment_info = RenderAttachmentInfo {
                        image_view: surface_image.borrow(),
                        clear_value: ClearValue {
                            color: ClearColor {
                                float32: [0.0, 0.0, 0.0, 1.0],
                            },
                        },
                    };
                    unsafe {
                        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

                        command_buffer.set_viewports(0, vec![viewport.clone()].into_iter());
                        command_buffer.set_scissors(0, vec![viewport.rect].into_iter());

                        command_buffer.begin_render_pass(
                            render_pass,
                            &framebuffer,
                            viewport.rect,
                            vec![render_attachment_info].into_iter(),
                            SubpassContents::Inline,
                        );

                        command_buffer.bind_graphics_pipeline(pipeline);

                        let shapes = if is_triangle {
                            create_triangles(
                                &surface_extent,
                                grid_width,
                                grid_height,
                                constants,
                                zoom,
                                translate_x,
                                translate_y,
                            )
                        } else {
                            create_squares(
                                &surface_extent,
                                grid_width,
                                grid_height,
                                constants,
                                zoom,
                                translate_x,
                                translate_y,
                            )
                        };
                        for shape in shapes.as_slice() {
                            // This encodes the actual push constants themselves
                            // into the command buffer. The vertex shader will be
                            // able to access these properties.
                            command_buffer.push_graphics_constants(
                                pipeline_layout,
                                ShaderStageFlags::VERTEX,
                                0,
                                push_constant_bytes(shape),
                            );

                            command_buffer.draw(0..if is_triangle { 3 } else { 4 }, 0..1);
                        }

                        command_buffer.end_render_pass();
                        command_buffer.finish();
                    }

                    unsafe {
                        let command_buffer = vec![&command_buffer].into_iter();
                        let wait_semaphores = vec![].into_iter();
                        let signal_semaphores = vec![&res.rendering_complete_semaphore].into_iter();

                        queue_group.queues[0].submit(
                            command_buffer,
                            wait_semaphores,
                            signal_semaphores,
                            Some(&mut res.submission_complete_fence),
                        );

                        let result = queue_group.queues[0].present(
                            &mut res.surface,
                            surface_image,
                            Some(&mut res.rendering_complete_semaphore),
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
    transform: [[f32; 4]; 4],
}

fn make_transform(dx: f32, dy: f32, scale_x: f32, scale_y: f32, angle: f32) -> [[f32; 4]; 4] {
    let (sin, cos) = (angle.sin(), angle.cos());
    let (dz, scale_z) = (0., 1.);

    [
        [scale_x * cos, -scale_x * sin, 0., 0.],
        [scale_y * sin, scale_y * cos, 0., 0.],
        [0., 0., scale_z, 0.],
        [dx, dy, dz, 1.],
    ]
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
    zoom: f32,
    dx: f32,
    dy: f32,
) -> Vec<PushConstants> {
    let (square_width, square_height) = (
        surface_extent.width as f32 / grid_width as f32,
        surface_extent.height as f32 / grid_height as f32,
    );
    let square_len = square_width.min(square_height);
    let (scale_x, scale_y) = (
        square_len / surface_extent.width as f32 * 2.0 * zoom,
        square_len / surface_extent.height as f32 * 2.0 * zoom,
    );

    let left_padding = 1.0 - scale_x / 2.0 * grid_width as f32;
    let top_padding = 1.0 - scale_y / 2.0 * grid_height as f32;

    let scale = cell_scale_for_gap(scale_x, scale_y);

    let mut res = Vec::new();
    for (idx, color) in states.iter() {
        let x_transform =
            -1.0 + left_padding + idx.0 as f32 * scale_x + (1.0 - scale) / 2.0 * scale_x + dx;
        let y_transform = -1.0
            + top_padding
            + (grid_height - idx.1 - 1) as f32 * scale_y
            + (1.0 - scale) / 2.0 * scale_y
            + dy;
        let rotation = 0.0;

        res.push(PushConstants {
            color: [color.r, color.g, color.b, color.a],
            transform: make_transform(
                x_transform,
                y_transform,
                scale * scale_x,
                scale * scale_y,
                rotation,
            ),
        });
    }
    res
}

fn create_triangles(
    surface_extent: &Extent2D,
    grid_width: u32,
    grid_height: u32,
    states: Vec<((u32, u32), ColorRGBA)>,
    zoom: f32,
    dx: f32,
    dy: f32,
) -> Vec<PushConstants> {
    const SIN_PI_3: f32 = 0.86602540378; // sqrt(3)/2
    let (triangle_width, triangle_height) = (
        surface_extent.width as f32 / grid_width as f32 * 2.0 / SIN_PI_3,
        surface_extent.height as f32 / grid_height as f32,
    );

    let triangle_width = triangle_width.min(triangle_height / SIN_PI_3);
    let triangle_height = triangle_height.min(triangle_width * SIN_PI_3);

    let (scale_x, scale_y) = (
        triangle_width / surface_extent.width as f32 * 2.0 * zoom,
        triangle_height / surface_extent.height as f32 * 2.0 * zoom,
    );

    let left_padding = 1.0 - scale_x / 4.0 * grid_width as f32;
    let top_padding = 1.0 - scale_y / 2.0 * grid_height as f32;

    let scale = cell_scale_for_gap(scale_x, scale_y);

    let mut res = Vec::new();
    for (idx, color) in states.iter() {
        let mut x_transform =
            -1.0 + left_padding + (idx.0 / 2) as f32 * scale_x + (1.0 - scale) / 2.0 * scale_x + dx;
        let mut y_transform = -1.0
            + top_padding
            + (grid_height - idx.1 - 1) as f32 * scale_y
            + (1.0 - scale) / 2.0 * scale_y
            + dy;
        let mut rotation = 0.0;

        let is_pointing_up = (idx.0 + idx.1) % 2 == 0;
        if !is_pointing_up {
            rotation = std::f32::consts::PI;
            x_transform += scale_x * scale;
            y_transform += scale_y * scale;
        }

        if idx.0 % 2 == 1 {
            let offset = scale_x * scale / 2.0 + (1.0 - scale) / 2.0 * scale_x;
            if idx.0 > 0 {
                x_transform += offset;
            } else {
                x_transform -= offset;
            }
        }

        res.push(PushConstants {
            color: [color.r, color.g, color.b, color.a],
            transform: make_transform(
                x_transform,
                y_transform,
                scale * scale_x,
                scale * scale_y,
                rotation,
            ),
        });
    }
    res
}

/// Dynamically calculate the gap between cells based on render zoom scale.
fn cell_scale_for_gap(scale_x: f32, scale_y: f32) -> f32 {
    let max_gap_scale = 0.9;
    let max_gap_len = 1. / 128.;
    let no_gap_len = 1. / 512.;

    let cell_len = scale_x.min(scale_y);
    if cell_len <= no_gap_len {
        1.
    } else if cell_len >= max_gap_len {
        max_gap_scale
    } else {
        let range_log = (max_gap_len / no_gap_len).ln() / 1.1f32.ln();
        let len_log = (cell_len / no_gap_len).ln() / 1.1f32.ln();
        let ratio = ((-3. + 6. * (len_log / range_log)).tanh() + 1.) / 2.;
        return max_gap_scale.max(1.0f32.min(max_gap_scale + (1. - max_gap_scale) * ratio));
    }
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
