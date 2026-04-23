use anyhow::Context;

use super::super::torrent_artifacts::build_torrent_bytes;
use super::build_payload_fixture::GeneratedPayload;

pub(in super::super) struct GeneratedTorrent {
    pub(in super::super) bytes: Vec<u8>,
}

pub(in super::super) fn build_torrent_fixture(
    payload: &GeneratedPayload,
    payload_name: &str,
    announce_url: &str,
    piece_length: usize,
) -> anyhow::Result<GeneratedTorrent> {
    let bytes = build_torrent_bytes(&payload.bytes, payload_name, announce_url, piece_length)
        .context("failed to build torrent fixture bytes from payload fixture")?;

    Ok(GeneratedTorrent { bytes })
}
