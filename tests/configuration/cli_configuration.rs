//! Executable-boundary configuration contracts for the tracker CLI.

#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

#[cfg(unix)]
#[path = "../common/native_tracker.rs"]
#[allow(dead_code)]
mod native_tracker;

#[cfg(unix)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[cfg(unix)]
use native_tracker::{NativeTracker, NativeTrackerConfigurationSources};

#[cfg(unix)]
#[tokio::test]
async fn it_should_select_the_cli_configuration_when_both_child_environment_base_sources_are_set() {
    // Arrange
    let cli_port = 43152;
    let environment_path_port = 43153;
    let environment_toml_port = 43154;
    let sources = NativeTrackerConfigurationSources::with_cli_health_check_port(cli_port)
        .with_environment_path_health_check_port(environment_path_port)
        .with_environment_toml_health_check_port(environment_toml_port);
    let mut tracker = NativeTracker::start_with_configuration_sources(sources);

    // Act
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should become ready using its CLI configuration");
    let health_check_address = tracker
        .health_check_address()
        .expect("ready tracker should expose its health-check address");
    let shutdown = tracker.gracefully_shutdown().await;

    // Assert
    assert_eq!(
        health_check_address,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), cli_port)
    );
    assert_ne!(health_check_address.port(), environment_path_port);
    assert_ne!(health_check_address.port(), environment_toml_port);
    shutdown.expect("tracker should gracefully shut down after the configuration scenario");
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_select_the_child_only_health_check_bind_address_override_over_the_cli_configuration() {
    // Arrange
    let cli_port = 43155;
    let override_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 43156);
    let sources = NativeTrackerConfigurationSources::with_cli_health_check_port(cli_port)
        .with_health_check_api_bind_address_override(override_address);
    let mut tracker = NativeTracker::start_with_configuration_sources(sources);

    // Act
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should become ready using its health-check bind-address override");
    let health_check_address = tracker
        .health_check_address()
        .expect("ready tracker should expose its health-check address");
    let shutdown = tracker.gracefully_shutdown().await;

    // Assert
    assert_eq!(health_check_address, override_address);
    assert_ne!(health_check_address.port(), cli_port);
    shutdown.expect("tracker should gracefully shut down after the configuration scenario");
}

#[cfg(not(unix))]
#[test]
fn it_should_skip_native_cli_configuration_scenarios_on_non_unix_platforms() {
    // The shared native fixture currently uses Unix process exit-status extensions.
}
