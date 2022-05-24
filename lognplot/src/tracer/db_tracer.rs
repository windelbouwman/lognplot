//! Trace events straight into a database.

use super::Tracer;
use crate::time::TimeStamp;
use crate::tsdb::observations::{Observation, Sample, Text};
use crate::tsdb::TsDbHandle;
use std::time::Instant;

/// A struct which allows recording
/// performance metrics to a database handle.
pub struct DbTracer {
    gui_start_instant: Instant,
    db: TsDbHandle,
}

impl DbTracer {
    pub fn new(db: TsDbHandle) -> Self {
        DbTracer {
            gui_start_instant: Instant::now(),
            db,
        }
    }

    fn get_timestamp(&self, timestamp: Instant) -> TimeStamp {
        let elapsed = timestamp.duration_since(self.gui_start_instant);
        let elapsed_seconds: f64 = elapsed.as_secs_f64();
        TimeStamp::new(elapsed_seconds)
    }
}

impl Tracer for DbTracer {
    /// This is cool stuff, log metrics about render time for example to database itself :)
    fn log_metric(&self, name: &str, timestamp: Instant, value: f64) {
        let timestamp = self.get_timestamp(timestamp);
        let observation = Observation::new(timestamp, Sample::new(value));
        self.db.add_value(name, observation);
    }

    fn log_text(&self, name: &str, timestamp: Instant, text: String) {
        let timestamp = self.get_timestamp(timestamp);
        let observation = Observation::new(timestamp, Text::new(text));
        self.db.add_text(name, observation);
    }
}
