use gol_core::{GridPoint2D, IndexedDataOwned};
use rayon::prelude::*;
use std::cmp::{max, min};

pub fn find_2d_bounds<T, U>(
    idx_and_state_vec: &Vec<IndexedDataOwned<GridPoint2D<U>, T>>,
) -> (U, U, U, U)
where
    T: Send + Sync,
    U: Send + Sync + Ord + Clone,
{
    // Another simpler but slower way:
    // idx_and_state_vec.par_sort_by(|a, b| b.0.y.cmp(&a.0.y).then(a.0.x.cmp(&b.0.x)));
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
                        max(a[1].clone(), b[1].clone()),
                        min(a[2].clone(), b[2].clone()),
                        max(a[3].clone(), b[3].clone()),
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
