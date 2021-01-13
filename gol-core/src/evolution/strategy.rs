use super::super::cell::common::{CellIdx, IndexedCellItem};

pub trait EvolutionStrategy<'data, 'dataref, T, I>: Send + Sync
where
    'data: 'dataref,
    T: 'data,
    I: Iterator<Item = IndexedCellItem<'dataref, T>>,
{
    fn next_state(&self, idx: CellIdx, cur_state: &'dataref T, neighbors: I) -> T;
}
