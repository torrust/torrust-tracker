//! The persistence module.
//!
//! Persistence is currently implemented with one [`Database`] trait.
//!
//! There are two implementations of the trait (two drivers):
//!
//! - [`Mysql`](crate::core::databases::mysql::Mysql)
//! - [`Sqlite`](crate::core::databases::sqlite::Sqlite)
//!
//! > **NOTICE**: There are no database migrations. If there are any changes,
//! we will implemented them or provide a script to migrate to the new schema.
//!
//! The persistent objects are:
//!
//! - [Torrent metrics](#torrent-metrics)
//! - [Torrent whitelist](torrent-whitelist)
//! - [Authentication keys](authentication-keys)
//!
//! # Torrent metrics
//!
//!  Field         | Sample data                              | Description
//! ---|---|---
//!  `id`          | 1                                        | Autoincrement id
//!  `info_hash`   | `c1277613db1d28709b034a017ab2cae4be07ae10` | `BitTorrent` infohash V1
//!  `completed`   | 20                                       | The number of peers that have ever completed downloading the torrent associated to this entry. See [`Entry`](crate::core::torrent::Entry) for more information.
//!
//! > **NOTICE**: The peer list for a torrent is not persisted. Since peer have to re-announce themselves on intervals, the data is be
//! regenerated again after some minutes.
//!
//! # Torrent whitelist
//!
//! Field         | Sample data                              | Description
//! ---|---|---
//! `id`          | 1                                        | Autoincrement id
//! `info_hash`   | `c1277613db1d28709b034a017ab2cae4be07ae10` | `BitTorrent` infohash V1
//!
//! # Authentication keys
//!
//! Field         | Sample data                      | Description                  
//! ---|---|---
//! `id`          | 1                                | Autoincrement id             
//! `key`         | `IrweYtVuQPGbG9Jzx1DihcPmJGGpVy82` | Token                        
//! `valid_until` | 1672419840                       | Timestamp for the expiring date  
//!
//! > **NOTICE**: All keys must have an expiration date.
pub mod driver;
pub mod error;
pub mod mysql;
pub mod sqlite;

use std::marker::PhantomData;

use async_trait::async_trait;

use self::error::Error;
use crate::core::auth::{self, Key};
use crate::shared::bit_torrent::info_hash::InfoHash;

struct Builder<T>
where
    T: Database,
{
    phantom: PhantomData<T>,
}

impl<T> Builder<T>
where
    T: Database + 'static,
{
    /// .
    ///
    /// # Errors
    ///
    /// Will return `r2d2::Error` if `db_path` is not able to create a database.
    pub(self) fn build(db_path: &str) -> Result<Box<dyn Database>, Error> {
        Ok(Box::new(T::new(db_path)?))
    }
}

/// The persistence trait. It contains all the methods to interact with the database.
#[async_trait]
pub trait Database: Sync + Send {
    /// It instantiates a new database driver.
    ///
    /// # Errors
    ///
    /// Will return `r2d2::Error` if `db_path` is not able to create a database.
    fn new(db_path: &str) -> Result<Self, Error>
    where
        Self: std::marker::Sized;

    // Schema

    /// It generates the database tables. SQL queries are hardcoded in the trait
    /// implementation.
    ///
    /// # Context: Schema
    ///
    /// # Errors
    ///
    /// Will return `Error` if unable to create own tables.
    fn create_database_tables(&self) -> Result<(), Error>;

    /// It drops the database tables.
    ///
    /// # Context: Schema
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to drop tables.
    fn drop_database_tables(&self) -> Result<(), Error>;

    // Torrent Metrics

    /// It loads the torrent metrics data from the database.
    ///
    /// It returns an array of tuples with the torrent
    /// [`InfoHash`] and the
    /// [`completed`](crate::core::torrent::Entry::completed) counter
    /// which is the number of times the torrent has been downloaded.
    /// See [`Entry::completed`](crate::core::torrent::Entry::completed).
    ///
    /// # Context: Torrent Metrics
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to load.
    async fn load_persistent_torrents(&self) -> Result<Vec<(InfoHash, u32)>, Error>;

    /// It saves the torrent metrics data into the database.
    ///
    /// # Context: Torrent Metrics
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to save.
    async fn save_persistent_torrent(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error>;

    // Whitelist

    /// It loads the whitelisted torrents from the database.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to load.
    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;

    /// It checks if the torrent is whitelisted.
    ///
    /// It returns `Some(InfoHash)` if the torrent is whitelisted, `None` otherwise.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to load.
    async fn get_info_hash_from_whitelist(&self, info_hash: &InfoHash) -> Result<Option<InfoHash>, Error>;

    /// It adds the torrent to the whitelist.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to save.
    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    /// It checks if the torrent is whitelisted.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to load.
    async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> Result<bool, Error> {
        Ok(self.get_info_hash_from_whitelist(info_hash).await?.is_some())
    }

    /// It removes the torrent from the whitelist.
    ///
    /// # Context: Whitelist
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to save.
    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    // Authentication keys

    /// It loads the expiring authentication keys from the database.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to load.
    async fn load_keys(&self) -> Result<Vec<auth::ExpiringKey>, Error>;

    /// It gets an expiring authentication key from the database.
    ///
    /// It returns `Some(ExpiringKey)` if a [`ExpiringKey`](crate::core::auth::ExpiringKey)
    /// with the input [`Key`](crate::core::auth::Key) exists, `None` otherwise.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to load.
    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<auth::ExpiringKey>, Error>;

    /// It adds an expiring authentication key to the database.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to save.
    async fn add_key_to_keys(&self, auth_key: &auth::ExpiringKey) -> Result<usize, Error>;

    /// It removes an expiring authentication key from the database.
    ///
    /// # Context: Authentication Keys
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to load.
    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error>;
}
