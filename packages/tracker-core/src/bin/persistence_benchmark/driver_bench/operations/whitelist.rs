use anyhow::{Context, Result};
use bittorrent_tracker_core::databases::WhitelistStore;

use super::super::sampling::{info_hash_from_index, measure_operation};
use super::super::RawOperationSamples;

/// Benchmarks whitelist-related persistence operations.
///
/// # Errors
///
/// Returns an error if any setup or measured database operation fails.
pub(super) fn benchmark_whitelist_operations(
    database: &dyn WhitelistStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(measure_operation(
        "add_info_hash_to_whitelist",
        ops,
        |index| info_hash_from_index(30_000 + index),
        |info_hash| {
            let _added_rows = database
                .add_info_hash_to_whitelist(info_hash)
                .context("add_info_hash_to_whitelist failed")?;
            Ok(())
        },
    )?);

    let whitelisted_info_hash = info_hash_from_index(40_000)?;
    let _added_rows = database
        .add_info_hash_to_whitelist(whitelisted_info_hash)
        .context("failed to seed get_info_hash_from_whitelist")?;
    operations.push(measure_operation(
        "get_info_hash_from_whitelist",
        ops,
        |_| Ok(()),
        |()| {
            let _info_hash_result = database
                .get_info_hash_from_whitelist(whitelisted_info_hash)
                .context("get_info_hash_from_whitelist failed")?;
            Ok(())
        },
    )?);

    operations.push(measure_operation(
        "load_whitelist",
        ops,
        |_| Ok(()),
        |()| {
            let whitelist = database.load_whitelist().context("load_whitelist failed")?;
            drop(whitelist);
            Ok(())
        },
    )?);

    operations.push(measure_operation(
        "remove_info_hash_from_whitelist",
        ops,
        |index| {
            let info_hash = info_hash_from_index(50_000 + index)?;
            let _added_rows = database
                .add_info_hash_to_whitelist(info_hash)
                .context("failed to seed remove_info_hash_from_whitelist")?;
            Ok(info_hash)
        },
        |info_hash| {
            let _removed_rows = database
                .remove_info_hash_from_whitelist(info_hash)
                .context("remove_info_hash_from_whitelist failed")?;
            Ok(())
        },
    )?);

    Ok(())
}
