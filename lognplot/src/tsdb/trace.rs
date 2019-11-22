//! The core idea of the time series database:
//! split the sample sequence into batches.
//! This will result in a tree of chunks, each chunk having either sub chunks
//! or leave chunks, with real data.
//! Also: keep track of certain metrics, such as min, max and sum.

// use std::cell::RefCell;
use super::metrics::SampleMetrics;
use super::query::{Query, QueryResult};
use super::sample::Sample;
use super::{Btree, Observation};

/// A trace is a single signal with a history in time.
#[derive(Debug)]
pub struct Trace {
    tree: Btree<Sample, SampleMetrics>,
}

impl Trace {
    /// Add a vector of values to this trace.
    pub fn add_values(&mut self, samples: Vec<Sample>) {
        for sample in samples {
            self.add_sample(sample);
        }

        // self.tree.append_samples(samples);
    }

    /// Add a single sample.
    pub fn add_sample(&mut self, sample: Sample) {
        let timestamp = sample.timestamp.clone();
        let observation = Observation::new(timestamp, sample);

        self.tree.append_sample(observation);
    }

    /// Query this trace for some data.
    pub fn query(&self, query: Query) -> QueryResult {
        let samples = self.tree.query_range(&query.interval, 1000);

        QueryResult {
            query,
            inner: samples,
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
