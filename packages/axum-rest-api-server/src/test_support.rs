#![cfg(test)]
//! Crate-private helpers for the unit tests of this package.
//!
//! This module is compiled only for unit tests and is not part of the public API. Code that other packages' tests
//! use lives in [`crate::testing`] instead.
use std::sync::Arc;

use axum::body::{Body, Bytes, to_bytes};
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::Response;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;

/// The most bytes that a test reads from a response body. The bodies under test are small, so a larger one is a
/// regression that should fail the test instead of being buffered.
pub const MAX_BODY_BYTES: usize = 1024 * 1024;

/// Reads a response body, failing the test if it is larger than [`MAX_BODY_BYTES`].
///
/// # Panics
///
/// Panics if the body cannot be read or exceeds the limit.
pub async fn read_body(body: Body) -> Bytes {
    to_bytes(body, MAX_BODY_BYTES)
        .await
        .expect("the body is readable and within the limit")
}

/// What a client receives from a response: the status, the content type (if any) and the text body.
#[derive(Debug, PartialEq, Eq)]
pub struct Observed {
    pub status: StatusCode,
    pub content_type: Option<String>,
    pub body: String,
}

impl Observed {
    /// The expected value for a response that carries a content type.
    #[must_use]
    pub fn new(status: StatusCode, content_type: &str, body: &str) -> Self {
        Self {
            status,
            content_type: Some(content_type.to_string()),
            body: body.to_string(),
        }
    }
}

/// Reads the status, the content type and the text body of a response.
///
/// # Panics
///
/// Panics if the content type or the body is not text, or the body cannot be read within [`MAX_BODY_BYTES`].
pub async fn observed(response: Response) -> Observed {
    let status = response.status();
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .map(|value| value.to_str().expect("the content type is text").to_string());
    let body = read_body(response.into_body()).await;

    Observed {
        status,
        content_type,
        body: String::from_utf8(body.to_vec()).expect("the body is text"),
    }
}

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
