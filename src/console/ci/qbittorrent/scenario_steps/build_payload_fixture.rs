use super::super::torrent_artifacts::build_payload_bytes;

pub(in super::super) struct GeneratedPayload {
    pub(in super::super) bytes: Vec<u8>,
}

pub(in super::super) fn build_payload_fixture(payload_size_bytes: usize) -> GeneratedPayload {
    GeneratedPayload {
        bytes: build_payload_bytes(payload_size_bytes),
    }
}
