/// Canvas package for drawing stuff on canvas
/// This means that we can be artists now!

mod canvas;
mod color;
mod svg_output;
mod softgl;
mod transform;

pub use canvas::Canvas;
pub use svg_output::SvgOutput;
pub use color::Color;
