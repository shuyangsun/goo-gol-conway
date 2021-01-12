use super::super::cell::cell::Cell;
use rayon::prelude::*;

pub trait Board<T>: Send + Sync
where
    T: Send + Sync + Clone,
{
    fn cell_state(&self, idx: usize) -> &T;

    fn set_cell_state(&mut self, idx: usize, new_state: T);

    fn get_cell_neighbor_states<'a, 'b, U>(&self, idx: usize) -> U
    where
        'a: 'b,
        T: 'a,
        U: Iterator<Item = (usize, &'b T)>;

    fn cell_iter<'a, 'b, U>(&self) -> U
    where
        'a: 'b,
        T: 'a,
        U: IndexedParallelIterator<Item = &'b dyn Cell<T>>;

    fn cell_iter_mut<'a, 'b, U>(&self) -> U
    where
        'a: 'b,
        T: 'a,
        U: IndexedParallelIterator<Item = &'b mut dyn Cell<T>>;

    fn advance<'a, 'b, U, V, W>(&mut self)
    where
        'a: 'b,
        T: 'a,
        U: IndexedParallelIterator<Item = &'b dyn Cell<T>>,
        V: IndexedParallelIterator<Item = &'b mut dyn Cell<T>>,
        W: Iterator<Item = (usize, &'b T)>,
    {
        let c_iter: U = self.cell_iter();
        let new_states: Vec<T> = c_iter
            .enumerate()
            .map(|(idx, cell)| {
                let neighbors: W = self.get_cell_neighbor_states(idx);
                cell.evolution_strategy()
                    .next_state(idx, &cell.state(), &neighbors)
            })
            .collect();

        // TODO: call back and set states.

        let c_iter_mut: V = self.cell_iter_mut();
        c_iter_mut
            .zip(new_states.par_iter())
            .for_each(|(cell, new_state)| cell.set_state((*new_state).clone()));
    }
}
