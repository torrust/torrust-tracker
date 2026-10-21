//! Completed downloads answered over HTTP are persisted by a graceful shutdown (#2416).
//!
//! This binary starts a tracker with persistent completed statistics and an
//! HTTP tracker, completes several downloads over HTTP, and stops the
//! application with the `JobManager` shutdown right after the last answer,
//! without waiting for the listeners. It then reads the `SQLite` database
//! through the application's own persistence. It guards the wiring of the stop
//! order and the listener drain end to end. It is regression coverage, not the
//! red proof of the bug: the race window is too small to fail reliably.
//!
//! ```text
//! cargo test --test persistence-completed-downloads-on-shutdown
//! ```
#[path = "../common/mod.rs"]
mod common;
// issue: #2416

use std::collections::HashMap;

use torrust_tracker_lib::container::AppContainer;

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

    [health_check_api]
    bind_address = "127.0.0.1:0"
"#;

const COMPLETED_DOWNLOADS: u8 = 20;
const PEER_ID: [u8; 20] = *b"-qB00000000000000001";
const PEER_PORT: u16 = 16881;

#[tokio::test]
async fn it_should_persist_every_download_answered_over_http_before_the_shutdown() {
    // Arrange
    // A running tracker with persistent completed statistics and one HTTP tracker.
    let fixture = common::TrackerApplicationFixture::start(PERSISTENT_COMPLETED_STAT_CONFIG).await;
    let app_container = fixture.app_container().clone();
    let http_tracker_url = common::http_tracker_urls(&app_container)
        .await
        .into_iter()
        .next()
        .expect("expected one HTTP tracker");

    // Act
    // Twenty peers each complete a download on their own torrent, and the application shuts down
    // as soon as the tracker answered the last `completed` announce.
    for index in 0..COMPLETED_DOWNLOADS {
        common::http_complete_download(&http_tracker_url, &[index + 1; 20], &PEER_ID, PEER_PORT).await;
    }
    fixture.shutdown().await;

    // Assert
    // Every download the tracker answered is in the database, with exactly one completion each.
    let persisted = persisted_downloads(&app_container).await;
    let missing: Vec<u8> = (0..COMPLETED_DOWNLOADS)
        .filter(|index| persisted.get(&[index + 1; 20]) != Some(&1))
        .map(|index| index + 1)
        .collect();
    assert!(
        missing.is_empty(),
        "every answered completed announce must be persisted by a graceful shutdown; torrents missing a persisted download (by repeated byte): {missing:?}"
    );
}

async fn persisted_downloads(app_container: &AppContainer) -> HashMap<[u8; 20], u32> {
    app_container
        .tracker_core_container
        .persistence
        .as_ref()
        .expect("persistent completed statistics require persistence")
        .database_stores
        .torrent_metrics_store
        .load_all_torrents_downloads()
        .await
        .expect("persisted downloads should be readable")
        .into_iter()
        .map(|(info_hash, downloads)| (info_hash.0, downloads))
        .collect()
}
