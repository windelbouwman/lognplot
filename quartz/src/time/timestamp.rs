/// Time stamp data type.
///
/// The best timestamp is the NTP timestamp. It's centered around 1 january 1900 (EPOCH) which is represented by 0.
/// Furthermore, it has 64 bits for the amount of seconds since then (signed, so + and -)
/// Next to that is has 64 bits for the fractional second, reaching pretty precise timestamps, but not infinite precise.
use super::TimeModifiers;

#[derive(Clone, Debug)]
pub struct TimeStamp {
    pub amount: f64,
}

impl TimeStamp {
    pub fn new(amount: f64) -> Self {
        Self { amount }
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
