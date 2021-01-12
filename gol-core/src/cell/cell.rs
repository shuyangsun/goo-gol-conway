use super::state::CellState;
use std::iter::Iterator;

pub trait Cell<T>
where
    T: CellState,
{
    fn state(&self) -> &T;
    fn set_state(&mut self, new_state: T);
    fn next_state(&self, neighbors: &dyn Iterator<Item = (usize, T)>) -> T;
}
