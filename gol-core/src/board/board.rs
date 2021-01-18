use crate::{
    BoardCallbackManager, BoardNeighborManager, BoardSpaceManager, BoardStateManager,
    BoardStrategyManager, IndexedDataOwned,
};
use rayon::prelude::*;
use std::time::{Duration, Instant};
pub trait Board<T, CI, I>
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

    fn advance(&mut self, max_iter: Option<usize>, interval: Option<Duration>) {
        let mut cur_iter = 0usize;
        let state_manager = self.state_manager();
        let cur_states = self
            .space_manager()
            .indices_par_iter()
            .map(|idx| (idx.clone(), state_manager.get_cell_state(&idx)))
            .collect();

        let mut last_start = Instant::now();
        self.callback_manager().setup_all();
        self.callback_manager().call(cur_states);

        loop {
            let next_states = self.advance_one_generation();

            match interval {
                Some(val) => {
                    let now = Instant::now();
                    let time_elapsed = now - last_start;
                    if time_elapsed < val {
                        let sleep_dur = val - time_elapsed;
                        std::thread::sleep(sleep_dur);
                    }
                }
                None => (),
            };
            last_start = Instant::now();
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

    fn advance_one_generation(&mut self) -> Vec<IndexedDataOwned<CI, T>> {
        let states = self.state_manager();
        let strat = self.strategy_manager();
        let neighbor_manager = self.neighbor_manager();

        let next_states: Vec<IndexedDataOwned<CI, T>> = self
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

        self.state_manager_mut()
            .update_cell_states_from_par_iter(next_states.clone().into_par_iter());
        next_states
    }
}
