pub trait ToGridPointND<T> {
    fn to_nd(&self) -> GridPointND<T>;
}

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

impl<T> ToGridPointND<T> for GridPoint1D<T> {
    fn to_nd(&self) -> GridPointND<T> {
        GridPointND{ idx: vec![self.x] }
    }
}

impl<T> ToGridPointND<T> for GridPoint2D<T> {
    fn to_nd(&self) -> GridPointND<T> {
        GridPointND{ idx: vec![self.x, self.y] }
    }
}

impl<T> ToGridPointND<T> for GridPoint3D<T> {
    fn to_nd(&self) -> GridPointND<T> {
        GridPointND{ idx: vec![self.x, self.y, self.z] }
    }
}

impl<T> GridPointND<T> {
    pub fn to_1d(&self) -> Option<GridPoint1D<T>>  {
        match self.idx.len() {
            1 => Some(GridPoint1D { x: self.idx[0] }),
            _ => None
        }
    }

    pub fn to_2d(&self) -> Option<GridPoint2D<T>> {
        match self.idx.len() {
            2 => Some(GridPoint2D { x: self.idx[0], y: self.idx[1] }),
            _ => None
        }
    }

    pub fn to_3d(&self) -> Option<GridPoint3D<T>> {
        match self.idx.len() {
            3 => Some(GridPoint3D { x: self.idx[0], y: self.idx[1], z: self.idx[2] }),
            _ => None
        }
    }
}
