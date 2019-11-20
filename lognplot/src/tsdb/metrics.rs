use super::sample::Sample;
use crate::time::TimeSpan;

/// Metrics collected about a certain trace
/// This can be used during query.
#[derive(Debug, Clone)]
pub struct SampleMetrics {
    /// The minimum value of all samples
    pub min: f64,

    /// The maximum value of all samples
    pub max: f64,

    /// The sum of all values. Together with the count, this
    /// allows to calculate the average value.
    pub sum: f64,

    pub sum_squared: f64,

    /// The total number of samples
    pub count: usize,

    /// The timespan this metric is about.
    pub timespan: TimeSpan,
}

impl SampleMetrics {
    pub fn from_sample(sample: &Sample) -> Self {
        let timespan = TimeSpan::new(sample.timestamp.clone(), sample.timestamp.clone());
        SampleMetrics {
            min: sample.value,
            max: sample.value,
            sum: sample.value,
            sum_squared: sample.value * sample.value,
            count: 1,
            timespan,
        }
    }

    /// Integrate a single sample into tha metrics.
    /// This involves updating the min and max values
    /// as well as the count and the sum.
    pub fn update(&mut self, sample: &Sample) {
        self.min = self.min.min(sample.value);
        self.max = self.max.max(sample.value);

        self.sum += sample.value;
        self.sum_squared += sample.value * sample.value;
        self.count += 1;

        // Adjust timespan:
        self.timespan.extend_to_include(&sample.timestamp);
        // if let Some(ts) = self.timespan {
        // sample.timestamp
        // }
    }

    /// Include other metrics into this metrics.
    pub fn include(&mut self, metrics: SampleMetrics) {
        self.min = self.min.min(metrics.min);
        self.max = self.max.max(metrics.max);
        self.count += metrics.count;
        self.sum += metrics.sum;
        self.sum_squared += metrics.sum_squared;

        // TODO: time span!
    }
}

#[cfg(test)]
mod tests {
    use super::{Sample, SampleMetrics};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn metric_updates() {
        // Create test samples:
        let t1 = TimeStamp::new(3.0);
        let sample1 = Sample::new(t1.clone(), 2.2);
        let sample2 = Sample::new(TimeStamp::new(8.0), 5.2);
        let t3 = TimeStamp::new(18.0);
        let sample3 = Sample::new(t3.clone(), -9.0);

        let mut metrics = SampleMetrics::from_sample(&sample1);
        metrics.update(&sample2);
        metrics.update(&sample3);
        assert_eq!(metrics.count, 3);
        assert_eq!(metrics.max, 5.2);
        assert_eq!(metrics.min, -9.0);

        // TODO: floating point noise:
        // assert_eq!(metrics.sum, -1.6);

        assert_eq!(metrics.timespan, TimeSpan::new(t1, t3));
    }
}
