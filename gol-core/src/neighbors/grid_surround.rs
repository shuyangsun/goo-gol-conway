use crate::cell::index::ToGridPointND;
use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use num_traits::PrimInt;

pub struct NeighborsGridSurround<T> {
    is_infinite: bool,
    margins: Vec<(T, T)>,
}

impl<T> NeighborsGridSurround<T> {
    /// Creates a new neighbor calculator with equal margin on all sides and dimensions.
    /// ```rust
    /// use gol_core::NeighborsGridSurround;
    ///
    /// // Create Conway's Game of Life margin: 1 on each side.
    /// let margins = NeighborsGridSurround::new(1);
    /// ```
    pub fn new(margin: T) -> Self
    where
        T: Clone,
    {
        let margin_two_sides = vec![(margin.clone(), margin)];
        Self {
            is_infinite: true,
            margins: margin_two_sides,
        }
    }

    /// Creates a new neighbor calculator with specific margin on each side and dimension. Elements
    /// in the vector represents different dimensions, the two values inside the vector represents
    /// margin on the negative and positive side along that dimension.
    /// ```rust
    /// use gol_core::NeighborsGridSurround;
    ///
    /// // Create 2D margin with 2 on all sides but positive y-axis.
    /// let margins = [(2, 2), (2, 1)];
    /// let neighbor_calc =
    ///     NeighborsGridSurround::new_with_variable_margin(margins.iter());
    /// ```
    pub fn new_with_variable_margin<'a, 'b, I>(margins: I) -> Self
    where
        'a: 'b,
        T: 'a + Clone,
        I: Iterator<Item = &'b (T, T)>,
    {
        let vec: Vec<(T, T)> = margins.map(|ele| (ele.0.clone(), ele.1.clone())).collect();
        assert!(!vec.is_empty());
        Self {
            is_infinite: false,
            margins: vec,
        }
    }

    fn calc_grid_point_surrounding<U>(&self, idx: &GridPointND<U>) -> Vec<GridPointND<U>>
    where
        T: std::convert::TryFrom<U>,
    {
        let res = Vec::new();
        // TODO
        res
    }
}

impl<T> BoardNeighborManager<GridPointND<T>, std::vec::IntoIter<GridPointND<T>>>
    for NeighborsGridSurround<T>
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPointND<T>) -> std::vec::IntoIter<GridPointND<T>> {
        self.calc_grid_point_surrounding(&idx).into_iter()
    }
}

impl<T> BoardNeighborManager<GridPoint3D<T>, std::vec::IntoIter<GridPoint3D<T>>>
    for NeighborsGridSurround<T>
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPoint3D<T>) -> std::vec::IntoIter<GridPoint3D<T>> {
        let res: Vec<GridPoint3D<T>> = self
            .calc_grid_point_surrounding(&idx.to_nd())
            .iter()
            .map(|ele| ele.to_3d().unwrap())
            .collect();
        res.into_iter()
    }
}

impl<T> BoardNeighborManager<GridPoint2D<T>, std::vec::IntoIter<GridPoint2D<T>>>
    for NeighborsGridSurround<T>
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPoint2D<T>) -> std::vec::IntoIter<GridPoint2D<T>> {
        let res: Vec<GridPoint2D<T>> = self
            .calc_grid_point_surrounding(&idx.to_nd())
            .iter()
            .map(|ele| ele.to_2d().unwrap())
            .collect();
        res.into_iter()
    }
}

impl<T> BoardNeighborManager<GridPoint1D<T>, std::vec::IntoIter<GridPoint1D<T>>>
    for NeighborsGridSurround<T>
where
    T: Send + Sync + PrimInt,
{
    fn get_neighbors_idx(&self, idx: GridPoint1D<T>) -> std::vec::IntoIter<GridPoint1D<T>> {
        let res: Vec<GridPoint1D<T>> = self
            .calc_grid_point_surrounding(&idx.to_nd())
            .iter()
            .map(|ele| ele.to_1d().unwrap())
            .collect();
        res.into_iter()
    }
}
