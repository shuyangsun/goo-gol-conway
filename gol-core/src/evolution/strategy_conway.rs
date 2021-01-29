use crate::{DiscreteState, EvolutionStrategy, IndexedDataOwned};
use num_traits::{PrimInt, Unsigned};

pub struct ConwayStrategy {}

impl<T, CI, I, const N: u8> EvolutionStrategy<CI, DiscreteState<T, N>, I> for ConwayStrategy
where
    T: PrimInt + Unsigned,
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
            alive_count += if state.val() > &T::zero() { 1 } else { 0 };
        }
        if alive_count == 3 || alive_count == 2 && cur_state.val() > &T::zero() {
            cur_state
        } else {
            cur_state.decay()
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
    use crate::{ConwayStrategy, DiscreteState, EvolutionStrategy};

    fn create_neighbors(alive_count: usize) -> Vec<DiscreteState<u8, 2>> {
        let mut res = vec![DiscreteState::<u8, 2>::new(); alive_count];
        res.append(
            &mut vec![DiscreteState::new(); 8 - alive_count]
                .iter()
                .map(|ele| ele.decay())
                .collect(),
        );
        res
    }

    #[test]
    fn conway_strat_test_0() {
        let strat = ConwayStrategy::new();
        let neighbors = create_neighbors(0);
        let neighbors_iter = neighbors.into_iter().enumerate();
        let alive_next = strat.next_state(0, DiscreteState::new(), neighbors_iter);
        assert_eq!(alive_next, DiscreteState::new().decay());
    }
}
