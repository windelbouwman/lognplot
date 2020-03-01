//! Thread usable handle. Wrapper around a database.

use super::ChangeSubscriber;
use super::{
    Aggregation, Observation, Query, QueryResult, QuickSummary, Sample, SampleMetrics, TsDb,
};
use crate::time::TimeSpan;
use std::sync::{Arc, Mutex};

pub type TsDbHandle = Arc<LockedTsDb>;

pub fn make_handle(db: TsDb) -> TsDbHandle {
    Arc::new(LockedTsDb::new(db))
}

#[derive(Debug)]
pub struct LockedTsDb {
    db: Mutex<TsDb>,
}

impl LockedTsDb {
    pub fn new(db: TsDb) -> Self {
        LockedTsDb { db: Mutex::new(db) }
    }

    pub fn get_signal_names(&self) -> Vec<String> {
        self.db.lock().unwrap().get_signal_names()
    }

    /// Create a new trace.
    pub fn new_trace(&self, name: &str) {
        self.db.lock().unwrap().new_trace(name)
    }

    /// Add a single observation.
    pub fn add_value(&self, name: &str, sample: Observation<Sample>) {
        self.db.lock().unwrap().add_value(name, sample);
    }

    /// Add a series of observations
    pub fn add_values(&self, name: &str, samples: Vec<Observation<Sample>>) {
        self.db.lock().unwrap().add_values(name, samples);
    }

    /// Query the database.
    pub fn query(&self, name: &str, query: Query) -> QueryResult {
        self.db.lock().unwrap().query(name, query)
    }

    pub fn get_raw_samples(&self, name: &str) -> Option<Vec<Observation<Sample>>> {
        self.db.lock().unwrap().get_raw_samples(name)
    }

    /// Grab a quick data summary.
    ///
    /// This summary includes:
    /// - sample count
    /// - last observation
    pub fn quick_summary(&self, name: &str) -> Option<QuickSummary> {
        self.db.lock().unwrap().quick_summary(name)
    }

    /// Retrieve a detailed summary of the data.
    ///
    /// Summary includes:
    /// - mean, min, max, standard deviation
    /// - sample count
    /// - first, last observations
    pub fn summary(
        &self,
        name: &str,
        timespan: Option<&TimeSpan>,
    ) -> Option<Aggregation<Sample, SampleMetrics>> {
        self.db.lock().unwrap().summary(name, timespan)
    }

    pub fn drop_all(&self) {
        self.db.lock().unwrap().drop_all();
    }

    pub fn register_notifier(&self, subscriber: ChangeSubscriber) {
        self.db.lock().unwrap().register_notifier(subscriber);
    }

    pub fn poll_events(&self) {
        self.db.lock().unwrap().poll_events();
    }
}

impl std::fmt::Display for LockedTsDb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.db.lock().unwrap())
    }
}
