//! Builder for the Torrust Tracker configuration file written into the E2E workspace.
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;

const CONFIG_FILE_NAME: &str = "tracker-config.toml";
const DEFAULT_DATABASE_PATH: &str = "/var/lib/torrust/tracker/database/sqlite3.db";
const DEFAULT_UDP_BIND_ADDRESS: &str = "0.0.0.0:6969";
const DEFAULT_HTTP_TRACKER_BIND_ADDRESS: &str = "0.0.0.0:7070";
const DEFAULT_HTTP_API_BIND_ADDRESS: &str = "0.0.0.0:1212";
const DEFAULT_HEALTH_CHECK_API_BIND_ADDRESS: &str = "0.0.0.0:1313";
const DEFAULT_ACCESS_TOKEN: &str = "MyAccessToken";

/// Builds and writes the Torrust Tracker configuration file for the E2E workspace.
///
/// All fields default to values suited for the E2E Docker Compose stack.  Call
/// [`write_to`](TrackerConfigBuilder::write_to) to write `tracker-config.toml`
/// into the supplied workspace root directory.
pub(crate) struct TrackerConfigBuilder {
    database_path: String,
    udp_bind_address: String,
    http_tracker_bind_address: String,
    http_api_bind_address: String,
    health_check_api_bind_address: String,
    access_token: String,
}

impl TrackerConfigBuilder {
    /// Creates a builder with all values set to their E2E container defaults.
    pub(crate) fn new() -> Self {
        Self {
            database_path: DEFAULT_DATABASE_PATH.to_string(),
            udp_bind_address: DEFAULT_UDP_BIND_ADDRESS.to_string(),
            http_tracker_bind_address: DEFAULT_HTTP_TRACKER_BIND_ADDRESS.to_string(),
            http_api_bind_address: DEFAULT_HTTP_API_BIND_ADDRESS.to_string(),
            health_check_api_bind_address: DEFAULT_HEALTH_CHECK_API_BIND_ADDRESS.to_string(),
            access_token: DEFAULT_ACCESS_TOKEN.to_string(),
        }
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn database_path(mut self, path: &str) -> Self {
        self.database_path = path.to_string();
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn udp_bind_address(mut self, addr: &str) -> Self {
        self.udp_bind_address = addr.to_string();
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn http_tracker_bind_address(mut self, addr: &str) -> Self {
        self.http_tracker_bind_address = addr.to_string();
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn http_api_bind_address(mut self, addr: &str) -> Self {
        self.http_api_bind_address = addr.to_string();
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn health_check_api_bind_address(mut self, addr: &str) -> Self {
        self.health_check_api_bind_address = addr.to_string();
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(crate) fn access_token(mut self, token: &str) -> Self {
        self.access_token = token.to_string();
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
        let config = self.format_config();

        fs::write(&config_path, config).with_context(|| format!("failed to write tracker config '{}'", config_path.display()))?;

        Ok(config_path)
    }

    fn format_config(&self) -> String {
        let database_path = &self.database_path;
        let udp_bind_address = &self.udp_bind_address;
        let http_tracker_bind_address = &self.http_tracker_bind_address;
        let http_api_bind_address = &self.http_api_bind_address;
        let health_check_api_bind_address = &self.health_check_api_bind_address;
        let access_token = &self.access_token;

        format!(
            "[metadata]\n\
             app = \"torrust-tracker\"\n\
             purpose = \"configuration\"\n\
             schema_version = \"2.0.0\"\n\
             \n\
             [logging]\n\
             threshold = \"info\"\n\
             \n\
             [core]\n\
             listed = false\n\
             private = false\n\
             \n\
             [core.database]\n\
             path = \"{database_path}\"\n\
             \n\
             [[udp_trackers]]\n\
             bind_address = \"{udp_bind_address}\"\n\
             \n\
             [[http_trackers]]\n\
             bind_address = \"{http_tracker_bind_address}\"\n\
             \n\
             [http_api]\n\
             bind_address = \"{http_api_bind_address}\"\n\
             \n\
             [http_api.access_tokens]\n\
             admin = \"{access_token}\"\n\
             \n\
             [health_check_api]\n\
             bind_address = \"{health_check_api_bind_address}\"\n"
        )
    }
}
