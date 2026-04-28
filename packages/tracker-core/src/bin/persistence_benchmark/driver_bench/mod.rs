use std::time::Duration;

use anyhow::Result;
use bittorrent_tracker_core::databases::driver::Driver;

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
    database::reset_database(active_database.database.as_ref().as_ref()).await?;

    let ops = ops.get();

    let mut operations_samples = Vec::new();
    operations::benchmark_torrent_operations(active_database.database.as_ref().as_ref(), ops, &mut operations_samples)?;
    operations::benchmark_whitelist_operations(active_database.database.as_ref().as_ref(), ops, &mut operations_samples)?;
    operations::benchmark_key_operations(active_database.database.as_ref().as_ref(), ops, &mut operations_samples)?;

    Ok(operations_samples)
}
