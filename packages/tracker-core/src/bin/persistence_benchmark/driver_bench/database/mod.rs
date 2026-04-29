use std::path::PathBuf;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use bittorrent_tracker_core::databases::driver::Driver;
use bittorrent_tracker_core::databases::setup::DatabaseStores;
use bittorrent_tracker_core::databases::SchemaMigrator;
use testcontainers::{ContainerAsync, GenericImage};

mod mysql;
mod sqlite;

pub(super) struct ActiveDatabase {
    pub(super) database: Option<DatabaseStores>,
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

pub(super) async fn reset_database(schema_migrator: &dyn SchemaMigrator) -> Result<()> {
    create_database_tables_with_retry(schema_migrator).await?;
    schema_migrator
        .drop_database_tables()
        .await
        .context("failed to drop benchmark database tables")?;
    create_database_tables_with_retry(schema_migrator).await
}

/// Retries table creation until the database is ready.
///
/// This primarily shields `MySQL` startup latency where the process may be up
/// before it is ready to accept migrations/queries.
///
/// # Errors
///
/// Returns an error if the database is still not ready after all retries.
async fn create_database_tables_with_retry(schema_migrator: &dyn SchemaMigrator) -> Result<()> {
    let mut last_error: Option<anyhow::Error> = None;

    for _ in 0..5 {
        match schema_migrator.create_database_tables().await {
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
