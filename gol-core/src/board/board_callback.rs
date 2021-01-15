use crate::{CellIndex, CellState, IndexedDataOwned};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

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
    callbacks: Vec<Arc<Mutex<Box<dyn BoardCallback<T, CI, I>>>>>,
    states_cache: Option<Vec<IndexedDataOwned<CI, T>>>,
    callback_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl<T, CI> BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>
where
    T: 'static + CellState,
    CI: 'static + CellIndex,
{
    pub fn new(
        callbacks: Vec<
            Box<dyn BoardCallback<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
        >,
    ) -> Self {
        Self {
            callbacks: callbacks
                .into_iter()
                .map(|ele| Arc::new(Mutex::new(ele)))
                .collect(),
            states_cache: None,
            callback_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn call(&self, next_states: Vec<IndexedDataOwned<CI, T>>) {
        self.block_until_finish();
        debug_assert!(self.callback_handle.lock().unwrap().is_none());

        let callbacks: Vec<
            Arc<
                Mutex<Box<dyn BoardCallback<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>>,
            >,
        > = self.callbacks.iter().map(|ele| Arc::clone(&ele)).collect();

        let mut handle = self.callback_handle.lock().unwrap();
        *handle = Some(thread::spawn(|| {
            callbacks.par_iter().for_each(|ele| {
                ele.lock()
                    .unwrap()
                    .execute(next_states.clone().into_par_iter())
            });
        }));
    }

    pub fn block_until_finish(&self) {
        self.callback_handle
            .lock()
            .unwrap()
            .unwrap()
            .join()
            .unwrap();
        let mut handle = self.callback_handle.lock().unwrap();
        *handle = None;
    }
}
