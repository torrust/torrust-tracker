//! UDP banning integration test — shared global connection-ID error limit.
//!
//! Two distinguishable port-zero listeners consume one v3 global invalid-connection-ID budget.
//! The companion reverse-order target verifies that declaration order is irrelevant.
#[path = "../common/mod.rs"]
mod common;

use torrust_clock::clock;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

const UDP_TRACKER_ONE_ID: ConfigurationInstanceId = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
const UDP_TRACKER_TWO_ID: ConfigurationInstanceId = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1);
const MAX_CONNECTION_ID_ERRORS_PER_IP: u32 = 2;

const CONFIGURATION: &str = r#"
    [metadata]
    app = "torrust-tracker"
    purpose = "configuration"
    schema_version = "3.0.0"

    [logging]
    trace_filter = "off"

    [core]
    listed = false
    private = false

    [udp_tracker_server]
    max_connection_id_errors_per_ip = 2
    connection_id_validation = "strict"

    [[udp_trackers]]
    bind_address = "127.0.0.1:0"

    [[udp_trackers]]
    bind_address = "0.0.0.0:0"

    [health_check_api]
    bind_address = "127.0.0.2:0"
"#;

#[tokio::test]
async fn it_should_share_the_v3_connection_id_error_limit_across_udp_listeners() {
    // Arrange
    let fixture = common::TrackerApplicationFixture::start(CONFIGURATION).await;
    let app_container = fixture.app_container();
    let listeners = [
        common::udp_socket_addr_for_identity(app_container, UDP_TRACKER_ONE_ID).await,
        common::udp_socket_addr_for_identity(app_container, UDP_TRACKER_TWO_ID).await,
    ];

    // Act
    common::send_invalid_connection_ids_across_listeners_until_banned(&listeners, MAX_CONNECTION_ID_ERRORS_PER_IP).await;

    // Assert
    assert_eq!(
        app_container
            .udp_tracker_core_services
            .ban_service
            .read()
            .await
            .get_banned_ips_total(),
        1,
        "the one client IP should be banned after consuming the shared budget across both listeners"
    );

    fixture.shutdown().await;
}
