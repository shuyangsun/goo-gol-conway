pub trait CellState: Clone + Copy {}

impl<T> CellState for T where T: Clone + Copy {}
