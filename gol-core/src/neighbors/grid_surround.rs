use super::util::{MarginPrimInt, PointPrimInt};
use crate::cell::index::ToGridPointND;
use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use itertools::izip;

pub struct NeighborsGridSurround<T> {
    should_repeat_margin: bool,
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
    /// let neighbor_calc = NeighborsGridSurround::new(1usize);
    /// let cur_point = GridPoint2D{ x: 10, y: 5 };
    /// let neighbors: Vec<GridPoint2D<i32>> =
    ///     neighbor_calc.get_neighbors_idx(&cur_point).collect();
    /// assert_eq!(neighbors.len(), 8);
    ///
    /// let neighbor_calc_2 = NeighborsGridSurround::new(1usize);
    /// let cur_point = GridPoint3D{ x: 10, y: 5, z: 9};
    /// let neighbors_2: Vec<GridPoint3D<i32>> =
    ///     neighbor_calc_2.get_neighbors_idx(&cur_point).collect();
    /// assert_eq!(neighbors_2.len(), 26);
    /// ```
    pub fn new(margin: T) -> Self
    where
        T: Clone,
    {
        let margin_two_sides = vec![(margin.clone(), margin)];
        Self {
            should_repeat_margin: true,
            margins: margin_two_sides,
        }
    }

    /// Creates a new neighbor calculator with specific margin on each side and dimension. Elements
    /// in the vector represents different dimensions, the two values inside the vector represents
    /// margin on the negative and positive side along that dimension.
    /// ```rust
    /// use gol_core::{GridPoint2D, NeighborsGridSurround, BoardNeighborManager}; // Create 2D margin with 2 on all sides but positive y-axis.
    /// let margins = [(2usize, 2), (2usize, 1)];
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
            should_repeat_margin: false,
            margins: vec,
        }
    }

    fn calc_grid_point_surrounding<U>(&self, idx: &GridPointND<U>) -> Vec<GridPointND<U>>
    where
        T: MarginPrimInt,
        U: PointPrimInt,
    {
        let (dim_ranges, dim_lens, volume) = self.calc_dim_ranges(idx);

        // Calculate the flattened index of idx to exclude itself from its neighbors.
        let mut i_exclude = 0usize;
        let idx_indices: Vec<&U> = idx.indices().collect();
        let mut cur_volume = volume;
        for (cur_idx, dim_len, (dim_min, _)) in izip!(&idx_indices, &dim_lens, &dim_ranges).rev() {
            cur_volume /= dim_len;
            i_exclude += (**cur_idx - *dim_min).to_usize().unwrap() * cur_volume;
        }

        let mut res = Vec::new();
        for i in 0..volume {
            if i == i_exclude {
                continue;
            }

            let (mut cur_i, mut cur_vol) = (i, volume);
            let mut cur_indices = Vec::with_capacity(dim_lens.len());

            for ((dim_min, _), dim_len) in dim_ranges.iter().zip(dim_lens.iter()).rev() {
                cur_vol /= dim_len;
                let dim_idx = cur_i / cur_vol;
                cur_indices.push(U::from_usize(dim_idx).unwrap() + *dim_min);
                cur_i %= cur_vol;
            }
            res.push(GridPointND::new(cur_indices.iter().rev()));
        }
        res
    }

    fn calc_dim_ranges<U>(&self, idx: &GridPointND<U>) -> (Vec<(U, U)>, Vec<usize>, usize)
    where
        T: MarginPrimInt,
        U: PointPrimInt,
    {
        let mut ranges = Vec::new();
        let mut dim_lens = Vec::new();
        let mut volume = 1;
        for (i, dim_idx) in idx.indices().enumerate() {
            let (neg, pos) = if self.should_repeat_margin {
                self.margins.first().unwrap()
            } else {
                self.margins.get(i).unwrap()
            };

            let mut dim_idx_min = None;
            for n in (0..=neg.to_usize().unwrap()).rev() {
                let n_u = U::from_usize(n).unwrap();
                match dim_idx.checked_sub(&n_u) {
                    Some(val) => {
                        dim_idx_min = Some(val);
                        break;
                    }
                    None => continue,
                }
            }

            let mut dim_idx_max = None;
            for n in (0..=pos.to_usize().unwrap()).rev() {
                let n_u = U::from_usize(n).unwrap();
                match dim_idx.checked_add(&n_u) {
                    Some(val) => {
                        dim_idx_max = Some(val);
                        break;
                    }
                    None => continue,
                }
            }

            // let dim_idx_min = dim_idx
            //     .checked_sub(&U::from_usize(neg.to_usize().unwrap()).unwrap())
            //     .expect("Cannot subtract point by margin.");
            let dim_idx_min = dim_idx_min.unwrap();
            let dim_idx_max = dim_idx_max.unwrap();

            ranges.push((dim_idx_min, dim_idx_max));
            let dim_len = (dim_idx_max - dim_idx_min + U::one()).to_usize().unwrap();
            dim_lens.push(dim_len);
            volume *= dim_len;
        }
        (ranges, dim_lens, volume)
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
        let one = U::one();
        vec![GridPoint1D::new(idx.x - one), GridPoint1D::new(idx.x + one)].into_iter()
    }
}

#[cfg(test)]
mod grid_surrounding_neighbor_test {
    use crate::{
        BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND,
        NeighborsGridSurround,
    };

    #[test]
    fn grid_surrounding_test_1d_1() {
        let neighbor_calc = NeighborsGridSurround::new(1usize);
        let point = GridPoint1D { x: 10 };
        let neighbors: Vec<GridPoint1D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 2);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint1D { x: 9 }));
        assert!(neighbors.contains(&GridPoint1D { x: 11 }));
    }

    #[test]
    fn grid_surrounding_test_1d_2() {
        let neighbor_calc = NeighborsGridSurround::new(1usize);
        let point = GridPoint1D { x: 0 };
        let neighbors: Vec<GridPoint1D<i64>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 2);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint1D { x: 1 }));
    }

    #[test]
    fn grid_surrounding_test_2d_1() {
        let neighbor_calc = NeighborsGridSurround::new(1usize);
        let point = GridPoint2D { x: 10, y: 5 };
        let neighbors: Vec<GridPoint2D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 8);
        assert!(!neighbors.contains(&point));
    }

    #[test]
    fn grid_surrounding_test_2d_2() {
        let neighbor_calc = NeighborsGridSurround::new(1usize);
        let point = GridPoint2D { x: 0, y: 0 };
        let neighbors: Vec<GridPoint2D<i64>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 8);
        assert!(!neighbors.contains(&point));
    }

    #[test]
    fn grid_surrounding_test_2d_3() {
        let neighbor_calc = NeighborsGridSurround::new(1usize);
        let point = GridPoint2D { x: 0, y: 1 };
        let neighbors: Vec<GridPoint2D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 8);
        assert!(!neighbors.contains(&point));
    }

    #[test]
    fn grid_surrounding_test_3d_1() {
        let neighbor_calc = NeighborsGridSurround::new(1usize);
        let point = GridPoint3D { x: 3, y: 10, z: 5 };
        let neighbors: Vec<GridPoint3D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 26);
        assert!(!neighbors.contains(&point));
    }

    #[test]
    fn grid_surrounding_test_3d_2() {
        let neighbor_calc = NeighborsGridSurround::new(2usize);
        let point = GridPoint3D { x: 0, y: 0, z: 0 };
        let neighbors: Vec<GridPoint3D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 124);
        assert!(!neighbors.contains(&point));
    }

    #[test]
    fn grid_surrounding_test_3d_3() {
        let neighbor_calc = NeighborsGridSurround::new(2usize);
        let point_1 = GridPoint3D { x: 0, y: 1, z: 1 };
        let point_2 = GridPoint3D { x: 1, y: 0, z: 1 };
        let point_3 = GridPoint3D { x: 1, y: 1, z: 0 };
        let neighbors_1: Vec<GridPoint3D<i32>> =
            neighbor_calc.get_neighbors_idx(&point_1).collect();
        let neighbors_2: Vec<GridPoint3D<i32>> =
            neighbor_calc.get_neighbors_idx(&point_2).collect();
        let neighbors_3: Vec<GridPoint3D<i32>> =
            neighbor_calc.get_neighbors_idx(&point_3).collect();
        assert_eq!(neighbors_1.len(), 124);
        assert_eq!(neighbors_2.len(), neighbors_1.len());
        assert_eq!(neighbors_3.len(), neighbors_1.len());
        assert!(!neighbors_1.contains(&point_1));
        assert!(!neighbors_2.contains(&point_2));
        assert!(!neighbors_3.contains(&point_3));
    }

    #[test]
    fn grid_surrounding_test_4d_1() {
        let neighbor_calc = NeighborsGridSurround::new(2usize);
        let point = GridPointND::new(vec![0, 0, 0, 0].iter());
        let neighbors: Vec<GridPointND<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 624);
        assert!(!neighbors.contains(&point));
    }

    #[test]
    fn grid_surrounding_test_4d_2() {
        let mut margins = Vec::new();
        margins.push((100usize, 2)); // dim_len = 103
        margins.push((50, 1)); // dim_len = 52
        margins.push((10, 2)); // dim_len = 13
        margins.push((0, 9)); // dim_len = 10
        let neighbor_calc = NeighborsGridSurround::new_with_variable_margin(margins.iter());
        let point = GridPointND::new(vec![0, 0, 1, 0].iter());
        let neighbors: Vec<GridPointND<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 696279);
        assert!(!neighbors.contains(&point));
    }
}
