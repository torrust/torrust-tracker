use std::env;
use std::str::FromStr as _;

use reqwest::Url;
use serde::Deserialize;
use tokio::time::Duration;
use torrust_info_hash::InfoHash;
use torrust_tracker_client::http::client::Client as HttpTrackerClient;
use torrust_tracker_http_protocol::v1::requests::announce::AnnounceBuilder;
use torrust_tracker_lib::app;
use torrust_tracker_rest_api_client::connection_info::{ConnectionInfo, Origin};
use torrust_tracker_rest_api_client::v1::client::ApiHttpClient as TrackerApiClient;

#[tokio::test]
async fn the_stats_api_endpoint_should_return_the_global_stats() {
    // Logging must be OFF otherwise your will get the following error:
    // `Unable to install global subscriber: SetGlobalDefaultError("a global default trace dispatcher has already been set")`
    // That's because we can't initialize the logger twice.
    // You can enable it if you run only this test.
    let config_with_two_http_trackers = r#"
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
        path = "./integration_tests_sqlite3.db"

        [[http_trackers]]
        bind_address = "0.0.0.0:7272"
        tracker_usage_statistics = true

        [[http_trackers]]
        bind_address = "0.0.0.0:7373"
        tracker_usage_statistics = true

        [http_api]
        bind_address = "0.0.0.0:1414"

        [http_api.access_tokens]
        admin = "MyAccessToken"
            "#;

    // SAFETY: `std::env::set_var` is unsafe in Rust 2024 because concurrent reads from
    // other threads in the same process are undefined behaviour. This test is the only
    // function in this integration binary that writes `TORRUST_TRACKER_CONFIG_TOML`, and
    // each test in this file binds to unique fixed ports, making parallel execution
    // impossible (port conflicts). In practice the tests therefore run serially, but the
    // safety guarantee is not formally enforced by the test runner. For strict soundness,
    // run the integration suite with `RUST_TEST_THREADS=1`.
    #[allow(unsafe_code)]
    unsafe {
        env::set_var("TORRUST_TRACKER_CONFIG_TOML", config_with_two_http_trackers);
    }

    let (_app_container, _jobs) = app::run().await;

    announce_to_tracker("http://127.0.0.1:7272").await;
    announce_to_tracker("http://127.0.0.1:7373").await;

    let global_stats = get_tracker_statistics("http://127.0.0.1:1414", "MyAccessToken").await;

    assert_eq!(global_stats.tcp4_announces_handled, 2);
}

/// Make a sample announce request to the tracker.
async fn announce_to_tracker(tracker_url: &str) {
    let response = HttpTrackerClient::new(Url::parse(tracker_url).unwrap(), Duration::from_secs(1))
        .unwrap()
        .announce(
            &AnnounceBuilder::with_default_values()
                .with_info_hash(&InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap()) // DevSkim: ignore DS173237
                .query(),
        )
        .await;

    assert!(response.is_ok());
}

/// Global statistics with only metrics relevant to the test.
#[derive(Deserialize)]
struct PartialGlobalStatistics {
    tcp4_announces_handled: u64,
}

async fn get_tracker_statistics(aip_url: &str, token: &str) -> PartialGlobalStatistics {
    let response = TrackerApiClient::new(ConnectionInfo::authenticated(Origin::new(aip_url).unwrap(), token))
        .unwrap()
        .get_tracker_statistics(None)
        .await;

    response
        .json::<PartialGlobalStatistics>()
        .await
        .expect("Failed to parse JSON response")
}
