//! Announce helpers — send HTTP and UDP announces to tracker instances.

use std::net::SocketAddr;

use url::Url;

/// Sends an HTTP announce to the given tracker URL.
///
/// Delegates to `torrust_tracker_test_helpers::http::http_announce`.
/// Panics if the announce fails.
//
// Not called by every integration-test binary — see note on `udp_tracker_urls`.
#[allow(dead_code)]
pub async fn http_announce(tracker_url: &Url, info_hash: &[u8; 20], peer_id: &[u8; 20], port: u16) {
    torrust_tracker_test_helpers::http::http_announce(tracker_url, info_hash, peer_id, port).await;
}

/// Sends a UDP announce to the given tracker address.
///
/// Delegates to `torrust_tracker_test_helpers::udp::udp_announce`.
/// Panics if the announce fails.
//
// Not called by every integration-test binary — see note on `udp_tracker_urls`.
#[allow(dead_code)]
pub async fn udp_announce(remote_addr: SocketAddr, info_hash: &[u8; 20], peer_id: &[u8; 20], port: u16) {
    torrust_tracker_test_helpers::udp::udp_announce(remote_addr, info_hash, peer_id, port).await;
}
