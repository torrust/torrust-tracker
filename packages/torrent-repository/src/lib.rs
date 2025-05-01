pub mod entry;
pub mod repository;

use torrust_tracker_clock::clock;

pub type TorrentEntry = entry::TorrentEntry;
pub type Torrent = entry::torrent::Torrent;
pub type Torrents = repository::TorrentsSkipMapMutexStd;

/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;
