/// Time stamp data type.
///
/// The best timestamp is the NTP timestamp. It's centered around 1 january 1900 (EPOCH) which is represented by 0.
/// Furthermore, it has 64 bits for the amount of seconds since then (signed, so + and -)
/// Next to that is has 64 bits for the fractional second, reaching pretty precise timestamps, but not infinite precise.

pub type TimeStamp = i64;
