use super::super::cell::common::IndexedDataRef;

pub trait EvolutionStrategy<'data, 'dref, CI, T, I>: Send + Sync
where
    'data: 'dref,
    T: 'data,
    I: Iterator<Item = IndexedDataRef<'dref, CI, T>>,
{
    fn next_state(&self, idx: CI, cur_state: &'dref T, neighbors: I) -> T;
}
