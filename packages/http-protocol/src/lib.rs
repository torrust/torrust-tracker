//! Primitive types and function for `BitTorrent` HTTP trackers.
pub mod percent_encoding;
pub mod v1;

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
