use super::super::cell::common::IndexedDataRef;
use super::super::cell::state::ConwayState;
use super::strategy::EvolutionStrategy;

pub struct ConwayStrategy {}

impl<'data, 'dataref, CI, I> EvolutionStrategy<'data, 'dataref, CI, ConwayState, I>
    for ConwayStrategy
where
    'data: 'dataref,
    ConwayState: 'data,
    I: Iterator<Item = IndexedDataRef<'dataref, CI, ConwayState>>,
{
    fn next_state(&self, _: CI, cur_state: &'dataref ConwayState, neighbors: I) -> ConwayState {
        let mut alive_count = 0;
        for (_, state) in neighbors {
            alive_count += match state {
                ConwayState::Alive => 1,
                ConwayState::Dead => 0,
            };
        }
        if alive_count == 3 || alive_count == 2 && *cur_state == ConwayState::Alive {
            ConwayState::Alive
        } else {
            ConwayState::Dead
        }
    }
}

impl ConwayStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod conway_strategy_test {
    use super::super::super::cell::state::ConwayState;
    use super::ConwayStrategy;
    use super::EvolutionStrategy;

    fn create_neighbors(alive_count: usize) -> Vec<ConwayState> {
        let mut res = vec![ConwayState::Alive; alive_count];
        res.append(&mut vec![ConwayState::Dead; 8 - alive_count]);
        res
    }

    #[test]
    fn conway_strat_test_0() {
        let strat = ConwayStrategy::new();
        let neighbors = create_neighbors(0);
        let neighbors_iter = neighbors.iter().enumerate();
        let alive_next = strat.next_state(0, &ConwayState::Alive, neighbors_iter);
        assert_eq!(alive_next, ConwayState::Dead);
    }
}
