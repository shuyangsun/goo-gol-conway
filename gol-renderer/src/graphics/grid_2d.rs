use crate::{util::grid_util::find_2d_bounds, ColorMapping};
use gol_core::{BoardCallback, GridPoint2D, IndexedDataOwned};
use num_traits::{CheckedSub, FromPrimitive, ToPrimitive};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{error::TryRecvError, Receiver};

pub struct GraphicalRendererGrid2D<M, I> {
    title: String,
    iter: usize,
    screen_dim: (u32, u32),
    grid_bounds: Option<(u32, u32, u32, u32)>,
    rx: Option<Receiver<char>>,
    color_map: M,
    cur_states: Arc<Mutex<Option<I>>>,
}

impl<T, U, I, M> BoardCallback<T, GridPoint2D<U>, I>
    for GraphicalRendererGrid2D<M, rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<U>, T>>>
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone + Ord + CheckedSub + ToPrimitive + FromPrimitive,
    I: ParallelIterator<Item = IndexedDataOwned<GridPoint2D<U>, T>>,
    M: Send + Sync + ColorMapping<T>,
{
    fn setup(&mut self) {}
    fn cleanup(&mut self) {}

    fn execute(&mut self, states: I) {
        let states_vec = states.collect();
        if self.grid_bounds.is_none() {
            let grid_bounds = find_2d_bounds(&states_vec);
            self.grid_bounds = Some((
                grid_bounds.0.to_u32().unwrap(),
                grid_bounds.1.to_u32().unwrap(),
                grid_bounds.2.to_u32().unwrap(),
                grid_bounds.3.to_u32().unwrap(),
            ));
        }
        *self.cur_states.lock().unwrap() = Some(states_vec.into_par_iter());
    }
}

impl<M, I> GraphicalRendererGrid2D<M, I> {
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
            cur_states: Arc::new(Mutex::new(None)),
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
            cur_states: Arc::new(Mutex::new(None)),
        }
    }
}
