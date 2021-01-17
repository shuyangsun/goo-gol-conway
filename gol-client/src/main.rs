use gol_core::{
    Board, BoardCallback, ConwayState, ConwayStrategy, EvolutionStrategy, GridPoint2D,
    IndexedDataOwned, StandardBoard, StandardBoardFactory,
};
use gol_renderer::TextRendererGrid2D;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    let strategy = Box::new(ConwayStrategy::new());
    let mut alive_cells = HashMap::new();
    alive_cells.insert(GridPoint2D { x: 0, y: 0 }, ConwayState::Alive);
    alive_cells.insert(GridPoint2D { x: 0, y: 1 }, ConwayState::Alive);
    alive_cells.insert(GridPoint2D { x: -1, y: 0 }, ConwayState::Alive);
    alive_cells.insert(GridPoint2D { x: 1, y: 0 }, ConwayState::Alive);
    let renderer = Box::new(TextRendererGrid2D::new(true))
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
    > = StandardBoardFactory::new_standard_2d_grid(
        (10usize, 10),
        ConwayState::Dead,
        1,
        &alive_cells,
        strategy,
        callbacks,
    );

    board.advance(Some(20), Some(Duration::new(1, 0)));
}
