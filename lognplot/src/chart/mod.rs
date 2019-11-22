//! Data visualisation.
//!
//! This module provides functions for defining a chart.
//! For rendering this chart, see the render module.

mod axis;
mod axis_options;
mod chart;
mod curve;

pub use chart::Chart;
pub use curve::{Curve, CurveData};

use crate::geometry::Size;
use crate::render::Canvas;

use crate::render::draw_chart;

/// Entry function to plot a series of x values versus a series of y values!
pub fn plot(canvas: &mut dyn Canvas, x: Vec<f64>, y: Vec<f64>, size: Size) {
    info!("Plotting len(x)= {:?} len(y)= {:?}", x.len(), y.len());
    let mut chart = Chart::default();
    chart.set_xlabel("Time");
    chart.set_ylabel("Value");
    chart.set_title("W00tie");
    let curve_data = CurveData::new(x, y);
    let curve = Curve::new(curve_data);
    chart.add_curve(curve);
    chart.autoscale();
    draw_chart(&chart, canvas, size);
}
