use crate::{BinaryState, EvolutionStrategy, IndexedDataOwned};

pub struct BinaryStrategy {}

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
        if alive_count == 3 || alive_count == 2 && cur_state == BinaryState::Alive {
            BinaryState::Alive
        } else {
            BinaryState::Dead
        }
    }
}

impl BinaryStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod conway_strategy_test {
    use crate::{BinaryState, BinaryStrategy, EvolutionStrategy};

    fn create_neighbors(alive_count: usize) -> Vec<BinaryState> {
        let mut res = vec![BinaryState::Alive; alive_count];
        res.append(&mut vec![BinaryState::Dead; 8 - alive_count]);
        res
    }

    #[test]
    fn conway_strat_test_0() {
        let strat = BinaryStrategy::new();
        let neighbors = create_neighbors(0);
        let neighbors_iter = neighbors.into_iter().enumerate();
        let alive_next = strat.next_state(0, BinaryState::Alive, neighbors_iter);
        assert_eq!(alive_next, BinaryState::Dead);
    }
}
