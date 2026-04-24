use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use anyhow::Context;
use sha1::{Digest as Sha1Digest, Sha1};

/// Verifies that a downloaded file matches the original payload file byte-for-byte.
///
/// Reads both files from disk and compares their contents. Logs the `SHA1` hash of the
/// verified payload on success.
pub(in super::super) fn verify_payload_integrity(downloaded_path: &Path, original_path: &Path) -> anyhow::Result<()> {
    let downloaded_bytes = fs::read(downloaded_path)
        .with_context(|| format!("failed to read downloaded payload from '{}'", downloaded_path.display()))?;
    let original_payload =
        fs::read(original_path).with_context(|| format!("failed to read original payload from '{}'", original_path.display()))?;

    if downloaded_bytes.len() != original_payload.len() {
        anyhow::bail!(
            "payload size mismatch: original {} bytes, downloaded {} bytes",
            original_payload.len(),
            downloaded_bytes.len()
        );
    }

    if downloaded_bytes != original_payload {
        let original_hash = sha1_hex(&original_payload);
        let downloaded_hash = sha1_hex(&downloaded_bytes);
        anyhow::bail!("payload content mismatch: original SHA1 {original_hash}, downloaded SHA1 {downloaded_hash}");
    }

    let hash = sha1_hex(&original_payload);

    tracing::info!(
        "Payload integrity verified: SHA1 {} ({} bytes match)",
        hash,
        original_payload.len()
    );

    Ok(())
}

fn sha1_hex(bytes: &[u8]) -> String {
    Sha1::digest(bytes).iter().fold(String::new(), |mut output, byte| {
        let _ = write!(output, "{byte:02x}");
        output
    })
}
