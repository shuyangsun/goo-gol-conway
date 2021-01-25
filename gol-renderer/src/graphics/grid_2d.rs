pub struct GraphicalRendererGrid2D<M> {
    title: String,
    iter: usize,
    screen_dim: (i32, i32),
    grid_bounds: Option<(i32, i32, i32, i32)>,
    color_map: M,
}
