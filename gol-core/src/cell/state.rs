pub trait CellState: Clone + Send + Sync {}

impl<T> CellState for T where T: Clone + Send + Sync {}
