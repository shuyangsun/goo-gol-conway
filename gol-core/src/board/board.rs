use crate::{
    BoardCallbackManager, BoardNeighborManager, BoardSpaceManager, BoardStateManager,
    BoardStrategyManager, IndexedDataOwned,
};

use rayon::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;

pub trait Board<T, CI, I>: Send + Sync
where
    T: 'static + Send + Sync + Clone,
    CI: 'static + Send + Sync + Clone,
    I: Iterator<Item = CI>,
{
    fn space_manager(&self) -> &dyn BoardSpaceManager<CI, I, rayon::vec::IntoIter<CI>>;
    fn neighbor_manager(&self) -> &dyn BoardNeighborManager<CI, I>;

    fn state_manager(
        &self,
    ) -> &dyn BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>;

    fn state_manager_mut(
        &mut self,
    ) -> &mut dyn BoardStateManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>;

    fn strategy_manager(
        &self,
    ) -> &dyn BoardStrategyManager<CI, T, std::vec::IntoIter<IndexedDataOwned<CI, T>>>;

    fn callback_manager(
        &mut self,
    ) -> &mut BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>;

    fn advance(&mut self, max_iter: Option<usize>)
    where
        CI: Eq + Hash,
        T: Eq + Hash,
    {
        let mut cur_iter = 0usize;
        let state_manager = self.state_manager();
        let cur_states = match self.state_manager().get_non_trivial_states() {
            Ok(non_trivial) => non_trivial.collect(),
            Err(_) => self
                .space_manager()
                .indices_par_iter()
                .map(|idx| (idx.clone(), state_manager.get_cell_state(&idx)))
                .collect(),
        };

        self.callback_manager().setup_all();
        self.callback_manager().call(cur_states);

        loop {
            let next_states = self.advance_one_generation();

            self.callback_manager().call(next_states);

            cur_iter += 1;
            match max_iter {
                Some(val) => {
                    if cur_iter >= val {
                        break;
                    }
                }
                None => continue,
            }
        }
        self.callback_manager().cleanup_all();
    }

    fn advance_one_generation(&mut self) -> Vec<IndexedDataOwned<CI, T>>
    where
        CI: Eq + Hash,
        T: Eq + Hash,
    {
        let states = self.state_manager();
        let strat = self.strategy_manager();
        let neighbor_manager = self.neighbor_manager();

        let next_states: Vec<IndexedDataOwned<CI, T>> =
            match self.state_manager().get_non_trivial_states() {
                Ok(non_trivial) => {
                    let non_trivial: HashSet<IndexedDataOwned<CI, T>> = non_trivial.collect();
                    let neighbors: HashSet<IndexedDataOwned<CI, T>> = non_trivial
                        .par_iter()
                        .map(|ele| {
                            neighbor_manager
                                .get_neighbors_idx(&ele.0)
                                .map(|neighbor_idx| {
                                    (ele.0.clone(), states.get_cell_state(&neighbor_idx))
                                })
                                .collect()
                        })
                        .reduce(
                            || HashSet::new(),
                            |a, b| {
                                let res: HashSet<IndexedDataOwned<CI, T>> = a.union(&b).collect();
                                res
                            },
                        );
                    let non_trivial_and_neighbors = non_trivial.union(&neighbors).collect();
                    non_trivial_and_neighbors
                        .par_iter()
                        .map(|ele| {
                            let neighbors: Vec<IndexedDataOwned<CI, T>> = neighbor_manager
                                .get_neighbors_idx(&ele.0)
                                .map(|neighbor_idx| {
                                    (ele.0.clone(), states.get_cell_state(&neighbor_idx))
                                })
                                .collect();
                            (
                                ele.0.clone(),
                                strat.get_strategy_at_index(ele.0.clone()).next_state(
                                    ele.0,
                                    ele.1,
                                    neighbors.into_iter(),
                                ),
                            )
                        })
                        .collect()
                }
                Err(_) => self
                    .space_manager()
                    .indices_par_iter()
                    .map(|idx| {
                        let cur_state = states.get_cell_state(&idx.clone());
                        let neighbors: Vec<IndexedDataOwned<CI, T>> = neighbor_manager
                            .get_neighbors_idx(&idx)
                            .map(|neighbor_idx| (idx.clone(), states.get_cell_state(&neighbor_idx)))
                            .collect();
                        (
                            idx.clone(),
                            strat.get_strategy_at_index(idx.clone()).next_state(
                                idx,
                                cur_state,
                                neighbors.into_iter(),
                            ),
                        )
                    })
                    .collect(),
            };

        self.state_manager_mut()
            .update_cell_states_from_par_iter(next_states.clone().into_par_iter());
        next_states
    }
}
