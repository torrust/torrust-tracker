//! Scaffolding integration test — demo and sample.
//!
//! This file is a **scaffolding sample** that demonstrates the integration-test
//! pattern adopted by this project. It is not intended to provide unique test
//! coverage. Instead, its purpose is to:
//!
//! - Verify that multiple top-level integration-test binaries can run
//!   concurrently without port or configuration conflicts.
//! - Show future contributors how to add a new integration-test binary for
//!   a different tracker configuration or lifecycle scenario.
//!
//! # Architecture
//!
//! Each top-level `tests/*.rs` file is a **separate OS process** (Cargo
//! integration-test binary). A binary runs **one tracker application
//! instance** with a fixed initial configuration. Scenario functions run
//! sequentially against that instance.
//!
//! A different initial configuration belongs in another binary.
//! For example, `tests/bootstrap.rs` would exercise the startup/shutdown
//! lifecycle, while `tests/metrics/port_zero.rs` exercises the global
//! statistics API under one configuration.
//!
//! ## Shared Helpers
//!
//! Common utilities live in [`tests/common/`](../common/index.html).
//! Import with `mod common;`.
//!
//! ## Requirements
//!
//! - Port `0` for all service bind addresses.
//! - Isolated temporary workspace per suite (`EphemeralTrackerWorkspace`).
//! - Registration-acknowledgement readiness for every configured service.
//! - Sequential scenarios that account for accumulated state.
//! - Explicit awaited shutdown through `TrackerApplicationFixture` before the
//!   temporary workspace is released.
//!
//! ## Endpoint Discovery
//!
//! Endpoint discovery uses side-effect-free runtime-registry snapshots. Helpers
//! select services by canonical role or exact configuration identity rather
//! than bind-IP conventions, registration delays, or registry-map ordering.
//!
//! # Example: Running this test
//!
//! ```text
//! cargo test --test scaffold
//! ```
//!
//! The `metrics-port-zero` and `scaffold` binaries can run in parallel:
//!
//! ```text
//! cargo test --test metrics-port-zero --test scaffold
//! ```
mod common;

use serde::Deserialize;
use torrust_tracker_rest_api_client::connection_info::{ConnectionInfo, Origin};
use torrust_tracker_rest_api_client::v1::client::ApiHttpClient as TrackerApiClient;
use url::Url;

/// Demo: the stats API should aggregate announces across multiple trackers.
///
/// This is a scaffolding sample that reproduces the global-stats scenario
/// to demonstrate that a second integration-test binary can boot its own
/// tracker application without conflicting with the main suite.
#[tokio::test]
async fn the_stats_api_endpoint_should_aggregate_announces_across_multiple_trackers() {
    // ── 1. Configuration ──────────────────────────────────────────────
    let config_toml = r#"
        [metadata]
        app = "torrust-tracker"
        purpose = "configuration"
        schema_version = "2.0.0"

        [logging]
        threshold = "off"

        [core]
        listed = false
        private = false

        [core.database]
        driver = "sqlite3"
        path = "{STORAGE_PATH}/sqlite3.db"

        [[http_trackers]]
        bind_address = "0.0.0.0:0"
        tracker_usage_statistics = true

        [[http_trackers]]
        bind_address = "0.0.0.0:0"
        tracker_usage_statistics = true

        [http_api]
        bind_address = "127.0.0.1:0"

        [http_api.access_tokens]
        admin = "MyAccessToken"

        [health_check_api]
        bind_address = "127.0.0.2:0"
    "#;

    // ── 2. Start tracker on isolated workspace ───────────────────────
    let fixture = common::TrackerApplicationFixture::start(config_toml).await;
    let app_container = fixture.app_container();

    // ── 3. Discover bound addresses ──────────────────────────────────
    let tracker_urls = common::http_tracker_urls(app_container).await;
    assert_eq!(tracker_urls.len(), 2, "expected two HTTP trackers");

    let api_url = common::http_api_url(app_container).await.expect("expected an HTTP API URL");

    // ── 4. Scenario: announce to both trackers ───────────────────────
    let client = reqwest::Client::new();
    for url in &tracker_urls {
        let announce_url = url
            .join("/announce?info_hash=%9c8b%22%13%e3%0b%ff%21%2b0%c3%60%d2o%9a%02%13d%22&peer_id=-qB00000000000000001&port=17548&event=started&compact=0")
            .expect("announce URL should be valid");
        let resp = client.get(announce_url.as_str()).send().await.unwrap();
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            panic!("announce to {url} failed: status {status}, body: {body}");
        }
    }

    // ── 5. Scenario: verify global stats ─────────────────────────────
    let stats = get_stats(&api_url, "MyAccessToken").await;
    assert_eq!(stats.tcp4_announces_handled, 2, "two announces should be aggregated");

    // ── 6. Shut down before releasing the temporary workspace ────────
    fixture.shutdown().await;
}

/// Statistics subset relevant to this demo.
#[derive(Deserialize)]
struct DemoStats {
    tcp4_announces_handled: u64,
}

async fn get_stats(api_url: &Url, token: &str) -> DemoStats {
    let response = TrackerApiClient::new(ConnectionInfo::authenticated(Origin::new(api_url.as_str()).unwrap(), token))
        .unwrap()
        .get_tracker_statistics(None)
        .await
        .expect("failed to get tracker statistics");

    response.json::<DemoStats>().await.expect("failed to parse JSON response")
}
