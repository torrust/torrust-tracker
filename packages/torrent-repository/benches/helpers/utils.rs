use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::peer::{Id, Peer};
use torrust_tracker_primitives::{AnnounceEvent, DurationSinceUnixEpoch, NumberOfBytes};

pub const DEFAULT_PEER: Peer = Peer {
    peer_id: Id([0; 20]),
    peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    updated: DurationSinceUnixEpoch::from_secs(0),
    uploaded: NumberOfBytes(0),
    downloaded: NumberOfBytes(0),
    left: NumberOfBytes(0),
    event: AnnounceEvent::Started,
};

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn generate_unique_info_hashes(size: usize) -> Vec<InfoHash> {
    let mut result = HashSet::new();

    let mut bytes = [0u8; 20];

    #[allow(clippy::cast_possible_truncation)]
    for i in 0..size {
        bytes[0] = (i & 0xFF) as u8;
        bytes[1] = ((i >> 8) & 0xFF) as u8;
        bytes[2] = ((i >> 16) & 0xFF) as u8;
        bytes[3] = ((i >> 24) & 0xFF) as u8;

        let info_hash = InfoHash::from_bytes(&bytes);
        result.insert(info_hash);
    }

    assert_eq!(result.len(), size);

    result.into_iter().collect()
}
