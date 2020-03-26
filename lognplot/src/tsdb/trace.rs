//! The core idea of the time series database:
//! split the sample sequence into batches.
//! This will result in a tree of chunks, each chunk having either sub chunks
//! or leaf chunks, with real data.
//! Also: keep track of certain metrics, such as min, max and sum.

use super::{Aggregation, Btree, Metrics, Observation, Query, RangeQueryResult};
use crate::time::TimeSpan;

/// A trace is a single signal with a history in time.
#[derive(Debug)]
pub struct Trace<V, M>
where
    M: Metrics<V> + From<V>,
{
    tree: Btree<V, M>,
    count: usize,
    last: Option<Observation<V>>,
}

impl<V, M> Trace<V, M>
where
    V: Clone,
    M: Metrics<V> + From<V> + Clone,
{
    /// Add a vector of values to this trace.
    pub fn add_observations(&mut self, observations: Vec<Observation<V>>) {
        if !observations.is_empty() {
            self.count += observations.len();
            self.last = Some(
                observations
                    .last()
                    .expect("At least a single sample.")
                    .clone(),
            );
            self.tree.append_samples(observations);
        }
    }

    /// Add a single observation.
    pub fn add_observation(&mut self, observation: Observation<V>) {
        self.count += 1;
        self.last = Some(observation.clone());
        self.tree.append_sample(observation);
    }

    /// Query this trace for some data.
    pub fn query(&self, query: Query) -> RangeQueryResult<V, M> {
        self.tree.query_range(&query.interval, query.amount)
    }

    pub fn quick_summary(&self) -> Option<(usize, Observation<V>)> {
        if let Some(last) = &self.last {
            Some((self.count, last.clone()))
        } else {
            None
        }
    }

    pub fn summary(&self, timespan: Option<&TimeSpan>) -> Option<Aggregation<V, M>> {
        if let Some(timespan) = timespan {
            self.tree.range_summary(timespan)
        } else {
            self.tree.summary()
        }
    }

    pub fn to_vec(&self) -> Vec<Observation<V>> {
        self.tree.to_vec()
    }
}

impl<V, M> Default for Trace<V, M>
where
    V: Clone,
    M: Metrics<V> + From<V> + Clone,
{
    fn default() -> Self {
        let tree = Default::default();

        Self {
            tree,
            count: 0,
            last: None,
        }
    }
}
