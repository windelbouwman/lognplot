//! Canvas package for drawing stuff on canvas
//! This means that we can be artists now!

mod canvas;
mod layout;
mod options;
mod render;
mod svg_output;

#[cfg(feature = "cairo")]
mod cairo;
#[cfg(feature = "cairo")]
mod cairo_canvas;

// re-exports

pub use canvas::Canvas;
use layout::ChartLayout;
use options::ChartOptions;
pub use render::draw_chart;
pub use svg_output::SvgOutput;

#[cfg(feature = "cairo")]
pub use cairo_canvas::CairoCanvas;
