use super::util::{MarginPrimInt, PointPrimInt};
use crate::cell::index::ToGridPointND;
use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use itertools::Itertools;

pub struct NeighborsGridDonut<T> {
    board_shape: Vec<T>,
    should_repeat_margin: bool,
    margins: Vec<(T, T)>,
}

impl<T> NeighborsGridDonut<T> {
    pub fn new<I>(margin: T, board_shape: I) -> Self
    where
        T: Clone,
        I: Iterator<Item = T>,
    {
        let margin_two_sides = vec![(margin.clone(), margin)];
        Self {
            should_repeat_margin: true,
            margins: margin_two_sides,
            board_shape: board_shape.collect(),
        }
    }

    pub fn new_with_variable_margin<'a, 'b, I1, I2>(margins: I1, board_shape: I2) -> Self
    where
        'a: 'b,
        T: 'a + Clone,
        I1: Iterator<Item = &'b (T, T)>,
        I2: Iterator<Item = T>,
    {
        let vec: Vec<(T, T)> = margins.map(|ele| (ele.0.clone(), ele.1.clone())).collect();
        assert!(!vec.is_empty());
        Self {
            should_repeat_margin: false,
            margins: vec,
            board_shape: board_shape.collect(),
        }
    }

    fn calc_grid_point_surrounding<U>(&self, idx: &GridPointND<U>) -> Vec<GridPointND<U>>
    where
        T: MarginPrimInt,
        U: PointPrimInt,
    {
        let dim_ranges = self.calc_dim_ranges(idx);

        // Expand dim ranges.
        let mut indices_each_dim = Vec::with_capacity(dim_ranges.len());
        for (ranges_1, ranges_2) in dim_ranges.iter() {
            let mut cur = Vec::new();
            let (cur_min, cur_max) = ranges_1;
            for i in cur_min.to_i64().unwrap()..cur_max.to_i64().unwrap() {
                cur.push(U::from_i64(i).unwrap());
            }
            if ranges_2.is_some() {
                let (cur_min, cur_max) = ranges_2.unwrap();
                for i in cur_min.to_i64().unwrap()..cur_max.to_i64().unwrap() {
                    cur.push(U::from_i64(i).unwrap());
                }
            }
            indices_each_dim.push(cur.into_iter());
        }

        let res = indices_each_dim
            .into_iter()
            .multi_cartesian_product()
            .map(|ele| GridPointND::new(ele.iter()))
            .filter(|ele| ele != idx)
            .collect();
        res
    }

    fn calc_dim_ranges<U>(&self, idx: &GridPointND<U>) -> Vec<((U, U), Option<(U, U)>)>
    where
        T: MarginPrimInt,
        U: PointPrimInt,
    {
        let is_point_origin_center = T::zero().checked_sub(&T::one()).is_some();

        let mut ranges = Vec::new();
        for (i, dim_idx) in idx.indices().enumerate() {
            let (neg, pos) = if self.should_repeat_margin {
                self.margins.first().unwrap()
            } else {
                self.margins.get(i).unwrap()
            };
            let u_two = U::one() + U::one();

            let board_dim_len = U::from_usize(self.board_shape[i].to_usize().unwrap()).unwrap();
            assert!(
                board_dim_len.to_usize().unwrap()
                    >= neg.to_usize().unwrap() + pos.to_usize().unwrap() + 1
            );

            let board_min = if is_point_origin_center {
                board_dim_len / u_two - board_dim_len + U::one()
            } else {
                U::zero()
            };

            let board_max = if is_point_origin_center {
                board_dim_len / u_two
            } else {
                board_dim_len
            };

            let mut wrapping_range: Option<(U, U)> = None;

            let mut dim_idx_min = None;
            for n in (0..=neg.to_usize().unwrap()).rev() {
                let n_u = U::from_usize(n).unwrap();
                match dim_idx.checked_sub(&n_u) {
                    Some(val) => {
                        if val < board_min {
                            if wrapping_range.is_none() {
                                let extension = board_min - val;
                                wrapping_range = Some((board_max - extension, board_max));
                            }
                            continue;
                        }
                        dim_idx_min = Some(val);
                        break;
                    }
                    None => {
                        if wrapping_range.is_none() {
                            let extension = n_u.checked_sub(dim_idx).unwrap();
                            wrapping_range = Some((board_max - extension, board_max));
                        }
                    }
                }
            }

            let mut dim_idx_max = None;
            for n in (0..=pos.to_usize().unwrap()).rev() {
                let n_u = U::from_usize(n).unwrap();
                match dim_idx.checked_add(&n_u) {
                    Some(val) => {
                        if val > board_max {
                            if wrapping_range.is_none() {
                                let extension = val - board_max;
                                wrapping_range = Some((board_min, board_min + extension));
                            }
                            continue;
                        }
                        dim_idx_max = Some(val);
                        break;
                    }
                    None => {
                        if wrapping_range.is_none() {
                            let extension = n_u.checked_sub(dim_idx).unwrap();
                            wrapping_range = Some((board_min, board_min + extension));
                        }
                    }
                }
            }

            let dim_idx_min = dim_idx_min.unwrap();
            let dim_idx_max = dim_idx_max.unwrap();

            ranges.push(((dim_idx_min, dim_idx_max), wrapping_range));
        }
        ranges
    }
}

impl<T, U> BoardNeighborManager<GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>
    for NeighborsGridDonut<T>
where
    T: MarginPrimInt,
    U: PointPrimInt,
{
    fn get_neighbors_idx(&self, idx: &GridPointND<U>) -> std::vec::IntoIter<GridPointND<U>> {
        self.calc_grid_point_surrounding(idx).into_iter()
    }
}

impl<T, U> BoardNeighborManager<GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>
    for NeighborsGridDonut<T>
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
    for NeighborsGridDonut<T>
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
    for NeighborsGridDonut<T>
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

#[cfg(test)]
mod grid_donut_neighbor_test {
    use crate::{
        BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND,
        NeighborsGridDonut,
    };

    #[test]
    fn grid_donut_test_1d_1() {
        let board_shape = vec![100];
        let neighbor_calc = NeighborsGridDonut::new(1, board_shape.into_iter());
        let point = GridPoint1D { x: 10 };
        let neighbors: Vec<GridPoint1D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        for n in neighbors.iter() {
            println!("{:?}", n);
        }
        assert_eq!(neighbors.len(), 2);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint1D { x: 9 }));
        assert!(neighbors.contains(&GridPoint1D { x: 11 }));
    }

    #[test]
    fn grid_donut_test_1d_2() {
        let board_shape = vec![3];
        let neighbor_calc = NeighborsGridDonut::new(1, board_shape.into_iter());
        let point = GridPoint1D { x: 0usize };
        let neighbors: Vec<GridPoint1D<usize>> = neighbor_calc.get_neighbors_idx(&point).collect();
        for n in neighbors.iter() {
            println!("{:?}", n);
        }
        assert_eq!(neighbors.len(), 2);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint1D { x: 1 }));
        assert!(neighbors.contains(&GridPoint1D { x: 2 }));
    }
}
