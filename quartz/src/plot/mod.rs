//!
//! Plot library.
//!

mod axis;
mod axis_options;
mod chart;
mod curve;

use crate::canvas::{Canvas, CanvasDrawAble};
pub use chart::Chart;
pub use curve::{Curve, CurveData};

/// Entry function to plot a series of x values versus a series of y values!
pub fn plot(canvas: &mut dyn Canvas, x: Vec<f64>, y: Vec<f64>) {
    info!("Plotting x= {:?} y= {:?}", x, y);
    let mut chart = Chart::default();
    chart.set_xlabel("Time");
    chart.set_ylabel("Value");
    chart.set_title("W00tie");
    let curve_data = CurveData::new(x, y);
    let curve = Curve::new(curve_data);
    chart.add_curve(curve);
    chart.autoscale();
    chart.draw(canvas);
}
