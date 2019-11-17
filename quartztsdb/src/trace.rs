//! The core idea of the time series database:
//! split the sample sequence into batches.
//! This will result in a tree of chunks, each chunk having either sub chunks
//! or leave chunks, with real data.
//! Also: keep track of certain metrics, such as min, max and sum.

// use std::cell::RefCell;
use super::query::{Query, QueryResult, SubResult};
use super::sample::Sample;
use super::Btree;

/// A trace is a single signal with a history in time.
#[derive(Debug)]
pub struct Trace {
    tree: Btree,
}

impl Trace {
    /// Add a vector of values to this trace.
    pub fn add_values(&mut self, samples: Vec<Sample>) {
        self.tree.append_samples(samples);
    }

    /// Add a single sample.
    pub fn add_sample(&mut self, value: Sample) {
        self.tree.append_sample(value);
    }

    /// Query this trace for some data.
    pub fn query(&self, query: Query) -> QueryResult {
        let samples = self.tree.query_range(&query.interval, 1000);
        let samples2 = SubResult::Single { samples };

        QueryResult {
            query,
            samples: vec![samples2],
        }
    }

    pub fn to_vec(&self) -> Vec<Sample> {
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
