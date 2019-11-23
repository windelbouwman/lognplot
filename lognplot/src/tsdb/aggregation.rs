use super::metrics::Metrics;
use super::Observation;
use crate::time::TimeSpan;

/// An aggregation of observations of some type.
#[derive(Debug, Clone)]
pub struct Aggregation<V, M>
where
    M: Metrics<V> + From<V>,
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

    _phantom: std::marker::PhantomData<V>,
}

impl<V, M> From<Observation<V>> for Aggregation<V, M>
where
    M: Metrics<V> + From<V>,
{
    fn from(sample: Observation<V>) -> Self {
        let timespan = TimeSpan::new(sample.timestamp.clone(), sample.timestamp.clone());
        let metrics = M::from(sample.value);
        Aggregation {
            metrics,
            timespan,
            count: 1,
            _phantom: Default::default(),
        }
    }
}

impl<V, M> Aggregation<V, M>
where
    M: Metrics<V> + From<V>,
{
    pub fn update(&mut self, sample: &Observation<V>) {
        self.metrics.update(&sample.value);
        self.count += 1;

        // Adjust timespan:
        self.timespan.extend_to_include(&sample.timestamp);
    }

    /// Update aggregation with another aggregation
    pub fn include(&mut self, aggregation: &Aggregation<V, M>) {
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
    use super::super::sample::{Sample, SampleMetrics};
    use super::{Aggregation, Observation};
    use crate::time::{TimeSpan, TimeStamp};

    #[test]
    fn metric_updates() {
        // Create test samples:
        let t1 = TimeStamp::from_seconds(3);
        let t2 = TimeStamp::from_seconds(8);
        let t3 = TimeStamp::from_seconds(18);

        let sample1 = Sample::new(2.2);
        let sample2 = Sample::new(5.2);
        let sample3 = Sample::new(-9.0);

        let observation1 = Observation::new(t1.clone(), sample1);
        let observation2 = Observation::new(t2.clone(), sample2);
        let observation3 = Observation::new(t3.clone(), sample3);

        let mut aggregation = Aggregation::<Sample, SampleMetrics>::from(observation1);
        aggregation.update(&observation2);
        aggregation.update(&observation3);
        assert_eq!(aggregation.count, 3);
        assert_eq!(aggregation.metrics().max, 5.2);
        assert_eq!(aggregation.metrics().min, -9.0);

        // TODO: floating point noise:
        // assert_eq!(aggregation.sum, -1.6);

        assert_eq!(aggregation.timespan, TimeSpan::new(t1, t3));
    }
}
