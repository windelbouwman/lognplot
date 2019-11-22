use super::metrics::Metrics;
use super::sample::Sample;
use crate::time::TimeSpan;

/// An aggregation of observations of some type.
#[derive(Debug, Clone)]
pub struct Aggregation<M>
where
    M: Metrics,
{
    /// The data summary
    metrics: M,

    /// Counter of the number of events.
    /// TODO: this is duplicated in the specific metrics?
    pub count: usize,

    /// The timespan this aggregation is about.
    /// This is some bucket of time which groups
    /// observed measurements.
    pub timespan: TimeSpan,
}

impl<M> Aggregation<M>
where
    M: Metrics,
{
    pub fn from_sample(sample: &Sample) -> Self {
        let timespan = TimeSpan::new(sample.timestamp.clone(), sample.timestamp.clone());
        let metrics = M::from_sample(sample);
        Aggregation {
            metrics,
            timespan,
            count: 1,
        }
    }

    pub fn update(&mut self, sample: &Sample) {
        self.metrics.update(sample);
        self.count += 1;

        // Adjust timespan:
        self.timespan.extend_to_include(&sample.timestamp);
    }

    /// Update aggregation with another aggregation
    pub fn include(&mut self, aggregation: &Aggregation<M>) {
        self.metrics.include(&aggregation.metrics);
        self.count += aggregation.count;

        // TODO: time span!
    }

    pub fn metrics(&self) -> &M {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::super::metrics::SampleMetrics;
    use super::{Aggregation, Sample};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn metric_updates() {
        // Create test samples:
        let t1 = TimeStamp::new(3.0);
        let sample1 = Sample::new(t1.clone(), 2.2);
        let sample2 = Sample::new(TimeStamp::from_seconds(8), 5.2);
        let t3 = TimeStamp::new(18.0);
        let sample3 = Sample::new(t3.clone(), -9.0);

        let mut aggregation = Aggregation::<SampleMetrics>::from_sample(&sample1);
        aggregation.update(&sample2);
        aggregation.update(&sample3);
        assert_eq!(aggregation.count, 3);
        assert_eq!(aggregation.metrics().max, 5.2);
        assert_eq!(aggregation.metrics().min, -9.0);

        // TODO: floating point noise:
        // assert_eq!(aggregation.sum, -1.6);

        assert_eq!(aggregation.timespan, TimeSpan::new(t1, t3));
    }
}
