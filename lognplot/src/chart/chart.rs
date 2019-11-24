//! Chart functionality!

use super::axis::Axis;
use super::curve::Curve;
use crate::tsdb::{Aggregation, Sample, SampleMetrics};

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

    /// Zoom horizontally.
    pub fn zoom_horizontal(&mut self, amount: f64) {
        let domain = self.x_axis.domain();
        let step = domain * amount;
        let x1 = self.x_axis.begin() - step;
        let x2 = self.x_axis.end() + step;
        self.x_axis.set_limits(x1, x2);
    }

    /// Perform vertical zooming
    pub fn zoom_vertical(&mut self, amount: f64) {
        let domain = self.y_axis.domain();
        let step = domain * amount;
        let y1 = self.y_axis.begin() - step;
        let y2 = self.y_axis.end() + step;
        self.y_axis.set_limits(y1, y2);
    }

    /// Perform a bit of horizontal panning
    pub fn pan_horizontal(&mut self, amount: f64) {
        let domain = self.x_axis.domain();
        let step = domain * amount;
        let x1 = self.x_axis.begin() + step;
        let x2 = self.x_axis.end() + step;
        self.x_axis.set_limits(x1, x2);
    }

    /// Perform vertical pan motion on the plot.
    pub fn pan_vertical(&mut self, amount: f64) {
        let domain = self.y_axis.domain();
        let step = domain * amount;
        let y1 = self.y_axis.begin() + step;
        let y2 = self.y_axis.end() + step;
        self.y_axis.set_limits(y1, y2);
    }

    /// Adjust scale ranges so we fit all data in view.
    pub fn autoscale(&mut self) {
        // Retrieve info from all curves:
        let summaries: Vec<Aggregation<Sample, SampleMetrics>> =
            self.curves.iter().filter_map(|c| c.summary()).collect();

        if let Some(summary) = Aggregation::from_aggregations(&summaries) {
            self.x_axis
                .set_limits(summary.timespan.start.amount, summary.timespan.end.amount);
            self.y_axis
                .set_limits(summary.metrics().min, summary.metrics().max);
        }
    }
}
