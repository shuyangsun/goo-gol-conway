use crate::{CellState, EvolutionStrategy, IndexedDataOwned};
use num_traits::{FromPrimitive, PrimInt, Unsigned};
use std::collections::HashSet;

pub struct NeighborCountStrategy {
    num_states: usize,
    alive_surive_counts: HashSet<usize>,
    newborn_counts: HashSet<usize>,
}

impl<CI, T, I> EvolutionStrategy<CI, CellState<T>, I> for NeighborCountStrategy
where
    T: PrimInt + Unsigned + FromPrimitive + std::ops::Sub<Output = T>,
    I: Iterator<Item = IndexedDataOwned<CI, CellState<T>>>,
{
    fn next_state(&self, _: CI, cur_state: CellState<T>, neighbors: I) -> CellState<T> {
        let mut alive_count = 0;
        for (_, state) in neighbors {
            alive_count += if state.val() <= &T::zero() { 0 } else { 1 };
        }

        let is_alive = cur_state.val() > &T::zero();
        if is_alive && !self.alive_surive_counts.contains(&alive_count) {
            CellState::new(*cur_state.val() - T::one())
        } else if !is_alive && self.newborn_counts.contains(&alive_count) {
            CellState::new(T::from_usize(self.num_states - 1).unwrap())
        } else {
            cur_state
        }
    }
}

impl NeighborCountStrategy {
    pub fn new(
        num_states: usize,
        alive_surive_counts: HashSet<usize>,
        newborn_counts: HashSet<usize>,
    ) -> Self {
        Self {
            num_states,
            alive_surive_counts,
            newborn_counts,
        }
    }
}
