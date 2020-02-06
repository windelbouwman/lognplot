//! Time series database which uses B+ trees to store tha data.

use super::handle::{make_handle, TsDbHandle};
use super::query::{Query, QueryResult};
use super::trace::Trace;
use super::ChangeSubscriber;
use super::{Aggregation, Observation};
use super::{Sample, SampleMetrics};
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
            if let Some(summary) = trace.summary(None) {
                let last_saved_observation_time = summary.timespan.end;

                if first_timestamp < &last_saved_observation_time {
                    // Copy trace into backup, and begin a new trace.
                    let trace = self.data.remove(name).unwrap();
                    let now = chrono::offset::Local::now();
                    let date_time_marker = now.format("%Y%m%d_%H%M%S");
                    let new_name = format!("{}_BACKUP_{}", name, date_time_marker);
                    // TODO: do not overwrite backup!
                    // assert!(!self.data.contains_key(new_name));
                    self.data.insert(new_name, trace);
                    self.new_trace(name);
                }
            }
        } else {
            self.new_trace(name);
        }

        self.data.get_mut(name).unwrap()
    }

    /// Add a batch of values
    pub fn add_values(&mut self, name: &str, samples: Vec<Observation<Sample>>) {
        if !samples.is_empty() {
            let first_observation = samples.first().expect("Must have an observation here.");
            let trace = self.get_or_create_trace(name, &first_observation.timestamp);
            trace.add_values(samples);
            self.new_data_event(name);
        }
    }

    pub fn new_trace(&mut self, name: &str) {
        let trace = Trace::default();
        self.data.insert(name.to_string(), trace);
        // self.get_trace(name)
    }

    pub fn add_value(&mut self, name: &str, observation: Observation<Sample>) {
        let trace = self.get_or_create_trace(name, &observation.timestamp);
        trace.add_sample(observation);
        self.new_data_event(name);
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
    pub fn register_notifier(&mut self, subscriber: ChangeSubscriber) {
        self.change_subscribers.push(subscriber);
    }

    /// Notify listeners of the newly arrived data.
    fn new_data_event(&mut self, name: &str) {
        for subscriber in &mut self.change_subscribers {
            subscriber.notify(name);
        }
    }
}
