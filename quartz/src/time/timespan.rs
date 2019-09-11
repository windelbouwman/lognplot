
use super::TimeStamp;

/// A timespan is a period between two moments in time.
/// It differs from a duration, in the sense that a duration
/// is not fixed on the global time scale.
pub struct TimeSpan {
    start: TimeStamp,
    end: TimeStamp,
}

impl TimeSpan {
    pub fn new(start: TimeStamp, end: TimeStamp) -> Self {
        TimeSpan {
            start, end
        }
    }
}
