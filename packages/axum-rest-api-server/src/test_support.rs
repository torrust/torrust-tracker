#![cfg(test)]
//! Crate-private helpers for the unit tests of this package.
//!
//! This module is compiled only for unit tests and is not part of the public API. Code that other packages' tests
//! use lives in [`crate::testing`] instead.
use std::sync::Arc;

use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;

/// The most bytes that a test reads from a response body. The bodies under test are small, so a larger one is a
/// regression that should fail the test instead of being buffered.
pub const MAX_BODY_BYTES: usize = 1024 * 1024;

/// Builds the REST API container for a configuration, as the application does at startup.
///
/// # Panics
///
/// Panics if the configuration enables no HTTP tracker, no UDP tracker or no HTTP API.
pub async fn api_container(configuration: &Configuration) -> Arc<TrackerHttpApiCoreContainer> {
    let core_config = Arc::new(configuration.core.clone());
    let http_tracker_config = Arc::new(
        configuration
            .http_trackers
            .as_ref()
            .and_then(|trackers| trackers.first())
            .expect("the configuration enables an HTTP tracker")
            .clone(),
    );
    let udp_tracker_config = Arc::new(
        configuration
            .udp_trackers
            .as_ref()
            .and_then(|trackers| trackers.first())
            .expect("the configuration enables a UDP tracker")
            .clone(),
    );
    let http_api_config = Arc::new(
        configuration
            .http_api
            .clone()
            .expect("the configuration enables the HTTP API"),
    );

    TrackerHttpApiCoreContainer::initialize(
        &core_config,
        &http_tracker_config,
        ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
        &udp_tracker_config,
        &configuration.udp_tracker_server,
        ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
        &http_api_config,
    )
    .await
}
