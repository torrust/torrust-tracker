mod keys;
mod torrent;
mod whitelist;

use anyhow::Result;
use torrust_tracker_core::databases::{AuthKeyStore, TorrentMetricsStore, WhitelistStore};

use super::RawOperationSamples;

pub(super) async fn benchmark_torrent_operations(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    torrent::benchmark_torrent_operations(database, ops, operations).await
}

pub(super) async fn benchmark_whitelist_operations(
    database: &dyn WhitelistStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    whitelist::benchmark_whitelist_operations(database, ops, operations).await
}

pub(super) async fn benchmark_key_operations(
    database: &dyn AuthKeyStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    keys::benchmark_key_operations(database, ops, operations).await
}
