#[cfg(feature = "ascii")]
extern crate ncurses;

#[cfg(feature = "ascii")]
use ncurses::*;

#[cfg(feature = "ascii")]
pub fn get_ncurses_win_height_width() -> (usize, usize) {
    initscr();
    raw();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    endwin();
    (max_y as usize, max_x as usize)
}
