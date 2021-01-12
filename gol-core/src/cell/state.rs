pub trait CellState: Clone {}

impl<T> CellState for T where T: Clone {}
