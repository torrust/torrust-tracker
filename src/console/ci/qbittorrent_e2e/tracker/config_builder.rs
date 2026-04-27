//! Builder for the Torrust Tracker configuration file written into the E2E workspace.
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};

use anyhow::Context;
use torrust_tracker_configuration::{Configuration, HealthCheckApi, HttpApi, HttpTracker, UdpTracker};

const CONFIG_FILE_NAME: &str = "tracker-config.toml";
const DEFAULT_DATABASE_PATH: &str = "/var/lib/torrust/tracker/database/sqlite3.db";
const TRACKER_BIND_HOST: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
const TRACKER_UDP_PORT: u16 = 6969;
const TRACKER_HTTP_TRACKER_PORT: u16 = 7070;
const TRACKER_HTTP_API_PORT: u16 = 1212;
const TRACKER_HEALTH_CHECK_API_PORT: u16 = 1313;
const DEFAULT_ACCESS_TOKEN: &str = "MyAccessToken";

/// Typed tracker configuration shared across the E2E workflow.
#[derive(Clone, Debug)]
pub(crate) struct TrackerConfig {
    database_path: String,
    udp_bind_address: SocketAddr,
    http_tracker_bind_address: SocketAddr,
    http_api_bind_address: SocketAddr,
    health_check_api_bind_address: SocketAddr,
    access_token: String,
}

impl Default for TrackerConfig {
    fn default() -> Self {
        Self {
            database_path: DEFAULT_DATABASE_PATH.to_string(),
            udp_bind_address: bind_address(TRACKER_UDP_PORT),
            http_tracker_bind_address: bind_address(TRACKER_HTTP_TRACKER_PORT),
            http_api_bind_address: bind_address(TRACKER_HTTP_API_PORT),
            health_check_api_bind_address: bind_address(TRACKER_HEALTH_CHECK_API_PORT),
            access_token: DEFAULT_ACCESS_TOKEN.to_string(),
        }
    }
}

impl TrackerConfig {
    pub(crate) fn udp_bind_address(&self) -> SocketAddr {
        self.udp_bind_address
    }

    pub(crate) fn http_tracker_bind_address(&self) -> SocketAddr {
        self.http_tracker_bind_address
    }

    pub(crate) fn health_check_api_bind_address(&self) -> SocketAddr {
        self.health_check_api_bind_address
    }

    pub(crate) fn announce_url_for_compose_service(&self) -> String {
        let announce_url = format!("http://tracker:{}/announce", self.http_tracker_bind_address.port()); // DevSkim: ignore DS137138

        announce_url
    }

    fn to_torrust_configuration(&self) -> Configuration {
        let mut configuration = Configuration::default();

        configuration.core.database.path.clone_from(&self.database_path);

        configuration.udp_trackers = Some(vec![UdpTracker {
            bind_address: self.udp_bind_address,
            ..UdpTracker::default()
        }]);

        configuration.http_trackers = Some(vec![HttpTracker {
            bind_address: self.http_tracker_bind_address,
            ..HttpTracker::default()
        }]);

        let mut http_api = HttpApi {
            bind_address: self.http_api_bind_address,
            ..HttpApi::default()
        };
        http_api.add_token("admin", &self.access_token);
        configuration.http_api = Some(http_api);

        configuration.health_check_api = HealthCheckApi {
            bind_address: self.health_check_api_bind_address,
        };

        configuration
    }
}

/// Builds and writes the Torrust Tracker configuration file for the E2E workspace.
///
/// All fields default to values suited for the E2E Docker Compose stack. Call
/// [`write_to`](TrackerConfigBuilder::write_to) to write `tracker-config.toml`
/// into the supplied workspace root directory.
pub(crate) struct TrackerConfigBuilder {
    tracker_config: TrackerConfig,
}

impl TrackerConfigBuilder {
    /// Creates a builder from a typed E2E tracker configuration object.
    pub(crate) fn new(tracker_config: TrackerConfig) -> Self {
        Self { tracker_config }
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn database_path(mut self, path: &str) -> Self {
        self.tracker_config.database_path = path.to_string();
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn udp_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.udp_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn http_tracker_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.http_tracker_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn http_api_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.http_api_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn health_check_api_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.health_check_api_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn access_token(mut self, token: &str) -> Self {
        self.tracker_config.access_token = token.to_string();
        self
    }

    /// Writes `tracker-config.toml` to `workspace_root`.
    ///
    /// Returns the path of the written file.
    ///
    /// # Errors
    ///
    /// Returns an error when writing the config file fails.
    pub(crate) fn write_to(&self, workspace_root: &Path) -> anyhow::Result<PathBuf> {
        let config_path = workspace_root.join(CONFIG_FILE_NAME);
        let config = self.tracker_config.to_torrust_configuration();
        let config_toml = toml::to_string(&config).context("failed to serialize tracker config to TOML")?;

        fs::write(&config_path, config_toml)
            .with_context(|| format!("failed to write tracker config '{}'", config_path.display()))?;

        Ok(config_path)
    }
}

fn bind_address(port: u16) -> SocketAddr {
    SocketAddr::new(TRACKER_BIND_HOST, port)
}
