//! The persistence module.
//!
//! Persistence is currently implemented using a single [`Database`] trait.
//!
//! There are two implementations of the trait (two drivers):
//!
//! - **`MySQL`**
//! - **`Sqlite`**
//!
//! > **NOTICE**: There are no database migrations at this time. If schema
//! > changes occur, either migration functionality will be implemented or a
//! > script will be provided to migrate to the new schema.
//!
//! The persistent objects handled by this module include:
//!
//! - **Torrent metrics**: Metrics such as the number of completed downloads for
//!   each torrent.
//! - **Torrent whitelist**: A list of torrents (by infohash) that are allowed.
//! - **Authentication keys**: Expiring authentication keys used to secure
//!   access to private trackers.
//!
//! # Torrent Metrics
//!
//! | Field       | Sample data                                | Description                                                                 |
//! |-------------|--------------------------------------------|-----------------------------------------------------------------------------|
//! | `id`        | 1                                          | Auto-increment id                                                           |
//! | `info_hash` | `c1277613db1d28709b034a017ab2cae4be07ae10` | `BitTorrent` infohash V1                                                    |
//! | `completed` | 20                                         | The number of peers that have completed downloading the associated torrent. |
//!
//! > **NOTICE**: The peer list for a torrent is not persisted. Because peers re-announce at
//! > intervals, the peer list is regenerated periodically.
//!
//! # Torrent Whitelist
//!
//! | Field       | Sample data                                | Description                    |
//! |-------------|--------------------------------------------|--------------------------------|
//! | `id`        | 1                                          | Auto-increment id              |
//! | `info_hash` | `c1277613db1d28709b034a017ab2cae4be07ae10` | `BitTorrent` infohash V1       |
//!
//! # Authentication Keys
//!
//! | Field         | Sample data                        | Description                          |
//! |---------------|------------------------------------|--------------------------------------|
//! | `id`          | 1                                  | Auto-increment id                    |
//! | `key`         | `IrweYtVuQPGbG9Jzx1DihcPmJGGpVy82` | Authentication token (32 chars)      |
//! | `valid_until` | 1672419840                         | Timestamp indicating expiration time |
//!
//! > **NOTICE**: All authentication keys must have an expiration date.
pub mod driver;
pub mod error;
pub mod setup;

use bittorrent_primitives::info_hash::InfoHash;
use mockall::automock;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use self::error::Error;
use crate::authentication::{self, Key};

/// The persistence trait.
///
/// This trait defines all the methods required to interact with the database,
/// including creating and dropping schema tables, and CRUD operations for
/// torrent metrics, whitelists, and authentication keys. Implementations of
/// this trait must ensure that operations are safe, consistent, and report
/// errors using the [`Error`] type.
#[automock]
pub trait Database: Sync + Send {
    /// Creates the necessary database tables.
    ///
    /// The SQL queries for table creation are hardcoded in the trait implementation.
    ///
    /// # Context: Schema
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the tables cannot be created.
    fn create_database_tables(&self) -> Result<(), Error>;

    /// Drops the database tables.
    ///
    /// This operation removes the persistent schema.
    ///
    /// # Context: Schema
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the tables cannot be dropped.
    fn drop_database_tables(&self) -> Result<(), Error>;

    // Torrent Metrics

    /// Loads torrent metrics data from the database for all torrents.
    ///
    /// This function returns the persistent torrent metrics as a collection of
    /// tuples, where each tuple contains an [`InfoHash`] and the `downloaded`
    /// counter (i.e. the number of times the torrent has been downloaded).
    ///
    /// # Context: Torrent Metrics
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the metrics cannot be loaded.
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error>;

    /// Loads torrent metrics data from the database for one torrent.
    ///
    /// # Context: Torrent Metrics
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
    /// # Context: Torrent Metrics
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
    /// # Context: Torrent Metrics
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the total downloads cannot be loaded.
    fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error>;

    /// Saves the total number of downloads for all torrents into the database.
    ///
    /// # Context: Torrent Metrics
    ///
    /// # Arguments
    ///
    /// * `info_hash` - A reference to the torrent's info hash.
    /// * `downloaded` - The number of times the torrent has been downloaded.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the total downloads cannot be saved.
    fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error>;

    /// Increases the total number of downloads for all torrents.
    ///
    /// # Context: Torrent Metrics
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the query failed.
    fn increase_global_downloads(&self) -> Result<(), Error>;

    // Whitelist

    /// Loads the whitelisted torrents from the database.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be loaded.
    fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;

    /// Retrieves a whitelisted torrent from the database.
    ///
    /// Returns `Some(InfoHash)` if the torrent is in the whitelist, or `None`
    /// otherwise.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be queried.
    fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error>;

    /// Adds a torrent to the whitelist.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the torrent cannot be added to the whitelist.
    fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    /// Checks whether a torrent is whitelisted.
    ///
    /// This default implementation returns `true` if the infohash is included
    /// in the whitelist, or `false` otherwise.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the whitelist cannot be queried.
    fn is_info_hash_whitelisted(&self, info_hash: InfoHash) -> Result<bool, Error> {
        Ok(self.get_info_hash_from_whitelist(info_hash)?.is_some())
    }

    /// Removes a torrent from the whitelist.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the torrent cannot be removed from the whitelist.
    fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    // Authentication keys

    /// Loads all authentication keys from the database.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the keys cannot be loaded.
    fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error>;

    /// Retrieves a specific authentication key from the database.
    ///
    /// Returns `Some(PeerKey)` if a key corresponding to the provided [`Key`]
    /// exists, or `None` otherwise.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be queried.
    fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error>;

    /// Adds an authentication key to the database.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be saved.
    fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error>;

    /// Removes an authentication key from the database.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the key cannot be removed.
    fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error>;
}
