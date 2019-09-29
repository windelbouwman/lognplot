//! Canvas package for drawing stuff on canvas
//! This means that we can be artists now!

mod canvas;
mod chart_render;
mod svg_output;

pub use canvas::{Canvas, CanvasDrawAble};
pub use svg_output::SvgOutput;
