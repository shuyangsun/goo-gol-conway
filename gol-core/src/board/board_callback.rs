use crate::{CellIndex, CellState, IndexedDataOwned};
use futures::Future;
use rayon::prelude::*;
use std::cell::RefCell;
use std::pin::Pin;

type FutureVec = Vec<Pin<Box<dyn Future<Output = ()>>>>;

pub trait BoardCallback<T, CI, I>: Send + Sync
where
    T: Send + Sync,
    CI: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn callback(&self, states: I);
}

pub struct BoardCallbackManager<'callback, T, CI, I>
where
    T: Send + Sync,
    CI: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    callbacks: Vec<Box<&'callback dyn BoardCallback<T, CI, I>>>,
    states_cache: RefCell<Option<Vec<IndexedDataOwned<CI, T>>>>,
    futures_res: RefCell<FutureVec>,
}

impl<'callback, T, CI>
    BoardCallbackManager<'callback, T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>
where
    T: CellState,
    CI: CellIndex,
{
    pub fn new(
        callbacks: Vec<
            Box<&'callback dyn BoardCallback<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>>,
        >,
    ) -> Self {
        Self {
            callbacks,
            states_cache: RefCell::new(None),
            futures_res: RefCell::new(Vec::with_capacity(callbacks.len())),
        }
    }

    pub fn call(&self, next_states: Vec<IndexedDataOwned<CI, T>>) {
        *self.states_cache.borrow_mut() = Some(next_states);
        for callback_obj in self.callbacks {
            let cur_future = Box::pin(async {
                callback_obj.callback(self.states_cache.borrow().unwrap().clone().into_par_iter());
            });
            self.futures_res.borrow_mut().push(cur_future);
        }
    }

    pub fn block_until_finish(&self) {
        // TODO: implementation
        self.futures_res.borrow_mut().clear();
    }
}
