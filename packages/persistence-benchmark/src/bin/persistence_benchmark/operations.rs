use anyhow::Result;
use torrust_tracker_primitives::Driver;

use super::types::{DbVersion, OpsCount};
use super::{driver_bench, metrics};

/// Collects benchmark operation samples and computes aggregate statistics.
///
/// # Errors
///
/// Returns an error if operation sampling or metrics computation fails.
pub async fn collect_operation_stats(
    driver: &Driver,
    db_version: &DbVersion,
    ops: OpsCount,
) -> Result<Vec<metrics::OperationStats>> {
    let raw_operations = driver_bench::run(driver.clone(), db_version.as_str(), ops).await?;

    metrics::compute(raw_operations)
}
