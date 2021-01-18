use crate::GridPoint2D;
use num_traits::{PrimInt, Signed};
use std::collections::HashSet;
use std::hash::Hash;

pub fn conway_2d_tetris<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut res = HashSet::new();
    res.insert(GridPoint2D::new(T::zero(), T::zero()));
    res.insert(GridPoint2D::new(T::zero(), T::one()));
    res.insert(GridPoint2D::new(T::one(), T::zero()));
    res.insert(GridPoint2D::new(T::one().neg(), T::zero()));
    res
}
