use anyhow::Context;

use super::super::super::torrent_artifacts::build_torrent_bytes;
use super::build_payload_fixture::GeneratedPayload;

/// In-memory `.torrent` fixture generated from a payload fixture.
pub struct GeneratedTorrent {
    pub bytes: Vec<u8>,
}

/// Builds torrent metadata bytes from a payload fixture.
///
/// # Errors
///
/// Returns an error when torrent metadata encoding fails.
pub fn build_torrent_fixture(
    payload: &GeneratedPayload,
    payload_name: &str,
    announce_url: &str,
    piece_length: usize,
) -> anyhow::Result<GeneratedTorrent> {
    let bytes = build_torrent_bytes(&payload.bytes, payload_name, announce_url, piece_length)
        .context("failed to build torrent fixture bytes from payload fixture")?;

    Ok(GeneratedTorrent { bytes })
}
