use num_traits::{CheckedAdd, CheckedSub, FromPrimitive, PrimInt, Signed, ToPrimitive, Unsigned};

pub trait MarginPrimInt: Send + Sync + PrimInt + ToPrimitive + Unsigned {}
pub trait PointPrimInt:
    Send + Sync + PrimInt + CheckedAdd + CheckedSub + FromPrimitive + Signed
{
}

impl<T> MarginPrimInt for T where T: Send + Sync + PrimInt + ToPrimitive + Unsigned {}
impl<T> PointPrimInt for T where
    T: Send + Sync + PrimInt + CheckedAdd + CheckedSub + FromPrimitive + Signed
{
}
