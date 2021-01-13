pub trait EvolutionStrategy<T>: Send + Sync {
    fn next_state(
        &self,
        idx: usize,
        cur_state: &T,
        neighbors: &dyn Iterator<Item = (usize, &T)>,
    ) -> T;
}
