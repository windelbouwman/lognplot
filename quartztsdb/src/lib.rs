//! Time series database, usable as a library.

#[macro_use]
extern crate log;

mod btree;
mod connection;
pub mod datasource;
mod db;
mod handle;
mod metrics;
mod query;
mod sample;
mod time;
mod trace;

use btree::Btree;
pub use db::TsDb;
pub use handle::TsDbHandle;
pub use query::{Query, QueryResult, SubResult};
pub use sample::Sample;
pub use time::{TimeSpan, TimeStamp};
pub use trace::Trace;

#[cfg(test)]
mod tests {
    use super::connection::Connection;
    use super::query::Query;
    use super::Sample;
    use super::TsDb;
    use crate::time::TimeModifiers;
    use crate::time::{Resolution, TimeSpan, TimeStamp};

    #[test]
    fn basic_usage() {
        let mut db = TsDb::new();
        db.open();
        let trace_name = "foo";

        // Create a trace:
        db.new_trace(trace_name);

        // Insert data:
        let ts = TimeStamp::new(0.0);
        let sample = Sample::new(ts.clone(), 3.1415926);
        db.add_value(trace_name, sample.clone());

        // Now onto the query part:
        let timespan = TimeSpan::new(ts.add_millis(-1), ts.add_millis(1));
        let query = Query::new(timespan, Resolution::NanoSeconds);
        let result = db.get_values(trace_name, query);
        // assert_eq!(1, result.samples.first().unwrap().len());
        db.close();
    }

    #[test]
    fn empty_query() {
        let mut db = TsDb::new();
        db.open();

        let trace_name = "foo";

        // Create a trace:
        db.new_trace(trace_name);

        // Insert data:
        let ts = TimeStamp::new(0.0);
        let sample = Sample::new(ts.clone(), 3.1415926);
        db.add_value(trace_name, sample.clone());

        // Query empty range:
        let timespan = TimeSpan::new(ts.add_millis(1), ts.add_millis(3));
        let query = Query::new(timespan, Resolution::NanoSeconds);
        let result = db.get_values(trace_name, query);
        // assert_eq!(0, result.samples.first().unwrap().len());

        db.close();
    }
}
