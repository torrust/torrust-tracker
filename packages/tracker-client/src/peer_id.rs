use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use bittorrent_udp_tracker_protocol::PeerId;

const DEFAULT_PRODUCTION_PEER_ID_PREFIX_BYTES: &[u8; 8] = b"-RC3000-";

/// Deterministic peer ID for tests and fixtures.
///
/// Format: `-<CC><VVVV>-<random-12-digits>`.
pub const DEFAULT_TEST_PEER_ID_BYTES: [u8; 20] = *b"-RC3000-000000000001";
pub const DEFAULT_TEST_PEER_ID: PeerId = PeerId(DEFAULT_TEST_PEER_ID_BYTES);

/// Returns the default production peer ID.
///
/// The 12-digit suffix is generated once per process and reused for the lifetime
/// of the process.
#[must_use]
pub fn default_production_peer_id() -> PeerId {
    static DEFAULT_PEER_ID: OnceLock<PeerId> = OnceLock::new();

    *DEFAULT_PEER_ID.get_or_init(|| PeerId(generate_default_production_peer_id_bytes()))
}

fn generate_default_production_peer_id_bytes() -> [u8; 20] {
    let mut bytes = [0_u8; 20];
    bytes[..8].copy_from_slice(DEFAULT_PRODUCTION_PEER_ID_PREFIX_BYTES);
    bytes[8..].copy_from_slice(random_suffix_12_digits().as_bytes());
    bytes
}

fn random_suffix_12_digits() -> String {
    let nanos_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    let process_id = u128::from(std::process::id());
    let mixed = nanos_since_epoch ^ (process_id << 64) ^ nanos_since_epoch.rotate_left(29);
    let value = mixed % 1_000_000_000_000;

    format!("{value:012}")
}

#[cfg(test)]
mod tests {
    use super::{default_production_peer_id, DEFAULT_TEST_PEER_ID};

    #[test]
    fn default_test_peer_id_should_use_rc_prefix_and_3000_version() {
        assert_eq!(DEFAULT_TEST_PEER_ID.0[..8], *b"-RC3000-");
    }

    #[test]
    fn default_production_peer_id_should_be_stable_within_a_process() {
        let first = default_production_peer_id();
        let second = default_production_peer_id();

        assert_eq!(first.0, second.0);
        assert_eq!(first.0[..8], *b"-RC3000-");
        assert!(first.0[8..].iter().all(u8::is_ascii_digit));
    }
}
