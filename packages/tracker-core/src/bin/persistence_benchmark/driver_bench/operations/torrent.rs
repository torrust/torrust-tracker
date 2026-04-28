use anyhow::{Context, Result};
use bittorrent_tracker_core::databases::Database;

use super::super::sampling::{downloads_from_index, info_hash_from_index, measure_operation};
use super::super::RawOperationSamples;

/// Benchmarks torrent statistics persistence operations.
///
/// This function seeds prerequisite records where needed so each measured
/// operation executes on realistic state.
///
/// # Errors
///
/// Returns an error if any setup or measured database operation fails.
pub(super) fn benchmark_torrent_operations(
    database: &dyn Database,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(measure_operation("save_torrent_downloads", ops, |index| {
        let info_hash = info_hash_from_index(index + 1)?;
        let downloads = downloads_from_index(index)?;
        database
            .save_torrent_downloads(&info_hash, downloads)
            .context("save_torrent_downloads failed")
    })?);

    let load_torrent_info_hash = info_hash_from_index(10_000)?;
    database
        .save_torrent_downloads(&load_torrent_info_hash, 123)
        .context("failed to seed load_torrent_downloads")?;
    operations.push(measure_operation("load_torrent_downloads", ops, |_| {
        let _downloads_result = database
            .load_torrent_downloads(&load_torrent_info_hash)
            .context("load_torrent_downloads failed")?;
        Ok(())
    })?);

    operations.push(measure_operation("load_all_torrents_downloads", ops, |_| {
        let all_downloads = database
            .load_all_torrents_downloads()
            .context("load_all_torrents_downloads failed")?;
        drop(all_downloads);
        Ok(())
    })?);

    let increasing_downloads_info_hash = info_hash_from_index(20_000)?;
    database
        .save_torrent_downloads(&increasing_downloads_info_hash, 0)
        .context("failed to seed increase_downloads_for_torrent")?;
    operations.push(measure_operation("increase_downloads_for_torrent", ops, |_| {
        database
            .increase_downloads_for_torrent(&increasing_downloads_info_hash)
            .context("increase_downloads_for_torrent failed")
    })?);

    operations.push(measure_operation("save_global_downloads", ops, |index| {
        let downloads = downloads_from_index(index)?;
        database
            .save_global_downloads(downloads)
            .context("save_global_downloads failed")
    })?);

    database
        .save_global_downloads(0)
        .context("failed to seed load_global_downloads")?;
    operations.push(measure_operation("load_global_downloads", ops, |_| {
        let _downloads_result = database.load_global_downloads().context("load_global_downloads failed")?;
        Ok(())
    })?);

    database
        .save_global_downloads(0)
        .context("failed to seed increase_global_downloads")?;
    operations.push(measure_operation("increase_global_downloads", ops, |_| {
        database
            .increase_global_downloads()
            .context("increase_global_downloads failed")
    })?);

    Ok(())
}
