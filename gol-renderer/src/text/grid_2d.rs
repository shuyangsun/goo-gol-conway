use gol_core::{BoardCallback, GridPoint2D, IndexedDataOwned};
use rayon::prelude::*;

pub struct TextRendererGrid2D {
    has_cell_boarder: bool,
}

impl<T, U, I> BoardCallback<T, GridPoint2D<U>, I> for TextRendererGrid2D
where
    T: Send + Sync + Clone + std::convert::Into<char>,
    U: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<GridPoint2D<U>, T>>,
{
    fn execute(&self, states: I) {
        let states: Vec<IndexedDataOwned<GridPoint2D<U>, T>> = states.collect();
        println!("first state: {}", states.first().unwrap().1.clone().into());
    }
}

impl TextRendererGrid2D {
    pub fn new(has_cell_boarder: bool) -> Self {
        Self { has_cell_boarder }
    }
}
