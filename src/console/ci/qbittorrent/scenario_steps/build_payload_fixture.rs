use super::super::torrent_artifacts::build_payload_bytes;

/// In-memory payload fixture used to generate torrent metadata and integrity checks.
pub(in super::super) struct GeneratedPayload {
    pub(in super::super) bytes: Vec<u8>,
}

/// Builds deterministic payload bytes for the E2E scenario.
///
/// The generated payload is stable for a given size, which keeps test behavior reproducible.
pub(in super::super) fn build_payload_fixture(payload_size_bytes: usize) -> GeneratedPayload {
    GeneratedPayload {
        bytes: build_payload_bytes(payload_size_bytes),
    }
}
