use std::fs;
use std::path::Path;

use anyhow::Context;

/// Verifies that a downloaded file matches the original payload file byte-for-byte.
///
/// Reads both files from disk and compares their contents byte-for-byte.
pub(in super::super) fn verify_payload_integrity(downloaded_path: &Path, original_path: &Path) -> anyhow::Result<()> {
    let downloaded_bytes = fs::read(downloaded_path)
        .with_context(|| format!("failed to read downloaded payload from '{}'", downloaded_path.display()))?;
    let original_bytes =
        fs::read(original_path).with_context(|| format!("failed to read original payload from '{}'", original_path.display()))?;

    if downloaded_bytes.len() != original_bytes.len() {
        anyhow::bail!(
            "payload size mismatch: original {} bytes, downloaded {} bytes",
            original_bytes.len(),
            downloaded_bytes.len()
        );
    }

    if downloaded_bytes != original_bytes {
        anyhow::bail!("payload content mismatch: files have the same size but different contents");
    }

    tracing::info!(bytes = original_bytes.len(), "payload integrity verified");

    Ok(())
}
