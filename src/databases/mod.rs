pub mod error;
pub mod mysql;
pub mod sqlite;

use async_trait::async_trait;
use torrust_tracker_primitives::DatabaseDriver;

use self::error::Error;
use crate::databases::mysql::Mysql;
use crate::databases::sqlite::Sqlite;
use crate::protocol::info_hash::InfoHash;
use crate::tracker::auth;

/// # Errors
///
/// Will return `r2d2::Error` if `db_path` is not able to create a database.
pub fn connect(db_driver: &DatabaseDriver, db_path: &str) -> Result<Box<dyn Database>, r2d2::Error> {
    let database: Box<dyn Database> = match db_driver {
        DatabaseDriver::Sqlite3 => {
            let db = Sqlite::new(db_path)?;
            Box::new(db)
        }
        DatabaseDriver::MySQL => {
            let db = Mysql::new(db_path)?;
            Box::new(db)
        }
    };

    database.create_database_tables().expect("Could not create database tables.");

    Ok(database)
}

#[async_trait]
pub trait Database: Sync + Send {
    /// # Errors
    ///
    /// Will return `Error` if unable to create own tables.
    fn create_database_tables(&self) -> Result<(), Error>;

    /// # Errors
    ///
    /// Will return `Err` if unable to drop tables.
    fn drop_database_tables(&self) -> Result<(), Error>;

    async fn load_persistent_torrents(&self) -> Result<Vec<(InfoHash, u32)>, Error>;

    async fn load_keys(&self) -> Result<Vec<auth::Key>, Error>;

    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;

    async fn save_persistent_torrent(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error>;

    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<InfoHash, Error>;

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    async fn get_key_from_keys(&self, key: &str) -> Result<auth::Key, Error>;

    async fn add_key_to_keys(&self, auth_key: &auth::Key) -> Result<usize, Error>;

    async fn remove_key_from_keys(&self, key: &str) -> Result<usize, Error>;

    async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> Result<bool, Error> {
        self.get_info_hash_from_whitelist(&info_hash.clone().to_string())
            .await
            .map_or_else(
                |e| match e {
                    Error::QueryReturnedNoRows => Ok(false),
                    e => Err(e),
                },
                |_| Ok(true),
            )
    }
}
