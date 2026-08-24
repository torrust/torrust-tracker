//! UDP error-metrics integration test — disabled port-zero listener.
#[path = "../common/mod.rs"]
mod common;

use torrust_clock::clock;

#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

#[tokio::test]
async fn it_should_not_record_cookie_error_from_metrics_disabled_port_zero_udp_listener() {
    // Arrange
    let fixture = common::TrackerApplicationFixture::start(common::PortZeroMetricsPolicyConfiguration::TOML).await;
    let app_container = fixture.app_container();
    let api_url = common::http_api_url(app_container).await.expect("expected an HTTP API URL");
    let udp_tracker_address = common::udp_socket_addr_for_identity(
        app_container,
        common::PortZeroMetricsPolicyConfiguration::METRICS_DISABLED_UDP_TRACKER_ID,
    )
    .await;
    let statistics_before = common::get_tracker_statistics(&api_url, "MyAccessToken").await;

    // Act
    let _tracker_response = common::send_invalid_connection_id_announce(udp_tracker_address).await;

    // Assert
    let statistics_after = common::get_tracker_statistics(&api_url, "MyAccessToken").await;
    assert_eq!(statistics_after.udp4_errors_handled, statistics_before.udp4_errors_handled);

    fixture.shutdown().await;
}
