//! Chart functionality!

use super::axis::ValueAxis;
use super::curve::Curve;
use super::Cursor;
use crate::geometry::Range;
use crate::time::{TimeSpan, TimeStamp};
use crate::tsdb::Summary;

/// A single 2D-chart
pub struct Chart {
    /// An optional title for the plot
    pub title: Option<String>,

    pub x_axis: ValueAxis,
    pub y_axis: ValueAxis,

    /// To show grid or not.
    pub grid: bool,

    /// The curves in the plot
    pub curves: Vec<Curve>,

    /// The position of the cursor.
    pub cursor: Option<Cursor>,
}

impl Default for Chart {
    fn default() -> Self {
        Chart {
            title: None,
            x_axis: ValueAxis::default(),
            y_axis: ValueAxis::default(),
            grid: true,
            curves: vec![],
            cursor: None,
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

    /// Remove all curves from this plot.
    pub fn clear_curves(&mut self) {
        self.curves.clear();
    }

    /// Zoom horizontally.
    pub fn zoom_horizontal(&mut self, amount: f64, around: Option<f64>) {
        self.x_axis.zoom(amount, around);
    }

    /// Perform vertical zooming
    pub fn zoom_vertical(&mut self, amount: f64) {
        self.y_axis.zoom(amount, None);
    }

    /// Perform a bit of relative horizontal panning
    pub fn pan_horizontal_relative(&mut self, amount: f64) {
        self.x_axis.pan_relative(amount);
    }

    pub fn pan_horizontal_absolute(&mut self, amount: f64) {
        self.x_axis.pan_absolute(amount);
    }

    /// Perform vertical pan motion on the plot.
    pub fn pan_vertical(&mut self, amount: f64) {
        self.y_axis.pan_relative(amount);
    }

    /// Adjust Y axis to fit all data as selected on X-axis in view.
    pub fn fit_y_axis(&mut self) {
        // First, determine metrics of data in view!
        let timespan = self.x_axis.timespan();

        if let Some(summary) = self.data_summary(Some(&timespan)) {
            if let Some(range) = summary.range {
                self.fit_y_axis_to_range(&range);
            }
        }
    }

    /// Zoom to the last x time
    pub fn zoom_to_last(&mut self, tail_duration: f64) {
        if let Some(summary) = self.data_summary(None) {
            let end = summary.timespan.end;
            let begin = end.clone() - tail_duration;
            let timespan = TimeSpan::new(begin, end);
            self.fit_x_axis_to_timespan(&timespan);
        }
    }

    pub fn get_last_timestamp(&self) -> Option<TimeStamp> {
        if let Some(summary) = self.data_summary(None) {
            Some(summary.timespan.end)
        } else {
            None
        }
    }

    /// Adjust Y-axis such that we view the given metrics.
    fn fit_y_axis_to_range(&mut self, range: &Range<f64>) {
        let mut domain = range.end() - range.begin();
        if domain.abs() < 1.0e-17 {
            domain = 1.0;
        }

        let minimum = range.begin() - 0.05 * domain;
        let maximum = range.end() + 0.05 * domain;
        self.y_axis.set_limits(minimum, maximum);
    }

    pub fn fit_x_axis_to_timespan(&mut self, timespan: &TimeSpan) {
        let mut domain = timespan.end.amount - timespan.start.amount;
        if domain.abs() < 1.0e-18 {
            domain = 1.0;
        }

        let minimum = timespan.start.amount - domain * 0.05;
        let maximum = timespan.end.amount + domain * 0.05;
        self.x_axis.set_limits(minimum, maximum);
    }

    /// Retrieve meta-data from all curves.
    fn data_summary(&self, timespan: Option<&TimeSpan>) -> Option<ChartDataSummary> {
        let summaries: Vec<Summary> = self
            .curves
            .iter()
            .filter_map(|c| c.data_summary(timespan))
            .collect();
        ChartDataSummary::from_summaries(&summaries)
    }

    /// Adjust scale ranges so we fit all data in view.
    pub fn autoscale(&mut self) {
        if let Some(summary) = self.data_summary(None) {
            self.fit_x_axis_to_timespan(&summary.timespan);
            if let Some(range) = summary.range {
                self.fit_y_axis_to_range(&range);
            }
        }
    }

    pub fn has_signal(&self, name: &str) -> bool {
        self.curves.iter().any(|c| c.name() == name)
    }
}

struct ChartDataSummary {
    timespan: TimeSpan,
    range: Option<Range<f64>>,
}

impl ChartDataSummary {
    fn from_summaries(summaries: &[Summary]) -> Option<Self> {
        if summaries.is_empty() {
            None
        } else {
            let mut timespan = summaries.first().unwrap().timespan().clone();
            let mut metrics = vec![];

            for summary in summaries {
                timespan.extend_to_include_span(summary.timespan());
                if let Summary::Value(value_summary) = summary {
                    let min_value = value_summary.metrics().min;
                    let max_value = value_summary.metrics().max;
                    metrics.push((min_value, max_value));
                }
            }

            let range: Option<Range<f64>> = if let Some((first, rest)) = metrics.split_first() {
                let (mut begin, mut end) = *first;
                for (min_value, max_value) in rest {
                    begin = begin.min(*min_value);
                    end = end.max(*max_value);
                }
                Some(Range::new(begin, end))
            } else {
                None
            };

            Some(ChartDataSummary { timespan, range })
        }
    }
}
