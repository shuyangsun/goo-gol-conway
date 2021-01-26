use crate::{util::grid_util::find_2d_bounds, ColorMapping};
use tokio::sync::broadcast::{error::TryRecvError, Receiver};

pub struct GraphicalRendererGrid2D<M> {
    title: String,
    iter: usize,
    screen_dim: (u32, u32),
    grid_bounds: Option<(u32, u32, u32, u32)>,
    rx: Option<Receiver<char>>,
    color_map: M,
}

impl<M> GraphicalRendererGrid2D<M> {
    pub fn new(color_map: M) -> Self {
        Self::new_with_title(color_map, String::from(""))
    }

    pub fn new_with_title(color_map: M, title: String) -> Self {
        Self {
            title,
            iter: 0,
            screen_dim: (0, 0),
            grid_bounds: None,
            rx: None,
            color_map,
        }
    }

    pub fn new_with_title_and_ch_receiver(
        color_map: M,
        title: String,
        receiver: Receiver<char>,
    ) -> Self {
        Self {
            title,
            iter: 0,
            screen_dim: (0, 0),
            grid_bounds: None,
            rx: Some(receiver),
            color_map,
        }
    }
}
