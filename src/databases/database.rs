use async_trait::async_trait;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::databases::mysql::MysqlDatabase;
use crate::databases::sqlite::SqliteDatabase;
use crate::protocol::common::InfoHash;
use crate::tracker::key::AuthKey;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum DatabaseDrivers {
    Sqlite3,
    MySQL,
}

pub fn connect_database(db_driver: &DatabaseDrivers, db_path: &str) -> Result<Box<dyn Database>, r2d2::Error> {
    let database: Box<dyn Database> = match db_driver {
        DatabaseDrivers::Sqlite3 => {
            let db = SqliteDatabase::new(db_path)?;
            Box::new(db)
        }
        DatabaseDrivers::MySQL => {
            let db = MysqlDatabase::new(db_path)?;
            Box::new(db)
        }
    };

    database.create_database_tables().expect("Could not create database tables.");

    Ok(database)
}

#[async_trait]
pub trait Database: Sync + Send {
    fn create_database_tables(&self) -> Result<(), Error>;

    async fn load_persistent_torrents(&self) -> Result<Vec<(InfoHash, u32)>, Error>;

    async fn load_keys(&self) -> Result<Vec<AuthKey>, Error>;

    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error>;

    async fn save_persistent_torrent(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error>;

    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<InfoHash, Error>;

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    async fn get_key_from_keys(&self, key: &str) -> Result<AuthKey, Error>;

    async fn add_key_to_keys(&self, auth_key: &AuthKey) -> Result<usize, Error>;

    async fn remove_key_from_keys(&self, key: &str) -> Result<usize, Error>;

    async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> Result<bool, Error> {
        if let Err(e) = self.get_info_hash_from_whitelist(&info_hash.to_owned().to_string()).await {
            if let Error::QueryReturnedNoRows = e {
                return Ok(false);
            } else {
                return Err(e);
            }
        }
        Ok(true)
    }
}

#[derive(Debug, Display, PartialEq, Eq, Error)]
#[allow(dead_code)]
pub enum Error {
    #[display(fmt = "Query returned no rows.")]
    QueryReturnedNoRows,
    #[display(fmt = "Invalid query.")]
    InvalidQuery,
    #[display(fmt = "Database error.")]
    DatabaseError,
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    fn from(e: r2d2_sqlite::rusqlite::Error) -> Self {
        match e {
            r2d2_sqlite::rusqlite::Error::QueryReturnedNoRows => Error::QueryReturnedNoRows,
            _ => Error::InvalidQuery,
        }
    }
}
