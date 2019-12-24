use super::axis_options::AxisOptions;
use crate::geometry::Range;
use crate::time::{TimeSpan, TimeStamp};

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

type TickLabels = Vec<(f64, String)>;

impl ValueAxis {
    pub fn set_limits(&mut self, begin: f64, end: f64) {
        self.range.set_begin(begin);
        self.range.set_end(end);
    }

    pub fn begin(&self) -> f64 {
        self.range.begin()
    }

    pub fn end(&self) -> f64 {
        self.range.end()
    }

    /// Get the time selected by this axis!
    pub fn timespan(&self) -> TimeSpan {
        // TODO: temp hack?
        TimeSpan::new(TimeStamp::new(self.begin()), TimeStamp::new(self.end()))
    }

    pub fn domain(&self) -> f64 {
        self.end() - self.begin()
    }

    pub fn zoom(&mut self, amount: f64) {
        let domain = self.domain();
        if (domain < 1.0e-18) && (amount < 0.0) {
            return;
        }

        if (domain > 1.0e18) && (amount > 0.0) {
            return;
        }

        let step = domain * amount;
        let begin = self.begin() - step;
        let end = self.end() + step;
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
}

/// Given the current axis, calculate sensible
/// tick spacing.
fn calc_tick_spacing(domain: f64, n_ticks: usize) -> (i32, f64) {
    assert!(n_ticks >= 2);

    let scale = domain.log10().floor();
    let approx = (10.0_f64).powf(-scale) * domain / (n_ticks as f64);

    // Snap to grid:
    // 10, 20, 25, 50
    let options = vec![0.1, 0.2, 0.5, 1.0, 2.0, 5.0];
    let best = options
        .iter()
        .min_by_key(|x| (((*x - approx).abs() * 1_000_000.0) as i64))
        .unwrap();

    trace!(
        "domain: {}, Scale {}, approx: {}, best: {}",
        domain,
        scale,
        approx,
        best
    );

    let tick_width = best * (10.0_f64).powf(scale);
    (scale as i32, tick_width)
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
            let label = if scale > 0 {
                x.to_string()
            } else {
                let digits = (-scale + 1) as usize;
                format!("{0:.width$}", x, width = digits)
            };
            (x, label)
        })
        .collect();

    trace!("Ticks: {:?}", res);
    res
}

/// Round the given number to an integer multiple of the given step size.
fn ceil_to_multiple_of(x: f64, step: f64) -> f64 {
    let offset = x % step;
    if offset > 0.0 {
        x + step - offset
    } else if offset < 0.0 {
        x - offset
    } else {
        x
    }
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
    use super::{TickLabels, ValueAxis};

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

    fn compare_ticks(ticks1: TickLabels, ticks2: TickLabels) {
        assert!(ticks1.len() == ticks2.len());
        for (t1, t2) in ticks1.into_iter().zip(ticks2.into_iter()) {
            assert_eq!(t1.1, t2.1);
            assert!((t1.0 - t2.0).abs() < 1.0e-6);
        }
    }
}
