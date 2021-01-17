extern crate ncurses;

use gol_core::{BoardCallback, GridPoint2D, IndexedDataOwned};
use ncurses::*;
use num_traits::{CheckedSub, ToPrimitive};
use rayon::prelude::*;
use std::cmp::{max, min};

pub struct TextRendererGrid2D {
    has_cell_boarder: bool,
}

impl<T, U, I> BoardCallback<T, GridPoint2D<U>, I> for TextRendererGrid2D
where
    T: Send + Sync + Clone + std::convert::Into<char>,
    U: Send + Sync + Clone + Ord + CheckedSub + ToPrimitive,
    I: ParallelIterator<Item = IndexedDataOwned<GridPoint2D<U>, T>>,
{
    fn execute(&self, states: I) {
        let mut states: Vec<IndexedDataOwned<GridPoint2D<U>, T>> = states.collect();
        states.par_sort_by(|a, b| b.0.y.cmp(&a.0.y).then(a.0.x.cmp(&b.0.x)));

        let (first, last) = (&states.first().unwrap().0, &states.last().unwrap().0);
        let (x_min, y_max) = (first.x.clone(), first.y.clone());
        let (x_max, y_min) = (last.x.clone(), last.y.clone());

        let win_width = x_max.checked_sub(&x_min).unwrap().to_i32().unwrap();
        let win_height = y_max.checked_sub(&y_min).unwrap().to_i32().unwrap();

        initscr();
        raw();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        // addstr("John Conway's Game of Life"); // TODO: call in setup.
        // refresh();

        /* Get the screen bounds. */
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        /* Start in the center. */
        let mut start_y = (max_y - win_height) / 2;
        let mut start_x = (max_x - win_width) / 2;
        // let mut win = create_win(start_y, start_x, win_height, win_width);

        for (idx, state) in states.iter() {
            let cur_x = start_x + idx.x.checked_sub(&x_min).unwrap().to_i32().unwrap();
            let cur_y = start_y + y_max.checked_sub(&idx.y).unwrap().to_i32().unwrap();
            let ch: char = state.clone().into();
            mvprintw(cur_y, cur_x, ch.to_string().as_str());
        }
        refresh();
        // wrefresh(win);

        // TODO: store window.
        // destroy_win(win);
    }
}

impl TextRendererGrid2D {
    pub fn new(has_cell_boarder: bool) -> Self {
        Self { has_cell_boarder }
    }
}

fn create_win(start_y: i32, start_x: i32, win_height: i32, win_width: i32) -> WINDOW {
    let win = newwin(win_height, win_width, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}

// fn find_2d_bounds<T, U>(
//     idx_and_state_vec: &Vec<IndexedDataOwned<GridPoint2D<U>, T>>,
// ) -> (U, U, U, U)
// where
//     T: Send + Sync,
//     U: Send + Sync + Ord + Clone,
// {
//     let xy_bounds = idx_and_state_vec
//         .par_iter()
//         .fold(
//             || None,
//             |res, ele: &IndexedDataOwned<GridPoint2D<U>, T>| {
//                 let (x, y) = (&ele.0.x, &ele.0.y);
//                 match res {
//                     None => Some([x.clone(), x.clone(), y.clone(), y.clone()]),
//                     Some(val) => {
//                         let (x_min, x_max) = (
//                             min(val[0].clone(), x.clone()),
//                             max(val[1].clone(), x.clone()),
//                         );
//                         let (y_min, y_max) = (
//                             min(val[2].clone(), y.clone()),
//                             max(val[3].clone(), y.clone()),
//                         );
//                         Some([x_min, x_max, y_min, y_max])
//                     }
//                 }
//             },
//         )
//         .reduce(
//             || None,
//             |res, ele| {
//                 if res.is_none() && ele.is_none() {
//                     None
//                 } else if res.is_none() {
//                     ele
//                 } else if ele.is_none() {
//                     res
//                 } else {
//                     let (a, b) = (res.unwrap(), ele.unwrap());
//                     Some([
//                         min(a[0].clone(), b[0].clone()),
//                         max(a[0].clone(), b[0].clone()),
//                         min(a[0].clone(), b[0].clone()),
//                         max(a[0].clone(), b[0].clone()),
//                     ])
//                 }
//             },
//         )
//         .unwrap();
//     (
//         xy_bounds[0].clone(),
//         xy_bounds[1].clone(),
//         xy_bounds[2].clone(),
//         xy_bounds[3].clone(),
//     )
// }
