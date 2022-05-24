//! A dummy database implementation, which does nothing, and drops all data into void.
//! 
//! 


use super::observations::{Observation, ProfileEvent, Sample, Text};
use super::ChangeSubscriber;
use super::{Query, QueryResult};
use super::{QuickSummary, Summary};
use crate::time::TimeSpan;
use super::handle::{LockedTsDb, make_handle};

use super::TsDbApi;

#[derive(Debug, Default)]
pub struct VoidDb {}

impl std::fmt::Display for VoidDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VoidDb")
    }
}

impl VoidDb {
    pub fn into_handle(self) -> std::sync::Arc<LockedTsDb<Self>> {
        make_handle(self)
    }
}

impl TsDbApi for VoidDb {
    fn add_value(&mut self, _name: &str, _observation: Observation<Sample>) {}
    fn add_values(&mut self, _name: &str, _samples: Vec<Observation<Sample>>) {}
    fn add_text(&mut self, _name: &str, _observation: Observation<Text>) {}
    fn add_profile_event(&mut self, _name: &str, _observation: Observation<ProfileEvent>) {}

    fn delete_all(&mut self) {}
    fn delete(&mut self, _name: &str) {}

    fn get_signal_names(&self) -> Vec<String> {
        vec![]
    }

    fn quick_summary(&self, _name: &str) -> Option<QuickSummary> {
        None
    }

    fn summary(&self, _name: &str, _timespan: Option<&TimeSpan>) -> Option<Summary> {
        None
    }

    fn get_raw_samples(&self, _name: &str) -> Option<Vec<Observation<Sample>>> {
        None
    }

    fn query(&self, _name: &str, _query: Query) -> Option<QueryResult> {
        None
    }

    // notifications
    fn register_notifier(&mut self, _subscriber: ChangeSubscriber) {}
    fn poll_events(&mut self) {}
}
