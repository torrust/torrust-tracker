use std::sync::Arc;

use torrust_tracker_clock::clock;

pub mod entry;
pub mod repository;

// Repo Entry
pub type EntryMutexStd = Arc<std::sync::Mutex<entry::torrent::Torrent>>;

// Repository
pub type TorrentsSkipMapMutexStd = repository::skip_map_mutex_std::TorrentsSkipMapMutexStd;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;
