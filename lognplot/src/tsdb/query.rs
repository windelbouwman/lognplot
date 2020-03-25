//! Query datatypes.
//!
//! The database can be queried, and will give a `QueryResult` back.

use crate::time::{Resolution, TimeSpan, TimeStamp};

#[derive(Debug)]
pub struct Query {
    pub interval: TimeSpan,
    pub resolution: Resolution,
    pub amount: usize,
}

impl Query {
    pub fn create() -> QueryBuilder {
        QueryBuilder::new()
    }

    pub fn new(interval: TimeSpan, resolution: Resolution, amount: usize) -> Self {
        Query {
            interval,
            resolution,
            amount,
        }
    }
}

pub struct QueryBuilder {
    start: Option<TimeStamp>,
    end: Option<TimeStamp>,
    amount: usize,
}

impl QueryBuilder {
    fn new() -> Self {
        QueryBuilder {
            start: None,
            end: None,
            amount: 10,
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

    /// Select this time period for query.
    pub fn span(mut self, timespan: &TimeSpan) -> Self {
        self.start = Some(timespan.start.clone());
        self.end = Some(timespan.end.clone());
        self
    }

    /// Select the minimum amount of results we want.
    pub fn amount(mut self, amount: usize) -> Self {
        self.amount = amount;
        self
    }

    /// Finish building the query, and construct it!
    pub fn build(self) -> Query {
        let start = self.start.expect("No 'start' value given for the query!");
        let end = self.end.expect("No 'end' value given for the query!");
        let interval = TimeSpan::new(start, end);
        Query::new(interval, Resolution::NanoSeconds, self.amount)
    }
}
