#![allow(dead_code)]

use std::str::FromStr;

use ::sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use ::sqlx::{Row, SqlitePool};
use torrust_tracker_primitives::NumberOfDownloads;

use crate::databases::driver::Driver;
use crate::databases::error::Error;

mod auth_key_store;
mod schema_migrator;
mod torrent_metrics_store;
mod whitelist_store;

const DRIVER: Driver = Driver::Sqlite3;

pub(crate) struct SqliteSqlx {
    pool: SqlitePool,
}

impl SqliteSqlx {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let options = SqliteConnectOptions::from_str(&format!("sqlite://{db_path}"))
            .map_err(|e| (e, DRIVER))?
            .create_if_missing(true);

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
        let insert = ::sqlx::query(
            "INSERT INTO torrent_aggregate_metrics (metric_name, value) VALUES (?1, ?2) ON CONFLICT(metric_name) DO UPDATE SET value = ?2",
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_configuration::Core;
    use torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database;

    use super::SqliteSqlx;
    use crate::databases::sqlx::driver::tests::run_tests;
    use crate::databases::sqlx::traits::AsyncDatabase;

    fn ephemeral_configuration() -> Core {
        let mut config = Core::default();
        let temp_file = ephemeral_sqlite_database();
        temp_file.to_str().unwrap().clone_into(&mut config.database.path);
        config
    }

    fn initialize_driver(config: &Core) -> Arc<Box<dyn AsyncDatabase>> {
        Arc::new(Box::new(SqliteSqlx::new(&config.database.path).unwrap()))
    }

    #[tokio::test]
    async fn run_sqlite_sqlx_driver_tests() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let config = ephemeral_configuration();

        let driver = initialize_driver(&config);

        run_tests(&driver).await;

        Ok(())
    }
}
