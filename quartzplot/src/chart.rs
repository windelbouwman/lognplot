//! Chart functionality!

use super::axis::Axis;
use super::curve::Curve;

/// A single 2D-chart
#[derive(Clone)]
pub struct Chart {
    /// An optional title for the plot
    pub title: Option<String>,

    pub x_axis: Axis,
    pub y_axis: Axis,

    /// To show grid or not.
    pub grid: bool,

    /// The curves in the plot
    pub curves: Vec<Curve>,
}

impl Default for Chart {
    fn default() -> Self {
        Chart {
            title: None,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            grid: true,
            curves: vec![],
        }
    }
}

impl Chart {
    /// Set the title of the chart!
    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn set_xlabel(&mut self, label: &str) {
        self.x_axis.label = Some(label.to_string());
    }

    pub fn set_ylabel(&mut self, label: &str) {
        self.y_axis.label = Some(label.to_string());
    }

    /// Drop a new curve into the mix!
    pub fn add_curve(&mut self, curve: Curve) {
        self.curves.push(curve);
    }

    /// Adjust scale ranges so we fit all data in view.
    pub fn autoscale(&mut self) {
        let mut spans = vec![];
        for curve in &self.curves {
            let span = curve.get_span();
            spans.push(span);
        }

        // self.curves
        self.x_axis.set_limits(2.0, 98.0);
        self.y_axis.set_limits(-70.0, 350.0);
    }
}
