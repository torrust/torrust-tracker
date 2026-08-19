//! Aggregate statistics integration test — fixed-port multi-instance scenarios.
//!
//! This binary starts a tracker with two HTTP and two UDP listeners on distinct
//! fixed ports. Each protocol has one metrics-disabled and one metrics-enabled
//! listener; aggregate statistics must count only the enabled listener.
//!
//! ```text
//! cargo test --test metrics-fixed-ports
//! ```
#[path = "../common/mod.rs"]
mod common;

use torrust_clock::clock;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

/// Configuration: two HTTP and two UDP listeners on distinct fixed ports, with
/// the first listener for each protocol metrics-disabled.
const FIXED_PORT_CONFIG: &str = r#"
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
    bind_address = "0.0.0.0:17091"
    tracker_usage_statistics = false

    [[http_trackers]]
    bind_address = "0.0.0.0:17092"
    tracker_usage_statistics = true

    [[udp_trackers]]
    bind_address = "0.0.0.0:17093"
    tracker_usage_statistics = false

    [[udp_trackers]]
    bind_address = "0.0.0.0:17094"
    tracker_usage_statistics = true

    [http_api]
    bind_address = "127.0.0.1:0"

    [http_api.access_tokens]
    admin = "MyAccessToken"

    [health_check_api]
    bind_address = "127.0.0.2:0"
"#;

#[tokio::test]
async fn it_should_apply_metrics_policy_to_fixed_port_tracker_instances() {
    // Arrange
    let workspace = common::EphemeralTrackerWorkspace::new(FIXED_PORT_CONFIG);
    let (app_container, _jobs) = common::start_tracker_with_config(&workspace).await;

    // Act
    it_should_aggregate_http_announces_only_from_metrics_enabled_listener(&app_container).await;
    it_should_aggregate_udp_events_only_from_metrics_enabled_listener(&app_container).await;
}

/// Both HTTP listeners are on distinct fixed ports, but only the
/// metrics-enabled listener contributes to aggregate HTTP statistics.
async fn it_should_aggregate_http_announces_only_from_metrics_enabled_listener(
    app_container: &std::sync::Arc<torrust_tracker_lib::container::AppContainer>,
) {
    // Arrange
    let tracker_urls = common::http_tracker_urls(app_container).await;
    assert_eq!(tracker_urls.len(), 2, "expected two HTTP trackers");

    let api_url = common::http_api_url(app_container).await.expect("expected an HTTP API URL");

    let info_hash = [
        0x9c, 0x8b, 0x22, 0x13, 0xe3, 0x0b, 0xff, 0x21, 0x2b, 0x0c, 0x36, 0x0d, 0x26, 0xf9, 0xa0, 0x21, 0x31, 0x64, 0x22, 0x00,
    ];
    let peer_id = *b"-qB00000000000000001";

    // Act
    for url in &tracker_urls {
        common::http_announce(url, &info_hash, &peer_id, 17548).await;
    }

    // Assert
    let global_stats = common::get_tracker_statistics(&api_url, "MyAccessToken").await;
    assert_eq!(global_stats.tcp4_announces_handled, 1);
}

/// Both UDP listeners are on distinct fixed ports, but only the
/// metrics-enabled listener contributes to aggregate UDP statistics.
async fn it_should_aggregate_udp_events_only_from_metrics_enabled_listener(
    app_container: &std::sync::Arc<torrust_tracker_lib::container::AppContainer>,
) {
    // Arrange
    let udp_urls = common::udp_tracker_urls(app_container).await;
    assert_eq!(udp_urls.len(), 2, "expected two UDP trackers");

    let api_url = common::http_api_url(app_container).await.expect("expected an HTTP API URL");

    let info_hash = [
        0x9c, 0x8b, 0x22, 0x13, 0xe3, 0x0b, 0xff, 0x21, 0x2b, 0x0c, 0x36, 0x0d, 0x26, 0xf9, 0xa0, 0x21, 0x31, 0x64, 0x22, 0x00,
    ];
    let peer_id = *b"-qB00000000000000001";

    // Act
    for url in &udp_urls {
        let addr = common::udp_socket_addr(url);
        common::udp_announce(addr, &info_hash, &peer_id, 17548).await;
    }

    // Assert
    let global_stats = common::get_tracker_statistics(&api_url, "MyAccessToken").await;
    assert_eq!(global_stats.udp4_announces_handled, 1);
    assert_eq!(global_stats.udp4_requests, 2);
    assert_eq!(global_stats.udp4_connections_handled, 1);
    assert_eq!(global_stats.udp4_responses, 2);
    assert_eq!(global_stats.udp4_errors_handled, 0);
    assert_eq!(global_stats.udp_requests_banned, 0);
    assert_eq!(global_stats.udp_banned_ips_total, 0);
}
