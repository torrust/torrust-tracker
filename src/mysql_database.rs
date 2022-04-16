use std::collections::BTreeMap;
use std::str::FromStr;

use async_trait::async_trait;
use log::debug;
use r2d2::Pool;
use r2d2_mysql::mysql::{Opts, OptsBuilder, params, TxOpts};
use r2d2_mysql::mysql::prelude::Queryable;
use r2d2_mysql::MysqlConnectionManager;

use crate::{AUTH_KEY_LENGTH, database, InfoHash};
use crate::database::Database;
use crate::key_manager::AuthKey;
use crate::torrent::TorrentEntry;

pub struct MysqlDatabase {
    pool: Pool<MysqlConnectionManager>,
}

impl MysqlDatabase {
    pub fn new(db_path: &str) -> Result<Self, r2d2::Error> {
        let opts = Opts::from_url(&db_path).expect("Failed to connect to MySQL database.");
        let builder = OptsBuilder::from_opts(opts);
        let manager = MysqlConnectionManager::new(builder);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create r2d2 MySQL connection pool.");

        Ok(Self {
            pool
        })
    }
}

#[async_trait]
impl Database for MysqlDatabase {
    fn create_database_tables(&self) -> Result<(), database::Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash BINARY(20) NOT NULL UNIQUE
        );".to_string();

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash BINARY(20) NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );".to_string();

        let create_keys_table = format!("
        CREATE TABLE IF NOT EXISTS `keys` (
          `id` INT NOT NULL AUTO_INCREMENT,
          `key` BINARY({}) NOT NULL,
          `valid_until` INT(10) NOT NULL,
          PRIMARY KEY (`id`),
          UNIQUE (`key`)
        );", AUTH_KEY_LENGTH as i8);

        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        conn.query_drop(&create_torrents_table).expect("Could not create torrents table.");
        conn.query_drop(&create_keys_table).expect("Could not create keys table.");
        conn.query_drop(&create_whitelist_table).expect("Could not create whitelist table.");

        Ok(())
    }

    async fn load_persistent_torrent_data(&self) -> Result<Vec<(InfoHash, u32)>, database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let torrents: Vec<(InfoHash, u32)> = conn.query_map("SELECT HEX(info_hash), completed FROM torrents", |(info_hash_string, completed): (String, u32)| {
            let info_hash = InfoHash::from_str(&info_hash_string).unwrap();
            (info_hash, completed)
        }).map_err(|_| database::Error::QueryReturnedNoRows)?;

        Ok(torrents)
    }

    async fn save_persistent_torrent_data(&self, torrents: &BTreeMap<InfoHash, TorrentEntry>) -> Result<(), database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let mut db_transaction = conn.start_transaction(TxOpts::default()).map_err(|_| database::Error::DatabaseError)?;

        for (info_hash, torrent_entry) in torrents {
            let (_seeders, completed, _leechers) = torrent_entry.get_stats();
            let _ = db_transaction.exec_drop("INSERT INTO torrents (info_hash, completed) VALUES (UNHEX(?), ?) ON DUPLICATE KEY UPDATE completed = completed", (info_hash.to_string(), completed.to_string()));
            debug!("INSERT INTO torrents (info_hash, completed) VALUES (UNHEX('{}'), {}) ON DUPLICATE KEY UPDATE completed = completed", info_hash.to_string(), completed.to_string());
        }

        let _ = db_transaction.commit();

        Ok(())
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<InfoHash, database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        match conn.exec_first::<String, _, _>("SELECT HEX(info_hash) FROM whitelist WHERE info_hash = UNHEX(:info_hash)", params! { info_hash => info_hash })
            .map_err(|_| database::Error::QueryReturnedNoRows)? {
            Some(info_hash) => {
                Ok(InfoHash::from_str(&info_hash).unwrap())
            }
            None => {
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let info_hash_str = info_hash.to_string();

        match conn.exec_drop("INSERT INTO whitelist (info_hash) VALUES (UNHEX(:info_hash_str))", params! { info_hash_str }) {
            Ok(_) => {
                Ok(1)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let info_hash = info_hash.to_string();

        match conn.exec_drop("DELETE FROM whitelist WHERE info_hash = UNHEX(:info_hash)", params! { info_hash }) {
            Ok(_) => {
                Ok(1)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn get_key_from_keys(&self, key: &str) -> Result<AuthKey, database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        match conn.exec_first::<(String, i64), _, _>("SELECT `key`, valid_until FROM `keys` WHERE `key` = :key", params! { key })
            .map_err(|_| database::Error::QueryReturnedNoRows)? {
            Some((key, valid_until)) => {
                Ok(AuthKey {
                    key,
                    valid_until: Some(valid_until as u64),
                })
            }
            None => {
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn add_key_to_keys(&self, auth_key: &AuthKey) -> Result<usize, database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let key = auth_key.key.to_string();
        let valid_until = auth_key.valid_until.unwrap_or(0).to_string();

        match conn.exec_drop("INSERT INTO `keys` (`key`, valid_until) VALUES (:key, :valid_until)", params! { key, valid_until }) {
            Ok(_) => {
                Ok(1)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn remove_key_from_keys(&self, key: String) -> Result<usize, database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        match conn.exec_drop("DELETE FROM `keys` WHERE key = :key", params! { key }) {
            Ok(_) => {
                Ok(1)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }
}
