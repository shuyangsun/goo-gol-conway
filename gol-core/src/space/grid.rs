use crate::{BoardSpaceManager, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
use num_traits::{CheckedDiv, FromPrimitive, PrimInt, ToPrimitive, Unsigned};
use rayon::prelude::*;

pub enum GridOrigin {
    Center,
    Zero,
}

pub struct GridND<T> {
    indices: Vec<T>,
}

impl<T> BoardSpaceManager<T, std::vec::IntoIter<T>, rayon::vec::IntoIter<T>> for GridND<T>
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

impl<T> GridND<GridPointND<T>> {
    pub fn new<U, I>(shape: I) -> Self
    where
        T: PrimInt + CheckedDiv + std::convert::TryFrom<U> + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + FromPrimitive + Send + Sync,
        I: Iterator<Item = U>,
    {
        Self::new_with_origin(shape, GridOrigin::Center)
    }

    pub fn new_with_origin<U, I>(shape: I, origin: GridOrigin) -> Self
    where
        T: PrimInt + CheckedDiv + std::convert::TryFrom<U> + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + FromPrimitive + Send + Sync,
        I: Iterator<Item = U>,
    {
        let shape_vec: Vec<U> = shape.collect();
        let indices = Self::indices_vec(&shape_vec, origin);
        Self { indices }
    }

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

impl<T> GridND<GridPoint3D<T>> {
    pub fn new<U>(x: U, y: U, z: U) -> Self
    where
        T: PrimInt + FromPrimitive + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + Send + Sync,
    {
        let mut indices = Vec::new();
        for cur_x in 0..x.to_u64().unwrap() {
            for cur_y in 0..y.to_u64().unwrap() {
                for cur_z in 0..z.to_u64().unwrap() {
                    indices.push(GridPoint3D {
                        x: T::from_u64(cur_x).unwrap(),
                        y: T::from_u64(cur_y).unwrap(),
                        z: T::from_u64(cur_z).unwrap(),
                    });
                }
            }
        }
        Self { indices }
    }
}

impl<T> GridND<GridPoint2D<T>> {
    pub fn new<U>(x: U, y: U) -> Self
    where
        T: PrimInt + FromPrimitive + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + Send + Sync,
    {
        let mut indices = Vec::new();
        for cur_x in 0..x.to_u64().unwrap() {
            for cur_y in 0..y.to_u64().unwrap() {
                indices.push(GridPoint2D {
                    x: T::from_u64(cur_x).unwrap(),
                    y: T::from_u64(cur_y).unwrap(),
                });
            }
        }
        Self { indices }
    }
}

impl<T> GridND<GridPoint1D<T>> {
    pub fn new<U>(x: U) -> Self
    where
        T: PrimInt + FromPrimitive + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + Send + Sync,
    {
        let mut indices = Vec::new();
        for cur_x in 0..x.to_u64().unwrap() {
            indices.push(GridPoint1D {
                x: T::from_u64(cur_x).unwrap(),
            });
        }
        Self { indices }
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::{
        BoardSpaceManager, GridND, GridOrigin, GridPoint1D, GridPoint2D, GridPoint3D, GridPointND,
    };
    use rayon::prelude::*;

    #[test]
    fn grid_1d_test_1() {
        type Point = GridPoint1D<i32>;

        let grid = Box::new(GridND::<Point>::new(10u64))
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

        let grid = Box::new(GridND::<Point>::new(10u64))
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

        let grid = Box::new(GridND::<Point>::new(5u64, 10))
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

        let grid = Box::new(GridND::<Point>::new(5u64, 10, 6))
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

        let grid = Box::new(GridND::<Point>::new(vec![5u64, 10, 6, 10].into_iter()))
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

        let grid_1 = Box::new(GridND::<Point>::new(board_size.clone().into_iter()))
            as Box<
                dyn BoardSpaceManager<
                    Point,
                    std::vec::IntoIter<Point>,
                    rayon::vec::IntoIter<Point>,
                >,
            >;
        let grid_2 = Box::new(GridND::<Point>::new_with_origin(
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
