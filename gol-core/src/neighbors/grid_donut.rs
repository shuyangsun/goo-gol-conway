use super::util::{MarginPrimInt, PointPrimInt};
use crate::cell::index::ToGridPointND;
use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use itertools::Itertools;
use std::cmp::{max, min};

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
        U: PointPrimInt + std::fmt::Debug,
    {
        let dim_ranges = self.calc_dim_ranges(idx);
        for n in dim_ranges.iter() {
            eprint!("{:?}, ", n);
        }
        eprintln!();

        // Expand dim ranges.
        let mut indices_each_dim = Vec::with_capacity(dim_ranges.len());
        for (ranges_1, ranges_2) in dim_ranges.iter() {
            let mut cur = Vec::new();
            let (cur_min, cur_max) = ranges_1;
            for i in cur_min.to_i64().unwrap()..=cur_max.to_i64().unwrap() {
                cur.push(U::from_i64(i).unwrap());
            }
            if ranges_2.is_some() {
                let (cur_min, cur_max) = ranges_2.unwrap();
                for i in cur_min.to_i64().unwrap()..=cur_max.to_i64().unwrap() {
                    cur.push(U::from_i64(i).unwrap());
                }
            }
            indices_each_dim.push(cur.into_iter());
        }
        for n in indices_each_dim.iter() {
            eprint!("{:?}, ", n);
        }
        eprintln!();

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
        U: PointPrimInt + std::fmt::Debug,
    {
        let mut ranges = Vec::new();
        for (i, dim_idx) in idx.indices().enumerate() {
            let (neg, pos) = if self.should_repeat_margin {
                self.margins.first().unwrap()
            } else {
                self.margins.get(i).unwrap()
            };
            let neg = U::from_usize(neg.to_usize().unwrap())
                .expect("Index type too small to hold neighbor margin value.");
            let pos = U::from_usize(pos.to_usize().unwrap())
                .expect("Index type too small to hold neighbor margin value.");
            let u_two = U::one() + U::one();

            let board_dim_len = U::from_usize(self.board_shape[i].to_usize().unwrap()).unwrap();
            assert!(
                board_dim_len.to_usize().unwrap()
                    >= neg.to_usize().unwrap() + pos.to_usize().unwrap() + 1
            );

            let board_min = board_dim_len / u_two - board_dim_len + U::one();
            let board_max = board_dim_len / u_two;

            let mut wrapping_range: Option<(U, U)> = None;

            let dim_idx_min_unchecked = dim_idx
                .checked_sub(&neg)
                .expect("Could not subtract points by margin value.");
            let dim_idx_max_unchecked = dim_idx
                .checked_add(&pos)
                .expect("Could not add points by margin value.");
            let dim_idx_min = max(board_min, dim_idx_min_unchecked);
            let dim_idx_max = min(board_max, dim_idx_max_unchecked);

            if dim_idx_min_unchecked < board_min {
                let extension = dim_idx_min_unchecked - board_min;
                wrapping_range = Some((board_max + extension + U::one(), board_max));
            } else if dim_idx_max_unchecked > board_max {
                let extension = dim_idx_max_unchecked - board_max;
                wrapping_range = Some((board_min, board_min + extension - U::one()));
            }

            ranges.push(((dim_idx_min, dim_idx_max), wrapping_range));
        }
        ranges
    }
}

impl<T, U> BoardNeighborManager<GridPointND<U>, std::vec::IntoIter<GridPointND<U>>>
    for NeighborsGridDonut<T>
where
    T: MarginPrimInt,
    U: PointPrimInt + std::fmt::Debug,
{
    fn get_neighbors_idx(&self, idx: &GridPointND<U>) -> std::vec::IntoIter<GridPointND<U>> {
        self.calc_grid_point_surrounding(idx).into_iter()
    }
}

impl<T, U> BoardNeighborManager<GridPoint3D<U>, std::vec::IntoIter<GridPoint3D<U>>>
    for NeighborsGridDonut<T>
where
    T: MarginPrimInt,
    U: PointPrimInt + std::fmt::Debug,
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
    U: PointPrimInt + std::fmt::Debug,
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
    U: PointPrimInt + std::fmt::Debug,
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
    use crate::{BoardNeighborManager, GridPoint1D, GridPoint2D, NeighborsGridDonut};

    #[test]
    fn grid_donut_test_1d_1() {
        let board_shape = vec![100usize];
        let neighbor_calc = NeighborsGridDonut::new(1usize, board_shape.into_iter());
        let point = GridPoint1D { x: 10 };
        let neighbors: Vec<GridPoint1D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 2);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint1D { x: 9 }));
        assert!(neighbors.contains(&GridPoint1D { x: 11 }));
    }

    #[test]
    fn grid_donut_test_1d_2() {
        let board_shape = vec![3usize];
        let neighbor_calc = NeighborsGridDonut::new(1usize, board_shape.into_iter());
        let point = GridPoint1D { x: 0 };
        let neighbors: Vec<GridPoint1D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        assert_eq!(neighbors.len(), 2);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint1D { x: -1 }));
        assert!(neighbors.contains(&GridPoint1D { x: 1 }));
    }

    #[test]
    fn grid_donut_test_2d_1() {
        let board_shape = vec![5usize, 5];
        let neighbor_calc = NeighborsGridDonut::new(1usize, board_shape.into_iter());
        let point = GridPoint2D { x: -2, y: -2 };
        let neighbors: Vec<GridPoint2D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        for n in neighbors.iter() {
            eprintln!("{:?}", n);
        }
        assert_eq!(neighbors.len(), 8);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint2D { x: -2, y: -1 }));
        assert!(neighbors.contains(&GridPoint2D { x: -1, y: -1 }));
        assert!(neighbors.contains(&GridPoint2D { x: -1, y: -2 }));
        assert!(neighbors.contains(&GridPoint2D { x: 2, y: -2 }));
        assert!(neighbors.contains(&GridPoint2D { x: 2, y: -1 }));
        assert!(neighbors.contains(&GridPoint2D { x: -2, y: 2 }));
        assert!(neighbors.contains(&GridPoint2D { x: -1, y: 2 }));
        assert!(neighbors.contains(&GridPoint2D { x: 2, y: 2 }));
    }

    #[test]
    fn grid_donut_test_2d_2() {
        let board_shape = vec![5usize, 5];
        let neighbor_calc = NeighborsGridDonut::new(1usize, board_shape.into_iter());
        let point = GridPoint2D { x: 2, y: 2 };
        let neighbors: Vec<GridPoint2D<i32>> = neighbor_calc.get_neighbors_idx(&point).collect();
        for n in neighbors.iter() {
            eprintln!("{:?}", n);
        }
        assert_eq!(neighbors.len(), 8);
        assert!(!neighbors.contains(&point));
        assert!(neighbors.contains(&GridPoint2D { x: -2, y: -2 }));
        assert!(neighbors.contains(&GridPoint2D { x: -2, y: 2 }));
        assert!(neighbors.contains(&GridPoint2D { x: -2, y: 1 }));
        assert!(neighbors.contains(&GridPoint2D { x: 2, y: -2 }));
        assert!(neighbors.contains(&GridPoint2D { x: 1, y: -2 }));
        assert!(neighbors.contains(&GridPoint2D { x: 1, y: 2 }));
        assert!(neighbors.contains(&GridPoint2D { x: 1, y: 1 }));
        assert!(neighbors.contains(&GridPoint2D { x: 2, y: 1 }));
    }
}
