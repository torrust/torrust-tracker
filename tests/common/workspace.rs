//! Tracker workspace and URL discovery helpers.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock};

use tempfile::TempDir;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_lib::app;
use torrust_tracker_lib::bootstrap::jobs::manager::JobManager;
use torrust_tracker_lib::container::AppContainer;
use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
use url::Url;

/// Maximum time to await each tracker job after requesting cancellation.
const TRACKER_SHUTDOWN_GRACE_PERIOD: std::time::Duration = std::time::Duration::from_secs(10);

static ENVIRONMENT_LOCK: LazyLock<tokio::sync::Mutex<()>> = LazyLock::new(|| tokio::sync::Mutex::new(()));

struct ConfigurationEnvironmentGuard {
    original_path: Option<std::ffi::OsString>,
    original_toml: Option<std::ffi::OsString>,
}

impl ConfigurationEnvironmentGuard {
    #[allow(unsafe_code)]
    fn replace(path: &Path) -> Self {
        let original_path = std::env::var_os("TORRUST_TRACKER_CONFIG_TOML_PATH");
        let original_toml = std::env::var_os("TORRUST_TRACKER_CONFIG_TOML");

        // SAFETY: `ENVIRONMENT_LOCK` serializes configuration environment access in this test executable.
        unsafe {
            std::env::remove_var("TORRUST_TRACKER_CONFIG_TOML");
            std::env::set_var("TORRUST_TRACKER_CONFIG_TOML_PATH", path);
        }

        Self {
            original_path,
            original_toml,
        }
    }
}

impl Drop for ConfigurationEnvironmentGuard {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        // SAFETY: `ENVIRONMENT_LOCK` is held for the full guard lifetime.
        unsafe {
            if let Some(path) = &self.original_path {
                std::env::set_var("TORRUST_TRACKER_CONFIG_TOML_PATH", path);
            } else {
                std::env::remove_var("TORRUST_TRACKER_CONFIG_TOML_PATH");
            }
            if let Some(toml) = &self.original_toml {
                std::env::set_var("TORRUST_TRACKER_CONFIG_TOML", toml);
            } else {
                std::env::remove_var("TORRUST_TRACKER_CONFIG_TOML");
            }
        }
    }
}

/// A temporary workspace for an integration test.
///
/// Creates an isolated directory with config file and storage directory.
/// The `{STORAGE_PATH}` placeholder in the config TOML is replaced with
/// the absolute path to the temp storage directory.
pub struct EphemeralTrackerWorkspace {
    temp_dir: TempDir,
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

        Self { temp_dir, config_path }
    }

    #[must_use]
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    #[must_use]
    // Each integration target compiles this shared module independently; only
    // the lifecycle coverage target queries the workspace path.
    #[allow(dead_code)]
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }
}

/// Owns a tracker application and its isolated test workspace.
///
/// Call [`Self::shutdown`] explicitly after the suite scenarios finish. It
/// cancels and awaits the tracker jobs before releasing the application and
/// temporary workspace. Rust `Drop` cannot perform this asynchronous teardown.
pub struct TrackerApplicationFixture {
    app_container: Arc<AppContainer>,
    jobs: Option<JobManager>,
    workspace: EphemeralTrackerWorkspace,
}

impl TrackerApplicationFixture {
    /// Starts one tracker application in an isolated workspace.
    pub async fn start(config_toml: &str) -> Self {
        let workspace = EphemeralTrackerWorkspace::new(config_toml);
        let (app_container, jobs) = start_tracker_with_config(&workspace).await;

        Self {
            app_container,
            jobs: Some(jobs),
            workspace,
        }
    }

    /// Returns the application container used by suite scenarios.
    #[must_use]
    pub const fn app_container(&self) -> &Arc<AppContainer> {
        &self.app_container
    }

    /// Returns the temporary workspace path for lifecycle assertions.
    #[must_use]
    // See the per-target compilation note on `EphemeralTrackerWorkspace::path`.
    #[allow(dead_code)]
    pub fn workspace_path(&self) -> PathBuf {
        self.workspace.path().to_path_buf()
    }

    /// Gracefully stops tracker jobs before releasing the workspace.
    pub async fn shutdown(mut self) {
        let jobs = self.jobs.take().expect("tracker jobs must be available before shutdown");
        jobs.cancel();
        jobs.wait_for_all(TRACKER_SHUTDOWN_GRACE_PERIOD).await;
    }
}

impl Drop for TrackerApplicationFixture {
    fn drop(&mut self) {
        if let Some(jobs) = &self.jobs {
            jobs.cancel();
        }
    }
}

/// Starts the tracker application with the given workspace config.
///
/// Configuration environment access is serialized and restored before this
/// function returns, so tests in the same executable remain isolated.
///
pub async fn start_tracker_with_config(workspace: &EphemeralTrackerWorkspace) -> (Arc<AppContainer>, JobManager) {
    let (container, jobs) = {
        let _environment_lock = ENVIRONMENT_LOCK.lock().await;
        let _environment_guard = ConfigurationEnvironmentGuard::replace(workspace.config_path());
        app::start().await.expect("tracker application should start")
    };

    // Each service acknowledges registry insertion only after binding its
    // final listener. Wait for the exact configuration identities, rather than
    // a map-size threshold or a registration delay.
    let expected_identities = expected_service_identities(&container);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);

    loop {
        let services = container.registar.services().await;
        if expected_identities.iter().all(|identity| {
            services
                .iter()
                .any(|service| service.metadata().configuration_instance_id() == *identity)
        }) {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "timeout waiting for configured services to register in the registar"
        );
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    (container, jobs)
}

/// Returns the HTTP tracker URLs from the registar.
///
/// Uses the canonical HTTP tracker role, not a bind-IP convention. Wildcard
/// addresses are converted to `127.0.0.1` for client requests.
#[allow(dead_code)]
pub async fn http_tracker_urls(container: &AppContainer) -> Vec<Url> {
    container
        .registar
        .services_matching(|metadata| metadata.service_role() == ServiceRole::HttpTracker)
        .await
        .iter()
        .map(|service| loopback_url(service.service_binding().bind_address()))
        .collect()
}

/// Returns the UDP tracker URLs from the registar.
///
/// Uses the canonical UDP tracker role, not a bind-IP convention. Wildcard
/// addresses are converted to `127.0.0.1` for client requests.
//
// Each integration-test binary compiles this module independently. Not all
// binaries call every function here, so the compiler emits dead_code warnings
// for the binaries that don't. The attribute suppresses those per-binary
// false positives without hiding genuine dead code in the workspace as a whole.
#[allow(dead_code)]
pub async fn udp_tracker_urls(container: &AppContainer) -> Vec<Url> {
    container
        .registar
        .services_matching(|metadata| metadata.service_role() == ServiceRole::UdpTracker)
        .await
        .iter()
        .map(|service| udp_loopback_url(service.service_binding().bind_address()))
        .collect()
}

/// Returns the HTTP API URL from the registar.
///
/// Uses the canonical REST API role, not a bind-IP convention.
#[allow(dead_code)]
pub async fn http_api_url(container: &AppContainer) -> Option<Url> {
    container
        .registar
        .services_matching(|metadata| metadata.service_role() == ServiceRole::RestApi)
        .await
        .first()
        .map(|service| loopback_url(service.service_binding().bind_address()))
}

/// Returns the final binding for one exact canonical configuration identity.
///
/// This is side-effect free: registry visibility acknowledges that the service
/// has bound this listener.
#[allow(dead_code)]
pub async fn service_binding_for_identity(
    container: &AppContainer,
    configuration_instance_id: ConfigurationInstanceId,
) -> Option<ServiceBinding> {
    container
        .registar
        .services_matching(|metadata| metadata.configuration_instance_id() == configuration_instance_id)
        .await
        .into_iter()
        .next()
        .map(|service| service.service_binding().clone())
}

/// Returns a connectable UDP socket address for a configuration identity.
#[allow(dead_code)]
pub async fn udp_socket_addr_for_identity(
    container: &AppContainer,
    configuration_instance_id: ConfigurationInstanceId,
) -> SocketAddr {
    let binding = service_binding_for_identity(container, configuration_instance_id)
        .await
        .expect("configured UDP tracker should be registered");
    let address = binding.bind_address();

    SocketAddr::new(
        if address.ip().is_unspecified() {
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)
        } else {
            address.ip()
        },
        address.port(),
    )
}

fn expected_service_identities(container: &AppContainer) -> Vec<ConfigurationInstanceId> {
    let mut identities: Vec<_> = container
        .http_tracker_instance_containers
        .iter()
        .map(|(identity, _)| *identity)
        .chain(
            container
                .udp_tracker_instance_containers
                .iter()
                .map(|(identity, _)| *identity),
        )
        .collect();

    if container.http_api_config.is_some() {
        identities.push(ConfigurationInstanceId::new(ServiceRole::RestApi, 0));
    }

    identities.push(ConfigurationInstanceId::new(ServiceRole::HealthCheckApi, 0));

    identities
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
