use super::super::cell::common::IndexedCellItem;

pub trait EvolutionStrategy<'data, 'dataref, T, I>: Send + Sync
where
    'data: 'dataref,
    T: 'data,
    I: Iterator<Item = IndexedCellItem<'dataref, T>>,
{
    fn next_state(&self, idx: usize, cur_state: &'dataref T, neighbors: I) -> T;
}
