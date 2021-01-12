pub trait EvolutionStrategy<T> {
    fn next_state(&self, cur_state: &T, neighbors: &dyn Iterator<Item = (usize, &T)>) -> T;
}
