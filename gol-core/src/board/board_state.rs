use crate::IndexedDataOwned;
use rayon::iter::ParallelIterator;

pub trait BoardStateManager<'data, 'dref, T, CI, I>: Send + Sync
where
    'data: 'dref,
    T: 'data + Send + Sync,
    CI: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn get_cell_state(&self, idx: CI) -> &'dref T;
    fn update_cell_states_from_par_iter(&mut self, new_states: I);
}
