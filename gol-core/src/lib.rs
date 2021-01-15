pub mod board;
pub mod cell;
pub mod evolution;
pub mod space;

pub use board::board_callback::{BoardCallback, BoardCallbackManager};
pub use board::board_neighbor::BoardNeighborManager;
pub use board::board_space::BoardSpaceManager;
pub use board::board_state::BoardStateManager;
pub use board::board_strategy::BoardStrategyManager;
pub use cell::common::{CellIndex, CellState, IndexedDataOwned, IndexedDataRef};
pub use cell::index::{GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
pub use cell::state::ConwayState;
pub use evolution::strategy::EvolutionStrategy;
pub use evolution::strategy_conway::ConwayStrategy;
pub use space::grid::GridND;
