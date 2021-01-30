use crate::{EvolutionStrategy, IndexedDataOwned};
use num_traits::{FromPrimitive, PrimInt, Unsigned};
use std::collections::HashSet;

pub struct DiscreteDecayStrategy<T> {
    state_count: T,
    alive_surive_counts: HashSet<usize>,
    newborn_counts: HashSet<usize>,
}

impl<CI, T, I> EvolutionStrategy<CI, T, I> for DiscreteDecayStrategy<T>
where
    T: Send + Sync + PrimInt + Unsigned + FromPrimitive + std::ops::Sub<Output = T>,
    I: Iterator<Item = IndexedDataOwned<CI, T>>,
{
    fn next_state(&self, _: CI, cur_state: T, neighbors: I) -> T {
        let mut alive_count = 0;
        for (_, state) in neighbors {
            alive_count += if state <= T::zero() { 0 } else { 1 };
        }

        let is_alive = cur_state > T::zero();
        if is_alive && !self.alive_surive_counts.contains(&alive_count) {
            if cur_state > T::zero() {
                cur_state - T::one()
            } else {
                cur_state
            }
        } else if !is_alive && self.newborn_counts.contains(&alive_count) {
            self.state_count - T::one()
        } else {
            cur_state
        }
    }
}

impl<T> DiscreteDecayStrategy<T> {
    pub fn new(
        state_count: T,
        alive_surive_counts: HashSet<usize>,
        newborn_counts: HashSet<usize>,
    ) -> Self {
        Self {
            state_count,
            alive_surive_counts,
            newborn_counts,
        }
    }
}
