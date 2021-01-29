use crate::{DiscreteState, EvolutionStrategy, IndexedDataOwned};
use num_traits::{FromPrimitive, PrimInt, Unsigned};
use std::collections::HashSet;

pub struct NeighborCountStrategy {
    alive_surive_counts: HashSet<usize>,
    newborn_counts: HashSet<usize>,
}

impl<CI, T, I, const N: u8> EvolutionStrategy<CI, DiscreteState<T, N>, I> for NeighborCountStrategy
where
    T: PrimInt + Unsigned + FromPrimitive + std::ops::Sub<Output = T>,
    I: Iterator<Item = IndexedDataOwned<CI, DiscreteState<T, N>>>,
{
    fn next_state(
        &self,
        _: CI,
        cur_state: DiscreteState<T, N>,
        neighbors: I,
    ) -> DiscreteState<T, N> {
        let mut alive_count = 0;
        for (_, state) in neighbors {
            alive_count += if state.val() <= &T::zero() { 0 } else { 1 };
        }

        let is_alive = cur_state.val() > &T::zero();
        if is_alive && !self.alive_surive_counts.contains(&alive_count) {
            cur_state.decay()
        } else if !is_alive && self.newborn_counts.contains(&alive_count) {
            DiscreteState::new()
        } else {
            cur_state
        }
    }
}

impl NeighborCountStrategy {
    pub fn new(alive_surive_counts: HashSet<usize>, newborn_counts: HashSet<usize>) -> Self {
        Self {
            alive_surive_counts,
            newborn_counts,
        }
    }
}
