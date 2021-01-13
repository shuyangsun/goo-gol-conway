pub trait CellIndex: Clone + Copy + Send + Sync {}
pub type IndexedCellItem<'dref, T, CI> = (CI, &'dref T);
