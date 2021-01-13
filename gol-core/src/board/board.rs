// use super::super::evolution::strategy::EvolutionStrategy;
// use rayon::prelude::*;
//
// pub trait BoardData<'a, 'b, T, I1, I2, I3>
// where
//     T: 'a + Send + Sync,
//     'a: 'b,
//     I1: IndexedParallelIterator<Item = &'b T>,
//     I2: IndexedParallelIterator<Item = &'b mut T>,
//     I3: IndexedParallelIterator<Item = &'b dyn EvolutionStrategy<T>>,
// {
//     fn cell_state(&self, idx: usize) -> &T;
//     fn set_cell_state(&mut self, idx: usize, new_state: T);
//     fn get_cell_neighbor_states(&self, idx: usize) -> Box<dyn Iterator<Item = (usize, &T)>>;
//
//     fn cell_states_par_iter(&self) -> I1;
//     fn cell_states_par_iter_mut(&self) -> I2;
//
//     fn cell_strategy_par_iter_mut(&self) -> I3;
// }
//
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
