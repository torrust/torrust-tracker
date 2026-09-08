//! Executable-boundary per-value override contracts for the tracker CLI.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::native_tracker::{NativeTracker, NativeTrackerConfigurationSources};

#[tokio::test]
async fn it_should_select_the_health_check_bind_address_override_over_the_cli_configuration() {
    // Arrange
    let cli_port = 43155;
    let expected_health_check_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 43156);
    // The CLI file and per-value override both set `health_check_api.bind_address`.
    // The override must win.
    let sources = NativeTrackerConfigurationSources::with_cli_health_check_port(cli_port)
        .with_health_check_api_bind_address_override(expected_health_check_address);

    // Act
    let mut tracker = NativeTracker::start_with_configuration_sources(sources);
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should become ready using its health-check bind-address override");
    let actual_health_check_address = tracker
        .health_check_address()
        .expect("ready tracker should expose its health-check address");

    // Teardown
    let shutdown = tracker.gracefully_shutdown().await;

    // Assert
    assert_eq!(actual_health_check_address, expected_health_check_address);
    shutdown.expect("tracker should gracefully shut down after the configuration scenario");
}
