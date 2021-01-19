use num_traits::{CheckedAdd, CheckedSub, FromPrimitive, PrimInt, ToPrimitive};

pub trait MarginPrimInt: Send + Sync + PrimInt + ToPrimitive {}
pub trait PointPrimInt: Send + Sync + PrimInt + CheckedAdd + CheckedSub + FromPrimitive {}

impl<T> MarginPrimInt for T where T: Send + Sync + PrimInt + ToPrimitive {}
impl<T> PointPrimInt for T where T: Send + Sync + PrimInt + CheckedAdd + CheckedSub + FromPrimitive {}
