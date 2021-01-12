pub trait CellState: Copy {}

impl<T> CellState for T where T: Clone + Copy {}
