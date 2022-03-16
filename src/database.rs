use std::collections::BTreeMap;
use crate::{InfoHash, AUTH_KEY_LENGTH, TorrentEntry};
use log::debug;
use r2d2_sqlite::{SqliteConnectionManager, rusqlite};
use r2d2::{Pool};
use r2d2_sqlite::rusqlite::NO_PARAMS;
use crate::key_manager::AuthKey;
use std::str::FromStr;

pub struct SqliteDatabase {
    pool: Pool<SqliteConnectionManager>
}

impl SqliteDatabase {
    pub fn new(db_path: &str) -> Result<SqliteDatabase, rusqlite::Error> {
        let sqlite_connection_manager = SqliteConnectionManager::file(db_path);
        let sqlite_pool = r2d2::Pool::new(sqlite_connection_manager).expect("Failed to create r2d2 SQLite connection pool.");
        let sqlite_database = SqliteDatabase {
            pool: sqlite_pool
        };

        if let Err(error) = SqliteDatabase::create_database_tables(&sqlite_database.pool) {
            return Err(error)
        };

        Ok(sqlite_database)
    }

    pub fn create_database_tables(pool: &Pool<SqliteConnectionManager>) -> Result<usize, rusqlite::Error> {
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

        let conn = pool.get().unwrap();
        match conn.execute(&create_whitelist_table, NO_PARAMS) {
            Ok(updated) => {
                match conn.execute(&create_keys_table, NO_PARAMS) {
                    Ok(updated2) => {
                        match conn.execute(&create_torrents_table, NO_PARAMS) {
                            Ok(updated3) => Ok(updated + updated2 + updated3),
                            Err(e) => {
                                debug!("{:?}", e);
                                Err(e)
                            }
                        }
                    }
                    Err(e) => {
                        debug!("{:?}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn load_persistent_torrent_data(&self) -> Result<Vec<(InfoHash, u32)>, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT info_hash, completed FROM torrents")?;

        let torrent_iter = stmt.query_map(NO_PARAMS, |row| {
            let info_hash_string: String = row.get(0)?;
            let info_hash = InfoHash::from_str(&info_hash_string).unwrap();
            let completed: u32 = row.get(1)?;
            Ok((info_hash, completed))
        })?;

        let torrents: Vec<(InfoHash, u32)> = torrent_iter.filter_map(|x| x.ok() ).collect();

        Ok(torrents)
    }

    pub async fn save_persistent_torrent_data(&self, torrents: &BTreeMap<InfoHash, TorrentEntry>) -> Result<(), rusqlite::Error> {
        let mut conn = self.pool.get().unwrap();
        let db_transaction = conn.transaction()?;

        for (info_hash, torrent_entry) in torrents {
                let (_seeders, completed, _leechers) = torrent_entry.get_stats();
                let _ = db_transaction.execute("INSERT OR REPLACE INTO torrents (info_hash, completed) VALUES (?, ?)", &[info_hash.to_string(), completed.to_string()]);
        }

        let _ = db_transaction.commit();

        Ok(())
    }

    pub async fn get_info_hash_from_whitelist(&self, info_hash: &str) -> Result<InfoHash, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist WHERE info_hash = ?")?;
        let mut rows = stmt.query(&[info_hash])?;

        if let Some(row) = rows.next()? {
            let info_hash: String = row.get(0).unwrap();

            // should never be able to fail
            Ok(InfoHash::from_str(&info_hash).unwrap())
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        match conn.execute("INSERT INTO whitelist (info_hash) VALUES (?)", &[info_hash.to_string()]) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated) }
                Err(rusqlite::Error::ExecuteReturnedResults)
            },
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        match conn.execute("DELETE FROM whitelist WHERE info_hash = ?", &[info_hash.to_string()]) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated) }
                Err(rusqlite::Error::ExecuteReturnedResults)
            },
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_key_from_keys(&self, key: &str) -> Result<AuthKey, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys WHERE key = ?")?;
        let mut rows = stmt.query(&[key.to_string()])?;

        if let Some(row) = rows.next()? {
            let key: String = row.get(0).unwrap();
            let valid_until_i64: i64 = row.get(1).unwrap();

            Ok(AuthKey {
                key,
                valid_until: Some(valid_until_i64 as u64)
            })
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub async fn add_key_to_keys(&self, auth_key: &AuthKey) -> Result<usize, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        match conn.execute("INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
                           &[auth_key.key.to_string(), auth_key.valid_until.unwrap().to_string()]
        ) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated) }
                Err(rusqlite::Error::ExecuteReturnedResults)
            },
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn remove_key_from_keys(&self, key: String) -> Result<usize, rusqlite::Error> {
        let conn = self.pool.get().unwrap();
        match conn.execute("DELETE FROM keys WHERE key = ?", &[key]) {
            Ok(updated) => {
                if updated > 0 { return Ok(updated) }
                Err(rusqlite::Error::ExecuteReturnedResults)
            },
            Err(e) => {
                debug!("{:?}", e);
                Err(e)
            }
        }
    }
}
