use gol_core::{
    self, Board, ConwayState, ConwayStrategy, GridPoint2D, StandardBoard, StandardBoardFactory,
};

#[cfg(feature = "ascii")]
use super::util::get_ncurses_win_height_width;
#[cfg(feature = "ascii")]
use gol_renderer::TextRendererGrid2D;
use std::collections::HashSet;
use std::time::Duration;

pub fn run_demo(
    initial_states: &HashSet<GridPoint2D<i32>>,
    title: &str,
    max_iter: usize,
    initial_delay_secs: f64,
    interval_secs: f64,
) {
    let strategy = Box::new(ConwayStrategy::new());

    #[cfg(any(feature = "ascii"))]
    let mut callbacks = Vec::new();

    #[cfg(not(any(feature = "ascii")))]
    let callbacks = Vec::new();

    #[cfg(feature = "ascii")]
    {
        let text_renderer = Box::new(TextRendererGrid2D::new_with_title(String::from(title)))
            as Box<
                dyn gol_core::BoardCallback<
                    ConwayState,
                    GridPoint2D<i32>,
                    rayon::vec::IntoIter<gol_core::IndexedDataOwned<GridPoint2D<i32>, ConwayState>>,
                >,
            >;
        callbacks.push(text_renderer);
    }

    #[cfg(not(any(feature = "ascii")))]
    let win_size = (200usize, 50);

    #[cfg(feature = "ascii")]
    let win_size = {
        let (height, width) = get_ncurses_win_height_width();
        (width - 10, height - 15)
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
        initial_states,
        strategy,
        callbacks,
    );

    let one_billion_nano_sec: f64 = 1_000_000_000f64;
    let initial_delay_nano_sec = (initial_delay_secs * one_billion_nano_sec) as u64;
    let interval_nano_sec = (interval_secs * one_billion_nano_sec) as u64;
    board.advance(
        Some(max_iter),
        Some(Duration::from_nanos(initial_delay_nano_sec)),
        Some(Duration::from_nanos(interval_nano_sec)),
    );
}
