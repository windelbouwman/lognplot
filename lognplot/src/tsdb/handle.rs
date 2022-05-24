//! Thread usable handle. Wrapper around a database.

use super::observations::{Observation, ProfileEvent, Sample, Text};
use super::{ChangeSubscriber, DataChangeEvent};
use super::{Query, QueryResult, QuickSummary, Summary, TsDb};
use crate::time::TimeSpan;
use futures::channel::mpsc;
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

    /// Add a single observation.
    pub fn add_value(&self, name: &str, sample: Observation<Sample>) {
        self.db.lock().unwrap().add_value(name, sample);
    }

    /// Add a series of observations
    pub fn add_values(&self, name: &str, samples: Vec<Observation<Sample>>) {
        self.db.lock().unwrap().add_values(name, samples);
    }

    pub fn add_text(&self, name: &str, text: Observation<Text>) {
        self.db.lock().unwrap().add_text(name, text);
    }

    pub fn add_profile_event(&self, name: &str, event: Observation<ProfileEvent>) {
        self.db.lock().unwrap().add_profile_event(name, event);
    }

    /// Query the database.
    pub fn query(&self, name: &str, query: Query) -> Option<QueryResult> {
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
    pub fn summary(&self, name: &str, timespan: Option<&TimeSpan>) -> Option<Summary> {
        self.db.lock().unwrap().summary(name, timespan)
    }

    /// Delete all data from the database.
    pub fn delete_all(&self) {
        self.db.lock().unwrap().delete_all();
    }

    /// Register database change handler.
    pub fn new_notify_queue(&self) -> mpsc::Receiver<DataChangeEvent> {
        let (sender, receiver) = mpsc::channel::<DataChangeEvent>(0);
        let sub = ChangeSubscriber::new(sender);
        self.register_notifier(sub);
        receiver
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
