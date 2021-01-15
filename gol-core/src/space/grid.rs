use crate::{BoardSpaceManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use num_traits::{CheckedDiv, FromPrimitive, PrimInt, ToPrimitive, Unsigned};
use rayon::prelude::*;

pub enum GridOrigin {
    Center,
    Zero,
}

pub struct Grid<T> {
    indices: Vec<T>,
}

pub trait GridFactory<T, U, I>
where
    I: Iterator<Item = U>,
{
    fn new_with_origin(shape: I, origin: GridOrigin) -> Grid<T>;
    fn new(shape: I) -> Grid<T> {
        Self::new_with_origin(shape, GridOrigin::Center)
    }
}

impl<T> BoardSpaceManager<T, std::vec::IntoIter<T>, rayon::vec::IntoIter<T>> for Grid<T>
where
    T: Clone + Send + Sync,
{
    fn indices_iter(&self) -> std::vec::IntoIter<T> {
        self.indices.clone().into_iter()
    }

    fn indices_par_iter(&self) -> rayon::vec::IntoIter<T> {
        self.indices.clone().into_par_iter()
    }
}

impl<T, U, I> GridFactory<GridPointND<T>, U, I> for Grid<GridPointND<T>>
where
    T: PrimInt + CheckedDiv + std::convert::TryFrom<U> + Send + Sync,
    U: PrimInt + Unsigned + ToPrimitive + FromPrimitive + Send + Sync,
    I: Iterator<Item = U>,
{
    fn new_with_origin(shape: I, origin: GridOrigin) -> Grid<GridPointND<T>> {
        let shape_vec: Vec<U> = shape.collect();
        let indices = Self::indices_vec(&shape_vec, origin);
        Self { indices }
    }
}

impl<T> Grid<GridPointND<T>> {
    fn indices_vec<U>(shape: &Vec<U>, origin: GridOrigin) -> Vec<GridPointND<T>>
    where
        T: PrimInt + CheckedDiv + std::convert::TryFrom<U> + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + FromPrimitive + Send + Sync,
    {
        let mut num_cell = U::one();
        for dim in shape.iter() {
            num_cell = num_cell * *dim;
        }
        let zero_t = T::zero();
        let two_t = T::one() + T::one();
        (0..num_cell.to_u64().unwrap())
            .into_par_iter()
            .map(|i| {
                let i = U::from_u64(i).unwrap();
                let mut res = Vec::new();
                let mut cur = i;
                for dim in shape.iter() {
                    let dim_t = match T::try_from(*dim) {
                        Ok(val) => val,
                        Err(_) => panic!("Cannot convert size to index type."),
                    };
                    let cur_t = match T::try_from(cur) {
                        Ok(val) => val,
                        Err(_) => panic!("Cannot convert size to index type."),
                    };
                    let dim_idx = cur_t / dim_t
                        - match &origin {
                            GridOrigin::Center => dim_t / two_t,
                            GridOrigin::Zero => zero_t,
                        };
                    res.push(dim_idx);
                    cur = cur % *dim;
                }
                GridPointND { idx: res }
            })
            .collect()
    }
}

impl<T, U, I> GridFactory<GridPoint3D<T>, U, I> for Grid<GridPoint3D<T>>
where
    T: PrimInt + FromPrimitive + Send + Sync,
    U: PrimInt + Unsigned + ToPrimitive + Send + Sync,
    I: Iterator<Item = U>,
{
    fn new_with_origin(shape: I, origin: GridOrigin) -> Grid<GridPoint3D<T>> {
        let shape_vec: Vec<U> = shape.collect();
        assert_eq!(shape_vec.len(), 3);

        let (x_len, y_len, z_len) = (shape_vec[0], shape_vec[1], shape_vec[2]);

        let (x_half, y_half, z_half) = match origin {
            GridOrigin::Zero => (T::zero(), T::zero(), T::zero()),
            GridOrigin::Center => (
                T::from_u64(x_len.to_u64().unwrap() / 2).unwrap(),
                T::from_u64(y_len.to_u64().unwrap() / 2).unwrap(),
                T::from_u64(z_len.to_u64().unwrap() / 2).unwrap(),
            ),
        };

        let mut indices = Vec::new();
        for cur_x in 0..x_len.to_u64().unwrap() {
            for cur_y in 0..y_len.to_u64().unwrap() {
                for cur_z in 0..z_len.to_u64().unwrap() {
                    indices.push(GridPoint3D {
                        x: T::from_u64(cur_x).unwrap() - x_half,
                        y: T::from_u64(cur_y).unwrap() - y_half,
                        z: T::from_u64(cur_z).unwrap() - z_half,
                    });
                }
            }
        }
        Self { indices }
    }
}

impl<T, U, I> GridFactory<GridPoint2D<T>, U, I> for Grid<GridPoint2D<T>>
where
    T: PrimInt + FromPrimitive + Send + Sync,
    U: PrimInt + Unsigned + ToPrimitive + Send + Sync,
    I: Iterator<Item = U>,
{
    fn new_with_origin(shape: I, origin: GridOrigin) -> Grid<GridPoint2D<T>> {
        let shape_vec: Vec<U> = shape.collect();
        assert_eq!(shape_vec.len(), 2);

        let (x_len, y_len) = (shape_vec[0], shape_vec[1]);

        let (x_half, y_half) = match origin {
            GridOrigin::Zero => (T::zero(), T::zero()),
            GridOrigin::Center => (
                T::from_u64(x_len.to_u64().unwrap() / 2).unwrap(),
                T::from_u64(y_len.to_u64().unwrap() / 2).unwrap(),
            ),
        };

        let mut indices = Vec::new();
        for cur_x in 0..x_len.to_u64().unwrap() {
            for cur_y in 0..y_len.to_u64().unwrap() {
                indices.push(GridPoint2D {
                    x: T::from_u64(cur_x).unwrap() - x_half,
                    y: T::from_u64(cur_y).unwrap() - y_half,
                });
            }
        }
        Self { indices }
    }
}

impl<T, U, I> GridFactory<GridPoint1D<T>, U, I> for Grid<GridPoint1D<T>>
where
    T: PrimInt + FromPrimitive + Send + Sync,
    U: PrimInt + Unsigned + ToPrimitive + Send + Sync,
    I: Iterator<Item = U>,
{
    fn new_with_origin(shape: I, origin: GridOrigin) -> Grid<GridPoint1D<T>> {
        let shape_vec: Vec<U> = shape.collect();
        assert_eq!(shape_vec.len(), 1);

        let x_len = shape_vec[0];

        let x_half = match origin {
            GridOrigin::Zero => T::zero(),
            GridOrigin::Center => T::from_u64(x_len.to_u64().unwrap() / 2).unwrap(),
        };

        let mut indices = Vec::new();
        for cur_x in 0..x_len.to_u64().unwrap() {
            indices.push(GridPoint1D {
                x: T::from_u64(cur_x).unwrap() - x_half,
            });
        }
        Self { indices }
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::{
        BoardSpaceManager, Grid, GridFactory, GridOrigin, GridPoint1D, GridPoint2D, GridPoint3D,
        GridPointND,
    };
    use rayon::prelude::*;

    #[test]
    fn grid_1d_test_1() {
        type Point = GridPoint1D<i32>;

        let grid = Box::new(Grid::<Point>::new(vec![10u64].into_iter()))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let indices: Vec<Point> = grid.indices_iter().collect();
        let indices_par: Vec<Point> = grid.indices_par_iter().collect();
        assert_eq!(indices.len(), 10);
        assert_eq!(indices_par.len(), indices.len());
    }

    #[test]
    fn grid_1d_test_2() {
        type Point = GridPoint1D<i32>;

        let grid = Box::new(Grid::<Point>::new(vec![10u64].into_iter()))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let indices: Vec<Point> = grid.indices_iter().collect();
        let indices_par: Vec<Point> = grid.indices_par_iter().collect();
        assert_eq!(indices.len(), 10);
        assert_eq!(indices_par.len(), indices.len());
    }

    #[test]
    fn grid_2d_test_1() {
        type Point = GridPoint2D<i64>;

        let grid = Box::new(Grid::<Point>::new(vec![5u64, 10].into_iter()))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let indices: Vec<Point> = grid.indices_iter().collect();
        let indices_par: Vec<Point> = grid.indices_par_iter().collect();
        assert_eq!(indices.len(), 50);
        assert_eq!(indices_par.len(), indices.len());
    }

    #[test]
    fn grid_3d_test_1() {
        type Point = GridPoint3D<i32>;

        let grid = Box::new(Grid::<Point>::new(vec![5u64, 10, 6].into_iter()))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let indices: Vec<Point> = grid.indices_iter().collect();
        let indices_par: Vec<Point> = grid.indices_par_iter().collect();
        assert_eq!(indices.len(), 300);
        assert_eq!(indices_par.len(), indices.len());
    }

    #[test]
    fn grid_nd_test_1() {
        type Point = GridPointND<i32>;

        let grid = Box::new(Grid::<Point>::new(vec![5u64, 10, 6, 10].into_iter()))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let indices: Vec<Point> = grid.indices_iter().collect();
        let indices_par: Vec<Point> = grid.indices_par_iter().collect();
        assert_eq!(indices.len(), 3000);
        assert_eq!(indices_par.len(), indices.len());
    }

    #[test]
    fn grid_nd_test_2() {
        type Point = GridPointND<i32>;
        let board_size = vec![2u32, 2, 2, 2, 2];

        let grid_1 = Box::new(Grid::<Point>::new(board_size.clone().into_iter()))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let grid_2 = Box::new(Grid::<Point>::new_with_origin(
            board_size.into_iter(),
            GridOrigin::Zero,
        ))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let indices_1: Vec<Point> = grid_1.indices_iter().collect();
        let indices_par_1: Vec<Point> = grid_1.indices_par_iter().collect();
        let indices_2: Vec<Point> = grid_2.indices_iter().collect();
        let indices_par_2: Vec<Point> = grid_2.indices_par_iter().collect();
        assert_eq!(indices_1.len(), 32);
        assert_eq!(indices_par_1.len(), indices_1.len());
        assert_eq!(indices_2.len(), indices_1.len());
        assert_eq!(indices_par_2.len(), indices_1.len());
    }
}
