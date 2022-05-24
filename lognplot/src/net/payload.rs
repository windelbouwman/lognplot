use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::time::TimeStamp;
use crate::tsdb::observations::{Observation, ProfileEvent, Sample, Text};
use crate::tsdb::TsDbHandle;

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

    pub fn new_samples(name: String, samples: Vec<(f64, f64)>) -> Self {
        SampleBatch {
            name,
            payload: SamplePayload::Batch { samples },
        }
    }

    pub fn new_sampled_data(name: String, t0: f64, dt: f64, values: Vec<f64>) -> Self {
        SampleBatch {
            name,
            payload: SamplePayload::Sampled {
                t: t0,
                dt,
                data: values,
            },
        }
    }

    pub fn new_text(name: String, t: f64, text: String) -> Self {
        SampleBatch {
            name,
            payload: SamplePayload::Text { t, text },
        }
    }

    /// Feed this batch of observations into a database.
    pub fn to_db(&self, db: &TsDbHandle) {
        match &self.payload {
            SamplePayload::Sampled { t, dt, data } => {
                let samples = data
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        let t2 = t + dt * index as f64;
                        let timestamp = TimeStamp::new(t2);
                        Observation::new(timestamp, Sample::new(*value))
                    })
                    .collect();

                db.add_values(&self.name, samples);
            }
            SamplePayload::Batch { samples } => {
                let samples = samples
                    .iter()
                    .map(|(t, value)| {
                        let timestamp = TimeStamp::new(*t);
                        Observation::new(timestamp, Sample::new(*value))
                    })
                    .collect();
                db.add_values(&self.name, samples);
            }
            SamplePayload::Single { t, value } => {
                let timestamp = TimeStamp::new(*t);
                let value = Observation::new(timestamp, Sample::new(*value));
                db.add_value(&self.name, value);
            }
            SamplePayload::Text { t, text } => {
                let timestamp = TimeStamp::new(*t);
                let text = Observation::new(timestamp, Text::new(text.to_owned()));
                db.add_text(&self.name, text);
            }
            SamplePayload::Event {
                t: _,
                attributes: _,
            } => {
                // TODO
            }
            SamplePayload::Profile { t, event } => {
                let timestamp = TimeStamp::new(*t);
                let event = match event {
                    ProfileEventPayload::Enter { name } => ProfileEvent::FunctionEnter {
                        name: name.to_owned(),
                    },
                    ProfileEventPayload::Exit => ProfileEvent::FunctionExit,
                };
                let event = Observation::new(timestamp, event);
                db.add_profile_event(&self.name, event);
            }
        }
    }

    /// Encode data
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        ciborium::ser::into_writer(self, &mut data).unwrap();
        data
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        match ciborium::de::from_reader(data) {
            Ok(x) => Ok(x),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
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

    #[serde(rename = "text")]
    Text {
        /// Timestamp of the text
        t: f64,

        /// The text itself
        text: String,
    },

    #[serde(rename = "event")]
    Event {
        /// Timestamp of the event
        t: f64,

        attributes: HashMap<String, String>,
    },

    #[serde(rename = "profile")]
    Profile {
        t: f64,

        #[serde(flatten)]
        event: ProfileEventPayload,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
enum ProfileEventPayload {
    #[serde(rename = "enter")]
    Enter {
        #[serde(rename = "callee")]
        name: String,
    },

    #[serde(rename = "exit")]
    Exit,
}

#[cfg(test)]
mod tests {
    use super::SampleBatch;

    #[test]
    /// Check a simple roundtrip operation (to bytes and back to data)
    fn roundtrip() {
        let batch = SampleBatch::new_sample("bla".to_string(), 3.14, 2.5);
        let data = batch.to_bytes();
        let batch2: SampleBatch = SampleBatch::from_bytes(&data).unwrap();
        assert_eq!(batch.name, batch2.name);
    }
}
