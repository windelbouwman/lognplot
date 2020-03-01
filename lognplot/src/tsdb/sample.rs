//! Log samples of scalar values.
//!
//! This module enables logging of scalar values.

use super::Metrics;
use super::Observation;

/// A simple scalar value.
#[derive(Clone, Debug)]
pub struct Sample {
    pub value: f64,
}

impl Sample {
    pub fn new(value: f64) -> Self {
        Sample { value }
    }
}

/// Less detailed summary, but easier to keep track of.
#[derive(Debug, Clone)]
pub struct QuickSummary {
    pub count: usize,
    pub last: Observation<Sample>,
}

impl QuickSummary {
    pub fn new(count: usize, last: Observation<Sample>) -> Self {
        QuickSummary { count, last }
    }
}

/// Metrics collected about a certain trace
/// This can be used during query.
#[derive(Debug, Clone)]
pub struct SampleMetrics {
    /// The minimum value of all samples
    pub min: f64,

    /// The maximum value of all samples
    pub max: f64,

    /// The first observed value
    pub first: f64,

    /// The last measured value
    pub last: f64,

    /// The mean of all values.
    mean: f64,

    /// This value allows for calculation of the variance of the
    /// signal.
    /// This variable should be private, to hide this weird math from the outside world.
    /// See also: https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm
    m2: f64,

    /// The total number of samples
    pub count: usize,
}

impl SampleMetrics {
    /// Create metrics from a single value!
    fn from_value(value: f64) -> Self {
        SampleMetrics {
            min: value,
            max: value,
            mean: value,
            first: value,
            last: value,
            m2: 0.0,
            count: 1,
        }
    }

    /// Include a single observation into the mix:
    fn inject_value(&mut self, value: f64) {
        // These updates are trivial:
        self.min = self.min.min(value);
        self.max = self.max.max(value);

        // Assume the value is appended:
        self.last = value;

        self.count += 1;

        // Less trivial update below.. statistical stuff!

        // Welford online algorithm:
        let new_mean = self.mean + (value - self.mean) / self.count as f64;
        let delta = (value - self.mean) * (value - new_mean);
        self.m2 += delta;
        self.mean = new_mean;
    }

    /// Create new metrics from a given series of values.
    pub fn from_values(values: &[f64]) -> Option<Self> {
        if values.is_empty() {
            None
        } else {
            let (first, rest) = values.split_first().unwrap();
            let mut metrics = SampleMetrics::from_value(*first);
            for value in rest {
                metrics.inject_value(*value);
            }
            Some(metrics)
        }
    }

    /// Calculate the mean value of this metrics.
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Retrieve the variance
    pub fn variance(&self) -> f64 {
        // Use population variance, since we have all samples!
        self.m2 / self.count as f64
    }

    /// Calculate the standard deviation
    pub fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl From<Sample> for SampleMetrics {
    fn from(sample: Sample) -> Self {
        SampleMetrics::from_value(sample.value)
    }
}

impl Metrics<Sample> for SampleMetrics {
    /// Integrate a single sample into tha metrics.
    /// This involves updating the min and max values
    /// as well as the count and the sum.
    fn update(&mut self, sample: &Sample) {
        self.inject_value(sample.value);
    }

    /// Include other metrics into this metrics.
    fn include(&mut self, metrics: &SampleMetrics) {
        self.min = self.min.min(metrics.min);
        self.max = self.max.max(metrics.max);

        // Assume metrics are appended
        self.last = metrics.last;

        let delta = metrics.mean - self.mean;
        let new_count = self.count + metrics.count;
        let new_mean = ((self.mean * self.count as f64) + (metrics.mean * metrics.count as f64))
            / (new_count as f64);
        let new_m2 = self.m2
            + metrics.m2
            + delta * delta * (self.count as f64 * metrics.count as f64) / new_count as f64;

        self.count = new_count;
        self.mean = new_mean;
        self.m2 = new_m2;
    }
}

#[cfg(test)]
mod tests {
    use super::Metrics;
    use super::{Sample, SampleMetrics};

    fn almost_equal(v1: f64, v2: f64, tolerance: f64) {
        assert!((v1 - v2).abs() < tolerance);
    }

    #[test]
    fn metric_updates() {
        // Create test samples:
        let sample1 = Sample::new(2.0);
        let sample2 = Sample::new(1.0);
        let sample3 = Sample::new(3.0);
        let sample4 = Sample::new(5.0);
        let sample5 = Sample::new(4.0);

        let mut metrics = SampleMetrics::from(sample1);
        metrics.update(&sample2);
        metrics.update(&sample3);
        metrics.update(&sample4);
        metrics.update(&sample5);

        assert_eq!(metrics.min, 1.0);
        assert_eq!(metrics.max, 5.0);
        assert_eq!(metrics.first, 2.0);
        assert_eq!(metrics.last, 4.0);
        assert_eq!(metrics.count, 5);
        assert_eq!(metrics.mean(), 3.0);
        assert_eq!(metrics.variance(), 2.0);
        almost_equal(metrics.stddev(), 1.414213562373, 1.0e-9);

        metrics.include(&metrics.clone());

        assert_eq!(metrics.min, 1.0);
        assert_eq!(metrics.max, 5.0);
        assert_eq!(metrics.count, 10);
        assert_eq!(metrics.mean(), 3.0);
        assert_eq!(metrics.variance(), 2.0);
        almost_equal(metrics.stddev(), 1.414213562373, 1.0e-9);
    }
}
