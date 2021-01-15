use crate::{BoardSpaceManager, GridPointND};
use rayon::prelude::*;

pub struct GridND {
    shape: Vec<usize>,
    indices_cache: Vec<GridPointND>,
}

impl
    BoardSpaceManager<
        GridPointND,
        std::vec::IntoIter<GridPointND>,
        rayon::vec::IntoIter<GridPointND>,
    > for GridND
{
    fn indices_iter(&self) -> std::vec::IntoIter<GridPointND> {
        self.indices_cache.clone().into_iter()
    }

    fn indices_par_iter(&self) -> rayon::vec::IntoIter<GridPointND> {
        self.indices_cache.clone().into_par_iter()
    }
}

impl GridND {
    fn new<I>(shape: I) -> Self
    where
        I: Iterator<Item = usize>,
    {
        let shape_vec = shape.collect();
        let indices_cache = Self::indices_vec(&shape_vec);
        Self {
            shape: shape_vec,
            indices_cache,
        }
    }

    fn indices_vec(shape: &Vec<usize>) -> Vec<GridPointND> {
        // let num_cell = shape.iter().product();
        // let mut final_iter = None;
        // for &dim in shape.iter() {
        //     let cur_iter = std::iter::repeat(0usize..dim)
        //         .take(num_cell / dim)
        //         .flatten()
        //         .into_iter();
        //     final_iter = match final_iter {
        //         None => Some(cur_iter),
        //         Some(prev_iter) => Some(prev_iter.zip(cur_iter)),
        //     }
        // }
        Vec::new()
    }
}
