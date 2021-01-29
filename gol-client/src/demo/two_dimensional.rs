use gol_core::{
    self, BinaryStatesCallback, Board, BoardCallback, ConwayState, ConwayStrategy, GridPoint2D,
    StandardBoard, StandardBoardFactory,
};

use gol_renderer::{DefaultColorMap, GraphicalRendererGrid2D};
use rand::prelude::*;
use std::collections::HashSet;
use std::time::Duration;

pub fn run_demo(
    width: Option<usize>,
    height: Option<usize>,
    initial_states: &HashSet<GridPoint2D<i32>>,
    title: &str,
    max_iter: usize,
    interval_secs: f64,
    is_board_donut: bool,
    alive_ratio: f64,
) {
    let strategy = Box::new(ConwayStrategy::new());

    let one_billion_nano_sec: f64 = 1_000_000_000f64;
    let interval_nano_sec = (interval_secs * one_billion_nano_sec) as u64;

    let (mut callbacks, keyboard_control) =
        crate::callback::standard_control_callbacks(Duration::from_nanos(interval_nano_sec));
    let binary_states_callback = BinaryStatesCallback::new_with_non_trivial_indices(
        ConwayState::Dead,
        ConwayState::Alive,
        initial_states.clone(),
    );
    let states_read_only = binary_states_callback.clone_read_only();
    let binary_states_callback = BoardCallback::WithStates(Box::new(binary_states_callback));
    callbacks.push(binary_states_callback);

    let win_size = (width.unwrap_or(200usize), height.unwrap_or(50));

    #[cfg(feature = "ascii")]
    let win_size = if width.is_some() && height.is_some() {
        (width.unwrap(), height.unwrap())
    } else {
        use super::util::get_ncurses_win_height_width;
        let (height, width) = get_ncurses_win_height_width();
        let height_new = height - 15;
        let width_new = width * height_new / height;
        (width_new, height_new)
    };

    let random_init_state = if alive_ratio <= 0.0 {
        HashSet::new()
    } else {
        gen_random_initial_positions(win_size, alive_ratio)
    };

    let mut board: StandardBoard<
        ConwayState,
        GridPoint2D<i32>,
        std::vec::IntoIter<GridPoint2D<i32>>,
    > = StandardBoardFactory::new_binary_2d_grid(
        win_size,
        ConwayState::Dead,
        ConwayState::Alive,
        1,
        if initial_states.is_empty() {
            &random_init_state
        } else {
            initial_states
        },
        strategy,
        callbacks,
        is_board_donut,
    );

    std::thread::spawn(move || {
        board.advance(Some(max_iter));
    });

    let mut handle: Option<std::thread::JoinHandle<()>> = None;

    #[cfg(feature = "ascii")]
    {
        use gol_renderer::{DefaultCharMap, TextRendererGrid2D};
        let mut text_renderer = TextRendererGrid2D::new_with_title_and_ch_txrx(
            win_size.0,
            win_size.1,
            DefaultCharMap::new(),
            states_read_only.clone(),
            String::from(title),
            keyboard_control.get_sender(),
            keyboard_control.get_receiver(),
        );
        handle = Some(std::thread::spawn(move || {
            text_renderer.run();
        }));
    }

    let graphical_renderer = GraphicalRendererGrid2D::new(
        win_size.0,
        win_size.1,
        DefaultColorMap::new_decay_alpha(),
        states_read_only.clone(),
    );

    match graphical_renderer {
        Ok(val) => {
            val.with_title(String::from(title)).run();
        }
        Err(err) => eprintln!("Error creating graphical renderer: {:?}", err),
    };

    match handle {
        Some(val) => val.join().unwrap(),
        None => (),
    }
}

fn gen_random_initial_positions(
    board_shape: (usize, usize),
    alive_ratio: f64,
) -> HashSet<GridPoint2D<i32>> {
    let mut rng = rand::thread_rng();
    let mut res = HashSet::new();
    for x in 0..board_shape.0 {
        for y in 0..board_shape.1 {
            if rng.gen::<f64>() < alive_ratio {
                res.insert(GridPoint2D::new(
                    (x as i64 - (board_shape.0 / 2) as i64) as i32,
                    (y as i64 - (board_shape.1 / 2) as i64) as i32,
                ));
            }
        }
    }
    res
}
