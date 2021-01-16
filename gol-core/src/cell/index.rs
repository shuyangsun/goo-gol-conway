pub trait ToGridPointND<T>
where
    T: Clone,
{
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
    indices: Vec<T>,
}

impl<T> ToGridPointND<T> for GridPoint1D<T>
where
    T: Clone,
{
    fn to_nd(&self) -> GridPointND<T> {
        GridPointND::new(vec![self.x.clone()].iter())
    }
}

impl<T> ToGridPointND<T> for GridPoint2D<T>
where
    T: Clone,
{
    fn to_nd(&self) -> GridPointND<T> {
        GridPointND::new(vec![self.x.clone(), self.y.clone()].iter())
    }
}

impl<T> ToGridPointND<T> for GridPoint3D<T>
where
    T: Clone,
{
    fn to_nd(&self) -> GridPointND<T> {
        GridPointND::new(vec![self.x.clone(), self.y.clone(), self.y.clone()].iter())
    }
}

impl<T> GridPointND<T> {
    pub fn new<'a, 'b, I>(indices: I) -> Self
    where
        'a: 'b,
        T: 'a + Clone,
        I: Iterator<Item = &'b T>,
    {
        Self {
            indices: indices.map(|ele| ele.clone()).collect(),
        }
    }

    pub fn indices<'a>(&'a self) -> std::slice::Iter<'a, T> {
        self.indices.iter()
    }

    pub fn to_1d(&self) -> Option<GridPoint1D<T>>
    where
        T: Clone,
    {
        let mut iter = self.indices();
        let x = iter.next();
        if x.is_none() {
            return None;
        }
        match iter.next() {
            Some(_) => None,
            None => Some(GridPoint1D {
                x: x.unwrap().clone(),
            }),
        }
    }

    pub fn to_2d(&self) -> Option<GridPoint2D<T>>
    where
        T: Clone,
    {
        let mut iter = self.indices();
        let x = iter.next();
        if x.is_none() {
            return None;
        }
        let y = iter.next();
        if y.is_none() {
            return None;
        }
        match iter.next() {
            Some(_) => None,
            None => Some(GridPoint2D {
                x: x.unwrap().clone(),
                y: y.unwrap().clone(),
            }),
        }
    }

    pub fn to_3d(&self) -> Option<GridPoint3D<T>>
    where
        T: Clone,
    {
        let mut iter = self.indices();
        let x = iter.next();
        if x.is_none() {
            return None;
        }
        let y = iter.next();
        if y.is_none() {
            return None;
        }
        let z = iter.next();
        if z.is_none() {
            return None;
        }
        match iter.next() {
            Some(_) => None,
            None => Some(GridPoint3D {
                x: x.unwrap().clone(),
                y: y.unwrap().clone(),
                z: z.unwrap().clone(),
            }),
        }
    }
}
