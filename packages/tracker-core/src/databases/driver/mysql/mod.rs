//! The `MySQL` database driver.
//!
//! This module provides implementations of the four narrow database traits
//! ([`SchemaMigrator`](crate::databases::SchemaMigrator),
//! [`TorrentMetricsStore`](crate::databases::TorrentMetricsStore),
//! [`WhitelistStore`](crate::databases::WhitelistStore),
//! [`AuthKeyStore`](crate::databases::AuthKeyStore)
//! for `MySQL` using the `r2d2_mysql` connection pool. It configures the `MySQL`
//! connection based on a URL, creates the necessary tables (for torrent metrics,
//! torrent whitelist, and authentication keys), and implements all CRUD
//! operations required by the persistence layer.
use r2d2::Pool;
use r2d2_mysql::mysql::{Opts, OptsBuilder};
use r2d2_mysql::MySqlConnectionManager;
use torrust_tracker_primitives::NumberOfDownloads;

use super::{Driver, Error};

mod auth_key_store;
mod schema_migrator;
mod torrent_metrics_store;
mod whitelist_store;

const DRIVER: Driver = Driver::MySQL;

/// `MySQL` driver implementation.
///
/// This struct encapsulates a connection pool for `MySQL`, built using the
/// `r2d2_mysql` connection manager. It implements the [`Database`] trait to
/// provide persistence operations.
pub(crate) struct Mysql {
    pool: Pool<MySqlConnectionManager>,
}

impl Mysql {
    /// It instantiates a new `MySQL` database driver.
    ///
    ///
    /// # Errors
    ///
    /// Will return `r2d2::Error` if `db_path` is not able to create `MySQL` database.
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let opts = Opts::from_url(db_path)?;
        let builder = OptsBuilder::from_opts(opts);
        let manager = MySqlConnectionManager::new(builder);
        let pool = r2d2::Pool::builder().build(manager).map_err(|e| (e, DRIVER))?;

        Ok(Self { pool })
    }

    fn load_torrent_aggregate_metric(&self, metric_name: &str) -> Result<Option<NumberOfDownloads>, Error> {
        use r2d2_mysql::mysql::params;
        use r2d2_mysql::mysql::prelude::Queryable;

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let query = conn.exec_first::<u32, _, _>(
            "SELECT value FROM torrent_aggregate_metrics WHERE metric_name = :metric_name",
            params! { "metric_name" => metric_name },
        );

        let persistent_torrent = query?;

        Ok(persistent_torrent)
    }

    fn save_torrent_aggregate_metric(&self, metric_name: &str, completed: NumberOfDownloads) -> Result<(), Error> {
        use r2d2_mysql::mysql::params;
        use r2d2_mysql::mysql::prelude::Queryable;

        const COMMAND : &str = "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (:metric_name, :completed) ON DUPLICATE KEY UPDATE value = VALUES(value)";

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        Ok(conn.exec_drop(COMMAND, params! { metric_name, completed })?)
    }
}

#[cfg(all(test, feature = "db-compatibility-tests"))]
mod tests {
    use std::sync::Arc;

    use testcontainers::core::IntoContainerPort;
    /*
    We run a MySQL container and run all the tests against the same container and database.

    Test for this driver are executed with:

    `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true \
     cargo test -p bittorrent-tracker-core --features db-compatibility-tests run_mysql_driver_tests`

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

    #[derive(Debug, Default)]
    struct StoppedMysqlContainer {}

    impl StoppedMysqlContainer {
        async fn run(self, config: &MysqlConfiguration) -> Result<RunningMysqlContainer, Box<dyn std::error::Error + 'static>> {
            let image_tag = std::env::var("TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG").unwrap_or_else(|_| "8.0".to_string());

            let container = GenericImage::new("mysql", image_tag.as_str())
                .with_exposed_port(config.internal_port.tcp())
                // todo: this does not work
                //.with_wait_for(WaitFor::message_on_stdout("ready for connections"))
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
        let driver: Arc<Box<dyn Database>> = Arc::new(Box::new(Mysql::new(&config.database.path).unwrap()));
        driver
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

        mysql_container.stop().await;

        Ok(())
    }
}
