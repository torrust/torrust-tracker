//! The `SQLite3` database driver.
//!
//! This module provides an implementation of the [`Database`] trait for
//! `SQLite3` using the `r2d2_sqlite` connection pool. It defines the schema for
//!  whitelist, torrent metrics, and authentication keys, and provides methods
//! to create and drop tables as well as perform CRUD operations on these
//! persistent objects.
use std::panic::Location;
use std::str::FromStr;

use bittorrent_primitives::info_hash::InfoHash;
use r2d2::Pool;
use r2d2_sqlite::rusqlite::params;
use r2d2_sqlite::rusqlite::types::Null;
use r2d2_sqlite::SqliteConnectionManager;
use torrust_tracker_primitives::{DurationSinceUnixEpoch, NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{Database, Driver, Error, TORRENTS_DOWNLOADS_TOTAL};
use crate::authentication::{self, Key};

const DRIVER: Driver = Driver::Sqlite3;

/// `SQLite` driver implementation.
///
/// This struct encapsulates a connection pool for `SQLite` using the `r2d2_sqlite`
/// connection manager.
pub(crate) struct Sqlite {
    pool: Pool<SqliteConnectionManager>,
}

impl Sqlite {
    /// Instantiates a new `SQLite3` database driver.
    ///
    /// This function creates a connection manager for the `SQLite` database
    /// located at `db_path` and then builds a connection pool using `r2d2`. If
    /// the pool cannot be created, an error is returned (wrapped with the
    /// appropriate driver information).
    ///
    /// # Arguments
    ///
    /// * `db_path` - A string slice representing the file path to the `SQLite` database.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the connection pool cannot be built.
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = r2d2::Pool::builder().build(manager).map_err(|e| (e, DRIVER))?;

        Ok(Self { pool })
    }

    fn load_torrent_aggregate_metric(&self, metric_name: &str) -> Result<Option<NumberOfDownloads>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT value FROM torrent_aggregate_metrics WHERE metric_name = ?")?;

        let mut rows = stmt.query([metric_name])?;

        let persistent_torrent = rows.next()?;

        Ok(persistent_torrent.map(|f| {
            let value: i64 = f.get(0).unwrap();
            u32::try_from(value).unwrap()
        }))
    }

    fn save_torrent_aggregate_metric(&self, metric_name: &str, completed: NumberOfDownloads) -> Result<(), Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = conn.execute(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (?1, ?2) ON CONFLICT(metric_name) DO UPDATE SET value = ?2",
            [metric_name.to_string(), completed.to_string()],
        )?;

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }
}

impl Database for Sqlite {
    /// Refer to [`databases::Database::create_database_tables`](crate::core::databases::Database::create_database_tables).
    fn create_database_tables(&self) -> Result<(), Error> {
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

        let create_torrent_aggregate_metrics_table = "
        CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            metric_name TEXT NOT NULL UNIQUE,
            value INTEGER DEFAULT 0 NOT NULL
        );"
        .to_string();

        let create_keys_table = "
        CREATE TABLE IF NOT EXISTS keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            valid_until INTEGER
         );"
        .to_string();

        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.execute(&create_whitelist_table, [])?;
        conn.execute(&create_keys_table, [])?;
        conn.execute(&create_torrents_table, [])?;
        conn.execute(&create_torrent_aggregate_metrics_table, [])?;

        Ok(())
    }

    /// Refer to [`databases::Database::drop_database_tables`](crate::core::databases::Database::drop_database_tables).
    fn drop_database_tables(&self) -> Result<(), Error> {
        let drop_whitelist_table = "
        DROP TABLE whitelist;"
            .to_string();

        let drop_torrents_table = "
        DROP TABLE torrents;"
            .to_string();

        let drop_keys_table = "
        DROP TABLE keys;"
            .to_string();

        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.execute(&drop_whitelist_table, [])
            .and_then(|_| conn.execute(&drop_torrents_table, []))
            .and_then(|_| conn.execute(&drop_keys_table, []))?;

        Ok(())
    }

    /// Refer to [`databases::Database::load_persistent_torrents`](crate::core::databases::Database::load_persistent_torrents).
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash, completed FROM torrents")?;

        let torrent_iter = stmt.query_map([], |row| {
            let info_hash_string: String = row.get(0)?;
            let info_hash = InfoHash::from_str(&info_hash_string).unwrap();
            let completed: u32 = row.get(1)?;
            Ok((info_hash, completed))
        })?;

        Ok(torrent_iter.filter_map(std::result::Result::ok).collect())
    }

    /// Refer to [`databases::Database::load_persistent_torrent`](crate::core::databases::Database::load_persistent_torrent).
    fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT completed FROM torrents WHERE info_hash = ?")?;

        let mut rows = stmt.query([info_hash.to_hex_string()])?;

        let persistent_torrent = rows.next()?;

        Ok(persistent_torrent.map(|f| {
            let completed: i64 = f.get(0).unwrap();
            u32::try_from(completed).unwrap()
        }))
    }

    /// Refer to [`databases::Database::save_persistent_torrent`](crate::core::databases::Database::save_persistent_torrent).
    fn save_torrent_downloads(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = conn.execute(
            "INSERT INTO torrents (info_hash, completed) VALUES (?1, ?2) ON CONFLICT(info_hash) DO UPDATE SET completed = ?2",
            [info_hash.to_string(), completed.to_string()],
        )?;

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }

    /// Refer to [`databases::Database::increase_number_of_downloads`](crate::core::databases::Database::increase_number_of_downloads).
    fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let _ = conn.execute(
            "UPDATE torrents SET completed = completed + 1 WHERE info_hash = ?",
            [info_hash.to_string()],
        )?;

        Ok(())
    }

    /// Refer to [`databases::Database::load_global_number_of_downloads`](crate::core::databases::Database::load_global_number_of_downloads).
    fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.load_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL)
    }

    /// Refer to [`databases::Database::save_global_number_of_downloads`](crate::core::databases::Database::save_global_number_of_downloads).
    fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.save_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL, downloaded)
    }

    /// Refer to [`databases::Database::increase_global_number_of_downloads`](crate::core::databases::Database::increase_global_number_of_downloads).
    fn increase_global_downloads(&self) -> Result<(), Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let metric_name = TORRENTS_DOWNLOADS_TOTAL;

        let _ = conn.execute(
            "UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = ?",
            [metric_name],
        )?;

        Ok(())
    }

    /// Refer to [`databases::Database::load_keys`](crate::core::databases::Database::load_keys).
    fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys")?;

        let keys_iter = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let opt_valid_until: Option<i64> = row.get(1)?;

            match opt_valid_until {
                Some(valid_until) => Ok(authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: Some(DurationSinceUnixEpoch::from_secs(valid_until.unsigned_abs())),
                }),
                None => Ok(authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: None,
                }),
            }
        })?;

        let keys: Vec<authentication::PeerKey> = keys_iter.filter_map(std::result::Result::ok).collect();

        Ok(keys)
    }

    /// Refer to [`databases::Database::load_whitelist`](crate::core::databases::Database::load_whitelist).
    fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist")?;

        let info_hash_iter = stmt.query_map([], |row| {
            let info_hash: String = row.get(0)?;

            Ok(InfoHash::from_str(&info_hash).unwrap())
        })?;

        let info_hashes: Vec<InfoHash> = info_hash_iter.filter_map(std::result::Result::ok).collect();

        Ok(info_hashes)
    }

    /// Refer to [`databases::Database::get_info_hash_from_whitelist`](crate::core::databases::Database::get_info_hash_from_whitelist).
    fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT info_hash FROM whitelist WHERE info_hash = ?")?;

        let mut rows = stmt.query([info_hash.to_hex_string()])?;

        let query = rows.next()?;

        Ok(query.map(|f| InfoHash::from_str(&f.get_unwrap::<_, String>(0)).unwrap()))
    }

    /// Refer to [`databases::Database::add_info_hash_to_whitelist`](crate::core::databases::Database::add_info_hash_to_whitelist).
    fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = conn.execute("INSERT INTO whitelist (info_hash) VALUES (?)", [info_hash.to_string()])?;

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert)
        }
    }

    /// Refer to [`databases::Database::remove_info_hash_from_whitelist`](crate::core::databases::Database::remove_info_hash_from_whitelist).
    fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let deleted = conn.execute("DELETE FROM whitelist WHERE info_hash = ?", [info_hash.to_string()])?;

        if deleted == 1 {
            // should only remove a single record.
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }

    /// Refer to [`databases::Database::get_key_from_keys`](crate::core::databases::Database::get_key_from_keys).
    fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let mut stmt = conn.prepare("SELECT key, valid_until FROM keys WHERE key = ?")?;

        let mut rows = stmt.query([key.to_string()])?;

        let key = rows.next()?;

        Ok(key.map(|f| {
            let valid_until: Option<i64> = f.get(1).unwrap();
            let key: String = f.get(0).unwrap();

            match valid_until {
                Some(valid_until) => authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: Some(DurationSinceUnixEpoch::from_secs(valid_until.unsigned_abs())),
                },
                None => authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: None,
                },
            }
        }))
    }

    /// Refer to [`databases::Database::add_key_to_keys`](crate::core::databases::Database::add_key_to_keys).
    fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let insert = match auth_key.valid_until {
            Some(valid_until) => conn.execute(
                "INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
                [auth_key.key.to_string(), valid_until.as_secs().to_string()],
            )?,
            None => conn.execute(
                "INSERT INTO keys (key, valid_until) VALUES (?1, ?2)",
                params![auth_key.key.to_string(), Null],
            )?,
        };

        if insert == 0 {
            Err(Error::InsertFailed {
                location: Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert)
        }
    }

    /// Refer to [`databases::Database::remove_key_from_keys`](crate::core::databases::Database::remove_key_from_keys).
    fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let deleted = conn.execute("DELETE FROM keys WHERE key = ?", [key.to_string()])?;

        if deleted == 1 {
            // should only remove a single record.
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use torrust_tracker_configuration::Core;
    use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

    use crate::databases::driver::sqlite::Sqlite;
    use crate::databases::driver::tests::run_tests;
    use crate::databases::Database;

    fn ephemeral_configuration() -> Core {
        let mut config = Core::default();
        let temp_file = ephemeral_sqlite_database();
        temp_file.to_str().unwrap().clone_into(&mut config.database.path);
        config
    }

    fn initialize_driver(config: &Core) -> Arc<Box<dyn Database>> {
        let driver: Arc<Box<dyn Database>> = Arc::new(Box::new(Sqlite::new(&config.database.path).unwrap()));
        driver
    }

    #[tokio::test]
    async fn run_sqlite_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let config = ephemeral_configuration();

        let driver = initialize_driver(&config);

        run_tests(&driver).await;

        Ok(())
    }
}
