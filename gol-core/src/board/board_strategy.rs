use crate::EvolutionStrategy;

pub trait BoardStrategyManager<CI, T, I>: Send + Sync {
    fn get_strategy_at_index(&self, idx: CI) -> &dyn EvolutionStrategy<CI, T, I>;
}
