//! The core idea of the time series database:
//! split the sample sequence into batches.
//! This will result in a tree of chunks, each chunk having either sub chunks
//! or leaf chunks, with real data.
//! Also: keep track of certain metrics, such as min, max and sum.

// use std::cell::RefCell;
use super::query::{Query, QueryResult};
use super::sample::{Sample, SampleMetrics};
use super::{Aggregation, Btree, Observation};
use crate::time::TimeSpan;

/// A trace is a single signal with a history in time.
#[derive(Debug)]
pub struct Trace {
    tree: Btree<Sample, SampleMetrics>,
}

impl Trace {
    /// Add a vector of values to this trace.
    pub fn add_values(&mut self, samples: Vec<Observation<Sample>>) {
        for sample in samples {
            self.add_sample(sample);
        }

        // self.tree.append_samples(samples);
    }

    /// Add a single sample.
    pub fn add_sample(&mut self, observation: Observation<Sample>) {
        self.tree.append_sample(observation);
    }

    /// Query this trace for some data.
    pub fn query(&self, query: Query) -> QueryResult {
        let samples = self.tree.query_range(&query.interval, query.amount);

        QueryResult {
            query,
            inner: Some(samples),
        }
    }

    pub fn summary(
        &self,
        timespan: Option<&TimeSpan>,
    ) -> Option<Aggregation<Sample, SampleMetrics>> {
        if let Some(timespan) = timespan {
            self.tree.range_summary(timespan)
        } else {
            self.tree.summary()
        }
    }

    pub fn to_vec(&self) -> Vec<Observation<Sample>> {
        self.tree.to_vec()
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }
}

impl Default for Trace {
    fn default() -> Self {
        let tree = Default::default();

        Self { tree }
    }
}
