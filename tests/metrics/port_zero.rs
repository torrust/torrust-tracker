//! Statistics integration test — aggregate statistics with port-zero listeners.
//!
//! This binary starts a tracker with two HTTP and two UDP listeners on port
//! zero, with metrics-disabled and metrics-enabled listeners. Scenario functions
//! verify that only enabled listeners contribute to aggregate statistics.
//!
//! ```text
//! cargo test --test metrics-port-zero
//! ```
#[path = "../common/mod.rs"]
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

#[tokio::test]
async fn it_should_apply_metrics_policy_to_port_zero_tracker_instances() {
    // Arrange
    let fixture = common::TrackerApplicationFixture::start(common::PortZeroMetricsPolicyConfiguration::TOML).await;
    let workspace_path = fixture.workspace_path();
    let app_container = fixture.app_container();

    // Assert
    it_should_preserve_distinct_configurations_for_duplicate_port_zero_instances(&app_container);
    it_should_preserve_runtime_identity_for_duplicate_port_zero_instances(&app_container).await;

    // Act and Assert
    it_should_aggregate_http_announces_only_from_metrics_enabled_port_zero_listener(&app_container).await;
    it_should_aggregate_udp_announces_only_from_metrics_enabled_port_zero_listener(&app_container).await;

    // Act
    fixture.shutdown().await;

    // Assert
    assert!(
        !workspace_path.exists(),
        "the workspace must be released only after awaited tracker shutdown"
    );
}

/// Repeated configuration blocks must retain their canonical identity after
/// receiving their distinct operating-system-assigned final bindings.
async fn it_should_preserve_runtime_identity_for_duplicate_port_zero_instances(
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
fn it_should_preserve_distinct_configurations_for_duplicate_port_zero_instances(
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

/// Both HTTP listeners use repeated port-zero bindings. Announces to both must
/// be filtered using canonical identity rather than their configured address.
async fn it_should_aggregate_http_announces_only_from_metrics_enabled_port_zero_listener(
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

/// Both UDP listeners use repeated port-zero bindings. Announces to both must
/// be filtered using canonical identity rather than their configured address.
async fn it_should_aggregate_udp_announces_only_from_metrics_enabled_port_zero_listener(
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
}
