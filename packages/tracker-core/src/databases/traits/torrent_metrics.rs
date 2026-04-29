//! The [`TorrentMetricsStore`] trait — torrent metrics context.
use bittorrent_primitives::info_hash::InfoHash;
use mockall::automock;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::super::error::Error;

/// Trait covering persistence operations for per-torrent and global download
/// counters.
#[automock]
pub trait TorrentMetricsStore: Sync + Send {
    /// Loads torrent metrics data from the database for all torrents.
    ///
    /// This function returns the persistent torrent metrics as a collection of
    /// tuples, where each tuple contains an [`InfoHash`] and the `downloaded`
    /// counter (i.e. the number of times the torrent has been downloaded).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the metrics cannot be loaded.
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error>;

    /// Loads torrent metrics data from the database for one torrent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the metrics cannot be loaded.
    fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error>;

    /// Saves torrent metrics data into the database.
    ///
    /// # Arguments
    ///
    /// * `info_hash` - A reference to the torrent's info hash.
    /// * `downloaded` - The number of times the torrent has been downloaded.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the metrics cannot be saved.
    fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: u32) -> Result<(), Error>;

    /// Increases the number of downloads for a given torrent.
    ///
    /// It does not create a new entry if the torrent is not found and it does
    /// not return an error.
    ///
    /// # Context: Torrent Metrics
    ///
    /// # Arguments
    ///
    /// * `info_hash` - A reference to the torrent's info hash.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the query failed.
    fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error>;

    /// Loads the total number of downloads for all torrents from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the total downloads cannot be loaded.
    fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error>;

    /// Saves the total number of downloads for all torrents into the database.
    ///
    /// # Arguments
    ///
    /// * `downloaded` - The total number of times all torrents have been downloaded.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the total downloads cannot be saved.
    fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error>;

    /// Increases the total number of downloads for all torrents.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the query failed.
    fn increase_global_downloads(&self) -> Result<(), Error>;
}
