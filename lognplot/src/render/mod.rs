//! Canvas package for drawing stuff on canvas
//! This means that we can be artists now!

mod canvas;
mod chart;
mod layout;
mod options;
mod svg_output;
mod transform;

#[cfg(feature = "cairo")]
mod cairo_canvas;

// re-exports

pub use canvas::Canvas;
pub use chart::draw_chart;
pub use layout::ChartLayout;
pub use options::ChartOptions;
pub use svg_output::SvgOutput;
pub use transform::{x_pixel_to_domain, x_pixels_to_domain, y_pixel_to_domain};

#[cfg(feature = "cairo")]
pub use cairo_canvas::CairoCanvas;
