mod duration;
mod resolution;
mod timespan;
mod timestamp;

pub use resolution::Resolution;
pub use timespan::TimeSpan;
pub use timestamp::TimeStamp;

pub trait TimeModifiers {
    fn add_nanos(&self, amount: isize) -> Self;
    fn add_millis(&self, amount: isize) -> Self;
}
