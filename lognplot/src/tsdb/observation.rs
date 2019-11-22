use crate::time::TimeStamp;

/// A single observation at some point in time.
#[derive(Clone, Debug)]
pub struct Observation<V> {
    /// The timestamp when the observation was made.
    pub timestamp: TimeStamp,

    /// The observed value.
    pub value: V,
}

impl<V> Observation<V> {
    /// Create a new observation at a given time.
    pub fn new(timestamp: TimeStamp, value: V) -> Self {
        Observation { timestamp, value }
    }
}
