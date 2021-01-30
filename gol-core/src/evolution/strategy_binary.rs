use crate::{BinaryState, EvolutionStrategy, IndexedDataOwned};
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct BinaryStrategy {
    alive_surive_counts: HashSet<usize>,
    newborn_counts: HashSet<usize>,
}

impl<CI, I> EvolutionStrategy<CI, BinaryState, I> for BinaryStrategy
where
    I: Iterator<Item = IndexedDataOwned<CI, BinaryState>>,
{
    fn next_state(&self, _: CI, cur_state: BinaryState, neighbors: I) -> BinaryState {
        let mut alive_count = 0;
        for (_, state) in neighbors {
            alive_count += match state {
                BinaryState::Alive => 1,
                BinaryState::Dead => 0,
            };
        }
        if cur_state == BinaryState::Alive && self.alive_surive_counts.contains(&alive_count)
            || cur_state == BinaryState::Dead && self.newborn_counts.contains(&alive_count)
        {
            BinaryState::Alive
        } else {
            BinaryState::Dead
        }
    }
}

impl BinaryStrategy {
    pub fn new(alive_survive: HashSet<usize>, newborn: HashSet<usize>) -> Self {
        Self {
            alive_surive_counts: alive_survive,
            newborn_counts: newborn,
        }
    }

    pub fn conway() -> Self {
        let (alive, newborn) = (
            HashSet::from_iter([2, 3].iter().cloned()),
            HashSet::from_iter([3].iter().cloned()),
        );
        Self::new(alive, newborn)
    }
}

#[cfg(test)]
mod binary_strategy_test {
    use crate::{BinaryState, BinaryStrategy, EvolutionStrategy};

    fn create_neighbors(alive_count: usize) -> Vec<BinaryState> {
        let mut res = vec![BinaryState::Alive; alive_count];
        res.append(&mut vec![BinaryState::Dead; 8 - alive_count]);
        res
    }

    #[test]
    fn conway_strat_test_0() {
        let strat = BinaryStrategy::conway();
        let neighbors = create_neighbors(0);
        let neighbors_iter = neighbors.into_iter().enumerate();
        let alive_next = strat.next_state(0, BinaryState::Alive, neighbors_iter);
        assert_eq!(alive_next, BinaryState::Dead);
    }
}
