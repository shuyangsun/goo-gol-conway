use super::super::cell::common::IndexedDataOwned;
use super::super::evolution::strategy::EvolutionStrategy;
use rayon::prelude::*;

pub trait BoardStateManager<'data, 'dref, T, CI, I1, I2, I3>
where
    'data: 'dref,
    T: 'data + Send + Sync,
    CI: Send + Sync,
    I1: Iterator<Item = CI>,
    I2: Iterator<Item = IndexedDataOwned<CI, T>>,
    I3: ParallelIterator<Item = IndexedDataOwned<CI, T>>,
{
    fn get_cell_state(&self, idx: CI) -> &'dref T;
    fn update_cell_states_from_iter(&mut self, new_states: I2);
    fn update_cell_states_from_par_iter(&mut self, new_states: I3);
}

pub trait BoardSpaceManager<CI, I1, I2>
where
    CI: Send + Sync,
    I1: Iterator<Item = CI>,
    I2: ParallelIterator<Item = CI>,
{
    fn convert_from_linear_index(&self, linear_idx: usize) -> CI;
    fn convert_to_linear_index(&self, idx: CI) -> usize;

    fn indices_iter(&self) -> I1;
    fn get_neighbors_idx(&self, idx: CI) -> I1;
}

pub trait BoardStrategyManager<'data, 'dref, CI, T, I> {
    fn get_strategy_at_index(&self, idx: CI) -> &dyn EvolutionStrategy<'data, 'dref, CI, T, I>;
}

// pub trait Board<'a, 'b, T, I1, I2, I3>: Send + Sync
// where
//     'a: 'b,
//     T: 'a + Send + Sync + Clone,
//     I1: IndexedParallelIterator<Item = &'b T>,
//     I2: IndexedParallelIterator<Item = &'b mut T>,
//     I3: IndexedParallelIterator<Item = &'b dyn EvolutionStrategy<'a, T>>,
// {
//     fn data(&self) -> &dyn BoardData<'a, 'b, T, I1, I2, I3>;
//     fn advance(&mut self) {
//         let c_iter: I1 = self.data().cell_states_par_iter();
//         let strategy_iter: I3 = self.data().cell_strategy_par_iter_mut();
//         let new_states: Vec<T> = c_iter
//             .zip(strategy_iter)
//             .enumerate()
//             .map(
//                 |(idx, (state, strat)): (usize, (&'b T, &dyn EvolutionStrategy<T>))| {
//                     let neighbors = Box::new(self.data().get_cell_neighbor_states(idx));
//                     strat.next_state(idx, state, neighbors)
//                 },
//             )
//             .collect();
//
//         // TODO: call back and set states.
//
//         let c_iter_mut: I2 = self.data().cell_states_par_iter_mut();
//         c_iter_mut
//             .zip(new_states.par_iter())
//             .for_each(|(old_state, new_state)| *old_state = (*new_state).clone());
//     }
// }
