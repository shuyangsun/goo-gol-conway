#[derive(Clone)]
pub struct GridPoint1D<T> {
    pub x: T,
}

#[derive(Clone)]
pub struct GridPoint2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone)]
pub struct GridPoint3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Clone)]
pub struct GridPointND<T> {
    pub idx: Vec<T>,
}
