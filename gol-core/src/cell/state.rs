pub trait CellState: Clone + Copy + Send + Sync {}

impl<T> CellState for T where T: Clone + Copy + Send + Sync {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConwayState {
    Alive,
    Dead,
}
