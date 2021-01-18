use gol_core::{
    predefined_states, Board, BoardCallback, ConwayState, ConwayStrategy, GridPoint2D,
    IndexedDataOwned, StandardBoard, StandardBoardFactory,
};
use gol_renderer::TextRendererGrid2D;
use std::time::Duration;

fn main() {
    let strategy = Box::new(ConwayStrategy::new());
    let renderer = Box::new(TextRendererGrid2D::new_with_title(String::from(
        "John Conway's Original Game of Life",
    )))
        as Box<
            dyn BoardCallback<
                ConwayState,
                GridPoint2D<i32>,
                rayon::vec::IntoIter<IndexedDataOwned<GridPoint2D<i32>, ConwayState>>,
            >,
        >;
    let mut callbacks = Vec::new();
    callbacks.push(renderer);
    let mut board: StandardBoard<
        ConwayState,
        GridPoint2D<i32>,
        std::vec::IntoIter<GridPoint2D<i32>>,
    > = StandardBoardFactory::new_binary_2d_grid(
        (200usize, 40),
        ConwayState::Dead,
        ConwayState::Alive,
        1,
        &predefined_states::conway_2d_tetris(),
        strategy,
        callbacks,
    );

    board.advance(Some(20), Some(Duration::from_millis(350)));
}
