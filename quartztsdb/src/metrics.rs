use super::sample::Sample;
use super::time::TimeSpan;

/// Metrics collected about a certain trace
/// This can be used during query.
#[derive(Debug)]
pub struct SampleMetrics {
    /// The minimum value of all samples
    pub min: Option<f64>,

    /// The maximum value of all samples
    pub max: Option<f64>,

    /// The sum of all values. Together with the count, this
    /// allows to calculate the average value.
    pub sum: f64,

    /// The total number of samples
    pub count: usize,

    /// The timespan this metric is about.
    pub timespan: Option<TimeSpan>,
}

impl SampleMetrics {
    /// Integrate a single sample into tha metrics.
    /// This involves updating the min and max values
    /// as well as the count and the sum.
    pub fn update(&mut self, sample: &Sample) {
        let min2 = if let Some(min) = self.min {
            min.min(sample.value)
        } else {
            sample.value
        };
        self.min = Some(min2);

        let max2 = if let Some(max) = self.max {
            max.max(sample.value)
        } else {
            sample.value
        };
        self.max = Some(max2);

        self.sum += sample.value;
        self.count += 1;

        // Adjust timespan:
        // if let Some(ts) = self.timespan {
        // sample.timestamp
        // }
    }
}

impl Default for SampleMetrics {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            sum: 0.0,
            count: 0,
            timespan: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Sample, SampleMetrics};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn metric_updates() {
        let mut metrics = SampleMetrics::default();
        let t1 = TimeStamp::new(3.0);
        let sample1 = Sample::new(t1.clone(), 2.2);
        let sample2 = Sample::new(TimeStamp::new(8.0), 5.2);
        let t3 = TimeStamp::new(18.0);
        let sample3 = Sample::new(t3.clone(), -9.0);
        metrics.update(&sample1);
        metrics.update(&sample2);

        metrics.update(&sample3);
        assert_eq!(metrics.count, 3);
        assert_eq!(metrics.max, Some(5.2));
        assert_eq!(metrics.min, Some(-9.0));

        // TODO: floating point noise:
        // assert_eq!(metrics.sum, -1.6);

        // TODO: implement
        // assert_eq!(metrics.timespan, Some(TimeSpan::new(t1, t3)));
    }
}
