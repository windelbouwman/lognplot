use lognplot::time::TimeStamp;
use lognplot::tsdb::{Observation, Sample, TsDbHandle};
use std::time::Instant;

/// A helper struct which allows recording internal
/// performance metrics.
pub struct MetricRecorder {
    gui_start_instant: Instant,
    db: TsDbHandle,
}

impl MetricRecorder {
    pub fn new(db: TsDbHandle) -> Self {
        MetricRecorder {
            gui_start_instant: Instant::now(),
            db,
        }
    }

    /// This is cool stuff, log metrics about render time for example to database itself :)
    pub fn log_meta_metric(&self, name: &str, timestamp: Instant, value: f64) {
        let elapsed = timestamp.duration_since(self.gui_start_instant);
        let elapsed_seconds: f64 = elapsed.as_secs_f64();
        let timestamp = TimeStamp::new(elapsed_seconds);
        let observation = Observation::new(timestamp, Sample::new(value));
        self.db.add_value(name, observation);
    }
}
