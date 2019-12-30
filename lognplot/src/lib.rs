//!
//! Log and plot library.
//!
//! This library serves the purpose of logging data
//! from a system, and plotting the data at the same
//! time.
//!
//! This can be handy when debugging a system under test.

#[macro_use]
extern crate log;

pub mod chart;
mod client;
pub mod geometry;
mod payload;
pub mod render;
pub mod style;
pub mod time;
pub mod tsdb;

pub use client::TcpClient;

#[cfg(feature = "server")]
pub mod server;
