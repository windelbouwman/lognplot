use std::time::Instant;

/// Client trace interface.
///
/// Use this trait to be able to log metrics.
pub trait Tracer {
    /// Log a single metric
    fn log_metric(&self, name: &str, timestamp: Instant, value: f64);

    /// Log a text
    fn log_text(&self, name: &str, timestamp: Instant, text: String);
}
