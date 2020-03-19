use super::{DbTracer, TcpTracer, Tracer};
use crate::net::TcpClient;
use crate::tsdb::TsDbHandle;
use std::time::Instant;

/// Application tracer with multiple data sinks.
pub enum AnyTracer {
    /// Use this tracing target to trace over a network.
    Net(TcpTracer),

    /// Use this tracing target to trace directly to a database.
    Db(DbTracer),

    /// Use this tracing target to ignore all tracing.
    Void,
}

impl AnyTracer {
    /// Create a new tracer which traces into the given tcp client.
    pub fn new_tcp(client: TcpClient) -> Self {
        AnyTracer::Net(TcpTracer::new(client))
    }

    /// Create a new tracer which traces data into the given database.
    pub fn new_db(db: TsDbHandle) -> Self {
        AnyTracer::Db(DbTracer::new(db))
    }

    pub fn new_void() -> Self {
        AnyTracer::Void
    }
}

impl Tracer for AnyTracer {
    fn log_meta_metric(&self, name: &str, timestamp: Instant, value: f64) {
        match self {
            AnyTracer::Net(t) => t.log_meta_metric(name, timestamp, value),
            AnyTracer::Db(t) => t.log_meta_metric(name, timestamp, value),
            AnyTracer::Void => {}
        }
    }
}
