use anyhow::{Context, Result};
use bittorrent_tracker_core::databases::TorrentMetricsStore;

use super::super::RawOperationSamples;
use super::super::sampling::{downloads_from_index, info_hash_from_index, measure_operation_async};

/// Benchmarks torrent statistics persistence operations.
///
/// This function seeds prerequisite records where needed so each measured
/// operation executes on realistic state.
///
/// # Errors
///
/// Returns an error if any setup or measured database operation fails.
pub(super) async fn benchmark_torrent_operations(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    benchmark_save_torrent_downloads(database, ops, operations).await?;
    benchmark_load_torrent_downloads(database, ops, operations).await?;
    benchmark_load_all_torrents_downloads(database, ops, operations).await?;
    benchmark_increase_downloads_for_torrent(database, ops, operations).await?;
    benchmark_save_global_downloads(database, ops, operations).await?;
    benchmark_load_global_downloads(database, ops, operations).await?;
    benchmark_increase_global_downloads(database, ops, operations).await?;

    Ok(())
}

async fn benchmark_save_torrent_downloads(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(
        measure_operation_async(
            "save_torrent_downloads",
            ops,
            |index| async move { Ok((info_hash_from_index(index + 1)?, downloads_from_index(index)?)) },
            |(info_hash, downloads)| async move {
                database
                    .save_torrent_downloads(&info_hash, downloads)
                    .await
                    .context("save_torrent_downloads failed")
            },
        )
        .await?,
    );

    Ok(())
}

async fn benchmark_load_torrent_downloads(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    let load_torrent_info_hash = info_hash_from_index(10_000)?;
    database
        .save_torrent_downloads(&load_torrent_info_hash, 123)
        .await
        .context("failed to seed load_torrent_downloads")?;

    operations.push(
        measure_operation_async(
            "load_torrent_downloads",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                let _downloads_result = database
                    .load_torrent_downloads(&load_torrent_info_hash)
                    .await
                    .context("load_torrent_downloads failed")?;
                Ok(())
            },
        )
        .await?,
    );

    Ok(())
}

async fn benchmark_load_all_torrents_downloads(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(
        measure_operation_async(
            "load_all_torrents_downloads",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                let all_downloads = database
                    .load_all_torrents_downloads()
                    .await
                    .context("load_all_torrents_downloads failed")?;
                drop(all_downloads);
                Ok(())
            },
        )
        .await?,
    );

    Ok(())
}

async fn benchmark_increase_downloads_for_torrent(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    let increasing_downloads_info_hash = info_hash_from_index(20_000)?;
    database
        .save_torrent_downloads(&increasing_downloads_info_hash, 0)
        .await
        .context("failed to seed increase_downloads_for_torrent")?;

    operations.push(
        measure_operation_async(
            "increase_downloads_for_torrent",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                database
                    .increase_downloads_for_torrent(&increasing_downloads_info_hash)
                    .await
                    .context("increase_downloads_for_torrent failed")
            },
        )
        .await?,
    );

    Ok(())
}

async fn benchmark_save_global_downloads(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(
        measure_operation_async(
            "save_global_downloads",
            ops,
            |index| async move { downloads_from_index(index) },
            |downloads| async move {
                database
                    .save_global_downloads(downloads)
                    .await
                    .context("save_global_downloads failed")
            },
        )
        .await?,
    );

    Ok(())
}

async fn benchmark_load_global_downloads(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    database
        .save_global_downloads(0)
        .await
        .context("failed to seed load_global_downloads")?;

    operations.push(
        measure_operation_async(
            "load_global_downloads",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                let _downloads_result = database
                    .load_global_downloads()
                    .await
                    .context("load_global_downloads failed")?;
                Ok(())
            },
        )
        .await?,
    );

    Ok(())
}

async fn benchmark_increase_global_downloads(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    database
        .save_global_downloads(0)
        .await
        .context("failed to seed increase_global_downloads")?;

    operations.push(
        measure_operation_async(
            "increase_global_downloads",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                database
                    .increase_global_downloads()
                    .await
                    .context("increase_global_downloads failed")
            },
        )
        .await?,
    );

    Ok(())
}
