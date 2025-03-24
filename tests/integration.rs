//! Scaffolding for integration tests.
//!
//! Integration tests are used to test the interaction between multiple modules,
//! multiple running trackers, etc. Tests for one specific module should be in
//! the corresponding package.
//!
//! ```text
//! cargo test --test integration
//! ```
mod servers;

use torrust_tracker_clock::clock;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;
