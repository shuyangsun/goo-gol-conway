use super::super::evolution::strategy::EvolutionStrategy;

pub trait Cell<T> {
    fn new_from_state(state: T) -> Self
    where
        Self: Sized;
    fn state(&self) -> &T;
    fn set_state(&mut self, new_state: T);
    fn evolution_strategy(&self) -> &dyn EvolutionStrategy<T>;
}
