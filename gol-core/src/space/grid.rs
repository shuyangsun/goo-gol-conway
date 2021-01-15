use crate::{BoardSpaceManager, GridPointND};
use num_traits::{CheckedDiv, FromPrimitive, PrimInt, ToPrimitive, Unsigned};
use rayon::prelude::*;

pub struct GridND<T> {
    indices_cache: Vec<GridPointND<T>>,
}

impl<T>
    BoardSpaceManager<
        GridPointND<T>,
        std::vec::IntoIter<GridPointND<T>>,
        rayon::vec::IntoIter<GridPointND<T>>,
    > for GridND<T>
where
    T: PrimInt + Send + Sync,
{
    fn indices_iter(&self) -> std::vec::IntoIter<GridPointND<T>> {
        self.indices_cache.clone().into_iter()
    }

    fn indices_par_iter(&self) -> rayon::vec::IntoIter<GridPointND<T>> {
        self.indices_cache.clone().into_par_iter()
    }
}

impl<T> GridND<T> {
    pub fn new<U, I>(shape: I) -> Self
    where
        T: PrimInt + CheckedDiv + std::convert::TryFrom<U> + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + FromPrimitive + Send + Sync,
        I: Iterator<Item = U>,
    {
        let shape_vec: Vec<U> = shape.collect();
        let indices_cache = Self::indices_vec(&shape_vec);
        Self { indices_cache }
    }

    fn indices_vec<U>(shape: &Vec<U>) -> Vec<GridPointND<T>>
    where
        T: PrimInt + CheckedDiv + std::convert::TryFrom<U> + Send + Sync,
        U: PrimInt + Unsigned + ToPrimitive + FromPrimitive + Send + Sync,
    {
        let mut num_cell = U::one();
        for dim in shape.iter() {
            num_cell = num_cell * *dim;
        }
        (0..num_cell.to_u64().unwrap())
            .into_par_iter()
            .map(|i| {
                let i = U::from_u64(i).unwrap();
                let mut res = Vec::new();
                let mut cur = i;
                for dim in shape.iter() {
                    res.push(T::try_from(cur / *dim).ok().unwrap());
                    cur = cur % *dim;
                }
                GridPointND { idx: res }
            })
            .collect()
    }
}
