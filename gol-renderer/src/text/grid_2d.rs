extern crate ncurses;

use crate::{util::grid_util::Size2D, CharMapping};
use gol_core::{BinaryStatesReadOnly, GridPoint2D};
use ncurses::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::char;
use std::hash::Hash;
use std::time::Instant;
use tokio::sync::broadcast::{error::TryRecvError, Receiver, Sender};

const TITLE_ROW: i32 = 1;
const GENERATION_ROW: i32 = 3;

pub struct TextRendererGrid2D<S, M> {
    title: String,
    cur_iter: Option<usize>,
    is_ready: bool,
    screen_size: Size2D,
    board_size: Size2D,
    tx: Option<Sender<char>>,
    rx: Option<Receiver<char>>,
    last_render_time: Option<Instant>,
    states_read_only: S,
    char_map: M,
}

impl<T, U, M> TextRendererGrid2D<BinaryStatesReadOnly<GridPoint2D<U>, T>, M>
where
    U: Hash + FromPrimitive + ToPrimitive + std::ops::Sub<Output = U> + Clone,
    M: CharMapping<T>,
{
    pub fn new(
        board_width: usize,
        board_height: usize,
        char_map: M,
        states: BinaryStatesReadOnly<GridPoint2D<U>, T>,
    ) -> Self {
        Self::new_with_title(
            board_width,
            board_height,
            char_map,
            states,
            String::from(""),
        )
    }

    pub fn new_with_title(
        board_width: usize,
        board_height: usize,
        char_map: M,
        states: BinaryStatesReadOnly<GridPoint2D<U>, T>,
        title: String,
    ) -> Self {
        Self {
            title,
            cur_iter: None,
            is_ready: false,
            screen_size: Size2D::new(0, 0),
            board_size: Size2D::new(board_width, board_height),
            tx: None,
            rx: None,
            last_render_time: None,
            states_read_only: states,
            char_map,
        }
    }

    pub fn new_with_title_and_ch_txrx(
        board_width: usize,
        board_height: usize,
        char_map: M,
        states: BinaryStatesReadOnly<GridPoint2D<U>, T>,
        title: String,
        sender: Sender<char>,
        receiver: Receiver<char>,
    ) -> Self {
        Self {
            title,
            cur_iter: None,
            is_ready: false,
            screen_size: Size2D::new(0, 0),
            board_size: Size2D::new(board_width, board_height),
            tx: Some(sender),
            rx: Some(receiver),
            last_render_time: None,
            states_read_only: states,
            char_map,
        }
    }

    pub fn run(&mut self) {
        self.setup_if_not_ready();

        loop {
            self.print_generation_and_speed();
            self.last_render_time = Some(Instant::now());

            self.draw();

            self.check_user_input(false);
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
            self.cleanup();
        }
    }

    fn cleanup(&mut self) {
        if self.is_ready {
            curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
            endwin();
        }
        self.is_ready = false;
    }

    fn print_generation_and_speed(&self) {
        mv(GENERATION_ROW, 0);
        clrtoeol();
        let fps = match self.last_render_time {
            Some(val) => {
                let time_diff = Instant::now() - val;
                1.0 / time_diff.as_secs_f64()
            }
            None => 0.0,
        };
        let iter_msg = format!(
            "Generation: {}, FPS: {:6.2}",
            self.cur_iter.unwrap_or_default(),
            fps
        );
        mvprintw(
            GENERATION_ROW,
            (self.screen_size.width() - iter_msg.len()) as i32 / 2,
            iter_msg.as_str(),
        );
        refresh();
    }

    fn setup_if_not_ready(&mut self) {
        if !self.is_ready {
            initscr();
            raw();
            curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

            /* Get the screen bounds. */
            let mut max_x = 0;
            let mut max_y = 0;
            getmaxyx(stdscr(), &mut max_y, &mut max_x);
            self.screen_size = Size2D::new(max_x as usize, max_y as usize);

            mvprintw(
                TITLE_ROW,
                (self.screen_size.width() - self.title.len()) as i32 / 2,
                self.title.as_str(),
            );

            let message = "SPACE: play/pause, k/j: speed up/down, q: exit.";
            mvprintw(
                self.screen_size.height() as i32 - 2,
                (self.screen_size.width() - message.len()) as i32 / 2,
                message,
            );

            refresh();

            self.is_ready = true;
        }
    }

    fn draw(&mut self) {
        let (win_width, win_height) = (self.board_size.width(), self.board_size.height());

        /* Start in the center. */
        let start_y = ((self.screen_size.height() - win_height) / 2) as i32;
        let start_x = ((self.screen_size.width() - win_width) / 2) as i32;

        loop {
            match self.states_read_only.try_read() {
                Ok(val) => {
                    if self.cur_iter.is_none() || self.cur_iter.unwrap() != val.0 {
                        let win = create_win(start_y, start_x, win_height as i32, win_width as i32);
                        for idx in val.1.iter() {
                            let x_min = self.board_size.x_idx_min();
                            let y_max = self.board_size.y_idx_max();
                            let cur_x = (idx.x.clone() - U::from_i64(x_min).unwrap())
                                .to_i32()
                                .unwrap()
                                + 1;
                            let cur_y = (y_max - idx.y.to_i64().unwrap()).to_i32().unwrap() + 2;
                            let ch: char = self
                                .char_map
                                .char_representation(&self.states_read_only.non_trivial_state());
                            mvwprintw(win, cur_y, cur_x, ch.to_string().as_str());
                        }
                        wrefresh(win);
                        self.cur_iter = Some(val.0);
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
    }
}

fn create_win(start_y: i32, start_x: i32, win_height: i32, win_width: i32) -> WINDOW {
    let win = newwin(win_height, win_width, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}
