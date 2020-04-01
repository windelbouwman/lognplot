//! Trace your application behavior
//!
//! The tracer can be used to trace to either a remotely
//! running lognplot GUI, or directly to a tsdb instance.

mod any_tracer;
mod db_tracer;
mod net_tracer;
mod tracer;

pub use any_tracer::AnyTracer;
pub use db_tracer::DbTracer;
pub use net_tracer::TcpTracer;
pub use tracer::Tracer;
