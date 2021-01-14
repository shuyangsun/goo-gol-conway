use crate::IndexedDataOwned;
use rayon::prelude::*;

pub trait BoardCallbackManager<T, CI, I>: Send + Sync
where
    T: Send + Sync,
    CI: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn callback(&self, states: I);
}
