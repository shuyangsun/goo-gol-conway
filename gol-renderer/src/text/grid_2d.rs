extern crate ncurses;

use gol_core::{BoardCallback, GridPoint2D, IndexedDataOwned};
use ncurses::*;
use num_traits::{CheckedSub, ToPrimitive};
use rayon::prelude::*;
use std::cmp::{max, min};

pub struct TextRendererGrid2D {}

impl<T, U, I> BoardCallback<T, GridPoint2D<U>, I> for TextRendererGrid2D
where
    T: Send + Sync + Clone + std::convert::Into<char>,
    U: Send + Sync + Clone + Ord + CheckedSub + ToPrimitive,
    I: ParallelIterator<Item = IndexedDataOwned<GridPoint2D<U>, T>>,
{
    fn setup(&mut self) {
        initscr();
        raw();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        addstr("John Conway's Game of Life");
        refresh();
    }

    fn cleanup(&mut self) {
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        endwin();
    }

    fn execute(&mut self, states: I) {
        let states: Vec<IndexedDataOwned<GridPoint2D<U>, T>> = states.collect();
        let (x_min, x_max, y_min, y_max) = find_2d_bounds(&states);

        let win_width = x_max.checked_sub(&x_min).unwrap().to_i32().unwrap() + 4;
        let win_height = y_max.checked_sub(&y_min).unwrap().to_i32().unwrap() + 4;

        /* Get the screen bounds. */
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        /* Start in the center. */
        let start_y = (max_y - win_height) / 2;
        let start_x = (max_x - win_width) / 2;
        let win = create_win(start_y, start_x, win_height, win_width);

        for (idx, state) in states.iter() {
            let cur_x = idx.x.checked_sub(&x_min).unwrap().to_i32().unwrap() + 1;
            let cur_y = y_max.checked_sub(&idx.y).unwrap().to_i32().unwrap() + 2;
            let ch: char = state.clone().into();
            mvwprintw(win, cur_y, cur_x, ch.to_string().as_str());
        }
        wrefresh(win);
    }
}

impl TextRendererGrid2D {
    pub fn new() -> Self {
        Self {}
    }
}

fn create_win(start_y: i32, start_x: i32, win_height: i32, win_width: i32) -> WINDOW {
    let win = newwin(win_height, win_width, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn find_2d_bounds<T, U>(
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
