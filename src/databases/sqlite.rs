use std::str::FromStr;

use async_trait::async_trait;
use log::debug;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::databases::database;
use crate::databases::database::{Database, Error};
use crate::protocol::clock::clock::DurationSinceUnixEpoch;
use crate::tracker::key::AuthKey;
use crate::InfoHash;

pub struct SqliteDatabase {
    pool: Pool<SqliteConnectionManager>,
}

impl SqliteDatabase {
    pub fn new(db_path: &str) -> Result<SqliteDatabase, r2d2::Error> {
        let cm = SqliteConnectionManager::file(db_path);
        let pool = Pool::new(cm).expect("Failed to create r2d2 SQLite connection pool.");
        Ok(SqliteDatabase { pool })
    }
}

#[async_trait]
impl Database for SqliteDatabase {
    fn create_database_tables(&self) -> Result<(), database::Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            info_hash TEXT NOT NULL UNIQUE
        );"
        .to_string();

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            info_hash TEXT NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );"
        .to_string();

        let create_keys_table = "
        CREATE TABLE IF NOT EXISTS keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            valid_until INTEGER NOT NULL
         );"
        .to_string();

        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        conn.execute(&create_whitelist_table, [])
            .and_then(|_| conn.execute(&create_keys_table, []))
            .and_then(|_| conn.execute(&create_torrents_table, []))
            .map_err(|_| database::Error::InvalidQuery)
            .map(|_| ())
    }

    async fn load_persistent_torrents(&self) -> Result<Vec<(InfoHash, u32)>, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        let mut stmt = conn.prepare("SELECT info_hash, completed FROM torrents")?;

        let torrent_iter = stmt.query_map([], |row| {
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

        let keys_iter = stmt.query_map([], |row| {
            let key = row.get(0)?;
            let valid_until: i64 = row.get(1)?;

            Ok(AuthKey {
                key,
                valid_until: Some(DurationSinceUnixEpoch::from_secs(valid_until as u64)),
            })
        })?;

        let keys: Vec<AuthKey> = keys_iter.filter_map(|x| x.ok()).collect();

        Ok(keys)
    }

    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist")?;

        let info_hash_iter = stmt.query_map([], |row| {
            let info_hash: String = row.get(0)?;

            Ok(InfoHash::from_str(&info_hash).unwrap())
        })?;

        let info_hashes: Vec<InfoHash> = info_hash_iter.filter_map(|x| x.ok()).collect();

        Ok(info_hashes)
    }

    async fn save_persistent_torrent(&self, info_hash: &InfoHash, completed: u32) -> Result<(), database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        match conn.execute(
            "INSERT INTO torrents (info_hash, completed) VALUES (?1, ?2) ON CONFLICT(info_hash) DO UPDATE SET completed = ?2",
            [info_hash.to_string(), completed.to_string()],
        ) {
            Ok(updated) => {
                if updated > 0 {
                    return Ok(());
                }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<InfoHash, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

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
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        match conn.execute("INSERT INTO whitelist (info_hash) VALUES (?)", [info_hash.to_string()]) {
            Ok(updated) => {
                if updated > 0 {
                    return Ok(updated);
                }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        match conn.execute("DELETE FROM whitelist WHERE info_hash = ?", [info_hash.to_string()]) {
            Ok(updated) => {
                if updated > 0 {
                    return Ok(updated);
                }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn get_key_from_keys(&self, key: &str) -> Result<AuthKey, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys WHERE key = ?")?;
        let mut rows = stmt.query([key.to_string()])?;

        if let Some(row) = rows.next()? {
            let key: String = row.get(0).unwrap();
            let valid_until_i64: i64 = row.get(1).unwrap();

            Ok(AuthKey {
                key,
                valid_until: Some(DurationSinceUnixEpoch::from_secs(valid_until_i64 as u64)),
            })
        } else {
            Err(database::Error::QueryReturnedNoRows)
        }
    }

    async fn add_key_to_keys(&self, auth_key: &AuthKey) -> Result<usize, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        match conn.execute(
            "INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
            [auth_key.key.to_string(), auth_key.valid_until.unwrap().as_secs().to_string()],
        ) {
            Ok(updated) => {
                if updated > 0 {
                    return Ok(updated);
                }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }

    async fn remove_key_from_keys(&self, key: &str) -> Result<usize, database::Error> {
        let conn = self.pool.get().map_err(|_| database::Error::DatabaseError)?;

        match conn.execute("DELETE FROM keys WHERE key = ?", &[key]) {
            Ok(updated) => {
                if updated > 0 {
                    return Ok(updated);
                }
                Err(database::Error::QueryReturnedNoRows)
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(database::Error::InvalidQuery)
            }
        }
    }
}
