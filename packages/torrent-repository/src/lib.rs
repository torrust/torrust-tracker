use std::sync::{Arc, Mutex};

use torrust_tracker_clock::clock;

pub mod entry;
pub mod repository;

// Repo entry
pub type TorrentEntry = EntryMutexStd;

// Repository
pub type Torrents = TorrentsSkipMapMutexStd;

// The internal type of the entry
pub(crate) type EntryMutexStd = Arc<Mutex<entry::torrent::Torrent>>;

// The internal type of the repository
pub(crate) type TorrentsSkipMapMutexStd = repository::TorrentsSkipMapMutexStd;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;
