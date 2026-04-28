use anyhow::Context;

use super::super::super::torrent_artifacts::build_torrent_bytes;
use super::super::super::types::{InfoHash, PieceLength};
use super::build_payload_fixture::GeneratedPayload;

/// In-memory `.torrent` fixture generated from a payload fixture.
pub struct GeneratedTorrent {
    /// Raw bytes of the `.torrent` metainfo file.
    pub bytes: Vec<u8>,
    /// v1 `InfoHash`: SHA-1 of the bencoded `info` dict, lowercase hex (40 chars).
    /// Matches the hash format returned by the qBittorrent Web API.
    pub info_hash: InfoHash,
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
    piece_length: PieceLength,
) -> anyhow::Result<GeneratedTorrent> {
    let artifacts = build_torrent_bytes(&payload.bytes, payload_name, announce_url, piece_length.as_usize())
        .context("failed to build torrent fixture bytes from payload fixture")?;

    Ok(GeneratedTorrent {
        bytes: artifacts.torrent_bytes,
        info_hash: artifacts.info_hash,
    })
}
