use gol_core::{BoardCallback, GridPoint2D, IndexedDataOwned};
use rayon::prelude::*;
use std::cmp::{max, min};

pub struct TextRendererGrid2D {
    has_cell_boarder: bool,
}

impl<T, U, I> BoardCallback<T, GridPoint2D<U>, I> for TextRendererGrid2D
where
    T: Send + Sync + Clone + std::convert::Into<char>,
    U: Send + Sync + Clone + Ord,
    I: ParallelIterator<Item = IndexedDataOwned<GridPoint2D<U>, T>>,
{
    fn execute(&self, states: I) {
        let states: Vec<IndexedDataOwned<GridPoint2D<U>, T>> = states.collect();
        let (x_min, x_max, y_min, y_max) = find_2d_bounds(&states);
        println!("xy bounds {}", states.first().unwrap().1.clone().into());
    }
}

impl TextRendererGrid2D {
    pub fn new(has_cell_boarder: bool) -> Self {
        Self { has_cell_boarder }
    }
}

fn find_2d_bounds<T, U>(
    idx_and_state_vec: &Vec<IndexedDataOwned<GridPoint2D<U>, T>>,
) -> (U, U, U, U)
where
    T: Send + Sync,
    U: Send + Sync + Ord + Clone,
{
    let xy_bounds = idx_and_state_vec
        .par_iter()
        .fold(
            || None,
            |res, ele: &IndexedDataOwned<GridPoint2D<U>, T>| {
                let (x, y) = (&ele.0.x, &ele.0.y);
                match res {
                    None => Some([x.clone(), x.clone(), y.clone(), y.clone()]),
                    Some(val) => {
                        let (x_min, x_max) = (
                            min(val[0].clone(), x.clone()),
                            max(val[1].clone(), x.clone()),
                        );
                        let (y_min, y_max) = (
                            min(val[2].clone(), y.clone()),
                            max(val[3].clone(), y.clone()),
                        );
                        Some([x_min, x_max, y_min, y_max])
                    }
                }
            },
        )
        .reduce(
            || None,
            |res, ele| {
                if res.is_none() && ele.is_none() {
                    None
                } else if res.is_none() {
                    ele
                } else if ele.is_none() {
                    res
                } else {
                    let (a, b) = (res.unwrap(), ele.unwrap());
                    Some([
                        min(a[0].clone(), b[0].clone()),
                        max(a[0].clone(), b[0].clone()),
                        min(a[0].clone(), b[0].clone()),
                        max(a[0].clone(), b[0].clone()),
                    ])
                }
            },
        )
        .unwrap();
    (
        xy_bounds[0].clone(),
        xy_bounds[1].clone(),
        xy_bounds[2].clone(),
        xy_bounds[3].clone(),
    )
}
