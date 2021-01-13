use super::super::cell::common::IndexedCellItem;

pub trait EvolutionStrategy<'data, 'dataref, T, CI, I>: Send + Sync
where
    'data: 'dataref,
    T: 'data,
    I: Iterator<Item = IndexedCellItem<'dataref, T, CI>>,
{
    fn next_state(&self, idx: CI, cur_state: &'dataref T, neighbors: I) -> T;
}
