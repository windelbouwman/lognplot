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
    M: Metrics<V> + From<V> + Clone,
    V: Clone,
{
    fn from(sample: Observation<V>) -> Self {
        let timespan = TimeSpan::new(sample.timestamp.clone(), sample.timestamp.clone());
        let metrics = M::from(sample.value);
        Aggregation::new(timespan, metrics, 1)
    }
}

/*
TODO: is this a good or bad idea?
impl<V, M> From<&Observation<V>> for Aggregation<V, M>
where
    M: Metrics<V> + From<V> + Clone,
{
    fn from(sample: &Observation<V>) -> Self {
        let timespan = TimeSpan::new(sample.timestamp.clone(), sample.timestamp.clone());
        let metrics = M::from(&sample.value);
        Aggregation::new(timespan, metrics, 1)
    }
}
*/

impl<V, M> From<&Aggregation<V, M>> for Aggregation<V, M>
where
    M: Metrics<V> + From<V> + Clone,
    V: Clone,
{
    fn from(reference: &Aggregation<V, M>) -> Self {
        let timespan = reference.timespan.clone();
        let metrics = reference.metrics.clone();
        Aggregation::new(timespan, metrics, 1)
    }
}

impl<V, M> Aggregation<V, M>
where
    M: Metrics<V> + From<V> + Clone,
    V: Clone,
{
    pub fn new(timespan: TimeSpan, metrics: M, count: usize) -> Self {
        Aggregation {
            metrics,
            timespan,
            count,
            _phantom: Default::default(),
        }
    }

    /// Merge given aggregations into a single aggregate
    pub fn from_aggregations(aggregations: &[Aggregation<V, M>]) -> Option<Self> {
        if aggregations.is_empty() {
            None
        } else {
            let (first, rest) = aggregations.split_first().unwrap();
            let mut merged_aggregations = Aggregation::from(first);
            for aggregation in rest {
                merged_aggregations.include_aggregation(aggregation);
            }
            Some(merged_aggregations)
        }
    }

    pub fn from_observations(observations: &[Observation<V>]) -> Option<Self> {
        if observations.is_empty() {
            None
        } else {
            let (first, rest) = observations.split_first().unwrap();
            let mut aggregation = Aggregation::from(first.clone());
            for observation in rest {
                aggregation.include_observation(observation);
            }
            Some(aggregation)
        }
    }

    pub fn include_observation(&mut self, sample: &Observation<V>) {
        self.metrics.update(&sample.value);
        self.count += 1;

        // Adjust timespan:
        self.timespan.extend_to_include(&sample.timestamp);
    }

    /// Update aggregation with another aggregation
    pub fn include_aggregation(&mut self, aggregation: &Aggregation<V, M>) {
        self.metrics.include(&aggregation.metrics);
        self.count += aggregation.count;

        // Adjust time span:
        self.timespan.extend_to_include_span(&aggregation.timespan);
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
        aggregation.include_observation(&observation2);
        aggregation.include_observation(&observation3);
        assert_eq!(aggregation.count, 3);
        assert_eq!(aggregation.metrics().max, 5.2);
        assert_eq!(aggregation.metrics().min, -9.0);

        // TODO: floating point noise:
        // assert_eq!(aggregation.sum, -1.6);

        assert_eq!(aggregation.timespan, TimeSpan::new(t1, t3));
    }
}
