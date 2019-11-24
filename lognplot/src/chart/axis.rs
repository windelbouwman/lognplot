use super::axis_options::AxisOptions;
use crate::geometry::Range;
use crate::time::{TimeSpan, TimeStamp};

#[derive(Default, Clone)]
pub struct Axis {
    pub options: AxisOptions,
    pub label: Option<String>,
    range: Range<f64>,
}

impl Axis {
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

    /// Given the current axis, calculate sensible
    /// tick spacing.
    fn calc_tick_spacing(&self, n_ticks: usize) -> f64 {
        let domain: f64 = self.domain();
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
        tick_width
    }

    /// Calculate the proper major tick and minor ticks for
    /// a given range.
    pub fn calc_tiks(&self, n_ticks: usize) -> Vec<(f64, String)> {
        trace!("Calculating ticks!");
        // let n_ticks = 7; // Always nice to have 7 ticks!
        let tick_step = self.calc_tick_spacing(n_ticks);
        trace!("tickz {:?}", tick_step);

        let mut x = ceil_to_multiple_of(self.range.begin(), tick_step);
        let mut res = vec![];

        while x < self.range.end() {
            let label = x.to_string();
            res.push((x, label));
            x += tick_step;
        }

        trace!("Ticks: {:?}", res);
        res
    }
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

#[cfg(test)]
mod tests {
    use super::Axis;

    #[test]
    fn tick_calculation() {
        // Set axis to 12 to 87 and expect ticks at 20, 30, 40, 50, 60, 70 and 80.
        let mut axis = Axis::default();
        axis.set_limits(12.0, 87.0);
        let ticks = axis.calc_tiks(7);

        assert_eq!(
            vec![
                // (10.0, "10".to_string()),
                (20.0, "20".to_string()),
                (30.0, "30".to_string()),
                (40.0, "40".to_string()),
                (50.0, "50".to_string()),
                (60.0, "60".to_string()),
                (70.0, "70".to_string()),
                (80.0, "80".to_string()),
            ],
            ticks
        );
    }

    #[test]
    fn tick_calculation_negative() {
        // Set axis to -44 to 46 and expect ticks at -40, -30, -20, -10, 0, 10, 20, 30 and 40.
        let mut axis = Axis::default();
        axis.set_limits(-44.0, 46.0);
        let ticks = axis.calc_tiks(7);

        assert_eq!(
            vec![
                (-40.0, "-40".to_string()),
                (-30.0, "-30".to_string()),
                (-20.0, "-20".to_string()),
                (-10.0, "-10".to_string()),
                (0.0, "0".to_string()),
                (10.0, "10".to_string()),
                (20.0, "20".to_string()),
                (30.0, "30".to_string()),
                (40.0, "40".to_string()),
            ],
            ticks
        );
    }
}
