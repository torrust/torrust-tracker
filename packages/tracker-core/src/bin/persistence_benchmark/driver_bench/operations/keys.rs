use anyhow::{Context, Result};
use bittorrent_tracker_core::authentication;
use bittorrent_tracker_core::databases::AuthKeyStore;

use super::super::sampling::measure_operation;
use super::super::RawOperationSamples;

/// Benchmarks authentication-key persistence operations.
///
/// # Errors
///
/// Returns an error if any setup or measured database operation fails.
pub(super) fn benchmark_key_operations(
    database: &dyn AuthKeyStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(measure_operation(
        "add_key_to_keys",
        ops,
        |_| Ok(authentication::key::generate_key(None)),
        |peer_key| {
            let _added_rows = database.add_key_to_keys(&peer_key).context("add_key_to_keys failed")?;
            Ok(())
        },
    )?);

    let persisted_peer_key = authentication::key::generate_key(None);
    let _added_rows = database
        .add_key_to_keys(&persisted_peer_key)
        .context("failed to seed get_key_from_keys")?;
    let persisted_key = persisted_peer_key.key();
    operations.push(measure_operation(
        "get_key_from_keys",
        ops,
        |_| Ok(()),
        |()| {
            let persisted_key_result = database
                .get_key_from_keys(&persisted_key)
                .context("get_key_from_keys failed")?;
            drop(persisted_key_result);
            Ok(())
        },
    )?);

    operations.push(measure_operation(
        "load_keys",
        ops,
        |_| Ok(()),
        |()| {
            let keys = database.load_keys().context("load_keys failed")?;
            drop(keys);
            Ok(())
        },
    )?);

    operations.push(measure_operation(
        "remove_key_from_keys",
        ops,
        |_| {
            let peer_key = authentication::key::generate_key(None);
            let _added_rows = database
                .add_key_to_keys(&peer_key)
                .context("failed to seed remove_key_from_keys")?;
            Ok(peer_key.key())
        },
        |key| {
            let _removed_rows = database.remove_key_from_keys(&key).context("remove_key_from_keys failed")?;
            Ok(())
        },
    )?);

    Ok(())
}
