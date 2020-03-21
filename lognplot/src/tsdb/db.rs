//! Time series database which uses B+ trees to store tha data.

use super::handle::{make_handle, TsDbHandle};
use super::query::{Query, QueryResult};
use super::trace::Trace;
use super::ChangeSubscriber;
use super::{Aggregation, Observation};
use super::{QuickSummary, Sample, SampleMetrics};
use crate::time::{TimeSpan, TimeStamp};
use std::collections::HashMap;

/// A time series database which can be used as a library.
/// Note that this struct is not usable in multiple threads.
/// To make it accessible from multiple threads, use the TsDbHandle wrapper.
#[derive(Debug)]
pub struct TsDb {
    path: String,
    pub data: HashMap<String, Trace>,
    change_subscribers: Vec<ChangeSubscriber>,
}

impl std::fmt::Display for TsDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TsDb {} with {} traces", self.path, self.data.len())
    }
}

impl Default for TsDb {
    fn default() -> Self {
        let path = "x".to_string();
        let data = HashMap::new();
        let change_subscribers = vec![];
        Self {
            path,
            data,
            change_subscribers,
        }
    }
}

/// Database for time series.
impl TsDb {
    pub fn into_handle(self) -> TsDbHandle {
        make_handle(self)
    }

    pub fn get_signal_names(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    fn get_or_create_trace(&mut self, name: &str, first_timestamp: &TimeStamp) -> &mut Trace {
        if self.data.contains_key(name) {
            let trace = self.data.get(name).expect("name to be present");
            if let Some(summary) = trace.quick_summary() {
                let last_saved_observation_time = summary.last.timestamp;

                if first_timestamp < &last_saved_observation_time {
                    // Copy trace into backup, and begin a new trace.
                    let trace = self.data.remove(name).unwrap();
                    let now = chrono::offset::Local::now();
                    let date_time_marker = now.format("%Y%m%d_%H%M%S");
                    let mut new_name = format!("{}_BACKUP_{}", name, date_time_marker);

                    while self.data.contains_key(&new_name) {
                        new_name.push_str("_a");
                    }

                    // Copy of the old data:
                    let no_data = self.data.insert(new_name.clone(), trace);
                    assert!(no_data.is_none()); // "Name must not be present already."
                    self.notify_signal_added(&new_name);
                    self.notify_signal_changed(&new_name);

                    // New trace:
                    self.new_trace(name);
                }
            }
        } else {
            self.new_trace(name);
            self.notify_signal_added(name);
        }

        self.data.get_mut(name).unwrap()
    }

    /// Add a batch of values
    pub fn add_values(&mut self, name: &str, samples: Vec<Observation<Sample>>) {
        if !samples.is_empty() {
            let first_observation = samples.first().expect("Must have an observation here.");
            let trace = self.get_or_create_trace(name, &first_observation.timestamp);
            trace.add_values(samples);
            self.notify_signal_changed(name);
        }
    }

    pub fn new_trace(&mut self, name: &str) {
        let trace = Trace::default();
        self.data.insert(name.to_string(), trace);
        // self.get_trace(name)
    }

    /// Add a single observation to the database.
    pub fn add_value(&mut self, name: &str, observation: Observation<Sample>) {
        let trace = self.get_or_create_trace(name, &observation.timestamp);
        trace.add_sample(observation);
        self.notify_signal_changed(name);
    }

    /// Delete all data from the database.
    pub fn delete_all(&mut self) {
        self.data.clear();
        self.data.shrink_to_fit();
        self.notify_delete_all();
    }

    /// Delete a single trace from the database.
    pub fn delete(&mut self, name: &str) {
        self.data.remove(name);
        self.delete_event(name);
    }

    /// Query the given trace for data.
    pub fn query(&self, name: &str, query: Query) -> QueryResult {
        if let Some(trace) = self.data.get(name) {
            trace.query(query)
        } else {
            QueryResult { query, inner: None }
        }
    }

    // Download raw samples.
    pub fn get_raw_samples(&self, name: &str) -> Option<Vec<Observation<Sample>>> {
        self.data.get(name).map(|t| t.to_vec())
    }

    pub fn quick_summary(&self, name: &str) -> Option<QuickSummary> {
        self.data.get(name)?.quick_summary()
    }

    /// Get a summary for a certain timerange (or all time) the given trace.
    pub fn summary(
        &self,
        name: &str,
        timespan: Option<&TimeSpan>,
    ) -> Option<Aggregation<Sample, SampleMetrics>> {
        self.data.get(name)?.summary(timespan)
    }

    // Events

    /// Register a subscriber which will be notified of any change.
    pub fn register_notifier(&mut self, mut subscriber: ChangeSubscriber) {
        // Add a new signal event for all currently present signals:
        for signal_name in self.data.keys() {
            subscriber.notify_signal_added(signal_name);
            subscriber.notify_signal_changed(signal_name);
        }

        // Poll once to flush the above 'new' signals.
        subscriber.poll_events();

        // Poll twice to mark the event as ready to be sent:
        subscriber.poll_events();
        self.change_subscribers.push(subscriber);
    }

    // Check if we have pending events, and emit them to queues.
    pub fn poll_events(&mut self) {
        for subscriber in &mut self.change_subscribers {
            subscriber.poll_events();
        }
    }

    /// Notify listeners of the newly arrived data.
    fn notify_signal_changed(&mut self, name: &str) {
        for subscriber in &mut self.change_subscribers {
            subscriber.notify_signal_changed(name);
        }
    }

    fn notify_signal_added(&mut self, name: &str) {
        for subscriber in &mut self.change_subscribers {
            subscriber.notify_signal_added(name);
        }
    }

    fn delete_event(&mut self, _name: &str) {
        unimplemented!("TODO");
    }

    fn notify_delete_all(&mut self) {
        for subscriber in &mut self.change_subscribers {
            subscriber.notify_delete_all();
        }
    }
}
