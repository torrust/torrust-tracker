//! The log of a request to the HTTP tracker holds no peer key.
//!
//! The peer key is a path segment (`/announce/<key>`, `/scrape/<key>`), so the URI the server logs
//! for every request, in the `request` event and in the span around it, must not show it. The
//! test binary installs a subscriber that captures every level, which only one binary can do.
use std::sync::Arc;
use std::time::Duration;

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_client::http::client::Client;
use torrust_tracker_test_helpers::{configuration, logging};

const QUERY: &str = "info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0&peer_id=-qB00000000000000001&port=17548";

#[tokio::test]
async fn it_should_not_log_the_peer_key_of_an_announce_or_a_scrape_on_a_private_tracker() {
    // Arrange
    logging::setup_capturing_every_level();
    let cfg = configuration::ephemeral_private();
    let env = Started::new(
        &Arc::new(cfg.core.clone()),
        &Arc::new(cfg.http_trackers.clone().unwrap()[0].clone()),
    )
    .await;
    let well_formed_key = "TestPeerKeyLoggedByAnnounce00001";
    let malformed_key = "TestPeerKeyLoggedByScrape-000001";
    let key_of_the_wrong_length = "ShortLoggedKey01";
    let client = Client::new(env.base_url(), Duration::from_secs(5)).unwrap();

    // Act
    client.get(&format!("announce/{well_formed_key}?{QUERY}")).await.unwrap();
    client.get(&format!("scrape/{malformed_key}?{QUERY}")).await.unwrap();
    client
        .get(&format!("announce/{key_of_the_wrong_length}?{QUERY}"))
        .await
        .unwrap();
    env.stop().await;

    // Assert
    let logs = logging::captured_logs();
    for key in [well_formed_key, malformed_key, key_of_the_wrong_length] {
        assert!(
            !logs.contains(key),
            "the log holds a test peer key of {} characters",
            key.len()
        );
    }
    assert!(
        logs.contains("uri=/announce/***?"),
        "the request should be logged with a redacted URI, got none in: {logs}"
    );
}
