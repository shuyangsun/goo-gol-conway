pub trait CellState: Clone + Send + Sync {}
pub trait CellIndex: Clone + Copy + Send + Sync {}

pub type IndexedDataRef<'dref, CI, T> = (CI, &'dref T);
pub type IndexedDataOwned<CI, T> = (CI, T);

impl<T> CellState for T where T: Clone + Send + Sync {}
impl<T> CellIndex for T where T: Clone + Copy + Send + Sync {}
