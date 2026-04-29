use anyhow::{Context, Result};
use bittorrent_tracker_core::databases::WhitelistStore;

use super::super::sampling::{info_hash_from_index, measure_operation_async};
use super::super::RawOperationSamples;

/// Benchmarks whitelist-related persistence operations.
///
/// # Errors
///
/// Returns an error if any setup or measured database operation fails.
pub(super) async fn benchmark_whitelist_operations(
    database: &dyn WhitelistStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(
        measure_operation_async(
            "add_info_hash_to_whitelist",
            ops,
            |index| async move { info_hash_from_index(30_000 + index) },
            |info_hash| async move {
                let _added_rows = database
                    .add_info_hash_to_whitelist(info_hash)
                    .await
                    .context("add_info_hash_to_whitelist failed")?;
                Ok(())
            },
        )
        .await?,
    );

    let whitelisted_info_hash = info_hash_from_index(40_000)?;
    let _added_rows = database
        .add_info_hash_to_whitelist(whitelisted_info_hash)
        .await
        .context("failed to seed get_info_hash_from_whitelist")?;
    operations.push(
        measure_operation_async(
            "get_info_hash_from_whitelist",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                let _info_hash_result = database
                    .get_info_hash_from_whitelist(whitelisted_info_hash)
                    .await
                    .context("get_info_hash_from_whitelist failed")?;
                Ok(())
            },
        )
        .await?,
    );

    operations.push(
        measure_operation_async(
            "load_whitelist",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                let whitelist = database.load_whitelist().await.context("load_whitelist failed")?;
                drop(whitelist);
                Ok(())
            },
        )
        .await?,
    );

    operations.push(
        measure_operation_async(
            "remove_info_hash_from_whitelist",
            ops,
            |index| async move {
                let info_hash = info_hash_from_index(50_000 + index)?;
                let _added_rows = database
                    .add_info_hash_to_whitelist(info_hash)
                    .await
                    .context("failed to seed remove_info_hash_from_whitelist")?;
                Ok(info_hash)
            },
            |info_hash| async move {
                let _removed_rows = database
                    .remove_info_hash_from_whitelist(info_hash)
                    .await
                    .context("remove_info_hash_from_whitelist failed")?;
                Ok(())
            },
        )
        .await?,
    );

    Ok(())
}
