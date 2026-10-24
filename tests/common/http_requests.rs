//! HTTP tracker test helpers.

#![allow(
    dead_code,
    reason = "each integration-test binary compiles this module and calls only some helpers"
)]

use std::time::Duration;

use torrust_tracker_client::http::client::Client;
use torrust_tracker_http_protocol::v1::requests::announce::{Announce, Event, NumberOfBytes, PeerIp};
use torrust_tracker_http_protocol::v1::requests::scrape_builder;
use torrust_tracker_http_protocol::v1::responses::scrape;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use url::Url;

/// Sends an HTTP announce to the given tracker URL.
///
/// # Panics
///
/// Panics if the client cannot build, send, or receive.
pub async fn http_announce(tracker_url: &Url, info_hash: &[u8; 20], peer_id: &[u8; 20], port: u16) {
    announce_event(tracker_url, info_hash, peer_id, port, Event::Started, None).await;
}

/// Sends the HTTP announces of a peer that starts and then completes a download.
///
/// It returns after the tracker answered the `completed` announce, so the
/// tracker has published the completed-download event on its event bus by then.
///
/// # Panics
///
/// Panics if the client cannot build, send, or receive.
pub async fn http_complete_download(tracker_url: &Url, info_hash: &[u8; 20], peer_id: &[u8; 20], port: u16) {
    announce_event(tracker_url, info_hash, peer_id, port, Event::Started, Some(1)).await;
    announce_event(tracker_url, info_hash, peer_id, port, Event::Completed, Some(0)).await;
}

async fn announce_event(
    tracker_url: &Url,
    info_hash: &[u8; 20],
    peer_id: &[u8; 20],
    port: u16,
    event: Event,
    bytes_left: Option<i64>,
) {
    let client = Client::new(tracker_url.clone(), Duration::from_secs(5)).expect("failed to create HTTP client");

    let query = Announce {
        info_hash: torrust_info_hash::InfoHash(*info_hash),
        peer_id: torrust_peer_id::PeerId(*peer_id),
        port,
        ip: PeerIp::Absent,
        downloaded: None,
        uploaded: None,
        left: bytes_left.map(NumberOfBytes::new),
        event: Some(event),
        compact: None,
        numwant: None,
    };

    client.announce(&query).await.expect("HTTP announce should succeed");
}

/// Sends an HTTP scrape for the given info-hashes and returns their swarm
/// metadata in request order.
///
/// # Panics
///
/// Panics if the client cannot build, send, or receive, the response is not a
/// valid bencoded scrape response, or a requested info-hash is missing.
pub async fn http_scrape(tracker_url: &Url, info_hashes: &[[u8; 20]]) -> Vec<SwarmMetadata> {
    let client = Client::new(tracker_url.clone(), Duration::from_secs(5)).expect("failed to create HTTP client");

    let query = scrape_builder::Query {
        info_hash: info_hashes
            .iter()
            .map(|info_hash| torrust_info_hash::InfoHash(*info_hash))
            .collect(),
    };

    let response = client.scrape(&query).await.expect("HTTP scrape should succeed");
    let bytes = response.bytes().await.expect("HTTP scrape response body should be readable");
    let response = scrape::Response::try_from_bencoded(&bytes).expect("HTTP scrape response should be bencoded");

    info_hashes
        .iter()
        .map(|info_hash| {
            let file = &response.files[&torrust_info_hash::InfoHash(*info_hash)];
            SwarmMetadata::new(counter(file.downloaded), counter(file.complete), counter(file.incomplete))
        })
        .collect()
}

fn counter(value: i64) -> u32 {
    u32::try_from(value).expect("HTTP scrape counters should fit in u32")
}
