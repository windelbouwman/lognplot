//! Chart functionality!

use super::axis::ValueAxis;
use super::curve::Curve;
use crate::time::TimeSpan;
use crate::tsdb::{Aggregation, Sample, SampleMetrics};

/// A single 2D-chart
#[derive(Clone)]
pub struct Chart {
    /// An optional title for the plot
    pub title: Option<String>,

    pub x_axis: ValueAxis,
    pub y_axis: ValueAxis,

    /// To show grid or not.
    pub grid: bool,

    /// The curves in the plot
    pub curves: Vec<Curve>,
}

impl Default for Chart {
    fn default() -> Self {
        Chart {
            title: None,
            x_axis: ValueAxis::default(),
            y_axis: ValueAxis::default(),
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
        self.x_axis.zoom(amount);
    }

    /// Perform vertical zooming
    pub fn zoom_vertical(&mut self, amount: f64) {
        self.y_axis.zoom(amount);
    }

    /// Perform a bit of horizontal panning
    pub fn pan_horizontal(&mut self, amount: f64) {
        self.x_axis.pan(amount);
    }

    /// Perform vertical pan motion on the plot.
    pub fn pan_vertical(&mut self, amount: f64) {
        self.y_axis.pan(amount);
    }

    /// Adjust Y axis to fit all data as selected on X-axis in view.
    pub fn fit_y_axis(&mut self) {
        // First, determine metrics of data in view!
        let timespan = self.x_axis.timespan();
        let summaries: Vec<Aggregation<Sample, SampleMetrics>> = self
            .curves
            .iter()
            .filter_map(|c| c.range_summary(&timespan))
            .collect();

        if let Some(summary) = Aggregation::from_aggregations(&summaries) {
            self.fit_y_axis_to_metrics(summary.metrics());
        }
    }

    /// Adjust Y-axis such that we view the given metrics.
    fn fit_y_axis_to_metrics(&mut self, metrics: &SampleMetrics) {
        let mut domain = metrics.max - metrics.min;
        if domain.abs() < 1.0e-17 {
            domain = 1.0;
        }

        let minimum = metrics.min - 0.05 * domain;
        let maximum = metrics.max + 0.05 * domain;
        self.y_axis.set_limits(minimum, maximum);
    }

    fn fit_x_axis_to_timespan(&mut self, timespan: &TimeSpan) {
        let mut domain = timespan.end.amount - timespan.start.amount;
        if domain.abs() < 1.0e-18 {
            domain = 1.0;
        }

        let minimum = timespan.start.amount - domain * 0.05;
        let maximum = timespan.end.amount + domain * 0.05;
        self.x_axis.set_limits(minimum, maximum);
    }

    /// Adjust scale ranges so we fit all data in view.
    pub fn autoscale(&mut self) {
        // Retrieve info from all curves:
        let summaries: Vec<Aggregation<Sample, SampleMetrics>> =
            self.curves.iter().filter_map(|c| c.summary()).collect();

        if let Some(summary) = Aggregation::from_aggregations(&summaries) {
            self.fit_x_axis_to_timespan(&summary.timespan);
            self.fit_y_axis_to_metrics(summary.metrics());
        }
    }
}
