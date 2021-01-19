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

pub fn conway_2d_glider<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut res = HashSet::new();
    res.insert(GridPoint2D::new(T::one().neg(), T::zero()));
    res.insert(GridPoint2D::new(T::one(), T::zero()));
    res.insert(GridPoint2D::new(T::one(), T::one()));
    res.insert(GridPoint2D::new(T::zero(), T::one().neg()));
    res.insert(GridPoint2D::new(T::one(), T::one().neg()));
    res
}

pub fn conway_2d_glider_gun<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    // Predefine some numbers
    let zero = T::zero();
    let one = T::one();
    let two = one + one;
    let three = one + two;
    let four = two * two;
    let pos_18 = four * four + two; // x-axis max
    let neg_17 = (four * four + one).neg(); // x-axis min

    let mut res = HashSet::new();

    // Left Square
    res.insert(GridPoint2D::new(neg_17, zero));
    res.insert(GridPoint2D::new(neg_17 + one, zero));
    res.insert(GridPoint2D::new(neg_17, one));
    res.insert(GridPoint2D::new(neg_17 + one, one));

    // Right Square
    res.insert(GridPoint2D::new(pos_18, two));
    res.insert(GridPoint2D::new(pos_18 - one, two));
    res.insert(GridPoint2D::new(pos_18, two + one));
    res.insert(GridPoint2D::new(pos_18 - one, two + one));

    // Center Left
    res.insert(GridPoint2D::new(zero, zero));
    res.insert(GridPoint2D::new(one.neg(), zero));
    res.insert(GridPoint2D::new(one.neg(), one));
    res.insert(GridPoint2D::new(one.neg(), one.neg()));
    res.insert(GridPoint2D::new(two.neg(), two));
    res.insert(GridPoint2D::new(two.neg(), two.neg()));
    res.insert(GridPoint2D::new((two + one).neg(), zero));
    res.insert(GridPoint2D::new(four.neg(), three));
    res.insert(GridPoint2D::new(four.neg(), (three).neg()));
    res.insert(GridPoint2D::new((one + four).neg(), three));
    res.insert(GridPoint2D::new((one + four).neg(), (three).neg()));
    res.insert(GridPoint2D::new((two + four).neg(), two));
    res.insert(GridPoint2D::new((two + four).neg(), two.neg()));
    res.insert(GridPoint2D::new((three + four).neg(), zero));
    res.insert(GridPoint2D::new((three + four).neg(), one));
    res.insert(GridPoint2D::new((three + four).neg(), one.neg()));

    // Center Right
    res.insert(GridPoint2D::new(two + one, one));
    res.insert(GridPoint2D::new(two + one, two));
    res.insert(GridPoint2D::new(two + one, three));
    res.insert(GridPoint2D::new(four, one));
    res.insert(GridPoint2D::new(four, two));
    res.insert(GridPoint2D::new(four, three));
    res.insert(GridPoint2D::new(four + one, zero));
    res.insert(GridPoint2D::new(four + one, four));
    res.insert(GridPoint2D::new(four + two + one, one.neg()));
    res.insert(GridPoint2D::new(four + two + one, zero));
    res.insert(GridPoint2D::new(four + two + one, four));
    res.insert(GridPoint2D::new(four + two + one, four + one));

    res
}

pub fn conway_2d_glider_gun_with_eater<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut res = conway_2d_glider_gun();

    let one = T::one();
    let two = one + one;
    let five = two * two + one;
    let pos_25 = five * five;
    let neg_23 = (pos_25 - two).neg();

    // Eater:
    res.insert(GridPoint2D::new(pos_25, neg_23));
    res.insert(GridPoint2D::new(pos_25, neg_23 - one));
    res.insert(GridPoint2D::new(pos_25 + one, neg_23));
    res.insert(GridPoint2D::new(pos_25 + two, neg_23 - one));
    res.insert(GridPoint2D::new(pos_25 + two, pos_25.neg()));
    res.insert(GridPoint2D::new(pos_25 + two, pos_25.neg() - one));
    res.insert(GridPoint2D::new(pos_25 + two + one, pos_25.neg() - one));

    res = res
        .iter()
        .map(|ele| GridPoint2D::new(ele.x - five, ele.y + five * two))
        .collect();

    res
}
