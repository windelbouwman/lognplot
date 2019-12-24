//! Canvas package for drawing stuff on canvas
//! This means that we can be artists now!

mod canvas;
mod layout;
mod options;
mod render;
mod svg_output;

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

use crate::chart::ValueAxis;
use crate::geometry::Size;

/// Calculate how many domain values a covered by the given amount of pixels.
pub fn x_pixels_to_domain(size: Size, axis: &ValueAxis, pixels: f64) -> f64 {
    let options = ChartOptions::default();
    let mut layout = ChartLayout::new(size);
    layout.layout(&options);
    let domain = axis.domain();
    if layout.plot_width < 1.0 {
        0.0
    } else {
        let a = domain / layout.plot_width;
        pixels * a
    }
}
