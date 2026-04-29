//! The `SQLite3` database driver.
//!
//! This module provides implementations of the four narrow database traits
//! ([`SchemaMigrator`](crate::databases::SchemaMigrator),
//! [`TorrentMetricsStore`](crate::databases::TorrentMetricsStore),
//! [`WhitelistStore`](crate::databases::WhitelistStore),
//! [`AuthKeyStore`](crate::databases::AuthKeyStore)
//! for `SQLite3` using the `r2d2_sqlite` connection pool. It defines the schema
//! for whitelist, torrent metrics, and authentication keys, and provides methods
//! to create and drop tables as well as perform CRUD operations on these
//! persistent objects.
use std::panic::Location;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use torrust_tracker_primitives::NumberOfDownloads;

use super::{Driver, Error};

mod auth_key_store;
mod schema_migrator;
mod torrent_metrics_store;
mod whitelist_store;

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

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use torrust_tracker_configuration::Core;
    use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

    use crate::databases::driver::sqlite::Sqlite;
    use crate::databases::driver::tests::run_tests;
    use crate::databases::traits::Database;

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
