//! The `PostgreSQL` database driver.
//!
//! This module provides an implementation of the [`Database`] trait for
//! `PostgreSQL` using the `r2d2_postgres` connection pool. It configures the
//! `PostgreSQL` connection based on a URL, creates the necessary tables (for
//! torrent metrics, torrent whitelist, and authentication keys), and implements
//! all CRUD operations required by the persistence layer.
//!
//! **Note on runtime compatibility:** The synchronous `postgres` crate
//! internally uses its own `tokio::runtime::Runtime`. To avoid panics when
//! called from within an existing tokio runtime (e.g., from async request
//! handlers or `#[tokio::test]`), all pool operations — including connection
//! checkout, query execution, and pool destruction — are executed inside
//! `std::thread::scope` so that connection creation and destruction happen
//! outside the caller's tokio context.
use std::str::FromStr;
use std::time::Duration;

use bittorrent_primitives::info_hash::InfoHash;
use r2d2::Pool;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{Database, Driver, Error, TORRENTS_DOWNLOADS_TOTAL};
use crate::authentication::key::AUTH_KEY_LENGTH;
use crate::authentication::{self, Key};

const DRIVER: Driver = Driver::PostgreSQL;

/// `PostgreSQL` driver implementation.
///
/// This struct encapsulates a connection pool for `PostgreSQL`, built using the
/// `r2d2_postgres` connection manager. It implements the [`Database`] trait to
/// provide persistence operations.
///
/// All database operations (and pool destruction) are executed in a scoped
/// thread to avoid conflicts with the tokio runtime. The sync `postgres` crate
/// creates its own internal tokio runtime, and `Runtime::block_on` panics if
/// called from a thread that already has a tokio context. This includes the
/// `Drop` implementation of `postgres::Client`, which is why the pool is also
/// dropped in a separate thread.
pub(crate) struct Postgres {
    /// Wrapped in `Option` so we can take ownership in `Drop` and move
    /// the pool to a dedicated thread for cleanup.
    pool: Option<Pool<PostgresConnectionManager<NoTls>>>,
}

impl Drop for Postgres {
    fn drop(&mut self) {
        // The postgres client's Drop calls block_on, which panics inside
        // a tokio runtime. Move the pool to a separate thread for cleanup.
        if let Some(pool) = self.pool.take() {
            std::thread::spawn(move || drop(pool)).join().ok();
        }
    }
}

impl Postgres {
    /// It instantiates a new `PostgreSQL` database driver.
    ///
    /// # Errors
    ///
    /// Will return `r2d2::Error` if `db_path` is not able to create `PostgreSQL` database.
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let db_path = db_path.to_string();
        // Build the connection pool in a separate thread to avoid tokio
        // runtime conflicts (r2d2 eagerly creates connections during build).
        std::thread::scope(|s| {
            s.spawn(|| {
                let manager = PostgresConnectionManager::new(
                    db_path.parse().map_err(|e: r2d2_postgres::postgres::Error| {
                        let source: std::sync::Arc<dyn std::error::Error + Send + Sync> = std::sync::Arc::new(e);
                        Error::GenericConnectionError {
                            source: source.into(),
                            driver: DRIVER,
                        }
                    })?,
                    NoTls,
                );
                let pool = r2d2::Pool::builder().build(manager).map_err(|e| (e, DRIVER))?;
                Ok(Self { pool: Some(pool) })
            })
            .join()
            .expect("PostgreSQL connection pool creation thread panicked")
        })
    }

    /// Returns a reference to the connection pool.
    fn pool(&self) -> &Pool<PostgresConnectionManager<NoTls>> {
        self.pool.as_ref().expect("PostgreSQL pool has been dropped")
    }

    /// Executes a closure with a pooled connection in a scoped thread.
    ///
    /// This avoids the "Cannot start a runtime from within a runtime" panic
    /// that occurs when the sync `postgres` crate's internal tokio runtime
    /// clashes with an outer tokio runtime.
    fn with_connection<F, T>(&self, f: F) -> Result<T, Error>
    where
        F: FnOnce(&mut r2d2::PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<T, Error> + Send,
        T: Send,
    {
        let pool = self.pool();
        std::thread::scope(|s| {
            s.spawn(|| {
                let mut conn = pool.get().map_err(|e| (e, DRIVER))?;
                f(&mut conn)
            })
            .join()
            .expect("PostgreSQL worker thread panicked")
        })
    }

    fn load_torrent_aggregate_metric(&self, metric_name: &str) -> Result<Option<NumberOfDownloads>, Error> {
        let metric_name = metric_name.to_string();
        self.with_connection(|conn| {
            let rows = conn.query(
                "SELECT value FROM torrent_aggregate_metrics WHERE metric_name = $1",
                &[&metric_name],
            )?;

            if let Some(row) = rows.first() {
                let value: i32 = row.get(0);
                Ok(Some(u32::try_from(value).unwrap()))
            } else {
                Ok(None)
            }
        })
    }

    fn save_torrent_aggregate_metric(&self, metric_name: &str, completed: NumberOfDownloads) -> Result<(), Error> {
        let metric_name = metric_name.to_string();
        self.with_connection(move |conn| {
            let completed_i32 = i32::try_from(completed).unwrap();

            conn.execute(
                "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES ($1, $2) ON CONFLICT (metric_name) DO UPDATE SET value = EXCLUDED.value",
                &[&metric_name, &completed_i32],
            )?;

            Ok(())
        })
    }
}

impl Database for Postgres {
    fn create_database_tables(&self) -> Result<(), Error> {
        self.with_connection(|conn| {
            let create_whitelist_table = "
            CREATE TABLE IF NOT EXISTS whitelist (
                id SERIAL PRIMARY KEY,
                info_hash VARCHAR(40) NOT NULL UNIQUE
            );";

            let create_torrents_table = "
            CREATE TABLE IF NOT EXISTS torrents (
                id SERIAL PRIMARY KEY,
                info_hash VARCHAR(40) NOT NULL UNIQUE,
                completed INTEGER DEFAULT 0 NOT NULL
            );";

            let create_torrent_aggregate_metrics_table = "
            CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (
                id SERIAL PRIMARY KEY,
                metric_name VARCHAR(50) NOT NULL UNIQUE,
                value INTEGER DEFAULT 0 NOT NULL
            );";

            let create_keys_table = format!(
                "
            CREATE TABLE IF NOT EXISTS keys (
              id SERIAL PRIMARY KEY,
              key VARCHAR({}) NOT NULL UNIQUE,
              valid_until BIGINT
            );",
                i8::try_from(AUTH_KEY_LENGTH).expect("authentication key length should fit within a i8!")
            );

            conn.execute(create_torrents_table, &[])
                .expect("Could not create torrents table.");
            conn.execute(create_torrent_aggregate_metrics_table, &[])
                .expect("Could not create torrent_aggregate_metrics table.");
            conn.execute(&create_keys_table, &[]).expect("Could not create keys table.");
            conn.execute(create_whitelist_table, &[])
                .expect("Could not create whitelist table.");

            Ok(())
        })
    }

    fn drop_database_tables(&self) -> Result<(), Error> {
        self.with_connection(|conn| {
            conn.execute("DROP TABLE whitelist;", &[])
                .expect("Could not drop whitelist table.");
            conn.execute("DROP TABLE torrents;", &[])
                .expect("Could not drop torrents table.");
            conn.execute("DROP TABLE torrent_aggregate_metrics;", &[])
                .expect("Could not drop torrent_aggregate_metrics table.");
            conn.execute("DROP TABLE keys;", &[]).expect("Could not drop keys table.");

            Ok(())
        })
    }

    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        self.with_connection(|conn| {
            let rows = conn.query("SELECT info_hash, completed FROM torrents", &[])?;

            let torrents: Vec<(InfoHash, u32)> = rows
                .iter()
                .map(|row| {
                    let info_hash_string: String = row.get(0);
                    let completed: i32 = row.get(1);
                    let info_hash = InfoHash::from_str(&info_hash_string).unwrap();
                    (info_hash, u32::try_from(completed).unwrap())
                })
                .collect();

            Ok(torrents.iter().copied().collect())
        })
    }

    fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        let info_hash_hex = info_hash.to_hex_string();
        self.with_connection(move |conn| {
            let rows = conn.query("SELECT completed FROM torrents WHERE info_hash = $1", &[&info_hash_hex])?;

            if let Some(row) = rows.first() {
                let completed: i32 = row.get(0);
                Ok(Some(u32::try_from(completed).unwrap()))
            } else {
                Ok(None)
            }
        })
    }

    fn save_torrent_downloads(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error> {
        let info_hash_str = info_hash.to_string();
        self.with_connection(move |conn| {
            let completed_i32 = i32::try_from(completed).unwrap();

            conn.execute(
                "INSERT INTO torrents (info_hash, completed) VALUES ($1, $2) ON CONFLICT (info_hash) DO UPDATE SET completed = EXCLUDED.completed",
                &[&info_hash_str, &completed_i32],
            )?;

            Ok(())
        })
    }

    fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        let info_hash_str = info_hash.to_string();
        self.with_connection(move |conn| {
            conn.execute(
                "UPDATE torrents SET completed = completed + 1 WHERE info_hash = $1",
                &[&info_hash_str],
            )?;

            Ok(())
        })
    }

    fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.load_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL)
    }

    fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.save_torrent_aggregate_metric(TORRENTS_DOWNLOADS_TOTAL, downloaded)
    }

    fn increase_global_downloads(&self) -> Result<(), Error> {
        self.with_connection(|conn| {
            let metric_name = TORRENTS_DOWNLOADS_TOTAL;

            conn.execute(
                "UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = $1",
                &[&metric_name],
            )?;

            Ok(())
        })
    }

    fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        self.with_connection(|conn| {
            let rows = conn.query("SELECT key, valid_until FROM keys", &[])?;

            let keys: Vec<authentication::PeerKey> = rows
                .iter()
                .map(|row| {
                    let key: String = row.get(0);
                    let valid_until: Option<i64> = row.get(1);
                    match valid_until {
                        Some(valid_until) => authentication::PeerKey {
                            key: key.parse::<Key>().unwrap(),
                            valid_until: Some(Duration::from_secs(valid_until.unsigned_abs())),
                        },
                        None => authentication::PeerKey {
                            key: key.parse::<Key>().unwrap(),
                            valid_until: None,
                        },
                    }
                })
                .collect();

            Ok(keys)
        })
    }

    fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        self.with_connection(|conn| {
            let rows = conn.query("SELECT info_hash FROM whitelist", &[])?;

            let info_hashes: Vec<InfoHash> = rows
                .iter()
                .map(|row| {
                    let info_hash: String = row.get(0);
                    InfoHash::from_str(&info_hash).unwrap()
                })
                .collect();

            Ok(info_hashes)
        })
    }

    fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        let info_hash_hex = info_hash.to_hex_string();
        self.with_connection(move |conn| {
            let rows = conn.query("SELECT info_hash FROM whitelist WHERE info_hash = $1", &[&info_hash_hex])?;

            if let Some(row) = rows.first() {
                let info_hash_string: String = row.get(0);
                Ok(Some(
                    InfoHash::from_str(&info_hash_string).expect("Failed to decode InfoHash String from DB!"),
                ))
            } else {
                Ok(None)
            }
        })
    }

    fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let info_hash_str = info_hash.to_string();
        self.with_connection(move |conn| {
            let rows_affected = conn.execute("INSERT INTO whitelist (info_hash) VALUES ($1)", &[&info_hash_str])?;
            Ok(usize::try_from(rows_affected).expect("rows affected should fit in usize"))
        })
    }

    fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let info_hash_str = info_hash.to_string();
        self.with_connection(move |conn| {
            let rows_affected = conn.execute("DELETE FROM whitelist WHERE info_hash = $1", &[&info_hash_str])?;
            Ok(usize::try_from(rows_affected).expect("rows affected should fit in usize"))
        })
    }

    fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let key_str = key.to_string();
        self.with_connection(move |conn| {
            let rows = conn.query("SELECT key, valid_until FROM keys WHERE key = $1", &[&key_str])?;

            if let Some(row) = rows.first() {
                let key_str: String = row.get(0);
                let valid_until: Option<i64> = row.get(1);
                Ok(Some(match valid_until {
                    Some(valid_until) => authentication::PeerKey {
                        key: key_str.parse::<Key>().unwrap(),
                        valid_until: Some(Duration::from_secs(valid_until.unsigned_abs())),
                    },
                    None => authentication::PeerKey {
                        key: key_str.parse::<Key>().unwrap(),
                        valid_until: None,
                    },
                }))
            } else {
                Ok(None)
            }
        })
    }

    fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        let key_str = auth_key.key.to_string();
        let valid_until = auth_key.valid_until;
        self.with_connection(move |conn| {
            let rows_affected = if let Some(valid_until) = valid_until {
                let valid_until_i64 = i64::try_from(valid_until.as_secs()).unwrap();
                conn.execute(
                    "INSERT INTO keys (key, valid_until) VALUES ($1, $2)",
                    &[&key_str, &valid_until_i64],
                )?
            } else {
                let null_value: Option<i64> = None;
                conn.execute(
                    "INSERT INTO keys (key, valid_until) VALUES ($1, $2)",
                    &[&key_str, &null_value],
                )?
            };

            Ok(usize::try_from(rows_affected).expect("rows affected should fit in usize"))
        })
    }

    fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let key_str = key.to_string();
        self.with_connection(move |conn| {
            let rows_affected = conn.execute("DELETE FROM keys WHERE key = $1", &[&key_str])?;
            Ok(usize::try_from(rows_affected).expect("rows affected should fit in usize"))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use testcontainers::core::IntoContainerPort;
    /*
    We run a PostgreSQL container and run all the tests against the same container and database.

    Test for this driver are executed with:

    `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true cargo test`

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

    use super::Postgres;
    use crate::databases::driver::tests::run_tests;
    use crate::databases::Database;

    #[derive(Debug, Default)]
    struct StoppedPostgresContainer {}

    impl StoppedPostgresContainer {
        async fn run(
            self,
            config: &PostgresConfiguration,
        ) -> Result<RunningPostgresContainer, Box<dyn std::error::Error + 'static>> {
            let container = GenericImage::new("postgres", "16")
                .with_exposed_port(config.internal_port.tcp())
                .with_env_var("POSTGRES_PASSWORD", config.db_root_password.clone())
                .with_env_var("POSTGRES_DB", config.database.clone())
                .with_env_var("POSTGRES_USER", config.db_user.clone())
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

    impl Default for PostgresConfiguration {
        fn default() -> Self {
            Self {
                internal_port: 5432,
                database: "torrust_tracker_test".to_string(),
                db_user: "postgres".to_string(),
                db_root_password: "test".to_string(),
            }
        }
    }

    struct PostgresConfiguration {
        pub internal_port: u16,
        pub database: String,
        pub db_user: String,
        pub db_root_password: String,
    }

    fn core_configuration(host: &url::Host, port: u16, pg_configuration: &PostgresConfiguration) -> Core {
        let mut config = Core::default();

        let database = pg_configuration.database.clone();
        let db_user = pg_configuration.db_user.clone();
        let db_password = pg_configuration.db_root_password.clone();

        config.database.path = format!("postgresql://{db_user}:{db_password}@{host}:{port}/{database}");

        config
    }

    fn initialize_driver(config: &Core) -> Arc<Box<dyn Database>> {
        let driver: Arc<Box<dyn Database>> = Arc::new(Box::new(Postgres::new(&config.database.path).unwrap()));
        driver
    }

    /// Runs the full `PostgreSQL` driver test suite using testcontainers.
    ///
    /// Enable with:
    /// `TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true cargo test`
    #[tokio::test]
    async fn run_postgres_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        if std::env::var("TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST").is_err() {
            println!("Skipping the PostgreSQL driver tests (testcontainers).");
            return Ok(());
        }

        let pg_configuration = PostgresConfiguration::default();

        let stopped_pg_container = StoppedPostgresContainer::default();

        let pg_container = stopped_pg_container.run(&pg_configuration).await.unwrap();

        let host = pg_container.get_host().await;
        let port = pg_container.get_host_port_ipv4().await;

        let config = core_configuration(&host, port, &pg_configuration);

        let driver = initialize_driver(&config);

        run_tests(&driver).await;

        pg_container.stop().await;

        Ok(())
    }

    /// Runs the full `PostgreSQL` driver test suite against a local `PostgreSQL`
    /// instance specified via environment variable.
    ///
    /// Enable with:
    /// `TORRUST_TRACKER_CORE_POSTGRES_DATABASE_URL="postgresql://user:pass@host:port/db" cargo test`
    #[tokio::test]
    async fn run_postgres_driver_tests_local() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let Ok(db_url) = std::env::var("TORRUST_TRACKER_CORE_POSTGRES_DATABASE_URL") else {
            println!("Skipping the local PostgreSQL driver tests.");
            return Ok(());
        };

        let mut config = Core::default();
        config.database.path = db_url;

        let driver = initialize_driver(&config);

        run_tests(&driver).await;

        Ok(())
    }
}
