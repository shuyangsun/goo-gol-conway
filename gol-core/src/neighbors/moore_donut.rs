use super::util::PointPrimInt;
use crate::{
    util::grid_util::{Size1D, Size2D, Size3D},
    BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D,
};

pub struct NeighborMooreDonut<T> {
    grid_size: T,
}

impl<T> NeighborMooreDonut<T> {
    pub fn new(grid_size: T) -> Self {
        Self { grid_size }
    }
}

impl<T> BoardNeighborManager<GridPoint3D<T>, std::vec::IntoIter<GridPoint3D<T>>>
    for NeighborMooreDonut<Size3D>
where
    T: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint3D<T>) -> std::vec::IntoIter<GridPoint3D<T>> {
        assert!(self.grid_size.width() >= 3);
        assert!(self.grid_size.height() >= 3);
        assert!(self.grid_size.depth() >= 3);
        let one_t = T::one();
        let (mut x_1, x_2, mut x_3) = (idx.x - one_t, idx.x, idx.x + one_t);
        let (mut y_1, y_2, mut y_3) = (idx.y - one_t, idx.y, idx.y + one_t);
        let (mut z_1, z_2, mut z_3) = (idx.z - one_t, idx.z, idx.z + one_t);
        if x_1 < T::from_i64(self.grid_size.x_idx_min()).unwrap() {
            x_1 = T::from_i64(self.grid_size.x_idx_max()).unwrap();
        } else if x_3 > T::from_i64(self.grid_size.x_idx_max()).unwrap() {
            x_3 = T::from_i64(self.grid_size.x_idx_min()).unwrap();
        }
        if y_1 < T::from_i64(self.grid_size.y_idx_min()).unwrap() {
            y_1 = T::from_i64(self.grid_size.y_idx_max()).unwrap();
        } else if y_3 > T::from_i64(self.grid_size.y_idx_max()).unwrap() {
            y_3 = T::from_i64(self.grid_size.y_idx_min()).unwrap();
        }
        if z_1 < T::from_i64(self.grid_size.z_idx_min()).unwrap() {
            z_1 = T::from_i64(self.grid_size.z_idx_max()).unwrap();
        } else if z_3 > T::from_i64(self.grid_size.z_idx_max()).unwrap() {
            z_3 = T::from_i64(self.grid_size.z_idx_min()).unwrap();
        }

        vec![
            GridPoint3D::new(x_1, y_1, z_1),
            GridPoint3D::new(x_1, y_2, z_1),
            GridPoint3D::new(x_1, y_3, z_1),
            GridPoint3D::new(x_2, y_1, z_1),
            GridPoint3D::new(x_2, y_2, z_1),
            GridPoint3D::new(x_2, y_3, z_1),
            GridPoint3D::new(x_3, y_1, z_1),
            GridPoint3D::new(x_3, y_2, z_1),
            GridPoint3D::new(x_3, y_3, z_1),
            GridPoint3D::new(x_1, y_1, z_2),
            GridPoint3D::new(x_1, y_2, z_2),
            GridPoint3D::new(x_1, y_3, z_2),
            GridPoint3D::new(x_2, y_1, z_2),
            GridPoint3D::new(x_2, y_3, z_2),
            GridPoint3D::new(x_3, y_1, z_2),
            GridPoint3D::new(x_3, y_2, z_2),
            GridPoint3D::new(x_3, y_3, z_2),
            GridPoint3D::new(x_1, y_1, z_3),
            GridPoint3D::new(x_1, y_2, z_3),
            GridPoint3D::new(x_1, y_3, z_3),
            GridPoint3D::new(x_2, y_1, z_3),
            GridPoint3D::new(x_2, y_2, z_3),
            GridPoint3D::new(x_2, y_3, z_3),
            GridPoint3D::new(x_3, y_1, z_3),
            GridPoint3D::new(x_3, y_2, z_3),
            GridPoint3D::new(x_3, y_3, z_3),
        ]
        .into_iter()
    }
}

impl<T> BoardNeighborManager<GridPoint2D<T>, std::vec::IntoIter<GridPoint2D<T>>>
    for NeighborMooreDonut<Size2D>
where
    T: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint2D<T>) -> std::vec::IntoIter<GridPoint2D<T>> {
        assert!(self.grid_size.width() >= 3);
        assert!(self.grid_size.height() >= 3);
        let one_t = T::one();
        let (mut x_1, x_2, mut x_3) = (idx.x - one_t, idx.x, idx.x + one_t);
        let (mut y_1, y_2, mut y_3) = (idx.y - one_t, idx.y, idx.y + one_t);
        if x_1 < T::from_i64(self.grid_size.x_idx_min()).unwrap() {
            x_1 = T::from_i64(self.grid_size.x_idx_max()).unwrap();
        } else if x_3 > T::from_i64(self.grid_size.x_idx_max()).unwrap() {
            x_3 = T::from_i64(self.grid_size.x_idx_min()).unwrap();
        }
        if y_1 < T::from_i64(self.grid_size.y_idx_min()).unwrap() {
            y_1 = T::from_i64(self.grid_size.y_idx_max()).unwrap();
        } else if y_3 > T::from_i64(self.grid_size.y_idx_max()).unwrap() {
            y_3 = T::from_i64(self.grid_size.y_idx_min()).unwrap();
        }
        vec![
            GridPoint2D::new(x_1, y_1),
            GridPoint2D::new(x_1, y_2),
            GridPoint2D::new(x_1, y_3),
            GridPoint2D::new(x_2, y_1),
            GridPoint2D::new(x_2, y_3),
            GridPoint2D::new(x_3, y_1),
            GridPoint2D::new(x_3, y_2),
            GridPoint2D::new(x_3, y_3),
        ]
        .into_iter()
    }
}

impl<T> BoardNeighborManager<GridPoint1D<T>, std::vec::IntoIter<GridPoint1D<T>>>
    for NeighborMooreDonut<Size1D>
where
    T: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint1D<T>) -> std::vec::IntoIter<GridPoint1D<T>> {
        assert!(self.grid_size.width() >= 3);
        let one_t = T::one();
        let left = idx.x - one_t;
        let right = idx.x + one_t;
        if left < T::from_i64(self.grid_size.x_idx_min()).unwrap() {
            vec![
                GridPoint1D::new(T::from_i64(self.grid_size.x_idx_max()).unwrap()),
                GridPoint1D::new(right),
            ]
        } else if right > T::from_i64(self.grid_size.x_idx_max()).unwrap() {
            vec![
                GridPoint1D::new(left),
                GridPoint1D::new(T::from_i64(self.grid_size.x_idx_min()).unwrap()),
            ]
        } else {
            vec![GridPoint1D::new(left), GridPoint1D::new(right)]
        }
        .into_iter()
    }
}
