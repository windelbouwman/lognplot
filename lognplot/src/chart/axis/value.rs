use super::date::calc_date_ticks;
use super::options::AxisOptions;
use super::TickLabels;
use crate::geometry::Range;
use crate::time::{TimeSpan, TimeStamp};

use super::util::{calc_tick_spacing, ceil_to_multiple_of, format_at_scale};

#[derive(Clone)]
pub struct ValueAxis {
    pub options: AxisOptions,
    pub label: Option<String>,
    range: Range<f64>,
}

impl Default for ValueAxis {
    fn default() -> Self {
        ValueAxis {
            options: AxisOptions::default(),
            label: None,
            range: Range::new(0.0, 10.0),
        }
    }
}

impl ValueAxis {
    pub fn set_limits(&mut self, begin: f64, end: f64) {
        self.range.set_begin(begin);
        self.range.set_end(end);
    }

    /// Take limit values from other axis.
    pub fn copy_limits(&mut self, other: &Self) {
        self.range.set_begin(other.begin());
        self.range.set_end(other.end());
    }

    pub fn begin(&self) -> f64 {
        self.range.begin()
    }

    pub fn end(&self) -> f64 {
        self.range.end()
    }

    pub fn contains(&self, t: &TimeStamp) -> bool {
        self.range.contains(t.amount)
    }

    /// Get the time selected by this axis!
    pub fn timespan(&self) -> TimeSpan {
        // TODO: temp hack?
        TimeSpan::new(TimeStamp::new(self.begin()), TimeStamp::new(self.end()))
    }

    pub fn domain(&self) -> f64 {
        self.end() - self.begin()
    }

    /// Zoom the axis by a certain percentage, optionally centered around some value.
    pub fn zoom(&mut self, amount: f64, around: Option<f64>) {
        let domain = self.domain();
        if (domain < 1.0e-18) && (amount < 0.0) {
            return;
        }

        if (domain > 1.0e18) && (amount > 0.0) {
            return;
        }

        let (left_percent, right_percent) = if let Some(around) = around {
            if self.begin() < around && around < self.end() {
                let left_percent = (around - self.begin()) / domain;
                assert!(left_percent < 1.0);
                let right_percent = 1.0 - left_percent;
                (left_percent, right_percent)
            } else {
                (0.5, 0.5)
            }
        } else {
            (0.5, 0.5)
        };

        let step = domain * amount * 2.0;
        let begin = self.begin() - step * left_percent;
        let end = self.end() + step * right_percent;

        self.set_limits(begin, end);
    }

    /// Perform a relative panning based on the scale of the axis.
    pub fn pan_relative(&mut self, amount: f64) {
        let domain = self.domain();
        let step = domain * amount;
        self.pan_absolute(step);
    }

    /// Pan an absolute amount.
    pub fn pan_absolute(&mut self, step: f64) {
        let begin = self.begin() + step;
        let end = self.end() + step;
        self.set_limits(begin, end);
    }

    pub fn calc_tiks(&self, n_ticks: usize) -> TickLabels {
        calc_tiks(self.range.begin(), self.range.end(), n_ticks)
    }

    /// Calculate date time tick markers.
    pub fn calc_date_tiks(&self, n_ticks: usize) -> (Option<String>, TickLabels) {
        let begin = self.range.begin();
        // If time in some range between 1973 and 2096, use data time stuff:
        if 1.0e8 < begin && begin < 4.0e9 {
            let (prefix, labels) = calc_date_ticks(begin, self.range.end(), n_ticks);
            (Some(prefix), labels)
        } else {
            (None, calc_tiks(begin, self.range.end(), n_ticks))
        }
    }
}

/// Calculate the proper major tick and minor ticks for
/// a given range.
fn calc_tiks(begin: f64, end: f64, n_ticks: usize) -> TickLabels {
    trace!("Calculating ticks!");
    let (scale, tick_step) = calc_tick_spacing(end - begin, n_ticks);
    trace!("tickz {:?}", tick_step);

    let first_tick = ceil_to_multiple_of(begin, tick_step);
    let tick_values = create_points(first_tick, end, tick_step);

    let res = tick_values
        .into_iter()
        .map(|x| {
            let label = format_at_scale(x, scale);
            (x, label)
        })
        .collect();

    trace!("Ticks: {:?}", res);
    res
}

fn create_points(start: f64, end: f64, step: f64) -> Vec<f64> {
    let mut res = vec![];
    let mut x = start;
    while x < end {
        res.push(x);
        x += step;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::super::tests::compare_ticks;
    use super::ValueAxis;

    #[test]
    fn tick_calculation() {
        // Set axis to 12 to 87 and expect ticks at 20, 30, 40, 50, 60, 70 and 80.
        let mut axis = ValueAxis::default();
        axis.set_limits(12.0, 87.0);
        let ticks = axis.calc_tiks(7);

        let expected_ticks = vec![
            (20.0, "20".to_string()),
            (30.0, "30".to_string()),
            (40.0, "40".to_string()),
            (50.0, "50".to_string()),
            (60.0, "60".to_string()),
            (70.0, "70".to_string()),
            (80.0, "80".to_string()),
        ];
        compare_ticks(expected_ticks, ticks);
    }

    #[test]
    fn tick_calculation_negative() {
        // Set axis to -44 to 46 and expect ticks at -40, -30, -20, -10, 0, 10, 20, 30 and 40.
        let mut axis = ValueAxis::default();
        axis.set_limits(-44.0, 46.0);
        let ticks = axis.calc_tiks(7);

        let expected_ticks = vec![
            (-40.0, "-40".to_string()),
            (-30.0, "-30".to_string()),
            (-20.0, "-20".to_string()),
            (-10.0, "-10".to_string()),
            (0.0, "0".to_string()),
            (10.0, "10".to_string()),
            (20.0, "20".to_string()),
            (30.0, "30".to_string()),
            (40.0, "40".to_string()),
        ];
        compare_ticks(expected_ticks, ticks);
    }

    #[test]
    fn small_ticks() {
        let mut axis = ValueAxis::default();
        axis.set_limits(1.5, 3.3);
        let ticks = axis.calc_tiks(7);

        let expected_ticks = vec![
            (1.6, "1.6".to_string()),
            (1.8, "1.8".to_string()),
            (2.0, "2.0".to_string()),
            (2.2, "2.2".to_string()),
            (2.4, "2.4".to_string()),
            (2.6, "2.6".to_string()),
            (2.8, "2.8".to_string()),
            (3.0, "3.0".to_string()),
            (3.2, "3.2".to_string()),
        ];
        compare_ticks(ticks, expected_ticks);
    }
}
