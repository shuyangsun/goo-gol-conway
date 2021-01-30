pub mod state_visual;

pub mod graphics;
#[cfg(feature = "ascii")]
pub mod text;

pub use graphics::grid_2d::GraphicalRendererGrid2D;
#[cfg(feature = "ascii")]
pub use text::grid_2d::TextRendererGrid2D;

pub use state_visual::mapping::{
    CharMapping, ColorMapping, ConwayStateCharMap, ConwayStateColorMap, DiscreteStateCharMap,
    DiscreteStateColorMap,
};
