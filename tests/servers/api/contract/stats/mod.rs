use serde::Deserialize;
use torrust_tracker_rest_api_client::connection_info::{ConnectionInfo, Origin};
use torrust_tracker_rest_api_client::v1::client::ApiHttpClient as TrackerApiClient;
use url::Url;

use crate::common::{self, EphemeralTrackerWorkspace};

/// The stats API endpoint should aggregate announces across multiple HTTP tracker instances.
///
/// This is an application-level integration test. It verifies that announces
/// sent to two separate HTTP tracker instances are both counted in the global
/// tracker statistics. This behavior cannot be tested at the package level
/// because it requires the full application container coordinating multiple
/// HTTP tracker instances.
///
/// Single-instance announce and scrape behavior is tested in the
/// `axum-http-server` package.
#[tokio::test]
async fn the_stats_api_endpoint_should_return_the_global_stats() {
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
    let workspace = EphemeralTrackerWorkspace::new(config_toml);
    let (app_container, _jobs) = common::start_tracker_with_config(&workspace).await;

    let tracker_urls = common::http_tracker_urls(&app_container).await;
    assert_eq!(tracker_urls.len(), 2, "expected two HTTP trackers");

    let api_url = common::http_api_url(&app_container).await.expect("expected an HTTP API URL");

    // ── 3. Announce to both tracker instances ────────────────────────
    let client = reqwest::Client::new();
    for url in &tracker_urls {
        let announce_url = url
            .join("/announce?info_hash=%9c8b%22%13%e3%0b%ff%21%2b0%c3%60%d2o%9a%02%13d%22&peer_id=-qB00000000000000001&port=17548&ip=127.0.0.1&event=started&compact=0")
            .expect("announce URL should be valid");
        let resp = client.get(announce_url.as_str()).send().await.unwrap();
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            panic!("announce to {url} failed: status {status}, body: {body}");
        }
    }

    // ── 4. Verify both announces are aggregated ──────────────────────
    let global_stats = get_tracker_statistics(&api_url, "MyAccessToken").await;
    assert_eq!(global_stats.tcp4_announces_handled, 2);

    // The tracker application and its temporary workspace are cleaned up
    // when `workspace` and `_jobs` are dropped at the end of this scope.
}

/// Global statistics with only metrics relevant to the test.
#[derive(Deserialize)]
struct PartialGlobalStatistics {
    tcp4_announces_handled: u64,
}

async fn get_tracker_statistics(api_url: &Url, token: &str) -> PartialGlobalStatistics {
    let response = TrackerApiClient::new(ConnectionInfo::authenticated(Origin::new(api_url.as_str()).unwrap(), token))
        .unwrap()
        .get_tracker_statistics(None)
        .await
        .expect("failed to get tracker statistics");

    response
        .json::<PartialGlobalStatistics>()
        .await
        .expect("Failed to parse JSON response")
}
