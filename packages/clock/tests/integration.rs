//! Integration tests.
//!
//! ```text
//! cargo test --test integration
//! ```

//mod common;
mod clock;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = torrust_tracker_clock::clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = torrust_tracker_clock::clock::Stopped;
