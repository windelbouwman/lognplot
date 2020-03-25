use super::metrics::Metrics;
use super::query::Query;
use super::{Aggregation, Observation};

/// This holds the result of a query to the database.
/// The result can be several things, depending upon query type.
/// It can be min/max/mean slices, or single values, if the data is present at the
/// proper resolution.
#[derive(Debug)]
pub struct QueryResult<V, M>
where
    M: Metrics<V> + From<V>,
{
    pub query: Query,
    pub inner: Option<RangeQueryResult<V, M>>,
}

impl<V, M> QueryResult<V, M>
where
    M: Metrics<V> + From<V>,
{
    pub fn len(&self) -> usize {
        self.inner.as_ref().map(|r| r.len()).unwrap_or(0)
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
