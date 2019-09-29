use crate::time::TimeStamp;

#[derive(Clone, Debug)]
pub struct Sample {
    pub timestamp: TimeStamp,
    pub value: f64,
}

impl Sample {
    pub fn new(timestamp: TimeStamp, value: f64) -> Self {
        Sample { timestamp, value }
    }
}
