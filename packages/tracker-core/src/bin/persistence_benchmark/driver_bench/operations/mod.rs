mod keys;
mod torrent;
mod whitelist;

use anyhow::Result;
use bittorrent_tracker_core::databases::Database;

use super::RawOperationSamples;

pub(super) fn benchmark_torrent_operations(
    database: &dyn Database,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    torrent::benchmark_torrent_operations(database, ops, operations)
}

pub(super) fn benchmark_whitelist_operations(
    database: &dyn Database,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    whitelist::benchmark_whitelist_operations(database, ops, operations)
}

pub(super) fn benchmark_key_operations(
    database: &dyn Database,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    keys::benchmark_key_operations(database, ops, operations)
}
