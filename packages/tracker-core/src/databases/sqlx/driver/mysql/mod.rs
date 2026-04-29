#![allow(dead_code)]

use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

use ::sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use ::sqlx::{MySqlPool, Row};
use tokio::sync::Mutex;
use torrust_tracker_primitives::NumberOfDownloads;

use crate::databases::driver::Driver;
use crate::databases::error::Error;
use crate::databases::sqlx::traits::AsyncSchemaMigrator;

mod auth_key_store;
mod schema_migrator;
mod torrent_metrics_store;
mod whitelist_store;

const DRIVER: Driver = Driver::MySQL;

pub(crate) struct MysqlSqlx {
    pool: MySqlPool,
    schema_ready: AtomicBool,
    schema_lock: Mutex<()>,
}

impl MysqlSqlx {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let options = MySqlConnectOptions::from_str(db_path).map_err(|e| (e, DRIVER))?;

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

        self.create_database_tables().await?;
        self.schema_ready.store(true, Ordering::Release);

        Ok(())
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
        let insert = ::sqlx::query(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (?, ?) ON DUPLICATE KEY UPDATE value = VALUES(value)",
        )
        .bind(metric_name)
        .bind(i64::from(completed))
        .execute(&self.pool)
        .await
        .map_err(|e| (e, DRIVER))?
        .rows_affected();

        if insert == 0 {
            Err(Error::InsertFailed {
                location: std::panic::Location::caller(),
                driver: DRIVER,
            })
        } else {
            Ok(())
        }
    }
}

#[cfg(all(test, feature = "db-compatibility-tests"))]
mod tests {
    use std::sync::Arc;

    use testcontainers::core::IntoContainerPort;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::{ContainerAsync, GenericImage, ImageExt};
    use torrust_tracker_configuration::Core;

    use super::MysqlSqlx;
    use crate::databases::sqlx::driver::tests::run_tests;
    use crate::databases::sqlx::traits::AsyncDatabase;

    #[derive(Debug, Default)]
    struct StoppedMysqlContainer {}

    impl StoppedMysqlContainer {
        async fn run(self, config: &MysqlConfiguration) -> Result<RunningMysqlContainer, Box<dyn std::error::Error + 'static>> {
            let image_tag = std::env::var("TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG").unwrap_or_else(|_| "8.0".to_string());

            let container = GenericImage::new("mysql", image_tag.as_str())
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

        let database = mysql_configuration.database.clone();
        let db_user = mysql_configuration.db_user.clone();
        let db_password = mysql_configuration.db_root_password.clone();

        config.database.path = format!("mysql://{db_user}:{db_password}@{host}:{port}/{database}");

        config
    }

    fn initialize_driver(config: &Core) -> Arc<Box<dyn AsyncDatabase>> {
        Arc::new(Box::new(MysqlSqlx::new(&config.database.path).unwrap()))
    }

    #[tokio::test]
    async fn run_mysql_sqlx_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        if std::env::var("TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST").is_err() {
            println!("Skipping the MySQL sqlx driver tests.");
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
