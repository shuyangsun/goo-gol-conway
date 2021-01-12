use super::super::cell::cell::Cell;
use itertools::izip;

pub trait Board<T> {
    fn cell_state(&self, idx: usize) -> &T;
    fn set_cell_state(&mut self, idx: usize, new_state: T);
    fn get_cell_neighbors_states(&self, idx: usize) -> Box<dyn Iterator<Item = (usize, &T)>>;
    fn cell_iter(&self) -> Box<dyn Iterator<Item = &dyn Cell<T>>>;
    fn cell_iter_mut(&self) -> Box<dyn Iterator<Item = &mut dyn Cell<T>>>;

    fn advance(&mut self) {
        let mut new_states: Vec<T> = Vec::new();
        for (idx, cell) in self.cell_iter().enumerate() {
            let neighbors = self.get_cell_neighbors_states(idx);
            let new_state = cell
                .evolution_strategy()
                .next_state(&cell.state(), &*neighbors);
            new_states.push(new_state);
        }

        // TODO: call back and set states.

        for (cell, new_state) in izip!(self.cell_iter_mut(), new_states) {
            cell.set_state(new_state);
        }
    }
}
