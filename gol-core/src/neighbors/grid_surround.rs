use crate::cell::index::ToGridPointND;
use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use num_traits::{CheckedAdd, CheckedSub, FromPrimitive, PrimInt, ToPrimitive};

pub trait MarginPrimInt: Send + Sync + PrimInt + ToPrimitive {}
pub trait PointPrimInt: Send + Sync + PrimInt + CheckedAdd + CheckedSub + FromPrimitive {}

pub struct NeighborsGridSurround<T> {
    is_infinite: bool,
    margins: Vec<(T, T)>,
}

impl<T> NeighborsGridSurround<T> {
    /// Creates a new neighbor calculator with equal margin on all sides and dimensions.
    /// ```rust
    /// use gol_core::{
    ///     NeighborsGridSurround, BoardNeighborManager, GridPoint2D, GridPoint3D
    /// };
    ///
    /// // Create Conway's Game of Life margin: 1 on each side.
    /// let neighbor_calc = NeighborsGridSurround::new(1);
    /// let cur_point = GridPoint2D{ x: 10, y: 5 };
    /// let neighbors: Vec<GridPoint2D<i32>> =
    ///     neighbor_calc.get_neighbors_idx(&cur_point).collect();
    /// assert_eq!(neighbors.len(), 8);
    ///
    /// let neighbor_calc_2 = NeighborsGridSurround::new(1);
    /// let cur_point = GridPoint3D{ x: 10, y: 5, z: 9};
    /// let neighbors_2: Vec<GridPoint3D<usize>> =
    ///     neighbor_calc_2.get_neighbors_idx(&cur_point).collect();
    /// assert_eq!(neighbors_2.len(), 26);
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
    /// use gol_core::{GridPoint2D, NeighborsGridSurround, BoardNeighborManager};
    ///
    /// // Create 2D margin with 2 on all sides but positive y-axis.
    /// let margins = [(2, 2), (2, 1)];
    /// let neighbor_calc =
    ///     NeighborsGridSurround::new_with_variable_margin(margins.iter());
    ///
    /// let cur_point = GridPoint2D{ x: 10, y: 5 };
    /// let neighbors: Vec<GridPoint2D<i32>> =
    ///     neighbor_calc.get_neighbors_idx(&cur_point).collect();
    /// assert_eq!(neighbors.len(), 19);
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
        T: MarginPrimInt,
        U: PointPrimInt,
    {
        let mut ranges = Vec::new();
        for (i, dim_idx) in idx.indices().enumerate() {
            let mut cur_dim_indices = Vec::new();
            let (neg, pos) = if self.is_infinite {
                self.margins.first().unwrap()
            } else {
                self.margins.get(i).unwrap()
            };

            for n in 0..neg.to_usize().unwrap() {
                let n_u = U::from_usize(n).unwrap();
                match dim_idx.checked_sub(&n_u) {
                    Some(true_neg) => {
                        cur_dim_indices.push(true_neg);
                        break;
                    }
                    None => continue,
                }
            }

            for n in 1..pos.to_usize().unwrap() + 1 {
                let n_u = U::from_usize(n).unwrap();
                match dim_idx.checked_add(&n_u) {
                    Some(true_pos) => {
                        cur_dim_indices.push(true_pos);
                        break;
                    }
                    None => continue,
                }
            }
            ranges.push(cur_dim_indices);
        }
        let res = Vec::new();
        // TODO
        res
    }
}

impl<T, U> BoardNeighborManager<GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>
    for NeighborsGridSurround<T>
where
    T: MarginPrimInt,
    U: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPointND<U>) -> std::vec::IntoIter<GridPointND<U>> {
        self.calc_grid_point_surrounding(idx).into_iter()
    }
}

impl<T, U> BoardNeighborManager<GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>
    for NeighborsGridSurround<T>
where
    T: MarginPrimInt,
    U: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint3D<U>) -> std::vec::IntoIter<GridPoint3D<U>> {
        let res: Vec<GridPoint3D<U>> = self
            .calc_grid_point_surrounding(&idx.to_nd())
            .iter()
            .map(|ele| ele.to_3d().unwrap())
            .collect();
        res.into_iter()
    }
}

impl<T, U> BoardNeighborManager<GridPoint2D<U>, std::vec::IntoIter<GridPoint2D<U>>>
    for NeighborsGridSurround<T>
where
    T: MarginPrimInt,
    U: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint2D<U>) -> std::vec::IntoIter<GridPoint2D<U>> {
        let res: Vec<GridPoint2D<U>> = self
            .calc_grid_point_surrounding(&idx.to_nd())
            .iter()
            .map(|ele| ele.to_2d().unwrap())
            .collect();
        res.into_iter()
    }
}

impl<T, U> BoardNeighborManager<GridPoint1D<U>, std::vec::IntoIter<GridPoint1D<U>>>
    for NeighborsGridSurround<T>
where
    T: MarginPrimInt,
    U: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPoint1D<U>) -> std::vec::IntoIter<GridPoint1D<U>> {
        let res: Vec<GridPoint1D<U>> = self
            .calc_grid_point_surrounding(&idx.to_nd())
            .iter()
            .map(|ele| ele.to_1d().unwrap())
            .collect();
        res.into_iter()
    }
}

impl<T> MarginPrimInt for T where T: Send + Sync + PrimInt + ToPrimitive {}
impl<T> PointPrimInt for T where T: Send + Sync + PrimInt + CheckedAdd + CheckedSub + FromPrimitive {}
