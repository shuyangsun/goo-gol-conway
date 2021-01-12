pub trait Cell<T> {
    fn state(&self) -> &T;
    fn set_state(&mut self, new_state: T);
    fn next_state(&self, neighbors: &dyn Iterator<Item = (usize, T)>) -> T;
}
