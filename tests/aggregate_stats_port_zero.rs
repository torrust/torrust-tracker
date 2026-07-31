//! Statistics integration test — aggregate statistics with port-zero listeners.
//!
//! This binary starts a tracker with two HTTP and two UDP listeners on port
//! zero, all enabled. Scenario functions verify that aggregate statistics
//! count announces from all listeners.
//!
//! ```text
//! cargo test --test aggregate_stats_port_zero
//! ```
mod common;

use torrust_clock::clock;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

/// Configuration: two HTTP and two UDP listeners on port zero, with different
/// settings per instance to prove bootstrap assigns distinct containers.
const PORT_ZERO_CONFIG: &str = r#"
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
    tracker_usage_statistics = false

    [[http_trackers]]
    bind_address = "0.0.0.0:0"
    tracker_usage_statistics = true

    [[udp_trackers]]
    bind_address = "0.0.0.0:0"
    tracker_usage_statistics = false

    [[udp_trackers]]
    bind_address = "0.0.0.0:0"
    tracker_usage_statistics = true

    [http_api]
    bind_address = "127.0.0.1:0"

    [http_api.access_tokens]
    admin = "MyAccessToken"

    [health_check_api]
    bind_address = "127.0.0.2:0"
"#;

#[tokio::test]
async fn stats_scenarios() {
    let workspace = common::EphemeralTrackerWorkspace::new(PORT_ZERO_CONFIG);
    let (app_container, _jobs) = common::start_tracker_with_config(&workspace).await;

    duplicate_port_zero_instances_should_receive_distinct_configurations(&app_container);
    duplicate_port_zero_instances_should_retain_runtime_identity(&app_container).await;
    two_http_trackers_on_port_zero_should_aggregate_announces_from_both_listeners(&app_container).await;
    two_udp_trackers_on_port_zero_should_aggregate_announces_from_both_listeners(&app_container).await;
}

/// Repeated configuration blocks must retain their canonical identity after
/// receiving their distinct operating-system-assigned final bindings.
async fn duplicate_port_zero_instances_should_retain_runtime_identity(
    app_container: &std::sync::Arc<torrust_tracker_lib::container::AppContainer>,
) {
    for service_role in [ServiceRole::HttpTracker, ServiceRole::UdpTracker] {
        let first = common::service_binding_for_identity(app_container, ConfigurationInstanceId::new(service_role, 0))
            .await
            .expect("first configured instance should be registered");
        let second = common::service_binding_for_identity(app_container, ConfigurationInstanceId::new(service_role, 1))
            .await
            .expect("second configured instance should be registered");

        assert_ne!(first.bind_address().port(), 0);
        assert_ne!(second.bind_address().port(), 0);
        assert_ne!(first.bind_address(), second.bind_address());
    }
}

/// Duplicate port-zero configuration blocks each receive their own container
/// with distinct settings, proving the bootstrap fix prevents the
/// address-keyed collision.
fn duplicate_port_zero_instances_should_receive_distinct_configurations(
    app_container: &std::sync::Arc<torrust_tracker_lib::container::AppContainer>,
) {
    // HTTP: first instance should have statistics disabled, second enabled.
    assert_eq!(app_container.http_tracker_instance_containers.len(), 2);
    assert!(
        !app_container.http_tracker_instance_containers[0]
            .1
            .http_tracker_config
            .tracker_usage_statistics
    );
    assert!(
        app_container.http_tracker_instance_containers[1]
            .1
            .http_tracker_config
            .tracker_usage_statistics
    );

    // UDP: first instance should have statistics disabled, second enabled.
    assert_eq!(app_container.udp_tracker_instance_containers.len(), 2);
    assert!(
        !app_container.udp_tracker_instance_containers[0]
            .1
            .udp_tracker_config
            .tracker_usage_statistics
    );
    assert!(
        app_container.udp_tracker_instance_containers[1]
            .1
            .udp_tracker_config
            .tracker_usage_statistics
    );
}

/// Both HTTP listeners on port zero. Announces to both should be counted
/// in the aggregate HTTP statistics.
async fn two_http_trackers_on_port_zero_should_aggregate_announces_from_both_listeners(
    app_container: &std::sync::Arc<torrust_tracker_lib::container::AppContainer>,
) {
    let tracker_urls = common::http_tracker_urls(app_container).await;
    assert_eq!(tracker_urls.len(), 2, "expected two HTTP trackers");

    let api_url = common::http_api_url(app_container).await.expect("expected an HTTP API URL");

    let info_hash = [
        0x9c, 0x8b, 0x22, 0x13, 0xe3, 0x0b, 0xff, 0x21, 0x2b, 0x0c, 0x36, 0x0d, 0x26, 0xf9, 0xa0, 0x21, 0x31, 0x64, 0x22, 0x00,
    ];
    let peer_id = *b"-qB00000000000000001";

    for url in &tracker_urls {
        common::http_announce(url, &info_hash, &peer_id, 17548).await;
    }

    let global_stats = common::get_tracker_statistics(&api_url, "MyAccessToken").await;
    assert_eq!(global_stats.tcp4_announces_handled, 2);
}

/// Both UDP listeners on port zero. Announces to both should be counted
/// in the aggregate UDP statistics.
async fn two_udp_trackers_on_port_zero_should_aggregate_announces_from_both_listeners(
    app_container: &std::sync::Arc<torrust_tracker_lib::container::AppContainer>,
) {
    let udp_urls = common::udp_tracker_urls(app_container).await;
    assert_eq!(udp_urls.len(), 2, "expected two UDP trackers");

    let api_url = common::http_api_url(app_container).await.expect("expected an HTTP API URL");

    let info_hash = [
        0x9c, 0x8b, 0x22, 0x13, 0xe3, 0x0b, 0xff, 0x21, 0x2b, 0x0c, 0x36, 0x0d, 0x26, 0xf9, 0xa0, 0x21, 0x31, 0x64, 0x22, 0x00,
    ];
    let peer_id = *b"-qB00000000000000001";

    for url in &udp_urls {
        let addr = common::udp_socket_addr(url);
        common::udp_announce(addr, &info_hash, &peer_id, 17548).await;
    }

    let global_stats = common::get_tracker_statistics(&api_url, "MyAccessToken").await;
    assert_eq!(global_stats.udp4_announces_handled, 2);
}
