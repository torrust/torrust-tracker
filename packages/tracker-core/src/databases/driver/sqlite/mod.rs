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
    #[allow(
        clippy::unnecessary_wraps,
        reason = "driver constructor keeps Result symmetry with fallible database drivers"
    )]
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

/// Fixture shared by the tests of the `SQLite` stores.
#[cfg(test)]
mod fixture {
    use torrust_tracker_configuration::v3_0_0::database::Database as DatabaseConfig;

    use super::Sqlite;
    use crate::databases::SchemaMigrator;
    use crate::test_helpers::tests::{EphemeralConfiguration, ephemeral_configuration};

    /// A store over a temporary database with its tables created. The returned configuration owns
    /// the database directory: keep it alive for the whole test.
    pub(super) async fn migrated_store() -> (Sqlite, EphemeralConfiguration) {
        let ephemeral = ephemeral_configuration();
        let Some(DatabaseConfig::Sqlite3 { path }) = ephemeral.config.database.as_ref() else {
            unreachable!("the test configuration uses SQLite persistence");
        };
        let store = Sqlite::new(path).unwrap();
        store.create_database_tables().await.unwrap();

        (store, ephemeral)
    }

    /// Runs one raw SQL statement on the store's pool, to build a database the store would never write.
    pub(super) async fn execute(store: &Sqlite, statement: &str) {
        ::sqlx::query(statement).execute(&store.pool).await.unwrap();
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use rstest::rstest;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::database::Database as DatabaseConfig;

    use crate::databases::driver::sqlite::Sqlite;
    use crate::databases::driver::sqlite::fixture::{execute, migrated_store};
    use crate::databases::driver::tests::run_tests;
    use crate::databases::error::Error;
    use crate::databases::traits::Database;
    use crate::test_helpers::tests::{EphemeralConfiguration, ephemeral_configuration};

    const A_METRIC: &str = "a_metric";

    fn initialize_driver(config: &Core) -> Arc<Box<dyn Database>> {
        Arc::new(Box::new(Sqlite::new(sqlite_path(config)).unwrap()))
    }

    fn sqlite_path(config: &Core) -> &str {
        let database = config
            .database
            .as_ref()
            .expect("test configuration includes SQLite persistence");
        let DatabaseConfig::Sqlite3 { path } = database else {
            unreachable!("test configuration uses SQLite persistence");
        };
        path
    }

    #[tokio::test]
    async fn run_sqlite_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let ephemeral = ephemeral_configuration();
        let config = &ephemeral.config;

        let driver = initialize_driver(config);

        run_tests(&driver).await;

        Ok(())
    }

    #[tokio::test]
    async fn create_database_tables_should_be_idempotent_on_a_fresh_database() {
        let ephemeral = ephemeral_configuration();
        let config = &ephemeral.config;
        let driver = initialize_driver(config);
        let options = ::sqlx::sqlite::SqliteConnectOptions::new()
            .filename(sqlite_path(config))
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

    /// A store whose `torrent_aggregate_metrics` table holds one row for `A_METRIC`, with the SQL
    /// literal `value_sql` as its value, or no table at all when `value_sql` is `None`.
    async fn store_with_aggregate_metric(value_sql: Option<&str>) -> (Sqlite, EphemeralConfiguration) {
        let (store, ephemeral) = migrated_store().await;

        let statement = value_sql.map_or_else(
            || "DROP TABLE torrent_aggregate_metrics".to_string(),
            |value| format!("INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES ('{A_METRIC}', {value})"),
        );
        execute(&store, &statement).await;

        (store, ephemeral)
    }

    #[rstest]
    #[case::a_value_below_zero("-1")]
    #[case::a_value_above_u32("4294967296")]
    #[tokio::test]
    async fn it_should_report_a_corrupted_aggregate_value_as_a_malformed_record(#[case] value_sql: &str) {
        let (store, _ephemeral) = store_with_aggregate_metric(Some(value_sql)).await;

        let result = store.load_torrent_aggregate_metric(A_METRIC).await;

        assert!(matches!(result, Err(Error::MalformedDatabaseRecord { .. })), "got {result:?}");
    }

    #[rstest]
    #[case::a_value_that_is_not_an_integer(Some("'many'"))]
    #[case::a_missing_aggregate_metrics_table(None)]
    #[tokio::test]
    async fn it_should_report_an_unreadable_aggregate_value_or_a_failed_query_as_an_invalid_query(
        #[case] value_sql: Option<&str>,
    ) {
        let (store, _ephemeral) = store_with_aggregate_metric(value_sql).await;

        let result = store.load_torrent_aggregate_metric(A_METRIC).await;

        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[tokio::test]
    async fn it_should_report_a_failed_save_of_an_aggregate_metric_as_an_invalid_query() {
        // Arrange
        let (store, _ephemeral) = store_with_aggregate_metric(None).await;

        // Act
        let result = store.save_torrent_aggregate_metric(A_METRIC, 1).await;

        // Assert
        assert!(matches!(result, Err(Error::InvalidQuery { .. })), "got {result:?}");
    }

    #[tokio::test]
    async fn it_should_keep_each_aggregate_metric_separate_when_several_are_stored() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;
        store.save_torrent_aggregate_metric("a", 1).await.unwrap();
        store.save_torrent_aggregate_metric("b", 2).await.unwrap();

        // Act
        let loaded = (
            store.load_torrent_aggregate_metric("a").await.unwrap(),
            store.load_torrent_aggregate_metric("b").await.unwrap(),
            store.load_torrent_aggregate_metric("c").await.unwrap(),
        );

        // Assert
        assert_eq!(loaded, (Some(1), Some(2), None));
    }

    #[tokio::test]
    async fn it_should_overwrite_an_aggregate_metric_that_is_saved_again() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;
        store.save_torrent_aggregate_metric(A_METRIC, 1).await.unwrap();

        // Act
        store.save_torrent_aggregate_metric(A_METRIC, 7).await.unwrap();

        // Assert
        assert_eq!(store.load_torrent_aggregate_metric(A_METRIC).await.unwrap(), Some(7));
    }

    #[tokio::test]
    async fn it_should_store_and_load_back_the_largest_aggregate_value() {
        // Arrange
        let (store, _ephemeral) = migrated_store().await;

        // Act
        store.save_torrent_aggregate_metric(A_METRIC, u32::MAX).await.unwrap();

        // Assert
        assert_eq!(store.load_torrent_aggregate_metric(A_METRIC).await.unwrap(), Some(u32::MAX));
    }
}
