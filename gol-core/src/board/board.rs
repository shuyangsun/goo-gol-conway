pub trait Board<T> {
    fn cell_state(&self, idx: usize) -> T;
    fn set_cell_state(&mut self, idx: usize, new_state: T);
    fn get_cell_neighbors(&self, idx: usize) -> Box<&dyn Iterator<Item = usize>>;

    fn advance(&mut self) {
        // TODO
    }
}
