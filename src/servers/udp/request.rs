use aquatic_udp_protocol::AnnounceRequest;

use crate::shared::bit_torrent::info_hash::InfoHash;

pub struct AnnounceWrapper {
    pub announce_request: AnnounceRequest,
    pub info_hash: InfoHash,
}

impl AnnounceWrapper {
    #[must_use]
    pub fn new(announce_request: &AnnounceRequest) -> Self {
        AnnounceWrapper {
            announce_request: announce_request.clone(),
            info_hash: InfoHash(announce_request.info_hash.0),
        }
    }
}
