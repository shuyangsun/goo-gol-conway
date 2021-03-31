use super::util::PointPrimInt;
use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D};

pub struct NeighborMooreTriangle {}

impl NeighborMooreTriangle {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> BoardNeighborManager<GridPoint2D<T>, std::vec::IntoIter<GridPoint2D<T>>>
    for NeighborMooreTriangle
where
    T: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint2D<T>) -> std::vec::IntoIter<GridPoint2D<T>> {
        let one_t = T::one();
        let two_t = one_t + one_t;
        let is_pointing_up = (idx.x + idx.y) % (one_t + one_t) == T::zero();

        if is_pointing_up {
            vec![
                GridPoint2D::new(idx.x + one_t, idx.y),
                GridPoint2D::new(idx.x - one_t, idx.y),
                GridPoint2D::new(idx.x, idx.y - one_t),
                GridPoint2D::new(idx.x, idx.y + one_t),
                GridPoint2D::new(idx.x - two_t, idx.y - one_t),
                GridPoint2D::new(idx.x + two_t, idx.y - one_t),
            ]
            .into_iter()
        } else {
            vec![
                GridPoint2D::new(idx.x + one_t, idx.y),
                GridPoint2D::new(idx.x - one_t, idx.y),
                GridPoint2D::new(idx.x, idx.y - one_t),
                GridPoint2D::new(idx.x, idx.y + one_t),
                GridPoint2D::new(idx.x - two_t, idx.y + one_t),
                GridPoint2D::new(idx.x + two_t, idx.y + one_t),
            ]
            .into_iter()
        }
    }
}

impl<T> BoardNeighborManager<GridPoint1D<T>, std::vec::IntoIter<GridPoint1D<T>>>
    for NeighborMooreTriangle
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
