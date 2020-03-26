use super::metrics::Metrics;
use super::{Aggregation, Observation};
use super::{CountMetrics, Sample, SampleMetrics, Text};

/// This holds the result of a query to the database.
/// The result can be several things, depending upon query type.
/// It can be min/max/mean slices, or single values, if the data is present at the
/// proper resolution.
#[derive(Debug)]
pub enum QueryResult {
    Value(RangeQueryResult<Sample, SampleMetrics>),
    Text(RangeQueryResult<Text, CountMetrics>),
}

impl QueryResult {
    pub fn len(&self) -> usize {
        match self {
            QueryResult::Value(r) => r.len(),
            QueryResult::Text(r) => r.len(),
        }
    }
}

/// Inner results, can be either a series of single
/// observations, or a series of aggregate observations.
#[derive(Debug)]
pub enum RangeQueryResult<V, M>
where
    M: Metrics<V> + From<V>,
{
    Observations(Vec<Observation<V>>),
    Aggregations(Vec<Aggregation<V, M>>),
}

impl<V, M> RangeQueryResult<V, M>
where
    M: Metrics<V> + From<V>,
{
    pub fn len(&self) -> usize {
        match self {
            RangeQueryResult::Observations(observations) => observations.len(),
            RangeQueryResult::Aggregations(aggregations) => aggregations.len(),
        }
    }
}
