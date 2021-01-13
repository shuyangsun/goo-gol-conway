pub trait CellIndex: Clone + Copy + Send + Sync {}
pub type IndexedCellItem<'dref, T, CI> = (CI, &'dref T);

impl<T> CellIndex for T where T: Clone + Copy + Send + Sync {}
