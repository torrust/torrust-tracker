//! The `MySQL` database driver.
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

use async_trait::async_trait;
use bittorrent_primitives::info_hash::InfoHash;
use sqlx::migrate::Migrator;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{ConnectOptions, MySqlPool, Row};
use tokio::sync::Mutex;
use torrust_tracker_primitives::{DurationSinceUnixEpoch, NumberOfDownloads, NumberOfDownloadsBTreeMap};

use super::{Driver, TORRENTS_DOWNLOADS_TOTAL};
use crate::authentication::{self, Key};
use crate::databases::error::Error;
use crate::databases::{AuthKeyStore, SchemaMigrator, TorrentMetricsStore, WhitelistStore};

const DRIVER: Driver = Driver::MySQL;
static MIGRATOR: Migrator = sqlx::migrate!("migrations/mysql");

/// `MySQL` driver implementation backed by `sqlx`.
pub(crate) struct Mysql {
    pool: MySqlPool,
    schema_ready: AtomicBool,
    schema_lock: Mutex<()>,
}

impl Mysql {
    /// Instantiates a new `MySQL` database driver.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the database URL cannot be parsed.
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let options = MySqlConnectOptions::from_str(db_path)
            .map_err(|err| Error::connection_error(DRIVER, err))?
            .disable_statement_logging();

        let pool = MySqlPoolOptions::new().connect_lazy_with(options);

        Ok(Self {
            pool,
            schema_ready: AtomicBool::new(false),
            schema_lock: Mutex::new(()),
        })
    }

    async fn ensure_schema(&self) -> Result<(), Error> {
        if self.schema_ready.load(Ordering::Acquire) {
            return Ok(());
        }

        let _guard = self.schema_lock.lock().await;

        if self.schema_ready.load(Ordering::Acquire) {
            return Ok(());
        }

        self.run_migrations().await
    }

    async fn run_migrations(&self) -> Result<(), Error> {
        MIGRATOR
            .run(&self.pool)
            .await
            .map_err(|err| Error::migration_error(DRIVER, err))?;

        self.schema_ready.store(true, Ordering::Release);

        Ok(())
    }

    fn decode_counter_i64(&self, value: i64) -> Result<NumberOfDownloads, Error> {
        u64::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
    }

    fn encode_counter(&self, value: NumberOfDownloads) -> Result<i64, Error> {
        i64::try_from(value).map_err(|err| Error::invalid_query(DRIVER, err))
    }

    fn decode_info_hash(&self, value: String) -> Result<InfoHash, Error> {
        InfoHash::from_str(&value).map_err(|err| {
            Error::invalid_query(
                DRIVER,
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{err:?}")),
            )
        })
    }

    fn decode_key(&self, value: String) -> Result<Key, Error> {
        value.parse::<Key>().map_err(|err| Error::invalid_query(DRIVER, err))
    }

    fn decode_valid_until(&self, value: Option<i64>) -> Result<Option<DurationSinceUnixEpoch>, Error> {
        value
            .map(|seconds| {
                u64::try_from(seconds)
                    .map(DurationSinceUnixEpoch::from_secs)
                    .map_err(|err| Error::invalid_query(DRIVER, err))
            })
            .transpose()
    }
}

#[async_trait]
impl SchemaMigrator for Mysql {
    async fn create_database_tables(&self) -> Result<(), Error> {
        self.run_migrations().await
    }

    async fn drop_database_tables(&self) -> Result<(), Error> {
        sqlx::query("DROP TABLE IF EXISTS whitelist")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS torrent_aggregate_metrics")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS torrents")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS `keys`")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;
        sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations")
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        Ok(())
    }
}

#[async_trait]
impl TorrentMetricsStore for Mysql {
    async fn load_all_torrents_downloads(&self) -> Result<NumberOfDownloadsBTreeMap, Error> {
        self.ensure_schema().await?;

        let rows = sqlx::query("SELECT info_hash, completed FROM torrents")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        let mut torrents = NumberOfDownloadsBTreeMap::new();

        for row in rows {
            let info_hash_string: String = row.try_get("info_hash").map_err(|err| (err, DRIVER))?;
            let completed: i64 = row.try_get("completed").map_err(|err| (err, DRIVER))?;

            torrents.insert(self.decode_info_hash(info_hash_string)?, self.decode_counter_i64(completed)?);
        }

        Ok(torrents)
    }

    async fn load_torrent_downloads(&self, info_hash: &InfoHash) -> Result<Option<NumberOfDownloads>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT completed FROM torrents WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let completed: i64 = row.try_get("completed").map_err(|err| (err, DRIVER))?;
            self.decode_counter_i64(completed)
        })
        .transpose()
    }

    async fn save_torrent_downloads(&self, info_hash: &InfoHash, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.ensure_schema().await?;

        let encoded_downloaded = self.encode_counter(downloaded)?;

        let insert = sqlx::query(
            "INSERT INTO torrents (info_hash, completed) VALUES (?, ?) ON DUPLICATE KEY UPDATE completed = VALUES(completed)",
        )
        .bind(info_hash.to_hex_string())
        .bind(encoded_downloaded)
        .execute(&self.pool)
        .await
        .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }

    async fn increase_downloads_for_torrent(&self, info_hash: &InfoHash) -> Result<(), Error> {
        self.ensure_schema().await?;

        sqlx::query("UPDATE torrents SET completed = completed + 1 WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        Ok(())
    }

    async fn load_global_downloads(&self) -> Result<Option<NumberOfDownloads>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT value FROM torrent_aggregate_metrics WHERE metric_name = ?")
            .bind(TORRENTS_DOWNLOADS_TOTAL)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let value: i64 = row.try_get("value").map_err(|err| (err, DRIVER))?;
            self.decode_counter_i64(value)
        })
        .transpose()
    }

    async fn save_global_downloads(&self, downloaded: NumberOfDownloads) -> Result<(), Error> {
        self.ensure_schema().await?;

        let encoded_downloaded = self.encode_counter(downloaded)?;

        let insert = sqlx::query(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (?, ?) ON DUPLICATE KEY UPDATE value = VALUES(value)",
        )
        .bind(TORRENTS_DOWNLOADS_TOTAL)
        .bind(encoded_downloaded)
        .execute(&self.pool)
        .await
        .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }

    async fn increase_global_downloads(&self) -> Result<(), Error> {
        self.ensure_schema().await?;

        sqlx::query("UPDATE torrent_aggregate_metrics SET value = value + 1 WHERE metric_name = ?")
            .bind(TORRENTS_DOWNLOADS_TOTAL)
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        Ok(())
    }
}

#[async_trait]
impl WhitelistStore for Mysql {
    async fn load_whitelist(&self) -> Result<Vec<InfoHash>, Error> {
        self.ensure_schema().await?;

        let rows = sqlx::query("SELECT info_hash FROM whitelist")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let info_hash: String = row.try_get("info_hash").map_err(|err| (err, DRIVER))?;
                self.decode_info_hash(info_hash)
            })
            .collect()
    }

    async fn get_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<Option<InfoHash>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT info_hash FROM whitelist WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let value: String = row.try_get("info_hash").map_err(|err| (err, DRIVER))?;
            self.decode_info_hash(value)
        })
        .transpose()
    }

    async fn add_info_hash_to_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let insert = sqlx::query("INSERT INTO whitelist (info_hash) VALUES (?)")
            .bind(info_hash.to_hex_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert.rows_affected() as usize)
        }
    }

    async fn remove_info_hash_from_whitelist(&self, info_hash: InfoHash) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let deleted = sqlx::query("DELETE FROM whitelist WHERE info_hash = ?")
            .bind(info_hash.to_hex_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        let deleted = deleted.rows_affected() as usize;

        if deleted == 1 {
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: std::panic::Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }
}

#[async_trait]
impl AuthKeyStore for Mysql {
    async fn load_keys(&self) -> Result<Vec<authentication::PeerKey>, Error> {
        self.ensure_schema().await?;

        let rows = sqlx::query("SELECT `key`, valid_until FROM `keys`")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        rows.into_iter()
            .map(|row| {
                let key: String = row.try_get("key").map_err(|err| (err, DRIVER))?;
                let valid_until: Option<i64> = row.try_get("valid_until").map_err(|err| (err, DRIVER))?;

                Ok(authentication::PeerKey {
                    key: self.decode_key(key)?,
                    valid_until: self.decode_valid_until(valid_until)?,
                })
            })
            .collect()
    }

    async fn get_key_from_keys(&self, key: &Key) -> Result<Option<authentication::PeerKey>, Error> {
        self.ensure_schema().await?;

        let row = sqlx::query("SELECT `key`, valid_until FROM `keys` WHERE `key` = ?")
            .bind(key.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        row.map(|row| {
            let key: String = row.try_get("key").map_err(|err| (err, DRIVER))?;
            let valid_until: Option<i64> = row.try_get("valid_until").map_err(|err| (err, DRIVER))?;

            Ok(authentication::PeerKey {
                key: self.decode_key(key)?,
                valid_until: self.decode_valid_until(valid_until)?,
            })
        })
        .transpose()
    }

    async fn add_key_to_keys(&self, auth_key: &authentication::PeerKey) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let valid_until = auth_key
            .valid_until
            .map(|valid_until| valid_until.as_secs())
            .map(i64::try_from)
            .transpose()
            .map_err(|err| Error::invalid_query(DRIVER, err))?;

        let insert = sqlx::query("INSERT INTO `keys` (`key`, valid_until) VALUES (?, ?)")
            .bind(auth_key.key.to_string())
            .bind(valid_until)
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        if insert.rows_affected() == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(insert.rows_affected() as usize)
        }
    }

    async fn remove_key_from_keys(&self, key: &Key) -> Result<usize, Error> {
        self.ensure_schema().await?;

        let deleted = sqlx::query("DELETE FROM `keys` WHERE `key` = ?")
            .bind(key.to_string())
            .execute(&self.pool)
            .await
            .map_err(|err| (err, DRIVER))?;

        let deleted = deleted.rows_affected() as usize;

        if deleted == 1 {
            Ok(deleted)
        } else {
            Err(Error::DeleteFailed {
                location: std::panic::Location::caller(),
                error_code: deleted,
                driver: DRIVER,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use testcontainers::core::IntoContainerPort;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::{ContainerAsync, GenericImage, ImageExt};
    use torrust_tracker_configuration::Core;

    use crate::databases::driver::build;
    use crate::databases::driver::tests::run_tests;
    use crate::databases::driver::Driver;

    #[derive(Debug, Default)]
    struct StoppedMysqlContainer {}

    impl StoppedMysqlContainer {
        async fn run(self, config: &MysqlConfiguration) -> Result<RunningMysqlContainer, Box<dyn std::error::Error + 'static>> {
            let image_name = std::env::var("TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE").unwrap_or_else(|_| "mysql".to_string());
            let image_tag = std::env::var("TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG").unwrap_or_else(|_| "8.0".to_string());

            let container = GenericImage::new(image_name, image_tag)
                .with_exposed_port(config.internal_port.tcp())
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

        config.database.driver = torrust_tracker_configuration::Driver::MySQL;
        config.database.path = format!(
            "mysql://{}:{}@{}:{}/{}",
            mysql_configuration.db_user,
            mysql_configuration.db_root_password,
            host,
            port,
            mysql_configuration.database
        );

        config
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
        let driver = build(&Driver::MySQL, &config.database.path)?;

        run_tests(&driver).await;

        mysql_container.stop().await;

        Ok(())
    }
}
