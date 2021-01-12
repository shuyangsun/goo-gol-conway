pub trait EvolutionStrategy<T> {
    fn next_state(
        &self,
        idx: usize,
        cur_state: &T,
        neighbors: &dyn Iterator<Item = (usize, &T)>,
    ) -> T;
}
