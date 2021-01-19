#[cfg(feature = "ascii")]
pub mod text;

#[cfg(feature = "ascii")]
pub use text::grid_2d::TextRendererGrid2D;

#[cfg(feature = "ascii")]
pub use text::grid_2d_shadow::TextRendererGrid2DShadow;
