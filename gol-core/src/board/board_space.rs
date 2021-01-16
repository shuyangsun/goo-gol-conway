use rayon::iter::ParallelIterator;

/// Manages the logical space between cells.
pub trait BoardSpaceManager<CI, I1, I2>: Send + Sync
where
    CI: Send + Sync,
    I1: Iterator<Item = CI>,
    I2: ParallelIterator<Item = CI>,
{
    fn indices_iter(&self) -> I1;
    fn indices_par_iter(&self) -> I2;
}
