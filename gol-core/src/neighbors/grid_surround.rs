use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use num_traits::PrimInt;

pub struct NeighborsGridSurroud {}

impl<T> BoardNeighborManager<GridPointND<T>, std::vec::IntoIter<GridPointND<T>>>
    for NeighborsGridSurroud
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPointND<T>) -> std::vec::IntoIter<GridPointND<T>> {
        Vec::new().into_iter() // TODO
    }
}

impl<T> BoardNeighborManager<GridPoint3D<T>, std::vec::IntoIter<GridPoint3D<T>>>
    for NeighborsGridSurroud
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPoint3D<T>) -> std::vec::IntoIter<GridPoint3D<T>> {
        Vec::new().into_iter() // TODO
    }
}

impl<T> BoardNeighborManager<GridPoint2D<T>, std::vec::IntoIter<GridPoint2D<T>>>
    for NeighborsGridSurroud
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPoint2D<T>) -> std::vec::IntoIter<GridPoint2D<T>> {
        Vec::new().into_iter() // TODO
    }
}

impl<T> BoardNeighborManager<GridPoint1D<T>, std::vec::IntoIter<GridPoint1D<T>>>
    for NeighborsGridSurroud
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPoint1D<T>) -> std::vec::IntoIter<GridPoint1D<T>> {
        Vec::new().into_iter() // TODO
    }
}
