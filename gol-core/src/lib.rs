pub mod board;
pub mod board_states;
pub mod callback;
pub mod cell;
pub mod evolution;
pub mod neighbors;
pub mod space;
mod util;

pub use board::board::Board;
pub use board::board_callback::{
    BoardCallback, BoardCallbackManager, BoardCallbackWithStates, BoardCallbackWithoutStates,
};
pub use board::board_neighbor::BoardNeighborManager;
pub use board::board_space::BoardSpaceManager;
pub use board::board_state::BoardStateManager;
pub use board::board_strategy::BoardStrategyManager;
pub use board::standard::{StandardBoard, StandardBoardFactory};
pub use board_states::predefined as predefined_states;
pub use board_states::sparse::SparseStates;
pub use board_states::sparse_binary::SparseBinaryStates;
pub use callback::{
    model_binary_states::BinaryStatesCallback, model_binary_states::BinaryStatesReadOnly,
    model_states::StatesCallback, model_states::StatesReadOnly,
};
pub use cell::common::IndexedDataOwned;
pub use cell::index::{GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
pub use cell::state::ConwayState;
pub use evolution::strategy::EvolutionStrategy;
pub use evolution::strategy_conway::ConwayStrategy;
pub use evolution::strategy_manager::SharedStrategyManager;
pub use neighbors::{
    grid_donut::NeighborsGridDonut, grid_surround::NeighborsGridSurround, moore::NeighborMoore,
};
pub use space::grid::{Grid, GridFactory, GridOrigin};
