use crate::time::TimeStamp;

#[derive(Clone, Debug)]
pub struct Sample {
    pub timestamp: TimeStamp,
    pub value: f64,
}

impl Sample {
    pub fn new(value: f64) -> Self {
        let timestamp = TimeStamp::default();
        Sample { timestamp, value }
    }
}
