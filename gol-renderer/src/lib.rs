pub mod state_visual;
mod util;

pub mod graphics;
#[cfg(feature = "ascii")]
pub mod text;

pub use graphics::grid_2d::GraphicalRendererGrid2D;
#[cfg(feature = "ascii")]
pub use text::grid_2d::TextRendererGrid2D;

pub use state_visual::mapping::{CharMapping, ColorMapping, DefaultCharMap, DefaultColorMap};
