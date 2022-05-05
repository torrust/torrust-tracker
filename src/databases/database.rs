use std::collections::BTreeMap;

use async_trait::async_trait;
use derive_more::{Display, Error};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::InfoHash;
use crate::key_manager::AuthKey;
use crate::databases::mysql::MysqlDatabase;
use crate::databases::sqlite::SqliteDatabase;
use crate::torrent::TorrentEntry;

#[derive(Serialize, Deserialize, Debug)]
pub enum DatabaseDrivers {
    Sqlite3,
    MySQL,
}

pub fn connect_database(db_driver: &DatabaseDrivers, db_path: &str) -> Result<Box<dyn Database>, r2d2::Error> {
    debug!("{:?}", db_driver);

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

    async fn load_persistent_torrent_data(&self) -> Result<Vec<(InfoHash, u32)>, Error>;

    async fn save_persistent_torrent_data(&self, torrents: &BTreeMap<InfoHash, TorrentEntry>) -> Result<(), Error>;

    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<InfoHash, Error>;

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error>;

    async fn get_key_from_keys(&self, key: &str) -> Result<AuthKey, Error>;

    async fn add_key_to_keys(&self, auth_key: &AuthKey) -> Result<usize, Error>;

    async fn remove_key_from_keys(&self, key: String) -> Result<usize, Error>;
}

#[derive(Debug, Display, PartialEq, Error)]
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
            _ => Error::InvalidQuery
        }
    }
}
