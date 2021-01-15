#[derive(Clone, Copy)]
pub struct GridPoint1D(i64);
#[derive(Clone, Copy)]
pub struct GridPoint2D(i64, i64);
#[derive(Clone, Copy)]
pub struct GridPoint3D(i64, i64, i64);
#[derive(Clone)]
pub struct GridPointND(Vec<i64>);
