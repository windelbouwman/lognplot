use crate::time::TimeStamp;

/// A single observation at some point in time.
pub struct Observation<S> {
    pub timestamp: TimeStamp,

    value: S,
}
