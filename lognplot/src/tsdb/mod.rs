//! Time series database, usable as a library.

mod btree;
mod connection;
mod db;
mod handle;
pub mod observations;

mod notify;
mod query;
mod query_result;

mod summary;

mod trace;
mod track;
mod track_type;

use btree::Btree;
pub use db::TsDb;
pub use handle::TsDbHandle;

pub use notify::{ChangeSubscriber, DataChangeEvent};
pub use query::Query;
pub use query_result::{QueryResult, RangeQueryResult};

pub use summary::{QuickSummary, Summary};

pub use trace::Trace;
pub use track::Track;
pub use track_type::TrackType;

#[cfg(test)]
mod tests {
    use super::connection::Connection;
    use super::observations::{Observation, Sample};
    use super::query::Query;
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
        let result = db.query(trace_name, query).unwrap();
        assert_eq!(1, result.len());

        // Query empty range:
        let query = Query::create()
            .start(ts.add_millis(1))
            .end(ts.add_millis(3))
            .build();
        let result = db.query(trace_name, query).unwrap();
        assert_eq!(0, result.len());

        // Summary info:
        let quick_summary = db.quick_summary(trace_name).unwrap();
        let summary = db.summary(trace_name, None).unwrap();
        assert_eq!(quick_summary.count, summary.count());

        db.close();
    }
}
