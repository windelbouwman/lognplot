//! Database API
//!
//! This trait defines the time series database API

use super::observations::{Observation, ProfileEvent, Sample, Text};
use super::ChangeSubscriber;
use super::{Query, QueryResult};
use super::{QuickSummary, Summary};
use crate::time::TimeSpan;

/// Database API
pub trait TsDbApi {
    // === Add api
    fn add_value(&mut self, name: &str, observation: Observation<Sample>);
    fn add_values(&mut self, name: &str, samples: Vec<Observation<Sample>>);
    fn add_text(&mut self, name: &str, observation: Observation<Text>);
    fn add_profile_event(&mut self, name: &str, observation: Observation<ProfileEvent>);

    // ==== Remove api
    fn delete_all(&mut self);
    fn delete(&mut self, name: &str);

    // ==== Query api
    fn get_signal_names(&self) -> Vec<String>;
    fn quick_summary(&self, name: &str) -> Option<QuickSummary>;
    fn summary(&self, name: &str, timespan: Option<&TimeSpan>) -> Option<Summary>;
    fn get_raw_samples(&self, name: &str) -> Option<Vec<Observation<Sample>>>;
    fn query(&self, name: &str, query: Query) -> Option<QueryResult>;

    // notifications
    fn register_notifier(&mut self, subscriber: ChangeSubscriber);
    fn poll_events(&mut self);
}
