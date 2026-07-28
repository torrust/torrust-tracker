//! Shared test utilities for integration tests.
//!
//! This module is shared across multiple integration-test binaries via
//! `mod common;`. Each top-level file under `tests/` is a separate Cargo
//! integration-test executable. Common helpers belong here rather than in
//! a top-level file, so all test binaries can reach them.
//!
//! # Architecture
//!
//! Each integration-test binary manages **one** tracker application instance
//! with a fixed initial configuration. Scenario functions run sequentially
//! against that instance. A different initial configuration belongs to
//! another top-level binary, which Cargo may run concurrently.
//!
//! See `docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level.md`
//! for the full decision record.
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tempfile::TempDir;
use torrust_tracker_lib::app;
use torrust_tracker_lib::bootstrap::jobs::manager::JobManager;
use torrust_tracker_lib::container::AppContainer;
use url::Url;

/// A temporary workspace for an integration test.
///
/// Creates an isolated directory with config file and storage directory.
/// The `{STORAGE_PATH}` placeholder in the config TOML is replaced with
/// the absolute path to the temp storage directory.
pub struct EphemeralTrackerWorkspace {
    _temp_dir: TempDir,
    config_path: PathBuf,
}

impl EphemeralTrackerWorkspace {
    #[must_use]
    pub fn new(config_toml: &str) -> Self {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let storage_path = temp_dir.path().join("tracker-storage");
        std::fs::create_dir_all(&storage_path).expect("failed to create storage dir");

        let config_path = temp_dir.path().join("tracker-config.toml");
        let resolved = config_toml.replace("{STORAGE_PATH}", &storage_path.to_string_lossy());
        std::fs::write(&config_path, resolved).expect("failed to write config file");

        Self {
            _temp_dir: temp_dir,
            config_path,
        }
    }

    #[must_use]
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
}

/// Starts the tracker application with the given workspace config.
///
/// Since the application reads its configuration from the
/// `TORRUST_TRACKER_CONFIG_TOML_PATH` environment variable,
/// tests in this binary must not run concurrently with other tests
/// that modify the same variable.
///
/// A short delay is added after startup to allow services to register
/// in the registar and bind to OS-assigned ports.
pub async fn start_tracker_with_config(workspace: &EphemeralTrackerWorkspace) -> (Arc<AppContainer>, JobManager) {
    // SAFETY: This binary must be the only test executable setting
    // `TORRUST_TRACKER_CONFIG_TOML_PATH`. Cargo may run different
    // integration-test binaries in parallel, but each binary is a
    // separate OS process with its own environment.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(
            "TORRUST_TRACKER_CONFIG_TOML_PATH",
            workspace.config_path().to_str().expect("config path must be valid UTF-8"),
        );
    }
    let (container, jobs) = app::run().await;
    // Allow spawned tasks (registar insertions, listener binds) to
    // complete before we attempt to read the registar or make requests.
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    (container, jobs)
}

/// Returns the HTTP tracker URLs from the registar.
///
/// TODO: Replace this temporary bind-IP classification after
/// `fix-duplicate-port-zero-tracker-instance-bootstrap` and
/// `add-runtime-service-registry-metadata` are implemented. Those issues establish stable
/// configuration-instance identity and role-based runtime discovery.
///
/// HTTP trackers bind to `0.0.0.0` (unspecified). The REST API and health
/// check bind to `127.0.0.1` (loopback). We identify trackers by their
/// unspecified IP, which is deterministic regardless of hash-map ordering.
/// Wildcard addresses are converted to `127.0.0.1` for client requests.
pub async fn http_tracker_urls(container: &AppContainer) -> Vec<Url> {
    let reg = container.registar.entries();
    let map = reg.lock().await;
    map.keys()
        .filter(|b| {
            b.protocol() == torrust_net_primitives::service_binding::Protocol::HTTP && b.bind_address().ip().is_unspecified()
        })
        .map(|b| loopback_url(b.bind_address()))
        .collect()
}

/// Returns the HTTP API URL from the registar.
///
/// TODO: Replace this temporary bind-IP classification with role-based runtime discovery. See
/// `docs/issues/open/2036-add-runtime-service-registry-metadata/ISSUE.md`.
///
/// The REST API binds to `127.0.0.1` (loopback), unlike the HTTP trackers
/// which bind to `0.0.0.0`.
pub async fn http_api_url(container: &AppContainer) -> Option<Url> {
    let reg = container.registar.entries();
    let map = reg.lock().await;
    map.keys()
        .find(|b| b.protocol() == torrust_net_primitives::service_binding::Protocol::HTTP && b.bind_address().ip().is_loopback())
        .map(|b| loopback_url(b.bind_address()))
}

/// Returns all registered service bindings for diagnostic purposes.
#[allow(dead_code)]
pub async fn all_service_bindings(container: &AppContainer) -> Vec<(String, Url)> {
    let reg = container.registar.entries();
    let map = reg.lock().await;
    map.keys()
        .map(|b| (b.protocol().to_string(), loopback_url(b.bind_address())))
        .collect()
}

/// Convert a socket address to a connectable loopback URL.
///
/// Tracker services bind to `0.0.0.0` (all interfaces), but clients must
/// connect to a reachable address. This replaces wildcard IPv4 with the
/// loopback address `127.0.0.1`, preserving the OS-assigned port.
fn loopback_url(addr: SocketAddr) -> Url {
    if addr.ip().is_unspecified() {
        Url::parse(&format!("http://127.0.0.1:{port}", port = addr.port()))
    } else {
        Url::parse(&format!("http://{addr}"))
    }
    .expect("loopback URL should always be valid")
}
