use std::time::Duration;

use anyhow::Result;
use torrust_tracker_core::databases::driver::Driver;

use super::types::OpsCount;

mod database;
mod operations;
mod sampling;

#[derive(Debug)]
pub struct RawOperationSamples {
    pub name: String,
    pub samples: Vec<Duration>,
}

/// Runs all persistence operation benchmarks for one driver/version pair.
///
/// # Errors
///
/// Returns an error if database setup fails or any benchmarked database
/// operation fails.
pub async fn run(driver: Driver, db_version: &str, ops: OpsCount) -> Result<Vec<RawOperationSamples>> {
    let active_database = database::ActiveDatabase::new(driver, db_version).await?;
    let stores = active_database.database.as_ref().unwrap();
    database::reset_database(&*stores.schema_migrator).await?;

    let ops = ops.get();

    let mut operations_samples = Vec::new();
    operations::benchmark_torrent_operations(&*stores.torrent_metrics_store, ops, &mut operations_samples).await?;
    operations::benchmark_whitelist_operations(&*stores.whitelist_store, ops, &mut operations_samples).await?;
    operations::benchmark_key_operations(&*stores.auth_key_store, ops, &mut operations_samples).await?;

    Ok(operations_samples)
}
