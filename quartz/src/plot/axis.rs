use crate::geometry::Range;

#[derive(Default)]
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

    /// Given the current axis, calculate sensible
    /// tick values. This means, a minimum tick, but also
    /// the inter tick distance!
    fn calc_tick_start_and_spacing(&self) -> (f64, f64) {
        // let axis_width: f64 = 1000.0;
        let n_ticks = 7; // Always nice to have 7 ticks!
        let width: f64 = self.range.end() - self.range.begin();
        let scale = width.log10().floor();
        let approx = (10.0_f64).powf(-scale) * width / (n_ticks as f64);

        // Snap to grid:
        let options = vec![0.1, 0.2, 0.5, 1.0, 2.0, 5.0];
        let best = options
            .iter()
            .min_by_key(|x| (((*x - approx).abs() * 1_000_000.0) as i64))
            .unwrap();

        trace!(
            "Width: {}, Scale {}, approx: {}, best: {}",
            width,
            scale,
            approx,
            best
        );

        let tick_width = best * (10.0_f64).powf(scale);
        let start_tick = self.range.begin();

        (start_tick, tick_width)
    }

    /// Calculate the proper major tick and minor ticks for
    /// a given range.
    pub fn calc_tiks(&self) -> Vec<(f64, String)> {
        trace!("Calculating ticks!");
        let mut x: f64 = self.range.begin();
        let mut z: f64 = 0.0;
        let (_, tickz) = self.calc_tick_start_and_spacing();
        trace!("tickz {:?}", tickz);
        let mut res = vec![];

        while x < self.range.end() {
            let label = z.to_string();
            res.push((x, label));
            x += 50.0;
            z += tickz;
        }

        trace!("Ticks: {:?}", res);
        res

        // vec![(30.0, "1".to_string()), (60.0, "2".to_string()), (160.0, "3".to_string()), (260.0, "4".to_string())]
    }
}

/// Axis options
pub struct AxisOptions {
    /// Draw major tick markers
    pub major_ticks: bool,

    /// Draw minor tick markers
    pub minor_ticks: bool,
}

/// Implement sensible default axis options.
impl Default for AxisOptions {
    fn default() -> Self {
        AxisOptions {
            major_ticks: true,
            minor_ticks: false,
        }
    }
}
