use std::str::FromStr;
use std::time::Instant;

use anyhow::{Context, Result, anyhow};
use bittorrent_primitives::info_hash::InfoHash;

use super::RawOperationSamples;

/// Async variant of operation measurement, for database operations requiring
/// `.await`.
///
/// # Errors
///
/// Returns an error if setup or any async operation invocation fails.
pub(super) async fn measure_operation_async<S, SetupFut, F, T, OpFut>(
    name: impl Into<String>,
    ops: usize,
    mut setup: S,
    mut operation: F,
) -> Result<RawOperationSamples>
where
    S: FnMut(usize) -> SetupFut,
    SetupFut: std::future::Future<Output = Result<T>>,
    F: FnMut(T) -> OpFut,
    OpFut: std::future::Future<Output = Result<()>>,
{
    let name = name.into();
    let mut samples = Vec::with_capacity(ops);

    for index in 0..ops {
        let prepared = setup(index).await?;
        let start = Instant::now();
        operation(prepared).await?;
        samples.push(start.elapsed());
    }

    Ok(RawOperationSamples { name, samples })
}

/// Converts a loop index into a valid download-count value.
///
/// # Errors
///
/// Returns an error if the index does not fit in `u32`.
pub(super) fn downloads_from_index(index: usize) -> Result<u32> {
    u32::try_from(index).context("failed to convert operation index to download count")
}

/// Builds a deterministic 40-hex-char `InfoHash` from an index.
///
/// # Errors
///
/// Returns an error if the generated value cannot be parsed as an `InfoHash`.
pub(super) fn info_hash_from_index(index: usize) -> Result<InfoHash> {
    let hex = format!("{index:040x}");
    InfoHash::from_str(&hex).map_err(|error| anyhow!("failed to generate benchmark info hash: {error:?}"))
}
