//! The `MySQL` database driver.
//!
//! This module provides an implementation of the [`Database`] trait for `MySQL`
//! using the `r2d2_mysql` connection pool. It configures the MySQL connection
//! based on a URL, creates the necessary tables (for torrent metrics, torrent
//! whitelist, and authentication keys), and implements all CRUD operations
//! required by the persistence layer.
use std::str::FromStr;
use std::time::Duration;

use bittorrent_primitives::info_hash::InfoHash;
use r2d2::Pool;
use r2d2_mysql::mysql::prelude::Queryable;
use r2d2_mysql::mysql::{params, Opts, OptsBuilder};
use r2d2_mysql::MySqlConnectionManager;
use torrust_tracker_primitives::{NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{Database, Driver, Error, TORRENTS_DOWNLOADS_TOTAL};
use crate::authentication::key::AUTH_KEY_LENGTH;
use crate::authentication::{self, Key};

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
    /// Refer to [`databases::Database::new`](crate::core::databases::Database::new).
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
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let query = conn.exec_first::<u32, _, _>(
            "SELECT value FROM torrent_aggregate_metrics WHERE metric_name = :metric_name",
            params! { "metric_name" => metric_name },
        );

        let persistent_torrent = query?;

        Ok(persistent_torrent)
    }

    fn save_torrent_aggregate_metric(&self, metric_name: &str, completed: NumberOfDownloads) -> Result<(), Error> {
        const COMMAND : &str = "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (:metric_name, :completed) ON DUPLICATE KEY UPDATE value = VALUES(value)";

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        Ok(conn.exec_drop(COMMAND, params! { metric_name, completed })?)
    }
}

impl Database for Mysql {
    /// Refer to [`databases::Database::create_database_tables`](crate::core::databases::Database::create_database_tables).
    fn create_database_tables(&self) -> Result<(), Error> {
        let create_whitelist_table = "
        CREATE TABLE IF NOT EXISTS whitelist (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash VARCHAR(40) NOT NULL UNIQUE
        );"
        .to_string();

        let create_torrents_table = "
        CREATE TABLE IF NOT EXISTS torrents (
            id integer PRIMARY KEY AUTO_INCREMENT,
            info_hash VARCHAR(40) NOT NULL UNIQUE,
            completed INTEGER DEFAULT 0 NOT NULL
        );"
        .to_string();

        let create_torrent_aggregate_metrics_table = "
        CREATE TABLE IF NOT EXISTS torrent_aggregate_metrics (
            id integer PRIMARY KEY AUTO_INCREMENT,
            metric_name VARCHAR(50) NOT NULL UNIQUE,
            value INTEGER DEFAULT 0 NOT NULL
        );"
        .to_string();

        let create_keys_table = format!(
            "
        CREATE TABLE IF NOT EXISTS `keys` (
          `id` INT NOT NULL AUTO_INCREMENT,
          `key` VARCHAR({}) NOT NULL,
          `valid_until` INT(10),
          PRIMARY KEY (`id`),
          UNIQUE (`key`)
        );",
            i8::try_from(AUTH_KEY_LENGTH).expect("authentication key length should fit within a i8!")
        );

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.query_drop(&create_torrents_table)
            .expect("Could not create torrents table.");
        conn.query_drop(&create_torrent_aggregate_metrics_table)
            .expect("Could not create create_torrent_aggregate_metrics_table table.");
        conn.query_drop(&create_keys_table).expect("Could not create keys table.");
        conn.query_drop(&create_whitelist_table)
            .expect("Could not create whitelist table.");

        Ok(())
    }

    /// Refer to [`databases::Database::drop_database_tables`](crate::core::databases::Database::drop_database_tables).
    fn drop_database_tables(&self) -> Result<(), Error> {
        let drop_whitelist_table = "
        DROP TABLE `whitelist`;"
            .to_string();

        let drop_torrents_table = "
        DROP TABLE `torrents`;"
            .to_string();

        let drop_keys_table = "
            DROP TABLE `keys`;"
            .to_string();

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.query_drop(&drop_whitelist_table)
            .expect("Could not drop `whitelist` table.");
        conn.query_drop(&drop_torrents_table)
            .expect("Could not drop `torrents` table.");
        conn.query_drop(&drop_keys_table).expect("Could not drop `keys` table.");

        Ok(())
    }

    /// Refer to [`databases::Database::load_persistent_torrents`](crate::core::databases::Database::load_persistent_torrents).
    fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let torrents = conn.query_map(
            "SELECT info_hash, completed FROM torrents",
            |(info_hash_string, completed): (String, u32)| {
                let info_hash = InfoHash::from_str(&info_hash_string).unwrap();
                (info_hash, completed)
            },
        )?;

        Ok(torrents.iter().copied().collect())
    }

    /// Refer to [`databases::Database::load_persistent_torrent`](crate::core::databases::Database::load_persistent_torrent).
    fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let query = conn.exec_first::<u32, _, _>(
            "SELECT completed FROM torrents WHERE info_hash = :info_hash",
            params! { "info_hash" => info_hash.to_hex_string() },
        );

        let persistent_torrent = query?;

        Ok(persistent_torrent)
    }

    /// Refer to [`databases::Database::save_persistent_torrent`](crate::core::databases::Database::save_persistent_torrent).
    fn save_torrent_downloads(&self, info_hash: &InfoHash, completed: u32) -> Result<(), Error> {
        const COMMAND : &str = "INSERT INTO torrents (info_hash, completed) VALUES (:info_hash_str, :completed) ON DUPLICATE KEY UPDATE completed = VALUES(completed)";

        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash_str = info_hash.to_string();

        Ok(conn.exec_drop(COMMAND, params! { info_hash_str, completed })?)
    }

    /// Refer to [`databases::Database::increase_number_of_downloads`](crate::core::databases::Database::increase_number_of_downloads).
    fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash_str = info_hash.to_string();

        conn.exec_drop(
            "UPDATE torrents SET completed = completed + 1 WHERE info_hash = :info_hash_str",
            params! { info_hash_str },
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
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let metric_name = TORRENTS_DOWNLOADS_TOTAL;

        conn.exec_drop(
            "UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = :metric_name",
            params! { metric_name },
        )?;

        Ok(())
    }

    /// Refer to [`databases::Database::load_keys`](crate::core::databases::Database::load_keys).
    fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let keys = conn.query_map(
            "SELECT `key`, valid_until FROM `keys`",
            |(key, valid_until): (String, Option<i64>)| match valid_until {
                Some(valid_until) => authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: Some(Duration::from_secs(valid_until.unsigned_abs())),
                },
                None => authentication::PeerKey {
                    key: key.parse::<Key>().unwrap(),
                    valid_until: None,
                },
            },
        )?;

        Ok(keys)
    }

    /// Refer to [`databases::Database::load_whitelist`](crate::core::databases::Database::load_whitelist).
    fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hashes = conn.query_map("SELECT info_hash FROM whitelist", |info_hash: String| {
            InfoHash::from_str(&info_hash).unwrap()
        })?;

        Ok(info_hashes)
    }

    /// Refer to [`databases::Database::get_info_hash_from_whitelist`](crate::core::databases::Database::get_info_hash_from_whitelist).
    fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let select = conn.exec_first::<String, _, _>(
            "SELECT info_hash FROM whitelist WHERE info_hash = :info_hash",
            params! { "info_hash" => info_hash.to_hex_string() },
        )?;

        let info_hash = select.map(|f| InfoHash::from_str(&f).expect("Failed to decode InfoHash String from DB!"));

        Ok(info_hash)
    }

    /// Refer to [`databases::Database::add_info_hash_to_whitelist`](crate::core::databases::Database::add_info_hash_to_whitelist).
    fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash_str = info_hash.to_string();

        conn.exec_drop(
            "INSERT INTO whitelist (info_hash) VALUES (:info_hash_str)",
            params! { info_hash_str },
        )?;

        Ok(1)
    }

    /// Refer to [`databases::Database::remove_info_hash_from_whitelist`](crate::core::databases::Database::remove_info_hash_from_whitelist).
    fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let info_hash = info_hash.to_string();

        conn.exec_drop("DELETE FROM whitelist WHERE info_hash = :info_hash", params! { info_hash })?;

        Ok(1)
    }

    /// Refer to [`databases::Database::get_key_from_keys`](crate::core::databases::Database::get_key_from_keys).
    fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        let query = conn.exec_first::<(String, Option<i64>), _, _>(
            "SELECT `key`, valid_until FROM `keys` WHERE `key` = :key",
            params! { "key" => key.to_string() },
        );

        let key = query?;

        Ok(key.map(|(key, opt_valid_until)| match opt_valid_until {
            Some(valid_until) => authentication::PeerKey {
                key: key.parse::<Key>().unwrap(),
                valid_until: Some(Duration::from_secs(valid_until.unsigned_abs())),
            },
            None => authentication::PeerKey {
                key: key.parse::<Key>().unwrap(),
                valid_until: None,
            },
        }))
    }

    /// Refer to [`databases::Database::add_key_to_keys`](crate::core::databases::Database::add_key_to_keys).
    fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        match auth_key.valid_until {
            Some(valid_until) => conn.exec_drop(
                "INSERT INTO `keys` (`key`, valid_until) VALUES (:key, :valid_until)",
                params! { "key" => auth_key.key.to_string(), "valid_until" => valid_until.as_secs().to_string() },
            )?,
            None => conn.exec_drop(
                "INSERT INTO `keys` (`key`) VALUES (:key)",
                params! { "key" => auth_key.key.to_string() },
            )?,
        }

        Ok(1)
    }

    /// Refer to [`databases::Database::remove_key_from_keys`](crate::core::databases::Database::remove_key_from_keys).
    fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        let mut conn = self.pool.get().map_err(|e| (e, DRIVER))?;

        conn.exec_drop("DELETE FROM `keys` WHERE `key` = :key", params! { "key" => key.to_string() })?;

        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use testcontainers::core::IntoContainerPort;
    /*
    We run a MySQL container and run all the tests against the same container and database.

    Test for this driver are executed with:

    `TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test`

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
    use crate::databases::Database;

    #[derive(Debug, Default)]
    struct StoppedMysqlContainer {}

    impl StoppedMysqlContainer {
        async fn run(self, config: &MysqlConfiguration) -> Result<RunningMysqlContainer, Box<dyn std::error::Error + 'static>> {
            let container = GenericImage::new("mysql", "8.0")
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
