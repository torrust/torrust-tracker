//! The `SQLite3` database driver.
use ::sqlx::migrate::Migrator;
use ::sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use ::sqlx::{Row, SqlitePool};
use torrust_tracker_primitives::NumberOfDownloads;

use super::{Driver, Error};

mod auth_key_store;
mod schema_migrator;
mod torrent_metrics_store;
mod whitelist_store;

const DRIVER: Driver = Driver::Sqlite3;

/// Embedded `sqlx` migrator for the `SQLite` backend.
///
/// All `.sql` files under `migrations/sqlite/` are compiled into the binary at
/// build time and applied in timestamp order by `MIGRATOR.run(&pool)`.
pub(super) static MIGRATOR: Migrator = ::sqlx::migrate!("migrations/sqlite");

/// `SQLite` driver implementation.
///
/// This struct encapsulates an async `sqlx` connection pool for `SQLite`.
pub(crate) struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    /// Instantiates a new `SQLite3` database driver.
    ///
    // Keep the `Result` return for API symmetry with the MySQL driver and
    // forward-compatibility (future option parsing may surface fallible cases).
    #[allow(clippy::unnecessary_wraps)]
    pub fn new(db_path: &str) -> Result<Self, Error> {
        // Build the connection options directly from the filesystem path so
        // relative paths (e.g. `./storage/...`) are preserved verbatim instead
        // of being parsed as the authority component of a `sqlite://` URL.
        let options = SqliteConnectOptions::new().filename(db_path).create_if_missing(true);

        let pool = SqlitePoolOptions::new().connect_lazy_with(options);

        Ok(Self { pool })
    }

    async fn load_torrent_aggregate_metric(&self, metric_name: &str) -> Result<Option<NumberOfDownloads>, Error> {
        let maybe_row = ::sqlx::query("SELECT value FROM torrent_aggregate_metrics WHERE metric_name = ?1")
            .bind(metric_name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| (e, DRIVER))?;

        maybe_row
            .map(|row| {
                let value: i64 = row.try_get("value").map_err(|e| (e, DRIVER))?;
                u32::try_from(value).map_err(|e| Error::MalformedDatabaseRecord {
                    message: e.to_string(),
                    driver: DRIVER,
                })
            })
            .transpose()
    }

    async fn save_torrent_aggregate_metric(&self, metric_name: &str, completed: NumberOfDownloads) -> Result<(), Error> {
        // `ON CONFLICT ... DO UPDATE` may legitimately report `rows_affected() == 0`
        // when the row already exists with the same value (no-op update), so we
        // do not treat 0 as a failure here. A real failure surfaces as `Err`
        // from `execute()`.
        ::sqlx::query(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (?1, ?2) ON CONFLICT(metric_name) DO UPDATE SET value = ?2",
        )
        .bind(metric_name)
        .bind(i64::from(completed))
        .execute(&self.pool)
        .await
        .map_err(|e| (e, DRIVER))?;

        Ok(())
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
        Arc::new(Box::new(Sqlite::new(&config.database.path).unwrap()))
    }

    #[tokio::test]
    async fn run_sqlite_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let config = ephemeral_configuration();

        let driver = initialize_driver(&config);

        run_tests(&driver).await;

        Ok(())
    }

    #[tokio::test]
    async fn create_database_tables_should_be_idempotent_on_a_fresh_database() {
        let config = ephemeral_configuration();
        let driver = initialize_driver(&config);
        let options = ::sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&config.database.path)
            .create_if_missing(true);
        let pool = ::sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .expect("connect sqlite for migration count");

        // First call applies every embedded migration.
        driver
            .create_database_tables()
            .await
            .expect("first migration run should succeed on a fresh database");

        // Second call must be a no-op: the embedded `sqlx` migrator skips
        // migrations already recorded in `_sqlx_migrations`.
        driver
            .create_database_tables()
            .await
            .expect("second migration run should be a no-op");

        let recorded: i64 = ::sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&pool)
            .await
            .expect("count _sqlx_migrations");
        assert_eq!(recorded, 4, "all four migrations should be recorded");
    }
}
