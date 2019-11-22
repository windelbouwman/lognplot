use super::sample::Sample;

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

    /// This value allows for calculation of the variance of the
    /// signal.
    /// TODO: this value will be insanely large at some point
    /// in time.
    pub sum_squared: f64,

    /// The total number of samples
    pub count: usize,
}

struct LogMetrics {
    // TODO: amount of messages for each level
    /// Total amount of log messages.
    pub count: usize,
}

/// Implement this for specific observations.
pub trait Metrics {
    fn from_sample(sample: &Sample) -> Self;
    // TODO: we might merge update and include into a single function?
    fn update(&mut self, sample: &Sample);
    fn include(&mut self, metrics: &Self);
}

impl Metrics for SampleMetrics {
    fn from_sample(sample: &Sample) -> Self {
        SampleMetrics {
            min: sample.value,
            max: sample.value,
            sum: sample.value,
            sum_squared: sample.value * sample.value,
            count: 1,
        }
    }

    /// Integrate a single sample into tha metrics.
    /// This involves updating the min and max values
    /// as well as the count and the sum.
    fn update(&mut self, sample: &Sample) {
        self.min = self.min.min(sample.value);
        self.max = self.max.max(sample.value);

        self.sum += sample.value;
        self.sum_squared += sample.value * sample.value;
        self.count += 1;
    }

    /// Include other metrics into this metrics.
    fn include(&mut self, metrics: &SampleMetrics) {
        self.min = self.min.min(metrics.min);
        self.max = self.max.max(metrics.max);
        self.count += metrics.count;
        self.sum += metrics.sum;
        self.sum_squared += metrics.sum_squared;
    }
}
