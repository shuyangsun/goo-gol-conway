use crate::IndexedDataOwned;

pub trait EvolutionStrategy<CI, T, I>: Send + Sync
where
    I: Iterator<Item = IndexedDataOwned<CI, T>>,
{
    fn next_state(&self, idx: CI, cur_state: T, neighbors: I) -> T;
}
