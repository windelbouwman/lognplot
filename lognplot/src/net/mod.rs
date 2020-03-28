//! Send log events via network.
//! 
//! Ideas:
//! 
//! Different data sources
//! In other words, how to get some data?
//! Options:
//! - Receive data via TCP/IP over tha network
//! - Read data from file
//! - Demo data (random values)

mod client;
mod payload;

#[cfg(feature = "server")]
mod peer;
#[cfg(feature = "server")]
mod peer_processor;
#[cfg(feature = "server")]
mod server;

#[cfg(feature = "server")]
pub use server::run_server;

pub use client::TcpClient;
