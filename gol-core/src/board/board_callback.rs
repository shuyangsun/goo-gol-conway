use crate::IndexedDataOwned;
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
    callbacks: Arc<Mutex<Vec<Box<dyn BoardCallback<T, CI, I>>>>>,
    callback_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl<T, CI> BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>
where
    T: Send + Sync + Clone,
    CI: Send + Sync + Clone,
{
    pub fn new(
        callbacks: Vec<
            Box<dyn BoardCallback<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
        >,
    ) -> Self {
        Self {
            callbacks: Arc::new(Mutex::new(callbacks)),
            callback_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn call(&self, next_states: Vec<IndexedDataOwned<CI, T>>) {
        self.block_until_finish();
        debug_assert!(self.callback_handle.lock().unwrap().is_none());

        let mut handle = self.callback_handle.lock().unwrap();
        let callbacks = Arc::clone(&self.callbacks);
        *handle = Some(thread::spawn(move || {
            Arc::clone(&callbacks)
                .lock()
                .unwrap()
                .par_iter()
                .for_each(|ele| ele.execute(next_states.clone().into_par_iter()));
        }));
    }

    fn block_until_finish(&self) {
        if let Some(handle) = self.callback_handle.lock().unwrap().take() {
            handle.join().unwrap();
        }
        let mut handle = self.callback_handle.lock().unwrap();
        *handle = None;
    }
}
