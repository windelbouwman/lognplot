use crate::time::{Resolution, TimeSpan, TimeStamp};
/// Time series database
use std::collections::HashMap;

pub struct HermesDb {
    path: String,
    data: HashMap<String, Vec<Sample>>,
}

/// Database for time series.
impl HermesDb {
    pub fn new() -> Self {
        let path = "x".to_string();
        let data = HashMap::new();
        HermesDb { path, data }
    }

    /// Add a batch of values
    pub fn add_values(&self) {}

    pub fn new_trace(&mut self, name: &str) {
        self.data.insert(name.to_string(), vec![]);
    }

    pub fn add_value(&mut self, name: &str, sample: Sample) {
        self.data.get_mut(name).unwrap().push(sample);
        // trace.push(sample);
    }
}

impl Connection for HermesDb {
    /// Open database
    fn open(&self) {
        //info!("opening {:?}", path);
    }

    /// Close database
    fn close(&self) {
        //trace!("closing {:?}", path);
    }
}

pub trait Connection {
    fn open(&self);
    fn close(&self);
}

#[derive(Clone)]
pub struct Sample {
    timestamp: TimeStamp,
    value: f64,
}

impl Sample {
    pub fn new(value: f64) -> Self {
        let timestamp = TimeStamp::default();
        Sample { timestamp, value }
    }
}

pub trait Query {
    fn get_values(&self, interval: &TimeSpan, resolution: &Resolution) -> Vec<Sample>;
}

impl Query for HermesDb {
    fn get_values(&self, interval: &TimeSpan, resolution: &Resolution) -> Vec<Sample> {
        self.data.get("foo").unwrap().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut db = HermesDb::new();
        db.open();
        let sample = Sample::new(3.1415926);
        db.new_trace("foo");
        db.add_value("foo", sample.clone());
        // db.add_values();
        let timespan = TimeSpan::new(
            sample.timestamp.add_millis(-1),
            sample.timestamp.add_millis(1),
        );
        let values = db.get_values(&timespan, &Resolution::NanoSeconds);
        assert_eq!(1, values.len());
        db.close();
    }
}
