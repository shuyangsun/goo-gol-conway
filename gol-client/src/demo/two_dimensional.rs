use gol_core::{
    self, BinaryState, BinaryStatesCallback, BinaryStrategy, Board, BoardCallback, GridPoint2D,
    StandardBoard, StandardBoardFactory,
};

use gol_renderer::{BinaryStateColorMap, CellularAutomatonRenderer, GraphicalRendererGrid2D};
use rand::prelude::*;
use std::collections::HashSet;
use std::time::Duration;

pub fn run_demo(
    width: Option<usize>,
    height: Option<usize>,
    initial_states: HashSet<GridPoint2D<i32>>,
    title: &str,
    max_iter: usize,
    interval_secs: f64,
    is_board_donut: bool,
    alive_ratio: f64,
) {
    let strategy = Box::new(BinaryStrategy::conway());

    let one_billion_nano_sec: f64 = 1_000_000_000f64;
    let interval_nano_sec = (interval_secs * one_billion_nano_sec) as u64;

    let (mut callbacks, keyboard_control) =
        crate::callback::standard_control_callbacks(true, Duration::from_nanos(interval_nano_sec));
    let binary_states_callback = BinaryStatesCallback::new(BinaryState::Dead, BinaryState::Alive);
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
        BinaryState,
        GridPoint2D<i32>,
        std::vec::IntoIter<GridPoint2D<i32>>,
    > = StandardBoardFactory::new_binary_2d_grid(
        win_size,
        BinaryState::Dead,
        BinaryState::Alive,
        1,
        if initial_states.is_empty() {
            random_init_state
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
        use gol_renderer::{BinaryStateCharMap, TextRendererGrid2D};
        let mut text_renderer = TextRendererGrid2D::new(
            win_size.0,
            win_size.1,
            BinaryStateCharMap::new(),
            states_read_only.clone(),
        )
        .with_title(String::from(title))
        .with_keyboard_control(keyboard_control.clone());
        handle = Some(std::thread::spawn(move || {
            text_renderer.run();
        }));
    }

    let graphical_renderer = GraphicalRendererGrid2D::new(
        win_size.0,
        win_size.1,
        BinaryStateColorMap::new(),
        states_read_only.clone(),
    );

    match graphical_renderer {
        Ok(val) => {
            val.with_title(String::from(title))
                .with_keyboard_control(keyboard_control.clone())
                .run();
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
