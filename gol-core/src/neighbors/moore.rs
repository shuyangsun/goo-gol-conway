use super::util::PointPrimInt;
use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};

pub struct NeighborMoore {}

impl NeighborMoore {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> BoardNeighborManager<GridPoint3D<T>, std::vec::IntoIter<GridPoint3D<T>>> for NeighborMoore
where
    T: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint3D<T>) -> std::vec::IntoIter<GridPoint3D<T>> {
        let one_t = T::one();
        vec![
            GridPoint3D::new(idx.x - one_t, idx.y - one_t, idx.z - one_t),
            GridPoint3D::new(idx.x - one_t, idx.y, idx.z - one_t),
            GridPoint3D::new(idx.x - one_t, idx.y + one_t, idx.z - one_t),
            GridPoint3D::new(idx.x, idx.y - one_t, idx.z - one_t),
            GridPoint3D::new(idx.x, idx.y, idx.z - one_t),
            GridPoint3D::new(idx.x, idx.y + one_t, idx.z - one_t),
            GridPoint3D::new(idx.x + one_t, idx.y - one_t, idx.z - one_t),
            GridPoint3D::new(idx.x + one_t, idx.y, idx.z - one_t),
            GridPoint3D::new(idx.x + one_t, idx.y + one_t, idx.z - one_t),
            GridPoint3D::new(idx.x - one_t, idx.y - one_t, idx.z),
            GridPoint3D::new(idx.x - one_t, idx.y, idx.z),
            GridPoint3D::new(idx.x - one_t, idx.y + one_t, idx.z),
            GridPoint3D::new(idx.x, idx.y - one_t, idx.z),
            GridPoint3D::new(idx.x, idx.y + one_t, idx.z),
            GridPoint3D::new(idx.x + one_t, idx.y - one_t, idx.z),
            GridPoint3D::new(idx.x + one_t, idx.y, idx.z),
            GridPoint3D::new(idx.x + one_t, idx.y + one_t, idx.z),
            GridPoint3D::new(idx.x - one_t, idx.y - one_t, idx.z + one_t),
            GridPoint3D::new(idx.x - one_t, idx.y, idx.z + one_t),
            GridPoint3D::new(idx.x - one_t, idx.y + one_t, idx.z + one_t),
            GridPoint3D::new(idx.x, idx.y - one_t, idx.z + one_t),
            GridPoint3D::new(idx.x, idx.y, idx.z + one_t),
            GridPoint3D::new(idx.x, idx.y + one_t, idx.z + one_t),
            GridPoint3D::new(idx.x + one_t, idx.y - one_t, idx.z + one_t),
            GridPoint3D::new(idx.x + one_t, idx.y, idx.z + one_t),
            GridPoint3D::new(idx.x + one_t, idx.y + one_t, idx.z + one_t),
        ]
        .into_iter()
    }
}

impl<T> BoardNeighborManager<GridPoint2D<T>, std::vec::IntoIter<GridPoint2D<T>>> for NeighborMoore
where
    T: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint2D<T>) -> std::vec::IntoIter<GridPoint2D<T>> {
        let one_t = T::one();
        vec![
            GridPoint2D::new(idx.x - one_t, idx.y - one_t),
            GridPoint2D::new(idx.x - one_t, idx.y),
            GridPoint2D::new(idx.x - one_t, idx.y + one_t),
            GridPoint2D::new(idx.x, idx.y - one_t),
            GridPoint2D::new(idx.x, idx.y + one_t),
            GridPoint2D::new(idx.x + one_t, idx.y - one_t),
            GridPoint2D::new(idx.x + one_t, idx.y),
            GridPoint2D::new(idx.x + one_t, idx.y + one_t),
        ]
        .into_iter()
    }
}

impl<T> BoardNeighborManager<GridPoint1D<T>, std::vec::IntoIter<GridPoint1D<T>>> for NeighborMoore
where
    T: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint1D<T>) -> std::vec::IntoIter<GridPoint1D<T>> {
        let one_t = T::one();
        vec![
            GridPoint1D::new(idx.x - one_t),
            GridPoint1D::new(idx.x + one_t),
        ]
        .into_iter()
    }
}
