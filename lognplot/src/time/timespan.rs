use super::TimeStamp;

/// A timespan is a period between two moments in time.
/// It differs from a duration, in the sense that a duration
/// is not fixed on the global time scale.
#[derive(Debug, PartialEq, Clone)]
pub struct TimeSpan {
    pub start: TimeStamp,
    pub end: TimeStamp,
}

impl TimeSpan {
    pub fn new(start: TimeStamp, end: TimeStamp) -> Self {
        TimeSpan { start, end }
    }

    pub fn from_seconds(start: isize, end: isize) -> Self {
        Self::new(TimeStamp::from_seconds(start), TimeStamp::from_seconds(end))
    }

    pub fn extend_to_include(&mut self, time_point: &TimeStamp) {
        if time_point < &self.start {
            self.start = time_point.clone();
        }

        if time_point > &self.end {
            self.end = time_point.clone();
        }
    }

    pub fn contains(&self, timestamp: &TimeStamp) -> bool {
        (&self.start <= timestamp) && (timestamp <= &self.end)
    }

    /// Test if those two timespans overlap.
    pub fn overlap(&self, other: &Self) -> bool {
        assert!(self.start <= self.end);
        assert!(other.start <= other.end);

        (self.start <= other.end) && (other.start <= self.end)
    }
}
