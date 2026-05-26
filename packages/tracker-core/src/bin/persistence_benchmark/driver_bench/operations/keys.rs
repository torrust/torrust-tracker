use anyhow::{Context, Result};
use torrust_tracker_core::authentication;
use torrust_tracker_core::databases::AuthKeyStore;

use super::super::RawOperationSamples;
use super::super::sampling::measure_operation_async;

/// Benchmarks authentication-key persistence operations.
///
/// # Errors
///
/// Returns an error if any setup or measured database operation fails.
pub(super) async fn benchmark_key_operations(
    database: &dyn AuthKeyStore,
    ops: usize,
    operations: &mut Vec<RawOperationSamples>,
) -> Result<()> {
    operations.push(
        measure_operation_async(
            "add_key_to_keys",
            ops,
            |_| async move { Ok(authentication::key::generate_key(None)) },
            |peer_key| async move {
                let _added_rows = database.add_key_to_keys(&peer_key).await.context("add_key_to_keys failed")?;
                Ok(())
            },
        )
        .await?,
    );

    let persisted_peer_key = authentication::key::generate_key(None);
    let _added_rows = database
        .add_key_to_keys(&persisted_peer_key)
        .await
        .context("failed to seed get_key_from_keys")?;
    let persisted_key = persisted_peer_key.key();
    operations.push(
        measure_operation_async(
            "get_key_from_keys",
            ops,
            |_| async move { Ok(()) },
            |()| {
                let persisted_key = persisted_key.clone();
                async move {
                    let persisted_key_result = database
                        .get_key_from_keys(&persisted_key)
                        .await
                        .context("get_key_from_keys failed")?;
                    drop(persisted_key_result);
                    Ok(())
                }
            },
        )
        .await?,
    );

    operations.push(
        measure_operation_async(
            "load_keys",
            ops,
            |_| async move { Ok(()) },
            |()| async move {
                let keys = database.load_keys().await.context("load_keys failed")?;
                drop(keys);
                Ok(())
            },
        )
        .await?,
    );

    operations.push(
        measure_operation_async(
            "remove_key_from_keys",
            ops,
            |_| async move {
                let peer_key = authentication::key::generate_key(None);
                let _added_rows = database
                    .add_key_to_keys(&peer_key)
                    .await
                    .context("failed to seed remove_key_from_keys")?;
                Ok(peer_key.key())
            },
            |key| async move {
                let _removed_rows = database
                    .remove_key_from_keys(&key)
                    .await
                    .context("remove_key_from_keys failed")?;
                Ok(())
            },
        )
        .await?,
    );

    Ok(())
}
