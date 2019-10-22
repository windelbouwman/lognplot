//! Time series database

mod chunk;
mod connection;
pub mod datasource;
mod db;
mod metrics;
mod query;
mod sample;
mod trace;

pub use db::{TsDb, TsDbHandle};
pub use sample::Sample;
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
    fn it_works() {
        let db = TsDb::new();
        db.open();
        let ts = TimeStamp::new(0.0);
        let sample = Sample::new(ts, 3.1415926);
        db.new_trace("foo");
        db.add_value("foo", sample.clone());
        // db.add_values();
        let timespan = TimeSpan::new(
            sample.timestamp.add_millis(-1),
            sample.timestamp.add_millis(1),
        );
        let values = db.get_values("foo", &timespan, &Resolution::NanoSeconds);
        assert_eq!(1, values.len());
        db.close();
    }
}
