//! Canvas package for drawing stuff on canvas
//! This means that we can be artists now!

mod canvas;
mod chart;
mod layout;
mod options;
mod svg_output;

#[cfg(feature = "cairo")]
mod cairo_canvas;

// re-exports

pub use canvas::Canvas;
pub use chart::draw_chart;
use layout::ChartLayout;
use options::ChartOptions;
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

pub fn x_pixel_to_domain(pixel: f64, axis: &ValueAxis, size: Size) -> f64 {
    let options = ChartOptions::default();
    let mut layout = ChartLayout::new(size);
    layout.layout(&options);

    let domain = axis.domain();
    if layout.plot_width < 1.0 {
        0.0
    } else {
        let a = domain / layout.plot_width;
        a * (pixel - layout.plot_left) + axis.begin()
    }
}

/// Take an y pixel and transform it to a domain value on the given axis.
pub fn y_pixel_to_domain(pixel: f64, axis: &ValueAxis, size: Size) -> f64 {
    let options = ChartOptions::default();
    let mut layout = ChartLayout::new(size);
    layout.layout(&options);

    let domain = axis.domain();
    if layout.plot_height < 1.0 {
        0.0
    } else {
        let a = domain / layout.plot_height;
        axis.begin() - a * (pixel - layout.plot_bottom)
    }
}
