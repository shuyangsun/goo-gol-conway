use crate::{CellIndex, CellState, IndexedDataOwned};
use rayon::prelude::*;
use tokio;

pub trait BoardCallback<T, CI, I>: Send + Sync
where
    T: Send + Sync,
    CI: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn execute(&self, states: I);
}

pub struct BoardCallbackManager<T, CI, I>
where
    T: Send + Sync,
    CI: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    callbacks: Vec<Box<dyn BoardCallback<T, CI, I>>>,
    states_cache: Option<Vec<IndexedDataOwned<CI, T>>>,
    futures_res: Vec<usize>,
}

impl<T, CI> BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>
where
    T: CellState,
    CI: CellIndex,
{
    pub fn new(
        callbacks: Vec<
            Box<dyn BoardCallback<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
        >,
    ) -> Self {
        Self {
            callbacks,
            states_cache: None,
            futures_res: Vec::new(),
        }
    }

    //     pub async fn call(&self, next_states: Vec<IndexedDataOwned<CI, T>>)
    //     where
    //         CI: 'static,
    //         T: 'static,
    //     {
    //         let asdf: tokio::task::JoinHandle<()> =
    //             tokio::spawn(async { self.callbacks[0].execute(next_states.into_par_iter()) });
    //     }
    //
    //     pub fn block_until_finish(&mut self) {
    //         // TODO: implementation
    //         self.futures_res = Vec::new();
    //     }
}
