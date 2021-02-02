use crate::{BoardStrategyManager, EvolutionStrategy, IndexedDataOwned};

pub struct SharedStrategyManager<CI, T, I>
where
    I: Iterator<Item = IndexedDataOwned<CI, T>>,
{
    strategy: Box<dyn EvolutionStrategy<CI, T, I>>,
}

impl<CI, T, I> BoardStrategyManager<CI, T, I> for SharedStrategyManager<CI, T, I>
where
    I: Iterator<Item = IndexedDataOwned<CI, T>>,
{
    fn get_strategy_at_index(&self, _: CI) -> &dyn EvolutionStrategy<CI, T, I> {
        &*self.strategy
    }
}

impl<CI, T, I> SharedStrategyManager<CI, T, I>
where
    I: Iterator<Item = IndexedDataOwned<CI, T>>,
{
    pub fn new(strategy: Box<dyn EvolutionStrategy<CI, T, I>>) -> Self {
        Self { strategy }
    }
}

#[cfg(test)]
mod shared_strat_manager_test {
    use crate::{
        BoardStrategyManager, DecayLifeLikeStrategy, IndexedDataOwned, SharedStrategyManager,
    };

    #[test]
    fn shared_strat_test_1() {
        let strat = Box::new(DecayLifeLikeStrategy::gol());
        let strat_manager =
            SharedStrategyManager::<i32, u8, std::vec::IntoIter<IndexedDataOwned<i32, u8>>>::new(
                strat,
            );
        let _ = strat_manager.get_strategy_at_index(0);
    }
}
