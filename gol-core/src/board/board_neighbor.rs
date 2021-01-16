pub trait BoardNeighborManager<CI, I>: Send + Sync
where
    CI: Send + Sync,
    I: Iterator<Item = CI>,
{
    fn get_neighbors_idx(&self, idx: &CI) -> I;
}
