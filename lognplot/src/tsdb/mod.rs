//! Time series database, usable as a library.

mod aggregation;
mod btree;
mod connection;
mod db;
mod handle;
// mod logrecords;
mod metrics;
mod notify;
mod observation;
mod query;
mod query_result;
mod sample;
mod summary;
mod text;
mod trace;

pub use aggregation::Aggregation;
use btree::Btree;
pub use db::TsDb;
pub use handle::TsDbHandle;
pub use metrics::{CountMetrics, Metrics};
pub use notify::{ChangeSubscriber, DataChangeEvent};
pub use observation::Observation;
pub use query::Query;
pub use query_result::{QueryResult, RangeQueryResult};
pub use sample::{Sample, SampleMetrics};
pub use summary::QuickSummary;
pub use text::Text;
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

        // Summary info:
        let quick_summary = db.quick_summary(trace_name).unwrap();
        let summary = db.summary(trace_name, None).unwrap();
        assert_eq!(quick_summary.count, summary.count);

        db.close();
    }
}
