//! Data visualisation.
//!
//! This module provides functions for defining a chart.
//! For rendering this chart, see the render module.

mod axis;
mod chart;
mod curve;

pub use axis::ValueAxis;
pub use chart::Chart;
pub use curve::{Curve, CurveData};

use crate::geometry::Size;
use crate::render::Canvas;

use crate::render::draw_chart;

/// Entry function to plot a series of x values versus a series of y values!
pub fn plot<C>(canvas: &mut C, x: Vec<f64>, y: Vec<f64>, size: Size)
where
    C: Canvas,
{
    info!("Plotting len(x)= {:?} len(y)= {:?}", x.len(), y.len());
    let mut chart = Chart::default();
    chart.set_xlabel("Time");
    chart.set_ylabel("Value");
    chart.set_title("W00tie");
    let curve_data = CurveData::points(x, y);
    let curve = Curve::new(curve_data, "red");
    chart.add_curve(curve);
    chart.autoscale();
    draw_chart(&chart, canvas, size);
}
