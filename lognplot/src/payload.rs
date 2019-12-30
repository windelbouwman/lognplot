use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::time::TimeStamp;
use crate::tsdb::{Observation, Sample};

/// A chunk of data at fixed sample rate.
#[derive(Serialize, Deserialize, Debug)]
pub struct SampleBatch {
    /// The name of the signal.
    name: String,

    #[serde(flatten)]
    payload: SamplePayload,
}

impl SampleBatch {
    /// Create a new sample batch with a single sample.
    pub fn new_sample(name: String, t: f64, value: f64) -> Self {
        SampleBatch {
            name,
            payload: SamplePayload::Single { t, value },
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Convert a batch of samples received over the wire to
    /// a vector of samples
    pub fn to_samples(&self) -> Vec<Observation<Sample>> {
        match &self.payload {
            SamplePayload::Sampled { t, dt, data } => data
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let t2 = t + dt * index as f64;
                    let timestamp = TimeStamp::new(t2);
                    Observation::new(timestamp, Sample::new(*value))
                })
                .collect(),
            SamplePayload::Batch { samples } => samples
                .iter()
                .map(|(t, value)| {
                    let timestamp = TimeStamp::new(*t);
                    Observation::new(timestamp, Sample::new(*value))
                })
                .collect(),
            SamplePayload::Single { t, value } => {
                let timestamp = TimeStamp::new(*t);
                vec![Observation::new(timestamp, Sample::new(*value))]
            }
            SamplePayload::Event { .. } => vec![],
        }
    }
    /*
        fn size(&self) -> usize {
            match &self.payload {
                SamplePayload::Batch { dt: _, data} => { data.len() },
            }
        }
    */
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum SamplePayload {
    /// A bulk of measurements.
    #[serde(rename = "batch")]
    Batch {
        #[serde(rename = "batch")]
        samples: Vec<(f64, f64)>,
    },

    /// A chunk of data sampled at a certain fixed interval.
    #[serde(rename = "samples")]
    Sampled {
        /// Timestamp of the first sample
        t: f64,

        /// Spacing in time of the samples.
        dt: f64,

        /// The data points
        #[serde(rename = "values")]
        data: Vec<f64>,
    },

    #[serde(rename = "sample")]
    Single {
        /// Timestamp of the sample
        t: f64,

        /// The sample value
        value: f64,
    },

    #[serde(rename = "event")]
    Event {
        /// Timestamp of the event
        t: f64,

        attributes: HashMap<String, String>,
    },
}
