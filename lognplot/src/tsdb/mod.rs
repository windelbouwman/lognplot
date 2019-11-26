//! Time series database, usable as a library.

mod aggregation;
mod btree;
mod connection;
mod db;
mod handle;
mod logrecords;
mod metrics;
mod observation;
mod query;
mod sample;
mod trace;

pub use aggregation::Aggregation;
use btree::Btree;
pub use btree::RangeQueryResult;
pub use db::TsDb;
pub use handle::TsDbHandle;
pub use metrics::Metrics;
pub use observation::Observation;
pub use query::{Query, QueryResult};
pub use sample::{Sample, SampleMetrics};
pub use trace::Trace;

#[cfg(test)]
mod tests {
    use super::connection::Connection;
    use super::query::Query;
    use super::Observation;
    use super::Sample;
    use super::TsDb;
    use crate::time::TimeModifiers;
    use crate::time::TimeStamp;

    #[test]
    fn basic_usage() {
        let mut db = TsDb::default();
        db.open();
        let trace_name = "foo";

        // Create a trace:
        db.new_trace(trace_name);

        // Insert data:
        let ts = TimeStamp::from_seconds(0);
        let sample = Sample::new(3.1415926);
        let observation = Observation::new(ts.clone(), sample);
        db.add_value(trace_name, observation);

        // Now onto the query part:
        let query = Query::create()
            .start(ts.add_millis(-1))
            .end(ts.add_millis(1))
            .build();
        let result = db.query(trace_name, query);
        assert_eq!(1, result.len());

        // Query empty range:
        let query = Query::create()
            .start(ts.add_millis(1))
            .end(ts.add_millis(3))
            .build();
        let result = db.query(trace_name, query);
        assert_eq!(0, result.len());

        db.close();
    }
}
