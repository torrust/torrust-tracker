//! The `PostgreSQL` database driver.
use std::str::FromStr;

use ::sqlx::migrate::Migrator;
use ::sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use ::sqlx::{PgPool, Row};
use torrust_tracker_primitives::NumberOfDownloads;

use super::{Driver, Error};

mod auth_key_store;
mod schema_migrator;
mod torrent_metrics_store;
mod whitelist_store;

const DRIVER: Driver = Driver::PostgreSQL;

/// Embedded `sqlx` migrator for the `PostgreSQL` backend.
///
/// All `.sql` files under `migrations/postgresql/` are compiled into the binary at
/// build time and applied in timestamp order by `MIGRATOR.run(&pool)`.
pub(super) static MIGRATOR: Migrator = ::sqlx::migrate!("migrations/postgresql");

/// `PostgreSQL` driver implementation.
///
/// This struct encapsulates an async `sqlx` connection pool for `PostgreSQL`.
/// It implements the [`Database`] trait to provide persistence operations.
pub(crate) struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let options = PgConnectOptions::from_str(db_path).map_err(|e| (e, DRIVER))?;

        let pool = PgPoolOptions::new().connect_lazy_with(options);

        Ok(Self { pool })
    }

    async fn load_torrent_aggregate_metric(&self, metric_name: &str) -> Result<Option<NumberOfDownloads>, Error> {
        let maybe_row = ::sqlx::query("SELECT value FROM torrent_aggregate_metrics WHERE metric_name = $1")
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
        // `ON CONFLICT ... DO UPDATE SET` may legitimately report `rows_affected() == 0`
        // when the row already exists with the same value (no-op update), so we
        // do not treat 0 as a failure here. A real failure surfaces as `Err`
        // from `execute()`.
        ::sqlx::query(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES ($1, $2) \
             ON CONFLICT (metric_name) DO UPDATE SET value = EXCLUDED.value",
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

    use secrecy::SecretString;
    use testcontainers::core::IntoContainerPort;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::{ContainerAsync, GenericImage, ImageExt};
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::database::{ConnectionInfo, Database as ConfigurationDatabase};

    use super::Postgres;
    use crate::databases::driver::tests::run_tests;
    use crate::databases::traits::Database;
    use crate::test_helpers::tests::random_info_hash;

    #[derive(Debug, Default)]
    struct StoppedPostgresContainer {}

    impl StoppedPostgresContainer {
        async fn run(
            self,
            config: &PostgresConfiguration,
        ) -> Result<RunningPostgresContainer, Box<dyn std::error::Error + 'static>> {
            let image_tag = std::env::var("TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG").unwrap_or_else(|_| "16".to_string());

            let container = GenericImage::new("postgres", image_tag.as_str())
                .with_exposed_port(config.internal_port.tcp())
                .with_env_var("POSTGRES_PASSWORD", config.db_password.clone())
                .with_env_var("POSTGRES_USER", config.db_user.clone())
                .with_env_var("POSTGRES_DB", config.database.clone())
                .start()
                .await?;

            Ok(RunningPostgresContainer::new(container, config.internal_port))
        }
    }

    struct RunningPostgresContainer {
        container: ContainerAsync<GenericImage>,
        internal_port: u16,
    }

    impl RunningPostgresContainer {
        const fn new(container: ContainerAsync<GenericImage>, internal_port: u16) -> Self {
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

    impl Default for PostgresConfiguration {
        fn default() -> Self {
            Self {
                internal_port: 5432,
                database: "torrust_tracker_test".to_string(),
                db_user: "postgres".to_string(),
                db_password: "test".to_string(),
            }
        }
    }

    struct PostgresConfiguration {
        pub internal_port: u16,
        pub database: String,
        pub db_user: String,
        pub db_password: String,
    }

    fn core_configuration(host: &url::Host, port: u16, postgres_configuration: &PostgresConfiguration) -> Core {
        Core {
            database: Some(ConfigurationDatabase::PostgreSQL(ConnectionInfo {
                host: host.to_string(),
                port,
                user: postgres_configuration.db_user.clone(),
                password: SecretString::from(postgres_configuration.db_password.clone()),
                database: postgres_configuration.database.clone(),
            })),
            ..Core::default()
        }
    }

    fn initialize_driver(config: &Core) -> Arc<Box<dyn Database>> {
        let database_url = config
            .database
            .as_ref()
            .expect("PostgreSQL driver test configuration must include a database")
            .connection_url();
        Arc::new(Box::new(Postgres::new(&database_url).unwrap()))
    }

    // This test is invoked by `.github/workflows/testing.yaml` in the
    // `database-compatibility` job to validate supported PostgreSQL versions.
    #[tokio::test]
    async fn run_postgres_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        if std::env::var("TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST").is_err() {
            tracing::info!("Skipping the PostgreSQL driver tests.");
            return Ok(());
        }

        let postgres_configuration = PostgresConfiguration::default();

        let stopped_postgres_container = StoppedPostgresContainer::default();

        let postgres_container = stopped_postgres_container.run(&postgres_configuration).await.unwrap();

        let host = postgres_container.get_host().await;
        let port = postgres_container.get_host_port_ipv4().await;

        let config = core_configuration(&host, port, &postgres_configuration);

        let driver = initialize_driver(&config);

        run_tests(&driver).await;

        // Idempotency: a second `create_database_tables()` call must be a
        // no-op (embedded `sqlx` migrator skips migrations already recorded
        // in `_sqlx_migrations`).
        driver
            .create_database_tables()
            .await
            .expect("second migration run should be a no-op");

        // PostgreSQL has no legacy pre-v4 databases, so we skip the
        // legacy bootstrap test. PostgreSQL support was added in v4+.
        driver.drop_database_tables().await.expect("drop tables for fresh test");

        let raw_pool = ::sqlx::postgres::PgPoolOptions::new()
            .connect(
                &config
                    .database
                    .as_ref()
                    .expect("PostgreSQL driver test configuration must include a database")
                    .connection_url(),
            )
            .await
            .expect("connect to postgres for raw DDL");
        create_legacy_pre_v4_schema(&raw_pool).await;

        driver
            .create_database_tables()
            .await
            .expect("fresh schema creation should succeed");

        let recorded: i64 = ::sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&raw_pool)
            .await
            .expect("count _sqlx_migrations");
        assert_eq!(recorded, 4, "all migrations should be recorded after migrator run");

        assert_postgres_column_type(&raw_pool, "torrents", "completed", "bigint").await;
        assert_postgres_column_type(&raw_pool, "torrent_aggregate_metrics", "value", "bigint").await;

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

        drop(raw_pool);

        postgres_container.stop().await;

        Ok(())
    }

    /// Create a minimal schema for `PostgreSQL`.
    ///
    /// `PostgreSQL` support was added in v4, so there are no pre-v4 databases.
    /// This helper creates a fresh schema to test idempotency of the migrator.
    async fn create_legacy_pre_v4_schema(pool: &::sqlx::PgPool) {
        for stmt in [
            "CREATE TABLE IF NOT EXISTS whitelist (id SERIAL PRIMARY KEY, info_hash VARCHAR(40) NOT NULL UNIQUE)",
            "CREATE TABLE IF NOT EXISTS torrents (id SERIAL PRIMARY KEY, info_hash VARCHAR(40) NOT NULL UNIQUE, completed INTEGER DEFAULT 0 NOT NULL)",
            "CREATE TABLE IF NOT EXISTS keys (id SERIAL PRIMARY KEY, key VARCHAR(32) NOT NULL UNIQUE, valid_until BIGINT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (id SERIAL PRIMARY KEY, metric_name VARCHAR(50) NOT NULL UNIQUE, value INTEGER DEFAULT 0 NOT NULL)",
        ] {
            ::sqlx::query(stmt).execute(pool).await.expect("schema DDL");
        }
    }

    async fn assert_postgres_column_type(pool: &::sqlx::PgPool, table: &str, column: &str, expected_type: &str) {
        let data_type: String =
            ::sqlx::query_scalar("SELECT data_type FROM information_schema.columns WHERE table_name = $1 AND column_name = $2")
                .bind(table)
                .bind(column)
                .fetch_one(pool)
                .await
                .expect("column type query should succeed");

        assert_eq!(
            data_type.to_lowercase(),
            expected_type,
            "{table}.{column} should be {expected_type}"
        );
    }
}
