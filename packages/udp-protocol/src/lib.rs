//! Primitive types and functions for `BitTorrent` UDP trackers.

pub use bittorrent_udp_tracker_protocol::{common, request, response, *};
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
