//! Persisted scrape downloads integration test — restart scenario (#2406).
//!
//! This binary starts a tracker with persistent completed statistics, registers
//! a completed download, restarts the application on the same `SQLite`
//! database, and scrapes over HTTP and UDP before any new announce. The
//! restart drops all in-memory swarms, so the counts can only come from the
//! database.
//!
//! ```text
//! cargo test --test persistence-scrape-after-restart
//! ```
#[path = "../common/mod.rs"]
mod common;

use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

const PERSISTENT_COMPLETED_STAT_CONFIG: &str = r#"
    [metadata]
    app = "torrust-tracker"
    purpose = "configuration"
    schema_version = "3.0.0"

    [logging]
    trace_filter = "off"

    [core]
    listed = false
    private = false

    [core.database]
    driver = "sqlite3"
    path = "{STORAGE_PATH}/sqlite3.db"

    [core.tracker_policy]
    persistent_torrent_completed_stat = true

    [[http_trackers]]
    bind_address = "127.0.0.1:0"
    tracker_usage_statistics = false

    [[udp_trackers]]
    bind_address = "127.0.0.1:0"
    tracker_usage_statistics = false

    [health_check_api]
    bind_address = "127.0.0.1:0"
"#;

const COMPLETED_TORRENT: [u8; 20] = [0x11; 20];
const UNKNOWN_TORRENT: [u8; 20] = [0x33; 20];
const PEER_ID: [u8; 20] = *b"-qB00000000000000001";
const PEER_PORT: u16 = 16881;

#[tokio::test]
async fn it_should_scrape_the_persisted_downloads_of_a_torrent_after_a_restart() {
    // Arrange
    let mut fixture = common::TrackerApplicationFixture::start(PERSISTENT_COMPLETED_STAT_CONFIG).await;
    common::udp_complete_download(udp_tracker_addr(&fixture).await, &COMPLETED_TORRENT, &PEER_ID, PEER_PORT).await;
    fixture.restart().await;
    let requested_torrents = [COMPLETED_TORRENT, UNKNOWN_TORRENT];

    // Act
    let http_scrape = common::http_scrape(&http_tracker_url(&fixture).await, &requested_torrents).await;
    let udp_scrape = common::udp_scrape(udp_tracker_addr(&fixture).await, &requested_torrents).await;

    // Assert
    let expected = vec![SwarmMetadata::new(1, 0, 0), SwarmMetadata::zeroed()];
    assert_eq!(http_scrape, expected);
    assert_eq!(udp_scrape, expected);

    fixture.shutdown().await;
}

async fn http_tracker_url(fixture: &common::TrackerApplicationFixture) -> url::Url {
    common::http_tracker_urls(fixture.app_container())
        .await
        .into_iter()
        .next()
        .expect("expected one HTTP tracker")
}

async fn udp_tracker_addr(fixture: &common::TrackerApplicationFixture) -> std::net::SocketAddr {
    let url = common::udp_tracker_urls(fixture.app_container())
        .await
        .into_iter()
        .next()
        .expect("expected one UDP tracker");
    common::udp_socket_addr(&url)
}
