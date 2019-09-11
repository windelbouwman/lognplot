
mod timespan;
mod timestamp;
mod duration;

pub use timestamp::TimeStamp;
pub use timespan::TimeSpan;

pub enum Resolution {
    NanoSeconds,
    Seconds,
    Days,
}

pub trait TimeModifiers {
    fn add_nanos(&self, amount: isize) -> Self;
    fn add_millis(&self, amount: isize) -> Self;
}

impl TimeModifiers for TimeStamp {
    fn add_millis(&self, amount: isize) -> Self {
        self + amount as i64
    }

    fn add_nanos(&self, amount: isize) -> Self {
        self.clone() + amount as i64
    }
}
