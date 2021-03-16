use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shape1D {
    width: usize,
}

impl Shape1D {
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn volume(&self) -> usize {
        self.width()
    }

    pub fn x_idx_min(&self) -> i64 {
        dim_idx_min(self.width())
    }

    pub fn x_idx_max(&self) -> i64 {
        dim_idx_max(self.width())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shape2D {
    width: usize,
    height: usize,
}

impl Shape2D {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn volume(&self) -> usize {
        self.width() * self.height()
    }

    pub fn x_idx_min(&self) -> i64 {
        dim_idx_min(self.width())
    }

    pub fn x_idx_max(&self) -> i64 {
        dim_idx_max(self.width())
    }

    pub fn y_idx_min(&self) -> i64 {
        dim_idx_min(self.height())
    }

    pub fn y_idx_max(&self) -> i64 {
        dim_idx_max(self.height())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shape3D {
    width: usize,
    height: usize,
    depth: usize,
}

impl Shape3D {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn volume(&self) -> usize {
        self.width() * self.height() * self.depth()
    }

    pub fn x_idx_min(&self) -> i64 {
        dim_idx_min(self.width())
    }

    pub fn x_idx_max(&self) -> i64 {
        dim_idx_max(self.width())
    }

    pub fn y_idx_min(&self) -> i64 {
        dim_idx_min(self.height())
    }

    pub fn y_idx_max(&self) -> i64 {
        dim_idx_max(self.height())
    }

    pub fn z_idx_min(&self) -> i64 {
        dim_idx_min(self.depth())
    }

    pub fn z_idx_max(&self) -> i64 {
        dim_idx_max(self.depth())
    }
}

#[inline(always)]
fn dim_idx_min(len: usize) -> i64 {
    -(len as i64) / 2
}

#[inline(always)]
fn dim_idx_max(len: usize) -> i64 {
    len as i64 + dim_idx_min(len) - 1
}
