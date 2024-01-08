use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use torrust_tracker::core::peer::{Id, Peer};
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;
use torrust_tracker::shared::clock::DurationSinceUnixEpoch;

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

        let info_hash = InfoHash(bytes);
        result.insert(info_hash);
    }

    assert_eq!(result.len(), size);

    result.into_iter().collect()
}

#[must_use]
pub fn within_acceptable_range(test: &Duration, norm: &Duration) -> bool {
    let test_secs = test.as_secs_f64();
    let norm_secs = norm.as_secs_f64();

    // Calculate the upper and lower bounds for the 10% tolerance
    let tolerance = norm_secs * 0.1;

    // Calculate the upper and lower limits
    let upper_limit = norm_secs + tolerance;
    let lower_limit = norm_secs - tolerance;

    test_secs < upper_limit && test_secs > lower_limit
}

#[must_use]
pub fn get_average_and_adjusted_average_from_results(mut results: Vec<Duration>) -> (Duration, Duration) {
    #[allow(clippy::cast_possible_truncation)]
    let average = results.iter().sum::<Duration>() / results.len() as u32;

    results.retain(|result| within_acceptable_range(result, &average));

    let mut adjusted_average = Duration::from_nanos(0);

    #[allow(clippy::cast_possible_truncation)]
    if results.len() > 1 {
        adjusted_average = results.iter().sum::<Duration>() / results.len() as u32;
    }

    (average, adjusted_average)
}
