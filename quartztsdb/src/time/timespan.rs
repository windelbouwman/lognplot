use super::TimeStamp;

/// A timespan is a period between two moments in time.
/// It differs from a duration, in the sense that a duration
/// is not fixed on the global time scale.
#[derive(Debug, PartialEq, Clone)]
pub struct TimeSpan {
    start: TimeStamp,
    end: TimeStamp,
}

impl TimeSpan {
    pub fn new(start: TimeStamp, end: TimeStamp) -> Self {
        TimeSpan { start, end }
    }

    pub fn extend_to_include(&mut self, time_point: &TimeStamp) {
        if time_point < &self.start {
            self.start = time_point.clone();
        }

        if time_point > &self.end {
            self.end = time_point.clone();
        }
    }
}
