//! Canvas package for drawing stuff on canvas
//! This means that we can be artists now!

#[macro_use]
extern crate log;

pub mod canvas;
pub use canvas::{Canvas, CanvasDrawAble};
pub mod geometry;
pub mod style;

mod svg_output;

pub use svg_output::SvgOutput;
