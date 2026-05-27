//! The `MySQL` database driver.
use std::str::FromStr;

use ::sqlx::migrate::Migrator;
use ::sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use ::sqlx::{MySqlPool, Row};
use torrust_tracker_primitives::NumberOfDownloads;

use super::{Driver, Error};

mod auth_key_store;
mod schema_migrator;
mod torrent_metrics_store;
mod whitelist_store;

const DRIVER: Driver = Driver::MySQL;

/// Embedded `sqlx` migrator for the `MySQL` backend.
///
/// All `.sql` files under `migrations/mysql/` are compiled into the binary at
/// build time and applied in timestamp order by `MIGRATOR.run(&pool)`.
pub(super) static MIGRATOR: Migrator = ::sqlx::migrate!("migrations/mysql");

/// `MySQL` driver implementation.
///
/// This struct encapsulates an async `sqlx` connection pool for `MySQL`.
/// It implements the [`Database`] trait to provide persistence operations.
pub(crate) struct Mysql {
    pool: MySqlPool,
}

impl Mysql {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let options = MySqlConnectOptions::from_str(db_path).map_err(|e| (e, DRIVER))?;

        let pool = MySqlPoolOptions::new().connect_lazy_with(options);

        Ok(Self { pool })
    }

    async fn load_torrent_aggregate_metric(&self, metric_name: &str) -> Result<Option<NumberOfDownloads>, Error> {
        let maybe_row = ::sqlx::query("SELECT value FROM torrent_aggregate_metrics WHERE metric_name = ?")
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
        // `ON DUPLICATE KEY UPDATE` may legitimately report `rows_affected() == 0`
        // when the row already exists with the same value (no-op update), so we
        // do not treat 0 as a failure here. A real failure surfaces as `Err`
        // from `execute()`.
        ::sqlx::query(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (?, ?) ON DUPLICATE KEY UPDATE value = VALUES(value)",
        )
        .bind(metric_name)
        .bind(i64::from(completed))
        .execute(&self.pool)
        .await
        .map_err(|e| (e, DRIVER))?;

        Ok(())
    }
}

#[cfg(all(test, feature = "db-compatibility-tests"))]
mod tests {
    use std::sync::Arc;

    use testcontainers::core::{IntoContainerPort, WaitFor};
    /*
    We run a MySQL container and run all the tests against the same container and database.

    Test for this driver are executed with:

    `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true \
     cargo test -p torrust-tracker-core --features db-compatibility-tests run_mysql_driver_tests`

    The `Database` trait is very simple and we only have one driver that needs
    a container. In the future we might want to use different approaches like:

    - https://github.com/testcontainers/testcontainers-rs/issues/707
    - https://www.infinyon.com/blog/2021/04/rust-custom-test-harness/
    - https://github.com/torrust/torrust-tracker/blob/develop/src/bin/e2e_tests_runner.rs

    If we increase the number of methods or the number or drivers.
    */
    use testcontainers::runners::AsyncRunner;
    use testcontainers::{ContainerAsync, GenericImage, ImageExt};
    use torrust_tracker_configuration::Core;

    use super::Mysql;
    use crate::databases::driver::tests::run_tests;
    use crate::databases::traits::Database;
    use crate::test_helpers::tests::random_info_hash;

    #[derive(Debug, Default)]
    struct StoppedMysqlContainer {}

    impl StoppedMysqlContainer {
        async fn run(self, config: &MysqlConfiguration) -> Result<RunningMysqlContainer, Box<dyn std::error::Error + 'static>> {
            let image_tag = std::env::var("TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG").unwrap_or_else(|_| "8.0".to_string());

            let container = GenericImage::new("mysql", image_tag.as_str())
                .with_exposed_port(config.internal_port.tcp())
                // MySQL 8.0 outputs "ready for connections" to stderr (not stdout).
                // The first occurrence is during internal init (port: 0); the second
                // includes "port: 3306" and indicates the server is ready for TCP
                // connections. We wait for the second message to avoid connecting
                // before MySQL accepts client traffic.
                .with_wait_for(WaitFor::message_on_stderr("port: 3306"))
                .with_env_var("MYSQL_ROOT_PASSWORD", config.db_root_password.clone())
                .with_env_var("MYSQL_DATABASE", config.database.clone())
                .with_env_var("MYSQL_ROOT_HOST", "%")
                .start()
                .await?;

            Ok(RunningMysqlContainer::new(container, config.internal_port))
        }
    }

    struct RunningMysqlContainer {
        container: ContainerAsync<GenericImage>,
        internal_port: u16,
    }

    impl RunningMysqlContainer {
        fn new(container: ContainerAsync<GenericImage>, internal_port: u16) -> Self {
            Self {
                container,
                internal_port,
            }
        }

        async fn stop(self) {
            self.container.stop().await.unwrap();
        }

        async fn get_host(&self) -> url::Host {
            self.container.get_host().await.unwrap()
        }

        async fn get_host_port_ipv4(&self) -> u16 {
            self.container.get_host_port_ipv4(self.internal_port).await.unwrap()
        }
    }

    impl Default for MysqlConfiguration {
        fn default() -> Self {
            Self {
                internal_port: 3306,
                database: "torrust_tracker_test".to_string(),
                db_user: "root".to_string(),
                db_root_password: "test".to_string(),
            }
        }
    }

    struct MysqlConfiguration {
        pub internal_port: u16,
        pub database: String,
        pub db_user: String,
        pub db_root_password: String,
    }

    fn core_configuration(host: &url::Host, port: u16, mysql_configuration: &MysqlConfiguration) -> Core {
        let mut config = Core::default();

        let database = mysql_configuration.database.clone();
        let db_user = mysql_configuration.db_user.clone();
        let db_password = mysql_configuration.db_root_password.clone();

        config.database.path = format!("mysql://{db_user}:{db_password}@{host}:{port}/{database}");

        config
    }

    fn initialize_driver(config: &Core) -> Arc<Box<dyn Database>> {
        Arc::new(Box::new(Mysql::new(&config.database.path).unwrap()))
    }

    // This test is invoked by `.github/workflows/testing.yaml` in the
    // `database-compatibility` job to validate supported MySQL versions.
    #[tokio::test]
    async fn run_mysql_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        if std::env::var("TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST").is_err() {
            println!("Skipping the MySQL driver tests.");
            return Ok(());
        }

        let mysql_configuration = MysqlConfiguration::default();

        let stopped_mysql_container = StoppedMysqlContainer::default();

        let mysql_container = stopped_mysql_container.run(&mysql_configuration).await.unwrap();

        let host = mysql_container.get_host().await;
        let port = mysql_container.get_host_port_ipv4().await;

        let config = core_configuration(&host, port, &mysql_configuration);

        let driver = initialize_driver(&config);

        run_tests(&driver).await;

        // Idempotency: a second `create_database_tables()` call must be a
        // no-op (embedded `sqlx` migrator skips migrations already recorded
        // in `_sqlx_migrations`).
        driver
            .create_database_tables()
            .await
            .expect("second migration run should be a no-op");

        // Legacy bootstrap: simulate a pre-v4 database (no `_sqlx_migrations`
        // table, all four legacy tables present) and verify
        // `create_database_tables()` seeds the migration history without
        // re-running the embedded migrations.
        driver
            .drop_database_tables()
            .await
            .expect("drop tables before legacy bootstrap test");

        let raw_pool = ::sqlx::mysql::MySqlPoolOptions::new()
            .connect(&config.database.path)
            .await
            .expect("connect to mysql for raw DDL");
        create_legacy_pre_v4_schema(&raw_pool).await;

        driver
            .create_database_tables()
            .await
            .expect("legacy bootstrap should succeed");

        let recorded: i64 = ::sqlx::query_scalar("SELECT COUNT(*) FROM `_sqlx_migrations`")
            .fetch_one(&raw_pool)
            .await
            .expect("count _sqlx_migrations");
        assert_eq!(
            recorded, 4,
            "all migrations should be recorded after bootstrap + migrator run"
        );

        assert_mysql_column_type(&raw_pool, "torrents", "completed", "bigint").await;
        assert_mysql_column_type(&raw_pool, "torrent_aggregate_metrics", "value", "bigint").await;

        let above_i32_max = 2_200_000_000_u32;
        let info_hash = random_info_hash();

        driver
            .save_torrent_downloads(&info_hash, above_i32_max)
            .await
            .expect("save torrent downloads above i32::MAX should succeed");
        let loaded_torrent_downloads = driver
            .load_torrent_downloads(&info_hash)
            .await
            .expect("load torrent downloads above i32::MAX should succeed");
        assert_eq!(loaded_torrent_downloads, Some(above_i32_max));

        driver
            .save_global_downloads(above_i32_max)
            .await
            .expect("save global downloads above i32::MAX should succeed");
        let loaded_global_downloads = driver
            .load_global_downloads()
            .await
            .expect("load global downloads above i32::MAX should succeed");
        assert_eq!(loaded_global_downloads, Some(above_i32_max));

        // Partial-state rejection: only two of four legacy tables present.
        driver
            .drop_database_tables()
            .await
            .expect("drop tables before partial-state test");
        for stmt in [
            "CREATE TABLE whitelist (id INTEGER PRIMARY KEY AUTO_INCREMENT)",
            "CREATE TABLE torrents (id INTEGER PRIMARY KEY AUTO_INCREMENT)",
        ] {
            ::sqlx::query(stmt).execute(&raw_pool).await.expect("partial DDL");
        }

        let err = driver
            .create_database_tables()
            .await
            .expect_err("partial legacy state must be rejected");
        match err {
            crate::databases::error::Error::LegacyDatabaseNotMigrated { reason, .. } => {
                assert!(reason.contains("apply every pre-v4 migration"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
        drop(raw_pool);

        mysql_container.stop().await;

        Ok(())
    }

    /// Recreate the schema produced by the three pre-v4 manual migrations.
    ///
    /// This raw DDL mirrors the cumulative state of
    /// `migrations/mysql/2024073018*.sql` and
    /// `migrations/mysql/20250527093000_*.sql` after they have been applied
    /// in order. We build it by hand so the legacy-bootstrap test path
    /// can build a database that looks exactly like a pre-v4 tracker on disk
    /// (legacy tables present, no `_sqlx_migrations` row).
    ///
    /// # Legacy compatibility
    ///
    /// Drop this helper at the same time as the
    /// `bootstrap_legacy_schema` function in
    /// `mysql/schema_migrator.rs` — see the legacy-compatibility note on
    /// that function.
    async fn create_legacy_pre_v4_schema(pool: &::sqlx::MySqlPool) {
        for stmt in [
            "CREATE TABLE whitelist (id INTEGER PRIMARY KEY AUTO_INCREMENT, info_hash VARCHAR(40) NOT NULL UNIQUE)",
            "CREATE TABLE torrents (id INTEGER PRIMARY KEY AUTO_INCREMENT, info_hash VARCHAR(40) NOT NULL UNIQUE, completed INTEGER DEFAULT 0 NOT NULL)",
            "CREATE TABLE `keys` (`id` INT NOT NULL AUTO_INCREMENT, `key` VARCHAR(32) NOT NULL, `valid_until` INT(10), PRIMARY KEY (`id`), UNIQUE (`key`))",
            "CREATE TABLE torrent_aggregate_metrics (id INTEGER PRIMARY KEY AUTO_INCREMENT, metric_name VARCHAR(50) NOT NULL UNIQUE, value INTEGER DEFAULT 0 NOT NULL)",
        ] {
            ::sqlx::query(stmt).execute(pool).await.expect("legacy DDL");
        }
    }

    async fn assert_mysql_column_type(pool: &::sqlx::MySqlPool, table: &str, column: &str, expected_type: &str) {
        let data_type_bytes: Vec<u8> = ::sqlx::query_scalar(
            "SELECT DATA_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ? AND COLUMN_NAME = ?",
        )
        .bind(table)
        .bind(column)
        .fetch_one(pool)
        .await
        .expect("column type query should succeed");

        let data_type = String::from_utf8_lossy(&data_type_bytes).to_lowercase();

        assert_eq!(data_type, expected_type, "{table}.{column} should be {expected_type}");
    }
}
