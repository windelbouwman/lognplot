//!
//! Plot library.
//!

#[macro_use]
extern crate log;

mod axis;
mod axis_options;
mod chart;
mod curve;
mod render;

pub use chart::Chart;
pub use curve::{Curve, CurveData};
use quartzcanvas::{Canvas, CanvasDrawAble};

/// Entry function to plot a series of x values versus a series of y values!
pub fn plot(canvas: &mut dyn Canvas, x: Vec<f64>, y: Vec<f64>) {
    info!("Plotting len(x)= {:?} len(y)= {:?}", x.len(), y.len());
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
