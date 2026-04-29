mod keys;
mod torrent;
mod whitelist;

use anyhow::Result;
use bittorrent_tracker_core::databases::{AuthKeyStore, TorrentMetricsStore, WhitelistStore};

use super::RawOperationSamples;

pub(super) fn benchmark_torrent_operations(
    database: &dyn TorrentMetricsStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    torrent::benchmark_torrent_operations(database, ops, operations)
}

pub(super) fn benchmark_whitelist_operations(
    database: &dyn WhitelistStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    whitelist::benchmark_whitelist_operations(database, ops, operations)
}

pub(super) fn benchmark_key_operations(
    database: &dyn AuthKeyStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    keys::benchmark_key_operations(database, ops, operations)
}
