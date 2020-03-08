//! Transformations from pixels to axis values.

use super::ChartLayout;
use crate::chart::ValueAxis;
use crate::time::TimeStamp;

/// Calculate how many domain values a covered by the given amount of pixels.
pub fn x_pixels_to_domain(layout: &ChartLayout, axis: &ValueAxis, pixels: f64) -> f64 {
    let domain = axis.domain();
    if layout.plot_width < 1.0 {
        0.0
    } else {
        let a = domain / layout.plot_width;
        pixels * a
    }
}

pub fn x_pixel_to_domain(pixel: f64, axis: &ValueAxis, layout: &ChartLayout) -> f64 {
    let domain = axis.domain();
    if layout.plot_width < 1.0 {
        0.0
    } else {
        let a = domain / layout.plot_width;
        a * (pixel - layout.plot_left) + axis.begin()
    }
}

/// Take an y pixel and transform it to a domain value on the given axis.
pub fn y_pixel_to_domain(pixel: f64, axis: &ValueAxis, layout: &ChartLayout) -> f64 {
    let domain = axis.domain();
    if layout.plot_height < 1.0 {
        0.0
    } else {
        let a = domain / layout.plot_height;
        axis.begin() - a * (pixel - layout.plot_bottom)
    }
}

pub fn x_domain_to_pixel(t: &TimeStamp, axis: &ValueAxis, layout: &ChartLayout) -> f64 {
    let x = t.amount;
    let domain = axis.domain();
    let a = (layout.plot_width) / domain;
    let x_pixel = a * (x - axis.begin()) + layout.plot_left;
    clip(x_pixel, layout.plot_left, layout.plot_right)
}

/// Convert a y value into a proper pixel y value given an axis and a chart layout.
pub fn y_domain_to_pixel(y: f64, axis: &ValueAxis, layout: &ChartLayout) -> f64 {
    let domain = axis.domain();
    let a = layout.plot_height / domain;
    let y_pixel = layout.plot_bottom - a * (y - axis.begin());
    clip(y_pixel, layout.plot_top, layout.plot_bottom)
}

/// Clip a value between bounds
fn clip(value: f64, lower: f64, upper: f64) -> f64 {
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ChartLayout, ChartOptions};
    use super::{x_domain_to_pixel, x_pixel_to_domain};
    use super::{y_domain_to_pixel, y_pixel_to_domain};
    use crate::chart::ValueAxis;
    use crate::geometry::Size;
    use crate::time::TimeStamp;

    #[test]
    fn x_axis_roundtrips() {
        let mut axis = ValueAxis::default();
        axis.set_limits(10.0, 1000.0);
        let size = Size::new(500.0, 500.0);
        let options = ChartOptions::default();
        let mut layout = ChartLayout::new(size);
        layout.layout(&options);

        let value = TimeStamp::new(100.0);

        let pixel = x_domain_to_pixel(&value, &axis, &layout);
        let value2 = x_pixel_to_domain(pixel, &axis, &layout);

        assert_almost_eq(value.amount, value2, 1.0e-9);
    }

    #[test]
    fn y_axis_roundtrips() {
        let mut axis = ValueAxis::default();
        axis.set_limits(10.0, 1000.0);
        let size = Size::new(500.0, 500.0);
        let options = ChartOptions::default();
        let mut layout = ChartLayout::new(size);
        layout.layout(&options);

        let value = 100.0;

        let pixel = y_domain_to_pixel(value, &axis, &layout);
        let value2 = y_pixel_to_domain(pixel, &axis, &layout);

        assert_almost_eq(value, value2, 1.0e-9);
    }

    fn assert_almost_eq(v1: f64, v2: f64, tolerance: f64) {
        assert!((v1 - v2).abs() < tolerance);
    }
}
