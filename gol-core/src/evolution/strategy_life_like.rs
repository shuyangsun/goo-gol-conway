use crate::{EvolutionStrategy, IndexedDataOwned};
use num_traits::{FromPrimitive, PrimInt, Unsigned};
use std::collections::HashSet;

pub struct DecayLifeLikeStrategy {
    state_count: usize,
    alive_surive_counts: HashSet<usize>,
    newborn_counts: HashSet<usize>,
}

impl<CI, T, I> EvolutionStrategy<CI, T, I> for DecayLifeLikeStrategy
where
    T: PrimInt + Unsigned + FromPrimitive + std::ops::Sub<Output = T>,
    I: Iterator<Item = IndexedDataOwned<CI, T>>,
{
    fn next_state(&self, _: CI, cur_state: T, neighbors: I) -> T {
        let mut alive_count = 0;
        for (_, state) in neighbors {
            alive_count += if state == T::from_usize(self.state_count - 1).unwrap() {
                1
            } else {
                0
            };
        }

        let is_alive = cur_state == T::from_usize(self.state_count - 1).unwrap();
        let is_zero = cur_state == T::zero();
        if !is_alive && !is_zero || is_alive && !self.alive_surive_counts.contains(&alive_count) {
            cur_state - T::one()
        } else if is_zero && self.newborn_counts.contains(&alive_count) {
            T::from_usize(self.state_count).unwrap() - T::one()
        } else {
            cur_state
        }
    }
}

impl DecayLifeLikeStrategy {
    pub fn new(
        state_count: usize,
        alive_surive_counts: HashSet<usize>,
        newborn_counts: HashSet<usize>,
    ) -> Self {
        Self {
            state_count,
            alive_surive_counts,
            newborn_counts,
        }
    }

    pub fn gol() -> Self {
        let survive: HashSet<usize> = vec![2, 3].into_iter().collect();
        let born: HashSet<usize> = vec![3].into_iter().collect();
        Self::new(2, survive, born)
    }
}
