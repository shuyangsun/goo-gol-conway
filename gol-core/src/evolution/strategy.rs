pub trait EvolutionStrategy<T>: Send + Sync {
    fn next_state(
        &self,
        idx: usize,
        cur_state: T,
        neighbors: impl Iterator<Item = (usize, T)>,
    ) -> T;
}
