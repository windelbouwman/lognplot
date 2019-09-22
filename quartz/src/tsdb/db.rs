//! Time series database which uses B+ trees to store tha data.

use super::sample::Sample;
use super::trace::Trace;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type TsDbHandle = Arc<Mutex<TsDb>>;

pub struct TsDb {
    path: String,
    pub data: HashMap<String, Trace>,
}

impl std::fmt::Display for TsDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TsDb {} with {} datas", self.path, self.data.len())
    }
}

/// Database for time series.
impl TsDb {
    pub fn new() -> Self {
        let path = "x".to_string();
        let data = HashMap::new();
        Self { path, data }
    }

    pub fn into_handle(self) -> TsDbHandle {
        Arc::new(Mutex::new(self))
    }

    /// Add a batch of values
    pub fn add_values(&mut self, name: &str, samples: Vec<Sample>) {
        let trace = self.data.get_mut(name).unwrap();
        for sample in samples {
            trace.push(sample);
        }
    }

    pub fn new_trace(&mut self, name: &str) {
        self.data.insert(name.to_string(), Default::default());
    }

    pub fn add_value(&mut self, name: &str, sample: Sample) {
        self.data.get_mut(name).unwrap().push(sample);
    }
}
