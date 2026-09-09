//! Executable-boundary base-source precedence contracts for the tracker CLI.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::native_tracker::{NativeTracker, NativeTrackerConfigurationSources};

#[tokio::test]
async fn it_should_select_the_cli_configuration_when_both_child_environment_base_sources_are_set() {
    // Arrange
    let cli_port = 43152;
    let environment_path_port = 43153;
    let environment_toml_port = 43154;
    let expected_health_check_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), cli_port);
    // All three base sources set `health_check_api.bind_address` differently.
    // The CLI file must win over both environment base sources.
    let sources = NativeTrackerConfigurationSources::with_cli_health_check_port(cli_port)
        .with_environment_path_health_check_port(environment_path_port)
        .with_environment_toml_health_check_port(environment_toml_port);

    // Act
    let mut tracker = NativeTracker::start_with_configuration_sources(sources);
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should become ready using its CLI configuration");
    let actual_health_check_address = tracker
        .health_check_address()
        .expect("ready tracker should expose its health-check address");

    // Teardown
    let shutdown = tracker.gracefully_shutdown().await;

    // Assert
    assert_eq!(actual_health_check_address, expected_health_check_address);
    shutdown.expect("tracker should gracefully shut down after the configuration scenario");
}
