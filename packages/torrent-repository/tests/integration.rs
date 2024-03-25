//! Integration tests.
//!
//! ```text
//! cargo test --test integration
//! ```

use torrust_tracker_clock::clock;

pub mod common;
mod entry;
mod repository;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;
