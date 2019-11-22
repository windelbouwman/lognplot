//! Query datatypes.
//!
//! The database can be queried, and will give a `QueryResult` back.

use super::metrics::SampleMetrics;
use super::sample::Sample;
use super::Aggregation;
use crate::time::{Resolution, TimeSpan, TimeStamp};

#[derive(Debug)]
pub struct Query {
    pub interval: TimeSpan,
    pub resolution: Resolution,
}

impl Query {
    pub fn create() -> QueryBuilder {
        QueryBuilder::new()
    }

    pub fn new(interval: TimeSpan, resolution: Resolution) -> Self {
        Query {
            interval,
            resolution,
        }
    }
}

pub struct QueryBuilder {
    start: Option<TimeStamp>,
    end: Option<TimeStamp>,
}

impl QueryBuilder {
    fn new() -> Self {
        QueryBuilder {
            start: None,
            end: None,
        }
    }

    /// Select the start point for this query!
    pub fn start(mut self, start: TimeStamp) -> Self {
        self.start = Some(start);
        self
    }

    /// Select the end timestamp for this query!
    pub fn end(mut self, end: TimeStamp) -> Self {
        self.end = Some(end);
        self
    }

    /// Finish building the query, and construct it!
    pub fn build(self) -> Query {
        let start = self.start.expect("No 'start' value given for the query!");
        let end = self.end.expect("No 'end' value given for the query!");
        let interval = TimeSpan::new(start, end);
        Query::new(interval, Resolution::NanoSeconds)
    }
}

/// This holds the result of a query to the database.
/// The result can be a combination of several things, depending upon query type.
/// It can be min/max/mean slices, or single values, if the data is present at the
/// proper resolution.
pub struct QueryResult {
    pub query: Query,
    pub samples: Vec<SubResult>,
}

impl QueryResult {
    pub fn into_vec(self) -> Vec<Sample> {
        let mut all_samples = vec![];
        for sub in self.samples.into_iter() {
            if let SubResult::Single { samples } = sub {
                all_samples.extend(samples);
            } else {
                panic!("Result contains aggregates!");
            }
        }
        all_samples
    }
}

pub enum SubResult {
    Single {
        samples: Vec<Sample>,
    },
    Aggregated {
        aggregates: Vec<Aggregation<SampleMetrics>>,
    },
}
