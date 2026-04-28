use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use bittorrent_tracker_core::databases::driver::Driver;
use bittorrent_tracker_core::databases::Database;
use testcontainers::{ContainerAsync, GenericImage};

mod mysql;
mod sqlite;

pub(super) struct ActiveDatabase {
    pub(super) database: Option<Arc<Box<dyn Database>>>,
    resource: Option<BenchmarkResource>,
}

enum BenchmarkResource {
    Sqlite(PathBuf),
    Mysql(Box<ContainerAsync<GenericImage>>),
}

impl ActiveDatabase {
    /// Creates an initialized benchmark database for the selected driver.
    ///
    /// For `sqlite3`, this creates a unique temporary database file.
    /// For `mysql`, this starts a temporary container and builds a connection
    /// URL from mapped host/port details.
    ///
    /// # Errors
    ///
    /// Returns an error if the `MySQL` container cannot be started or queried for
    /// connection details.
    pub(super) async fn new(driver: Driver, db_version: &str) -> Result<Self> {
        match driver {
            Driver::Sqlite3 => Ok(sqlite::initialize()),
            Driver::MySQL => mysql::initialize(db_version).await,
        }
    }
}

impl Drop for ActiveDatabase {
    fn drop(&mut self) {
        // Drop the database connection before cleaning up the resource.
        // For SQLite this ensures the file handle is released before removal.
        drop(self.database.take());
        match self.resource.take() {
            Some(BenchmarkResource::Sqlite(path)) => {
                let _removed_file_result = std::fs::remove_file(path);
            }
            Some(BenchmarkResource::Mysql(container)) => {
                drop(container);
            }
            None => {}
        }
    }
}

pub(super) async fn reset_database(database: &dyn Database) -> Result<()> {
    create_database_tables_with_retry(database).await?;
    database
        .drop_database_tables()
        .context("failed to drop benchmark database tables")?;
    create_database_tables_with_retry(database).await
}

/// Retries table creation until the database is ready.
///
/// This primarily shields `MySQL` startup latency where the process may be up
/// before it is ready to accept migrations/queries.
///
/// # Errors
///
/// Returns an error if the database is still not ready after all retries.
async fn create_database_tables_with_retry(database: &dyn Database) -> Result<()> {
    let mut last_error: Option<anyhow::Error> = None;

    for _ in 0..5 {
        match database.create_database_tables() {
            Ok(()) => return Ok(()),
            Err(error) => {
                last_error = Some(error.into());
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    match last_error {
        Some(error) => Err(anyhow!("database is not ready after retries; last error: {error}")),
        None => Err(anyhow!("database is not ready after retries")),
    }
}
