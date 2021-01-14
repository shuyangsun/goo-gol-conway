use super::super::cell::common::{CellIndex, CellState, IndexedDataOwned, IndexedDataRef};
use super::super::evolution::strategy::EvolutionStrategy;
use rayon::prelude::*;

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

pub trait BoardSpaceManager<CI, I1, I2>: Send + Sync
where
    CI: Send + Sync,
    I1: Iterator<Item = CI>,
    I2: ParallelIterator<Item = CI>,
{
    fn indices_iter(&self) -> I1;
    fn indices_par_iter(&self) -> I2;
    fn get_neighbors_idx(&self, idx: CI) -> I1;
}

pub trait BoardStrategyManager<'data, 'dref, CI, T, I>: Send + Sync {
    fn get_strategy_at_index(&self, idx: CI) -> &dyn EvolutionStrategy<'data, 'dref, CI, T, I>;
}

pub trait Board<'data, 'dref, T, CI, I>
where
    'data: 'dref,
    T: 'data + CellState,
    CI: CellIndex,
    I: Iterator<Item = CI>,
{
    fn state_manager(
        &self,
    ) -> &dyn BoardStateManager<'data, 'dref, T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>;

    fn state_manager_mut(
        &mut self,
    ) -> &mut dyn BoardStateManager<
        'data,
        'dref,
        T,
        CI,
        rayon::vec::IntoIter<IndexedDataOwned<CI, T>>,
    >;

    fn space_manager(&self) -> &dyn BoardSpaceManager<CI, I, rayon::vec::IntoIter<CI>>;

    fn strategy_manager(
        &self,
    ) -> &dyn BoardStrategyManager<
        'data,
        'dref,
        CI,
        T,
        std::vec::IntoIter<IndexedDataRef<'dref, CI, T>>,
    >;

    fn iter_count(&self) -> usize;

    fn advance(&mut self) {
        let states = self.state_manager();
        let strat = self.strategy_manager();
        let space = self.space_manager();
        let next_states: Vec<IndexedDataOwned<CI, T>> = self
            .space_manager()
            .indices_par_iter()
            .map(|idx| {
                let cur_state = states.get_cell_state(idx);
                let neighbors: Vec<IndexedDataRef<'dref, CI, T>> = space
                    .get_neighbors_idx(idx)
                    .map(|neighbor_idx| (idx, states.get_cell_state(neighbor_idx)))
                    .collect();
                (
                    idx,
                    strat.get_strategy_at_index(idx).next_state(
                        idx,
                        cur_state,
                        neighbors.into_iter(),
                    ),
                )
            })
            .collect();

        // TODO: Drawing and callbacks.

        self.state_manager_mut()
            .update_cell_states_from_par_iter(next_states.into_par_iter());
    }
}
