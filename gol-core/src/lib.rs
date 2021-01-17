pub mod board;
pub mod board_states;
pub mod cell;
pub mod evolution;
pub mod neighbors;
pub mod space;

pub use board::board_callback::{BoardCallback, BoardCallbackManager};
pub use board::board_neighbor::BoardNeighborManager;
pub use board::board_space::BoardSpaceManager;
pub use board::board_state::BoardStateManager;
pub use board::board_strategy::BoardStrategyManager;
pub use board::standard::StandardBoard;
pub use board_states::sparse::SparseStates;
pub use board_states::sparse_binary::SparseBinaryStates;
pub use cell::common::{IndexedDataOwned, IndexedDataRef};
pub use cell::index::{GridPoint1D, GridPoint2D, GridPoint3D, GridPointND};
pub use cell::state::ConwayState;
pub use evolution::strategy::EvolutionStrategy;
pub use evolution::strategy_conway::ConwayStrategy;
pub use evolution::strategy_manager::SharedStrategyManager;
pub use neighbors::grid_surround::NeighborsGridSurround;
pub use space::grid::{Grid, GridFactory, GridOrigin};
