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

    /// Retrieve the timestamp in the middle of this span.
    pub fn middle_timestamp(&self) -> TimeStamp {
        TimeStamp::new((self.start.amount + self.end.amount) / 2.0)
    }

    pub fn extend_to_include(&mut self, time_point: &TimeStamp) {
        if time_point < &self.start {
            self.start = time_point.clone();
        }

        if time_point > &self.end {
            self.end = time_point.clone();
        }
    }

    /// Adjust this timespan to include the given span.
    pub fn extend_to_include_span(&mut self, span: &Self) {
        if &span.start < &self.start {
            self.start = span.start.clone();
        }

        if &span.end > &self.end {
            self.end = span.end.clone();
        }
    }

    /// Test if this timespan contains a timestamp.
    pub fn contains(&self, timestamp: &TimeStamp) -> bool {
        (&self.start <= timestamp) && (timestamp <= &self.end)
    }

    /// Test if this timespan fully covers another timespan.
    pub fn covers(&self, other: &TimeSpan) -> bool {
        (&self.start <= &other.start) && (&other.end <= &self.end)
    }

    /// Test if those two timespans overlap.
    pub fn overlap(&self, other: &Self) -> bool {
        assert!(self.start <= self.end);
        assert!(other.start <= other.end);

        (self.start <= other.end) && (other.start <= self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeSpan;

    #[test]
    fn no_overlap1() {
        // Create test samples:
        let span1 = TimeSpan::from_seconds(1, 3);
        let span2 = TimeSpan::from_seconds(6, 7);
        assert!(!span1.overlap(&span2));
        assert!(!span2.overlap(&span1));
    }

    #[test]
    fn no_overlap2() {
        let span1 = TimeSpan::from_seconds(10, 13);
        let span2 = TimeSpan::from_seconds(6, 7);
        assert!(!span1.overlap(&span2));
        assert!(!span2.overlap(&span1));
    }

    #[test]
    fn overlap1() {
        let span1 = TimeSpan::from_seconds(1, 8);
        let span2 = TimeSpan::from_seconds(6, 17);
        assert!(span1.overlap(&span2));
        assert!(span2.overlap(&span1));
    }

    #[test]
    fn overlap2() {
        let span1 = TimeSpan::from_seconds(1, 19);
        let span2 = TimeSpan::from_seconds(6, 17);
        assert!(span1.overlap(&span2));
        assert!(span2.overlap(&span1));
    }

    #[test]
    fn overlap3() {
        let span1 = TimeSpan::from_seconds(8, 19);
        let span2 = TimeSpan::from_seconds(6, 17);
        assert!(span1.overlap(&span2));
        assert!(span2.overlap(&span1));
    }
}
