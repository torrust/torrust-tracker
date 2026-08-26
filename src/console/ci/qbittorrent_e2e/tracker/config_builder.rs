//! Builder for the Torrust Tracker configuration file written into the E2E workspace.
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};

use anyhow::Context;
use secrecy::SecretString;
use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_configuration::v3_0_0::database::{ConnectionInfo, Database};
use torrust_tracker_configuration::v3_0_0::health_check_api::HealthCheckApi;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;

const CONFIG_FILE_NAME: &str = "tracker-config.toml";
const DEFAULT_SQLITE3_DATABASE_PATH: &str = "/var/lib/torrust/tracker/database/sqlite3.db";
const DEFAULT_MYSQL_DATABASE_PATH: &str = "mysql://db_user:db_user_secret_password@mysql:3306/torrust_tracker";
const DEFAULT_POSTGRESQL_DATABASE_PATH: &str = "postgresql://postgres:postgres@postgres:5432/torrust_tracker";
const TRACKER_BIND_HOST: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
const TRACKER_UDP_PORT: u16 = 6969;
const TRACKER_HTTP_TRACKER_PORT: u16 = 7070;
const TRACKER_HTTP_API_PORT: u16 = 1212;
const TRACKER_HEALTH_CHECK_API_PORT: u16 = 1313;
const DEFAULT_ACCESS_TOKEN: &str = "MyAccessToken";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DatabaseDriver {
    Sqlite3,
    MySQL,
    PostgreSQL,
}

impl DatabaseDriver {
    const fn default_database_path(self) -> &'static str {
        match self {
            Self::Sqlite3 => DEFAULT_SQLITE3_DATABASE_PATH,
            Self::MySQL => DEFAULT_MYSQL_DATABASE_PATH,
            Self::PostgreSQL => DEFAULT_POSTGRESQL_DATABASE_PATH,
        }
    }
}

/// Typed tracker configuration shared across the E2E workflow.
#[derive(Clone, Debug)]
pub(crate) struct TrackerConfig {
    database_driver: DatabaseDriver,
    database_path: String,
    udp_bind_address: SocketAddr,
    http_tracker_bind_address: SocketAddr,
    http_api_bind_address: SocketAddr,
    health_check_api_bind_address: SocketAddr,
    access_token: String,
}

impl Default for TrackerConfig {
    fn default() -> Self {
        Self::for_database_driver(DatabaseDriver::Sqlite3)
    }
}

impl TrackerConfig {
    pub(crate) fn for_database_driver(database_driver: DatabaseDriver) -> Self {
        Self {
            database_driver,
            database_path: database_driver.default_database_path().to_string(),
            udp_bind_address: bind_address(TRACKER_UDP_PORT),
            http_tracker_bind_address: bind_address(TRACKER_HTTP_TRACKER_PORT),
            http_api_bind_address: bind_address(TRACKER_HTTP_API_PORT),
            health_check_api_bind_address: bind_address(TRACKER_HEALTH_CHECK_API_PORT),
            access_token: DEFAULT_ACCESS_TOKEN.to_string(),
        }
    }

    pub(crate) const fn udp_bind_address(&self) -> SocketAddr {
        self.udp_bind_address
    }

    pub(crate) const fn http_tracker_bind_address(&self) -> SocketAddr {
        self.http_tracker_bind_address
    }

    pub(crate) const fn health_check_api_bind_address(&self) -> SocketAddr {
        self.health_check_api_bind_address
    }

    pub(crate) const fn http_api_bind_address(&self) -> SocketAddr {
        self.http_api_bind_address
    }

    pub(crate) fn access_token(&self) -> &str {
        &self.access_token
    }

    pub(crate) fn announce_url_for_compose_service(&self) -> String {
        let announce_url = format!("http://tracker:{}/announce", self.http_tracker_bind_address.port()); // DevSkim: ignore DS137138

        announce_url
    }

    pub(crate) fn udp_announce_url_for_compose_service(&self) -> String {
        format!("udp://tracker:{}", self.udp_bind_address.port())
    }

    fn to_torrust_configuration(&self) -> anyhow::Result<Configuration> {
        let mut configuration = Configuration::default();

        configuration.core.database = Some(self.database_configuration()?);

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

        Ok(configuration)
    }

    fn database_configuration(&self) -> anyhow::Result<Database> {
        match self.database_driver {
            DatabaseDriver::Sqlite3 => Ok(Database::Sqlite3 {
                path: self.database_path.clone(),
            }),
            DatabaseDriver::MySQL => Ok(Database::MySQL(connection_info_from_url(
                &self.database_path,
                "mysql://",
                3306,
            )?)),
            DatabaseDriver::PostgreSQL => Ok(Database::PostgreSQL(connection_info_from_url(
                &self.database_path,
                "postgresql://",
                5432,
            )?)),
        }
    }
}

fn connection_info_from_url(url: &str, expected_scheme: &str, default_port: u16) -> anyhow::Result<ConnectionInfo> {
    let authority_and_database = url
        .strip_prefix(expected_scheme)
        .with_context(|| format!("database URL must start with '{expected_scheme}'"))?;
    let (credentials, host_and_database) = authority_and_database
        .split_once('@')
        .context("database URL must contain credentials and a host")?;
    let (user, password) = credentials
        .split_once(':')
        .context("database URL must contain a user and password")?;
    let (host_and_port, database) = host_and_database
        .split_once('/')
        .context("database URL must contain a database name")?;
    let (host, port) = match host_and_port.rsplit_once(':') {
        Some((host, port)) => (host, port.parse().context("database URL port must be a valid u16")?),
        None => (host_and_port, default_port),
    };

    if user.is_empty() || password.is_empty() || host.is_empty() || database.is_empty() {
        anyhow::bail!("database URL must contain non-empty credentials, host, and database name");
    }

    Ok(ConnectionInfo {
        host: host.to_string(),
        port,
        user: user.to_string(),
        password: SecretString::from(password.to_string()),
        database: database.to_string(),
    })
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
    pub(crate) const fn new(tracker_config: TrackerConfig) -> Self {
        Self { tracker_config }
    }

    // These builder methods allow future scenarios to override the default
    // tracker bind addresses, database path, and access token (e.g. for
    // private-tracker or multi-database scenarios). Tracked: <https://github.com/torrust/torrust-tracker/issues/1706>.
    #[expect(dead_code, reason = "reserved for future scenario configuration; see #1706")]
    pub(crate) fn database_path(mut self, path: &str) -> Self {
        self.tracker_config.database_path = path.to_string();
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration; see #1706")]
    pub(crate) const fn udp_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.udp_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration; see #1706")]
    pub(crate) const fn http_tracker_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.http_tracker_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration; see #1706")]
    pub(crate) const fn http_api_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.http_api_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration; see #1706")]
    pub(crate) const fn health_check_api_bind_address(mut self, addr: SocketAddr) -> Self {
        self.tracker_config.health_check_api_bind_address = addr;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration; see #1706")]
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
        let config = self
            .tracker_config
            .to_torrust_configuration()
            .context("failed to build tracker configuration")?;
        let config_path_as_str = config_path.to_str().context("tracker config path must be valid UTF-8")?;

        config
            .save_to_file(config_path_as_str)
            .with_context(|| format!("failed to write tracker config '{}'", config_path.display()))?;

        Ok(config_path)
    }
}

const fn bind_address(port: u16) -> SocketAddr {
    SocketAddr::new(TRACKER_BIND_HOST, port)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{DatabaseDriver, TrackerConfig, TrackerConfigBuilder};

    #[test]
    fn write_to_should_persist_the_tracker_api_access_token() {
        let temporary_directory = tempdir().expect("temporary E2E workspace should be created");
        let config = TrackerConfigBuilder::new(TrackerConfig::default());

        let config_path = config
            .write_to(temporary_directory.path())
            .expect("tracker configuration should be written");
        let written_configuration = fs::read_to_string(config_path).expect("tracker configuration should be readable");

        assert!(written_configuration.contains("[http_api.access_tokens]"));
        assert!(written_configuration.contains("admin = \"MyAccessToken\""));
        assert!(!written_configuration.contains("admin = \"***\""));
    }

    #[test]
    fn it_should_select_the_configured_database_driver_without_exposing_network_passwords() {
        // Arrange
        let configurations = [
            (DatabaseDriver::Sqlite3, "/tmp/qbittorrent-e2e.sqlite3", "sqlite3", None),
            (
                DatabaseDriver::MySQL,
                "mysql://mysql_user:mysql_password@mysql:3307/mysql_database",
                "mysql",
                Some("mysql_password"),
            ),
            (
                DatabaseDriver::PostgreSQL,
                "postgresql://postgres_user:postgres_password@postgres:5433/postgres_database",
                "postgresql",
                Some("postgres_password"),
            ),
        ];

        for (driver, configured_path, expected_driver, secret) in configurations {
            // Act
            let mut tracker_config = TrackerConfig::for_database_driver(driver);
            tracker_config.database_path = configured_path.to_string();
            let configuration = tracker_config
                .to_torrust_configuration()
                .expect("database configuration should be valid");
            let serialized = configuration.to_redacted_json();

            // Assert
            assert!(serialized.contains(&format!("\"driver\": \"{expected_driver}\"")));
            if let Some(secret) = secret {
                assert!(serialized.contains("\"password\": \"***\""));
                assert!(!serialized.contains(secret));
            }
        }
    }
}
