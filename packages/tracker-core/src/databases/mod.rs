//! The persistence module.
//!
//! Persistence is implemented through dedicated store traits per domain
//! context. Each backend is responsible for:
//!
//! - running schema migrations
//! - persisting torrent metrics
//! - persisting the whitelist
//! - persisting authentication keys
//!
//! The supported drivers are:
//!
//! - **`MySQL`**
//! - **`PostgreSQL`**
//! - **`Sqlite3`**
//!
//! The schema source of truth is the SQL files in `migrations/`.
pub mod driver;
pub mod error;
pub mod setup;

use std::sync::Arc;

use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;
use mockall::automock;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use self::error::Error;
use crate::authentication::{self, Key};

/// Shared persistence handles grouped by context.
#[derive(Clone)]
pub struct Persistence {
    schema_migrator: Arc<dyn SchemaMigrator>,
    torrent_metrics_store: Arc<dyn TorrentMetricsStore>,
    whitelist_store: Arc<dyn WhitelistStore>,
    auth_key_store: Arc<dyn AuthKeyStore>,
}

impl Persistence {
    /// Builds a new set of persistence handles.
    #[must_use]
    pub fn new(
        schema_migrator: Arc<dyn SchemaMigrator>,
        torrent_metrics_store: Arc<dyn TorrentMetricsStore>,
        whitelist_store: Arc<dyn WhitelistStore>,
        auth_key_store: Arc<dyn AuthKeyStore>,
    ) -> Self {
        Self {
            schema_migrator,
            torrent_metrics_store,
            whitelist_store,
            auth_key_store,
        }
    }

    /// Returns the schema migrator handle.
    #[must_use]
    pub fn schema_migrator(&self) -> Arc<dyn SchemaMigrator> {
        self.schema_migrator.clone()
    }

    /// Returns the torrent metrics store handle.
    #[must_use]
    pub fn torrent_metrics_store(&self) -> Arc<dyn TorrentMetricsStore> {
        self.torrent_metrics_store.clone()
    }

    /// Returns the whitelist store handle.
    #[must_use]
    pub fn whitelist_store(&self) -> Arc<dyn WhitelistStore> {
        self.whitelist_store.clone()
    }

    /// Returns the authentication key store handle.
    #[must_use]
    pub fn auth_key_store(&self) -> Arc<dyn AuthKeyStore> {
        self.auth_key_store.clone()
    }
}

/// Schema migration operations.
#[automock]
#[async_trait]
pub trait SchemaMigrator: Sync + Send {
    /// Creates or migrates the database schema.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the schema cannot be created or migrated.
    async fn create_database_tables(&self) -> Result<(), Error>;

    /// Drops all persistence tables.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any table cannot be dropped.
    async fn drop_database_tables(&self) -> Result<(), Error>;
}

/// Torrent metrics persistence.
#[automock]
#[async_trait]
pub trait TorrentMetricsStore: Sync + Send {
    /// Loads torrent download counters for all torrents.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the data cannot be loaded.
    async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error>;

    /// Loads torrent download counters for one torrent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the data cannot be loaded.
    async fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error>;

    /// Saves torrent download counters.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the data cannot be saved.
    async fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: NumberOfDownloads) -> Result<(), Error>;

    /// Increases the download counter for a torrent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the update fails.
    async fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error>;

    /// Loads the global number of downloads.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the data cannot be loaded.
    async fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error>;

    /// Saves the global number of downloads.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the data cannot be saved.
    async fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error>;

    /// Increases the global number of downloads.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the update fails.
    async fn increase_global_downloads(&self) -> Result<(), Error>;
}

/// Torrent whitelist persistence.
#[automock]
#[async_trait]
pub trait WhitelistStore: Sync + Send {
    /// Loads all whitelisted torrents.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be loaded.
    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;

    /// Retrieves a whitelisted torrent.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be queried.
    async fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error>;

    /// Adds a torrent to the whitelist.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the torrent cannot be added.
    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    /// Removes a torrent from the whitelist.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the torrent cannot be removed.
    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    /// Checks whether a torrent is whitelisted.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be queried.
    async fn is_info_hash_whitelisted(&self, info_hash: InfoHash) -> Result<bool, Error> {
        Ok(self.get_info_hash_from_whitelist(info_hash).await?.is_some())
    }
}

/// Authentication key persistence.
#[automock]
#[async_trait]
pub trait AuthKeyStore: Sync + Send {
    /// Loads all authentication keys.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the keys cannot be loaded.
    async fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error>;

    /// Retrieves a specific authentication key.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be queried.
    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error>;

    /// Adds an authentication key.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be saved.
    async fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error>;

    /// Removes an authentication key.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be removed.
    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error>;
}
