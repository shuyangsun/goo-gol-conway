extern crate ncurses;

use crate::{
    renderer::{
        board_info::RendererBoardInfo, fps_counter::FPSCounter, keyboard_control::KeyboardControl,
    },
    CellularAutomatonRenderer, CharMapping,
};
use gol_core::{util::grid_util::Size2D, BinaryStatesReadOnly, GridPoint2D};
use ncurses::*;
use num_traits::{FromPrimitive, ToPrimitive};
use std::char;
use std::hash::Hash;
use std::time::Instant;

const TITLE_ROW: i32 = 1;
const GENERATION_ROW: i32 = 3;

pub struct TextRendererGrid2D<S, M> {
    info: RendererBoardInfo<Size2D>,
    control: Option<KeyboardControl>,
    fps_counter: FPSCounter,
    screen_size: Option<Size2D>,
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
        let info = RendererBoardInfo::new(Size2D::new(board_width, board_height));
        Self {
            info,
            control: None,
            fps_counter: FPSCounter::new(10),
            screen_size: None,
            char_map,
            states_read_only: states,
        }
    }

    pub fn with_title(self, title: String) -> Self {
        let mut res = self;
        res.info.set_title(title);
        res
    }

    pub fn with_keyboard_control(self, control: KeyboardControl) -> Self {
        let mut res = self;
        control.start_monitoring(move || char::from_u32(getch() as u32).unwrap());
        res.control = Some(control);
        res
    }

    fn check_user_input(&mut self, should_block: bool) {
        if self.control.is_some() {
            loop {
                match self.control.as_mut().unwrap().try_receive() {
                    Some(val) => {
                        self.execute_user_input(val);
                        break;
                    }
                    None => {
                        if should_block {
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn execute_user_input(&mut self, ch: char) {
        if ch == 'q' {
            self.cleanup();
        }
    }

    fn cleanup(&mut self) {
        if self.screen_size.is_some() {
            curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
            endwin();
        }
        self.screen_size = None;
    }

    fn print_generation_and_speed(&self) {
        mv(GENERATION_ROW, 0);
        clrtoeol();
        let fps = self.fps_counter.fps();
        let iter_msg = format!(
            "Generation: {}, FPS: {:6.2}",
            self.info.iter_count().unwrap_or_default(),
            fps
        );
        if self.screen_size.is_some() {
            mvprintw(
                GENERATION_ROW,
                (self.screen_size.as_ref().unwrap().width() - iter_msg.len()) as i32 / 2,
                iter_msg.as_str(),
            );
        }
        refresh();
    }

    fn setup_if_not_ready(&mut self) {
        if self.screen_size.is_none() {
            initscr();
            raw();
            curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

            /* Get the screen bounds. */
            let mut max_x = 0;
            let mut max_y = 0;
            getmaxyx(stdscr(), &mut max_y, &mut max_x);
            self.screen_size = Some(Size2D::new(max_x as usize, max_y as usize));
            let screen_size = self.screen_size.as_ref().unwrap();

            mvprintw(
                TITLE_ROW,
                (screen_size.width() - self.info.title().len()) as i32 / 2,
                self.info.title().as_str(),
            );

            let message = "SPACE: play/pause, k/j: speed up/down, q: exit.";
            mvprintw(
                screen_size.height() as i32 - 2,
                (screen_size.width() - message.len()) as i32 / 2,
                message,
            );

            refresh();
        }
    }

    fn draw(&mut self) {
        let board_size = self.info.board_size();
        let (win_width, win_height) = (board_size.width(), board_size.height());

        if self.screen_size.is_none() {
            return;
        }

        let screen_size = self.screen_size.as_ref().unwrap();
        let start_y = ((screen_size.height() - win_height) / 2) as i32;
        let start_x = ((screen_size.width() - win_width) / 2) as i32;

        loop {
            match self.states_read_only.try_read() {
                Ok(val) => {
                    if self.info.iter_count().is_none() || self.info.iter_count().unwrap() != val.0
                    {
                        let win = create_win(start_y, start_x, win_height as i32, win_width as i32);
                        for idx in val.1.iter() {
                            let x_min = board_size.x_idx_min();
                            let y_max = board_size.y_idx_max();
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
                        self.info.set_iter_count(val.0);
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
    }
}

impl<T, U, M> CellularAutomatonRenderer
    for TextRendererGrid2D<BinaryStatesReadOnly<GridPoint2D<U>, T>, M>
where
    U: Hash + FromPrimitive + ToPrimitive + std::ops::Sub<Output = U> + Clone,
    M: CharMapping<T>,
{
    fn run(&mut self) {
        self.setup_if_not_ready();

        loop {
            self.print_generation_and_speed();
            self.fps_counter.lapse();

            self.draw();

            self.check_user_input(false);
        }
    }
}

fn create_win(start_y: i32, start_x: i32, win_height: i32, win_width: i32) -> WINDOW {
    let win = newwin(win_height, win_width, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}
