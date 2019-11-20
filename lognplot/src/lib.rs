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
pub mod geometry;
pub mod render;
pub mod style;
pub mod time;
pub mod tsdb;

#[cfg(feature = "server")]
pub mod server;
