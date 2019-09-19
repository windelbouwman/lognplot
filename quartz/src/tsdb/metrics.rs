use super::sample::Sample;

/// Metrics collected about a certain trace
pub struct SampleMetrics {
    min: f64,
    max: f64,
    sum: f64,
}

impl SampleMetrics {
    /// Integrate a single sample into tha metrics.
    pub fn update(&mut self, sample: &Sample) {
        self.min = self.min.min(sample.value);
        self.max = self.max.max(sample.value);
        self.sum += sample.value;
    }
}

impl Default for SampleMetrics {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
            sum: 0.0,
        }
    }
}
