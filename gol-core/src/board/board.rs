use crate::{
    BoardCallbackManager, BoardNeighborManager, BoardSpaceManager, BoardStateManager,
    BoardStrategyManager, IndexedDataOwned,
};
use rayon::prelude::*;
use std::time::{Duration, Instant};

pub trait Board<T, CI, I>
where
    T: Send + Sync,
    CI: Send + Sync,
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

    fn iter_count(&self) -> usize;

    fn callback_manager(
        &self,
    ) -> &BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>;
}

fn advance_impl<T, CI, I>(board: &mut dyn Board<T, CI, I>) -> Vec<IndexedDataOwned<CI, T>>
where
    T: Send + Sync + Clone,
    CI: Send + Sync + Clone,
    I: Iterator<Item = CI>,
{
    let states = board.state_manager();
    let strat = board.strategy_manager();
    let neighbor_manager = board.neighbor_manager();

    let next_states: Vec<IndexedDataOwned<CI, T>> = board
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
        .collect();

    board
        .state_manager_mut()
        .update_cell_states_from_par_iter(next_states.clone().into_par_iter());
    next_states
}

fn execute_callbacks_impl<'board, 'data, T, CI, I>(
    board: &'board mut dyn Board<T, CI, I>,
    next_states: Vec<IndexedDataOwned<CI, T>>,
) where
    'board: 'data,
    T: 'data + Send + Sync + Clone,
    CI: 'data + Send + Sync + Clone,
    I: Iterator<Item = CI>,
{
    board.callback_manager().call(next_states);
}
