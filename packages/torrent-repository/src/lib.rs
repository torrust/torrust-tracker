pub mod entry;
pub mod repository;

use std::sync::{Arc, Mutex};

use torrust_tracker_clock::clock;

pub type TorrentRepository = repository::TorrentRepository;
pub type TrackedTorrentHandle = Arc<Mutex<TrackedTorrent>>;
pub type TrackedTorrent = entry::torrent::TrackedTorrent;

/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;
