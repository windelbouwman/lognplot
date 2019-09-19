//! Time series database

mod chunk;
mod connection;
mod db;
mod metrics;
mod query;
mod sample;
pub mod server;
mod trace;

pub use db::TsDb;
pub use sample::Sample;

#[cfg(test)]
mod tests {
    use super::connection::Connection;
    use super::query::Query;
    use super::Sample;
    use super::TsDb;
    use super::*;
    use crate::time::TimeModifiers;
    use crate::time::{Resolution, TimeSpan, TimeStamp};

    #[test]
    fn it_works() {
        let mut db = TsDb::new();
        db.open();
        let sample = Sample::new(3.1415926);
        db.new_trace("foo");
        db.add_value("foo", sample.clone());
        // db.add_values();
        let timespan = TimeSpan::new(
            sample.timestamp.add_millis(-1),
            sample.timestamp.add_millis(1),
        );
        let values = db.get_values(&timespan, &Resolution::NanoSeconds);
        assert_eq!(1, values.len());
        db.close();
    }
}
