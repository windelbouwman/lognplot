//! Various functions to deal with axis
//!
//! Functionality:
//! - Calculate tick markers.

mod date;
mod options;
mod util;
mod value;

pub use value::ValueAxis;

type TickLabels = Vec<(f64, String)>;

#[cfg(test)]
mod tests {
    use super::TickLabels;

    pub fn compare_ticks(ticks1: TickLabels, ticks2: TickLabels) {
        assert!(ticks1.len() == ticks2.len());
        for (t1, t2) in ticks1.into_iter().zip(ticks2.into_iter()) {
            assert_eq!(t1.1, t2.1);
            assert_almost_eq(t1.0, t2.0, 1.0e-6);
        }
    }

    fn assert_almost_eq(v1: f64, v2: f64, tolerance: f64) {
        assert!((v1 - v2).abs() < tolerance);
    }
}
