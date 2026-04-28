use super::super::super::torrent_artifacts::build_payload_bytes;
use super::super::super::types::PayloadSize;

/// In-memory payload fixture used to generate torrent metadata and integrity checks.
pub struct GeneratedPayload {
    pub bytes: Vec<u8>,
}

/// Builds deterministic payload bytes for the E2E scenario.
///
/// The generated payload is stable for a given size, which keeps test behavior reproducible.
pub fn build_payload_fixture(payload_size_bytes: PayloadSize) -> GeneratedPayload {
    GeneratedPayload {
        bytes: build_payload_bytes(payload_size_bytes.as_usize()),
    }
}
