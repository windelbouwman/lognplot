//! Different data sources
//! In other words, how to get some data?
//! Options:
//! - Receive data via TCP/IP over tha network
//! - Read data from file
//! - Demo data (random values)

mod peer;
mod server;

pub use server::run_server;
