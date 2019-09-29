//! Time series database which uses B+ trees to store tha data.

use super::sample::Sample;
use super::trace::Trace;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type TsDbHandle = Arc<TsDb>;

pub struct TsDb {
    path: String,
    pub data: Mutex<HashMap<String, Arc<Trace>>>,
}

impl std::fmt::Display for TsDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TsDb {} with {} datas",
            self.path,
            self.data.lock().unwrap().len()
        )
    }
}

/// Database for time series.
impl TsDb {
    pub fn new() -> Self {
        let path = "x".to_string();
        let data = Mutex::new(HashMap::new());
        Self { path, data }
    }

    pub fn into_handle(self) -> TsDbHandle {
        Arc::new(self)
    }

    /// Add a batch of values
    pub fn add_values(&self, name: &str, samples: Vec<Sample>) {
        let trace = self.data.lock().unwrap().get_mut(name).unwrap().clone();
        trace.add_values(samples);
    }

    pub fn get_trace(&self, name: &str) -> Arc<Trace> {
        self.data.lock().unwrap().get(name).unwrap().clone()
    }

    pub fn new_trace(&self, name: &str) -> Arc<Trace> {
        let trace = Arc::<Trace>::default();
        self.data
            .lock()
            .unwrap()
            .insert(name.to_string(), trace.clone());
        trace
    }

    pub fn add_value(&self, name: &str, sample: Sample) {
        self.data
            .lock()
            .unwrap()
            .get_mut(name)
            .unwrap()
            .push(sample);
    }
}
