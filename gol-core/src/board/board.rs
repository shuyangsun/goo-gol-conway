use crate::{
    BoardCallbackManager, BoardNeighborManager, BoardSpaceManager, BoardStateManager,
    BoardStrategyManager, CellIndex, CellState, IndexedDataOwned, IndexedDataRef,
};
use rayon::prelude::*;

pub trait Board<'data, 'dref, T, CI, I>
where
    'data: 'dref,
    T: 'static + CellState,
    CI: 'static + CellIndex,
    I: Iterator<Item = CI>,
{
    fn space_manager(&self) -> &dyn BoardSpaceManager<CI, I, rayon::vec::IntoIter<CI>>;
    fn neighbor_manager(&self) -> &dyn BoardNeighborManager<CI, I>;

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

    fn callback_manager(
        &self,
    ) -> &BoardCallbackManager<T, CI, rayon::vec::IntoIter<IndexedDataOwned<CI, T>>>;

    fn advance(&mut self)
    where
        T: 'data,
        CI: 'data,
    {
        let states = self.state_manager();
        let strat = self.strategy_manager();
        let space = self.space_manager();
        let neighbor_manager = self.neighbor_manager();
        let next_states: Vec<IndexedDataOwned<CI, T>> = self
            .space_manager()
            .indices_par_iter()
            .map(|idx| {
                let cur_state = states.get_cell_state(idx);
                let neighbors: Vec<IndexedDataRef<'dref, CI, T>> = neighbor_manager
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

        self.state_manager_mut()
            .update_cell_states_from_par_iter(next_states.clone().into_par_iter());

        self.callback_manager().block_until_finish();
        self.callback_manager().call(next_states);
    }
}
