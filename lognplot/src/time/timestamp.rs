//! Time stamp data type.
//!
//! The best timestamp is the NTP timestamp. It's centered around 1 january 1900 (EPOCH) which is represented by 0.
//! Furthermore, it has 64 bits for the amount of seconds since then (signed, so + and -)
//! Next to that is has 64 bits for the fractional second, reaching pretty precise timestamps, but not infinite precise.

use super::TimeModifiers;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub struct TimeStamp {
    pub amount: f64,
}

impl TimeStamp {
    pub fn new(amount: f64) -> Self {
        Self { amount }
    }

    pub fn from_seconds(seconds: isize) -> Self {
        Self::new(seconds as f64)
    }
}

impl TimeModifiers for TimeStamp {
    fn add_millis(&self, amount: isize) -> Self {
        Self::new(self.amount + (amount as f64) * 1.0e-3)
    }

    fn add_nanos(&self, amount: isize) -> Self {
        Self::new(self.amount + (amount as f64) * 1.0e-9)
    }
}

impl PartialOrd for TimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl std::ops::Sub<f64> for TimeStamp {
    type Output = TimeStamp;

    fn sub(self, other: f64) -> TimeStamp {
        TimeStamp::new(self.amount - other)
    }
}
