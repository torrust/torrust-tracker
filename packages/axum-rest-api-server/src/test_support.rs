#![cfg(test)]
//! Crate-private helpers for the unit tests of this package.
//!
//! This module is compiled only for unit tests and is not part of the public API. Code that other packages' tests
//! use lives in [`crate::testing`] instead.
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use axum::body::{Body, Bytes, to_bytes};
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::Response;
use tempfile::TempDir;
use torrust_clock::DurationSinceUnixEpoch;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric_collection::MetricCollection;
use torrust_metrics::metric_name;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::{LabeledStats, Stats};
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

/// The absolute time that one test may spend awaiting I/O (container setup, a request and its body).
pub const TEST_DEADLINE: Duration = Duration::from_secs(5);

/// Awaits a future that does real I/O, failing the test instead of hanging it if the work does not finish within
/// [`TEST_DEADLINE`].
///
/// # Panics
///
/// Panics if the future does not complete within the deadline.
pub async fn within_deadline<T>(future: impl Future<Output = T>) -> T {
    tokio::time::timeout(TEST_DEADLINE, future)
        .await
        .expect("the awaited work finishes within the test deadline")
}

/// A REST API container together with the temporary directory that holds its SQLite database.
///
/// The container is declared first so it is dropped (and the database released) before the directory is removed, also
/// when the test fails by panicking.
pub struct TestApiContainer {
    pub container: Arc<TrackerHttpApiCoreContainer>,
    _database_directory: TempDir,
}

/// Builds the REST API container for a configuration, as the application does at startup.
///
/// A SQLite database of the configuration is moved into a temporary directory that the returned value removes when it
/// is dropped, so a test run leaves no database file behind.
///
/// # Panics
///
/// Panics if the configuration enables no HTTP tracker, no UDP tracker or no HTTP API, or the temporary directory
/// cannot be created.
pub async fn api_container(configuration: &Configuration) -> TestApiContainer {
    let database_directory = tempfile::tempdir().expect("a temporary directory can be created");
    let mut configuration = configuration.clone();
    if let Some(Database::Sqlite3 { path }) = configuration.core.database.as_mut() {
        *path = database_directory.path().join("tracker.db").to_string_lossy().into_owned();
    }
    let configuration = &configuration;

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

    let container = TrackerHttpApiCoreContainer::initialize(
        &core_config,
        &http_tracker_config,
        ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
        &udp_tracker_config,
        &configuration.udp_tracker_server,
        ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
        &http_api_config,
    )
    .await;

    TestApiContainer {
        container,
        _database_directory: database_directory,
    }
}

/// Statistics whose fields are all different (the value is the position of the field, from 1 to 32), so a field that a
/// test reads or prints in the wrong place shows. The one boolean, `completed_persisted_enabled`, is `true`.
#[must_use]
pub const fn a_stats() -> Stats {
    Stats {
        torrents: 1,
        seeders: 2,
        completed: 3,
        completed_in_session: 4,
        completed_persisted: 5,
        completed_persisted_enabled: true,
        leechers: 7,
        tcp4_connections_handled: 8,
        tcp4_announces_handled: 9,
        tcp4_scrapes_handled: 10,
        tcp6_connections_handled: 11,
        tcp6_announces_handled: 12,
        tcp6_scrapes_handled: 13,
        udp_requests_discarded: 14,
        udp_requests_aborted: 15,
        udp_requests_banned: 16,
        udp_banned_ips_total: 17,
        udp_avg_connect_processing_time_ns: 18,
        udp_avg_announce_processing_time_ns: 19,
        udp_avg_scrape_processing_time_ns: 20,
        udp4_requests: 21,
        udp4_connections_handled: 22,
        udp4_announces_handled: 23,
        udp4_scrapes_handled: 24,
        udp4_responses: 25,
        udp4_errors_handled: 26,
        udp6_requests: 27,
        udp6_connections_handled: 28,
        udp6_announces_handled: 29,
        udp6_scrapes_handled: 30,
        udp6_responses: 31,
        udp6_errors_handled: 32,
    }
}

/// Labeled statistics with one counter, `requests_total`, whose value is 1 and that has no labels.
///
/// # Panics
///
/// Panics if the counter cannot be recorded, which cannot happen in an empty collection.
#[must_use]
pub fn a_labeled_stats() -> LabeledStats {
    let mut metrics = MetricCollection::default();
    metrics
        .increment_counter(
            &metric_name!("requests_total"),
            &LabelSet::default(),
            DurationSinceUnixEpoch::from_secs(1),
        )
        .expect("the counter can be recorded in an empty collection");

    LabeledStats { metrics }
}
