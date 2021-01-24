extern crate ncurses;

use crate::CharMapping;
use gol_core::{BoardCallback, GridPoint2D, IndexedDataOwned};
use ncurses::*;
use num_traits::{CheckedSub, FromPrimitive, ToPrimitive};
use rayon::prelude::*;
use std::char;
use std::cmp::{max, min};
use std::time::Instant;
use tokio::sync::broadcast::{error::TryRecvError, Receiver};

const TITLE_ROW: i32 = 1;
const GENERATION_ROW: i32 = 3;

pub struct TextRendererGrid2D<M> {
    title: String,
    iter: usize,
    is_enabled: bool,
    screen_dim: (i32, i32),
    window_dim: Option<(i32, i32, i32, i32)>,
    rx: Option<Receiver<char>>,
    speed_level: i32,
    last_render_time: Option<Instant>,
    char_map: M,
}

impl<T, U, I, M> BoardCallback<T, GridPoint2D<U>, I> for TextRendererGrid2D<M>
where
    T: Send + Sync + Clone,
    U: Send + Sync + Clone + Ord + CheckedSub + ToPrimitive + FromPrimitive,
    I: ParallelIterator<Item = IndexedDataOwned<GridPoint2D<U>, T>>,
    M: Send + Sync + CharMapping<T>,
{
    fn setup(&mut self) {
        initscr();
        raw();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        /* Get the screen bounds. */
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        self.screen_dim = (max_y, max_x);

        mvprintw(
            TITLE_ROW,
            (max_x as usize - self.title.len()) as i32 / 2,
            self.title.as_str(),
        );

        let message = "SPACE: play/pause, k/j: speed up/down, q: exit.";
        mvprintw(
            max_y - 2,
            (max_x as usize - message.len()) as i32 / 2,
            message,
        );

        refresh();
    }

    fn cleanup(&mut self) {
        self.cleanup_impl();
    }

    fn execute(&mut self, states: I) {
        if self.is_enabled {
            self.print_generation_and_speed();
            self.last_render_time = Some(Instant::now());

            let states: Vec<IndexedDataOwned<GridPoint2D<U>, T>> = states.collect();
            if self.window_dim.is_none() {
                let window_dim = find_2d_bounds(&states);
                self.window_dim = Some((
                    window_dim.0.to_i32().unwrap(),
                    window_dim.1.to_i32().unwrap(),
                    window_dim.2.to_i32().unwrap(),
                    window_dim.3.to_i32().unwrap(),
                ));
            }
            let (x_min, x_max, y_min, y_max) = self.window_dim.unwrap();

            let win_width = x_max.checked_sub(x_min).unwrap().to_i32().unwrap() + 4;
            let win_height = y_max.checked_sub(y_min).unwrap().to_i32().unwrap() + 4;

            /* Start in the center. */
            let start_y = (self.screen_dim.0 - win_height) / 2;
            let start_x = (self.screen_dim.1 - win_width) / 2;
            let win = create_win(start_y, start_x, win_height, win_width);

            for (idx, state) in states.iter() {
                let cur_x = idx
                    .x
                    .checked_sub(&(U::from_i32(x_min).unwrap()))
                    .unwrap()
                    .to_i32()
                    .unwrap()
                    + 1;
                let cur_y = y_max
                    .checked_sub(idx.y.to_i32().unwrap())
                    .unwrap()
                    .to_i32()
                    .unwrap()
                    + 2;
                let ch: char = self.char_map.char_representation(state);
                mvwprintw(win, cur_y, cur_x, ch.to_string().as_str());
            }

            self.iter += 1;
            wrefresh(win);

            self.check_user_input(false);
        }
    }
}

impl<M> TextRendererGrid2D<M> {
    pub fn new(char_map: M) -> Self {
        Self::new_with_title(char_map, String::from(""))
    }

    pub fn new_with_title(char_map: M, title: String) -> Self {
        Self {
            title,
            iter: 0,
            is_enabled: true,
            screen_dim: (0, 0),
            window_dim: None,
            rx: None,
            speed_level: 0,
            last_render_time: None,
            char_map,
        }
    }

    pub fn new_with_title_and_ch_receiver(
        char_map: M,
        title: String,
        receiver: Receiver<char>,
    ) -> Self {
        Self {
            title,
            iter: 0,
            is_enabled: true,
            screen_dim: (0, 0),
            window_dim: None,
            rx: Some(receiver),
            speed_level: 0,
            last_render_time: None,
            char_map,
        }
    }

    fn check_user_input(&mut self, should_block: bool) {
        if self.rx.is_some() {
            loop {
                match self.rx.as_mut().unwrap().try_recv() {
                    Ok(val) => {
                        self.execute_user_input(val);
                        break;
                    }
                    Err(err) => match err {
                        TryRecvError::Empty => {
                            if should_block {
                                continue;
                            } else {
                                break;
                            }
                        }
                        TryRecvError::Closed => panic!("Error getting user input: {}", err),
                        TryRecvError::Lagged(_) => continue,
                    },
                }
            }
        }
    }

    fn execute_user_input(&mut self, ch: char) {
        // TODO: implement rewind
        if ch == 'q' {
            self.cleanup_impl();
        } else if ch == 'j' {
            self.speed_level -= 1;
        } else if ch == 'k' {
            self.speed_level += 1;
        }
    }

    fn cleanup_impl(&mut self) {
        if self.is_enabled {
            curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
            endwin();
        }
        self.is_enabled = false;
    }

    fn print_generation_and_speed(&self) {
        mv(GENERATION_ROW, 0);
        clrtoeol();
        let speed = 2u32.pow(self.speed_level.abs() as u32);
        let speed_str = if self.speed_level < 0 {
            format!("1/{}", speed)
        } else {
            format!("{}", speed)
        };
        let fps = match self.last_render_time {
            Some(val) => {
                let time_diff = Instant::now() - val;
                1.0 / time_diff.as_secs_f64()
            }
            None => 0.0,
        };
        let iter_msg = format!(
            "Generation: {}, Speed: {}x, FPS: {:6.2}",
            self.iter, speed_str, fps
        );
        mvprintw(
            GENERATION_ROW,
            (self.screen_dim.1 as usize - iter_msg.len()) as i32 / 2,
            iter_msg.as_str(),
        );
        refresh();
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
