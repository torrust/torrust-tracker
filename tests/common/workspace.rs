//! Tracker workspace and URL discovery helpers.

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
    // We require at least two services to be registered before proceeding.
    // This covers the common case of one HTTP tracker plus one UDP tracker.
    // We intentionally do NOT wait for all services (HTTP API, health check,
    // etc.) because scenarios only need the tracker listeners to be ready.
    // Configurations with fewer services (e.g., health-check only) should
    // use a lower threshold or bypass this wait.
    const MIN_REGISTERED_SERVICES: usize = 2;

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

    // Wait for services to register in the registar and bind to ports.
    // Polls the registar instead of using a fixed sleep to avoid
    // flakiness on slow machines and unnecessary delay on fast ones.
    //
    // TODO: This gate can pass before the specific services scenarios need are
    // registered (e.g., if HTTP API + health check register first). Consider
    // waiting on concrete predicates per test binary when flakiness appears.
    // Tracked by #1430.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);

    loop {
        let entries = container.registar.entries();
        let map = entries.lock().await;
        if map.len() >= MIN_REGISTERED_SERVICES {
            break;
        }
        drop(map);
        assert!(
            std::time::Instant::now() < deadline,
            "timeout waiting for services to register in the registar"
        );
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    (container, jobs)
}

/// Returns the HTTP tracker URLs from the registar.
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

/// Returns the UDP tracker URLs from the registar.
///
/// UDP trackers bind to `0.0.0.0` (unspecified). We identify them by their
/// unspecified IP, which is deterministic regardless of hash-map ordering.
/// Wildcard addresses are converted to `127.0.0.1` for client requests.
//
// Each integration-test binary compiles this module independently. Not all
// binaries call every function here, so the compiler emits dead_code warnings
// for the binaries that don't. The attribute suppresses those per-binary
// false positives without hiding genuine dead code in the workspace as a whole.
#[allow(dead_code)]
pub async fn udp_tracker_urls(container: &AppContainer) -> Vec<Url> {
    let reg = container.registar.entries();
    let map = reg.lock().await;
    map.keys()
        .filter(|b| {
            b.protocol() == torrust_net_primitives::service_binding::Protocol::UDP && b.bind_address().ip().is_unspecified()
        })
        .map(|b| udp_loopback_url(b.bind_address()))
        .collect()
}

/// Returns the HTTP API URL from the registar.
///
/// The REST API binds to `127.0.0.1` (loopback), unlike the HTTP trackers
/// which bind to `0.0.0.0`. We filter specifically for the REST API bind IP
/// (`127.0.0.1`) to avoid matching the health-check API on `127.0.0.2`.
pub async fn http_api_url(container: &AppContainer) -> Option<Url> {
    let reg = container.registar.entries();
    let map = reg.lock().await;
    map.keys()
        .find(|b| {
            b.protocol() == torrust_net_primitives::service_binding::Protocol::HTTP
                && b.bind_address().ip() == std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
        })
        .map(|b| loopback_url(b.bind_address()))
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
        Url::parse(&format!("http://{addr}")) // DevSkim: ignore DS137138
    }
    .expect("loopback URL should always be valid")
}

/// Convert a UDP socket address to a connectable loopback URL.
// Not called by every integration-test binary — see note on `udp_tracker_urls`.
#[allow(dead_code)]
fn udp_loopback_url(addr: SocketAddr) -> Url {
    if addr.ip().is_unspecified() {
        Url::parse(&format!("udp://127.0.0.1:{port}", port = addr.port()))
    } else {
        Url::parse(&format!("udp://{addr}"))
    }
    .expect("loopback URL should always be valid")
}

/// Extract the `SocketAddr` from a `udp://` URL.
//
// Uses the `Url` host/port accessors rather than slicing the URL string.
// Not called by every integration-test binary — see note on `udp_tracker_urls`.
#[allow(dead_code)]
pub fn udp_socket_addr(url: &Url) -> SocketAddr {
    let host = url
        .host_str()
        .expect("UDP URL must have a host")
        .parse()
        .expect("UDP URL host must be a valid IP");
    let port = url.port().expect("UDP URL must have a port");
    SocketAddr::new(host, port)
}
