use crate::IndexedDataOwned;
use rayon::iter::ParallelIterator;

pub trait BoardStateManager<T, CI, I>: Send + Sync
where
    T: Send + Sync,
    CI: Send + Sync,
    I: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn get_cell_state(&self, idx: &CI) -> T;
    fn update_cell_states_from_par_iter(&mut self, new_states: I);
}
