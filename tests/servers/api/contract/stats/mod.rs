use std::env;
use std::str::FromStr as _;
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_client::http::client::requests::announce::QueryBuilder;
use bittorrent_tracker_client::http::client::Client as HttpTrackerClient;
use reqwest::Url;
use serde::Deserialize;
use tokio::time::Duration;
use torrust_rest_tracker_api_client::connection_info::{ConnectionInfo, Origin};
use torrust_rest_tracker_api_client::v1::client::Client as TrackerApiClient;
use torrust_tracker_lib::{app, bootstrap};

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

    env::set_var("TORRUST_TRACKER_CONFIG_TOML", config_with_two_http_trackers);

    let (config, app_container) = bootstrap::app::setup();

    let app_container = Arc::new(app_container);

    let _jobs = app::start(&config, &app_container).await;

    announce_to_tracker("http://127.0.0.1:7272").await;
    announce_to_tracker("http://127.0.0.1:7373").await;

    let partial_metrics = get_partial_metrics("http://127.0.0.1:1414", "MyAccessToken").await;

    assert_eq!(partial_metrics.tcp4_announces_handled, 2);
}

/// Make a sample announce request to the tracker.
async fn announce_to_tracker(tracker_url: &str) {
    let response = HttpTrackerClient::new(Url::parse(tracker_url).unwrap(), Duration::from_secs(1))
        .unwrap()
        .announce(
            &QueryBuilder::with_default_values()
                .with_info_hash(&InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap()) // DevSkim: ignore DS173237
                .query(),
        )
        .await;

    assert!(response.is_ok());
}

/// Metrics only relevant to the test.
#[derive(Deserialize)]
struct PartialMetrics {
    tcp4_announces_handled: u64,
}

async fn get_partial_metrics(aip_url: &str, token: &str) -> PartialMetrics {
    let response = TrackerApiClient::new(ConnectionInfo::authenticated(Origin::new(aip_url).unwrap(), token))
        .unwrap()
        .get_tracker_statistics(None)
        .await;

    response
        .json::<PartialMetrics>()
        .await
        .expect("Failed to parse JSON response")
}
