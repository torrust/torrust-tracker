//! The [`AsyncTorrentMetricsStore`] trait — torrent metrics context.
use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use crate::databases::error::Error;

/// Trait covering async persistence operations for per-torrent and global
/// download counters.
#[async_trait]
pub trait AsyncTorrentMetricsStore: Send + Sync {
    /// Loads torrent metrics data from the database for all torrents.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the metrics cannot be loaded.
    async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error>;

    /// Loads torrent metrics data from the database for one torrent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the metrics cannot be loaded.
    async fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error>;

    /// Saves torrent metrics data into the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the metrics cannot be saved.
    async fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: u32) -> Result<(), Error>;

    /// Increases the number of downloads for a given torrent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the query failed.
    async fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error>;

    /// Loads the total number of downloads for all torrents from the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the total downloads cannot be loaded.
    async fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error>;

    /// Saves the total number of downloads for all torrents into the database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the total downloads cannot be saved.
    async fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error>;

    /// Increases the total number of downloads for all torrents.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the query failed.
    async fn increase_global_downloads(&self) -> Result<(), Error>;
}
