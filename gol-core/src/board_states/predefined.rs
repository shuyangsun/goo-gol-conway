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

/// Center is top left cell.
pub fn conway_2d_eater<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut res = HashSet::new();

    let zero = T::zero();
    let one = T::one();
    let two = one + one;
    let three = one + two;

    res.insert(GridPoint2D::new(zero, zero));
    res.insert(GridPoint2D::new(zero, one.neg()));
    res.insert(GridPoint2D::new(one, zero));
    res.insert(GridPoint2D::new(two, one.neg()));
    res.insert(GridPoint2D::new(two, two.neg()));
    res.insert(GridPoint2D::new(two, three.neg()));
    res.insert(GridPoint2D::new(three, three.neg()));

    res
}

pub fn conway_2d_glider_gun_with_eater<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut gun = conway_2d_glider_gun();
    let mut eater = conway_2d_eater();

    let one = T::one();
    let two = one + one;
    let three = one + two;
    let five = two * two + one;

    gun = gun
        .iter()
        .map(|ele| GridPoint2D::new(ele.x - five, ele.y + five * two))
        .collect();

    eater = eater
        .iter()
        .map(|ele| GridPoint2D::new(ele.x + five * (five - one), ele.y - (five * three) + two))
        .collect();

    gun.extend(eater);
    gun
}

pub fn conway_2d_and_gate_11<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut gun_top = conway_2d_glider_gun();

    let one = T::one();
    let two = one + one;
    let three = one + two;
    let five = two * two + one;
    let six = five + one;
    let top_right_x_shift = six * six + two;

    let gun_right: HashSet<GridPoint2D<T>> = gun_top
        .iter()
        .map(|ele: &GridPoint2D<T>| {
            GridPoint2D::new(ele.x.neg() + top_right_x_shift, ele.y + five * five - two)
        })
        .collect();

    gun_top = gun_top
        .iter()
        .map(|ele| GridPoint2D::new(ele.x - five * two - two, ele.y + five * five - one))
        .collect();

    let gun: HashSet<GridPoint2D<T>> = conway_2d_glider_gun();
    let gun_bottom: HashSet<GridPoint2D<T>> = gun
        .iter()
        .map(|ele| GridPoint2D::new(ele.x - three * three * three, ele.y + two * three))
        .collect();

    gun_top.extend(gun_bottom);
    gun_top.extend(gun_right);
    gun_top
}

pub fn conway_2d_and_gate_01<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut guns = conway_2d_and_gate_11();

    let one = T::one();
    let two = one + one;
    let four = two * two;

    let eater: HashSet<GridPoint2D<T>> = conway_2d_eater()
        .iter()
        .map(|ele: &GridPoint2D<T>| GridPoint2D::new(ele.x - four, ele.y + four * four + two))
        .collect();

    guns.extend(eater);
    guns
}

pub fn conway_2d_and_gate_10<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut guns = conway_2d_and_gate_11();

    let one = T::one();
    let two = one + one;
    let four = two * two;
    let x_shift = four * four + two + one;

    let eater: HashSet<GridPoint2D<T>> = conway_2d_eater()
        .iter()
        .map(|ele: &GridPoint2D<T>| GridPoint2D::new(ele.x - x_shift, ele.y))
        .collect();

    guns.extend(eater);
    guns
}

pub fn conway_2d_and_gate_00<T>() -> HashSet<GridPoint2D<T>>
where
    T: Hash + PrimInt + Signed,
{
    let mut a = conway_2d_and_gate_01();
    let b = conway_2d_and_gate_10();

    a.extend(b);
    a
}
