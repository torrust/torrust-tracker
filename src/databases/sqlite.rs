use std::collections::BTreeMap;
use std::str::FromStr;

use async_trait::async_trait;
use log::debug;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2_sqlite::rusqlite::NO_PARAMS;

use crate::{AUTH_KEY_LENGTH, InfoHash};
use crate::databases::database::{Database, Error};
use crate::databases::database;
use crate::tracker::key::AuthKey;
use crate::tracker::torrent::TorrentEntry;

pub struct SqliteDatabase {
    pool: Pool<SqliteConnectionManager>,
}

impl SqliteDatabase {
    pub fn new(db_path: &str) -> Result<SqliteDatabase, r2d2::Error> {
        let cm = SqliteConnectionManager::file(db_path);
        let pool = Pool::new(cm).expect("Failed to create r2d2 SQLite connection pool.");
        Ok(SqliteDatabase {
            pool
        })
    }
}

#[async_trait]
impl Database for SqliteDatabase {
    fn create_database_tables(&self) -> Result<(), database::Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id integer PRIMARY KEY AUTOINCREMENT,
            info_hash VARCHAR(20) NOT NULL UNIQUE
        );".to_string();

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id integer PRIMARY KEY AUTOINCREMENT,
            info_hash VARCHAR(20) NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );".to_string();

        let create_keys_table = format!("
        CREATE TABLE IF NOT EXISTS keys (
            id integer PRIMARY KEY AUTOINCREMENT,
            key VARCHAR({}) NOT NULL UNIQUE,
            valid_until INT(10) NOT NULL
         );", AUTH_KEY_LENGTH as i8);

        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        conn.execute(&create_whitelist_table, NO_PARAMS)
            .and_then(|_| conn.execute(&create_keys_table, NO_PARAMS))
            .and_then(|_| conn.execute(&create_torrents_table, NO_PARAMS))
            .map_err(|_| database::Error::InvalidQuery)
            .map(|_| ())
    }

    async fn load_persistent_torrents(&self) -> Result<Vec<(InfoHash, u32)>, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let mut stmt = conn.prepare("SELECT info_hash, completed FROM torrents")?;

        let torrent_iter = stmt.query_map(NO_PARAMS, |row| {
            let info_hash_string: String = row.get(0)?;
            let info_hash = InfoHash::from_str(&info_hash_string).unwrap();
            let completed: u32 = row.get(1)?;
            Ok((info_hash, completed))
        })?;

        let torrents: Vec<(InfoHash, u32)> = torrent_iter.filter_map(|x| x.ok()).collect();

        Ok(torrents)
    }

    async fn load_keys(&self) -> Result<Vec<AuthKey>, Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys")?;

        let keys_iter = stmt.query_map(NO_PARAMS, |row| {
            let key = row.get(0)?;
            let valid_until: i64 = row.get(1)?;

            Ok(AuthKey {
                key,
                valid_until: Some(valid_until as u64)
            })
        })?;

        let keys: Vec<AuthKey> = keys_iter.filter_map(|x| x.ok()).collect();

        Ok(keys)
    }

    async fn save_persistent_torrent_data(&self, torrents: &BTreeMap<InfoHash, TorrentEntry>) -> Result<(), database::Error> {
        let mut conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let db_transaction = conn.transaction()?;

        for (info_hash, torrent_entry) in torrents {
            let (_seeders, completed, _leechers) = torrent_entry.get_stats();
            let _ = db_transaction.execute("INSERT OR IGNORE INTO torrents (info_hash, completed) VALUES (?, ?)", &[info_hash.to_string(), completed.to_string()]);
            let _ = db_transaction.execute("UPDATE torrents SET completed = ? WHERE info_hash = ?", &[completed.to_string(), info_hash.to_string()]);
        }

        let _ = db_transaction.commit();

        Ok(())
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<InfoHash, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist WHERE info_hash = ?")?;
        let mut rows = stmt.query(&[info_hash])?;

        if let Some(row) = rows.next()? {
            let info_hash: String = row.get(0).unwrap();

            // should never be able to fail
            Ok(InfoHash::from_str(&info_hash).unwrap())
        } else {
            Err(database::Error::InvalidQuery)
        }
    }

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        match conn.execute("INSERT INTO whitelist (info_hash) VALUES (?)", &[info_hash.to_string()]) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated); }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        match conn.execute("DELETE FROM whitelist WHERE info_hash = ?", &[info_hash.to_string()]) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated); }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn get_key_from_keys(&self, key: &str) -> Result<AuthKey, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys WHERE key = ?")?;
        let mut rows = stmt.query(&[key.to_string()])?;

        if let Some(row) = rows.next()? {
            let key: String = row.get(0).unwrap();
            let valid_until_i64: i64 = row.get(1).unwrap();

            Ok(AuthKey {
                key,
                valid_until: Some(valid_until_i64 as u64),
            })
        } else {
            Err(database::Error::QueryReturnedNoRows)
        }
    }

    async fn add_key_to_keys(&self, auth_key: &AuthKey) -> Result<usize, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        match conn.execute("INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
                           &[auth_key.key.to_string(), auth_key.valid_until.unwrap().to_string()],
        ) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated); }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn remove_key_from_keys(&self, key: &str) -> Result<usize, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::InvalidQuery)?;

        match conn.execute("DELETE FROM keys WHERE key = ?", &[key]) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated); }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }
}
