use crate::EvolutionStrategy;

pub trait BoardStrategyManager<'data, 'dref, CI, T, I>: Send + Sync {
    fn get_strategy_at_index(&self, idx: CI) -> &dyn EvolutionStrategy<'data, 'dref, CI, T, I>;
}
