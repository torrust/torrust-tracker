//! HTTP tracker test helpers.

use std::time::Duration;

use torrust_tracker_client::http::client::Client;
use torrust_tracker_http_protocol::v1::requests::announce::{Announce, Event};
use url::Url;

/// Sends an HTTP announce to the given tracker URL.
///
/// # Panics
///
/// Panics if the client cannot build, send, or receive.
pub async fn http_announce(tracker_url: &Url, info_hash: &[u8; 20], peer_id: &[u8; 20], port: u16) {
    let client = Client::new(tracker_url.clone(), Duration::from_secs(5)).expect("failed to create HTTP client");

    let query = Announce {
        info_hash: torrust_info_hash::InfoHash(*info_hash),
        peer_id: torrust_peer_id::PeerId(*peer_id),
        port,
        ip: None,
        downloaded: None,
        uploaded: None,
        left: None,
        event: Some(Event::Started),
        compact: None,
        numwant: None,
    };

    client.announce(&query).await.expect("HTTP announce should succeed");
}
