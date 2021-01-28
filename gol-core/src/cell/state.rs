#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellState<T> {
    val: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConwayState {
    Alive,
    Dead,
}

impl<T> CellState<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }

    pub fn val(&self) -> &T {
        &self.val
    }
}
