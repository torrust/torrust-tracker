use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};

/// Port-zero configuration with one metrics-disabled and one metrics-enabled
/// listener for each public tracker protocol.
#[allow(dead_code)]
pub struct PortZeroMetricsPolicyConfiguration;

#[allow(dead_code)]
impl PortZeroMetricsPolicyConfiguration {
    /// Canonical ID of the first `[[udp_trackers]]` entry in [`Self::TOML`].
    ///
    /// This entry sets `tracker_usage_statistics = false`.
    pub const METRICS_DISABLED_UDP_TRACKER_ID: ConfigurationInstanceId = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);

    /// Canonical ID of the second `[[udp_trackers]]` entry in [`Self::TOML`].
    ///
    /// This entry sets `tracker_usage_statistics = true`.
    pub const METRICS_ENABLED_UDP_TRACKER_ID: ConfigurationInstanceId = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1);

    /// TOML source for the port-zero metrics-policy integration fixture.
    pub const TOML: &str = r#"
    [metadata]
    app = "torrust-tracker"
    purpose = "configuration"
    schema_version = "2.0.0"

    [logging]
    threshold = "off"

    [core]
    listed = false
    private = false

    [core.database]
    driver = "sqlite3"
    path = "{STORAGE_PATH}/sqlite3.db"

    [[http_trackers]]
    bind_address = "0.0.0.0:0"
    tracker_usage_statistics = false

    [[http_trackers]]
    bind_address = "0.0.0.0:0"
    tracker_usage_statistics = true

    [[udp_trackers]]
    bind_address = "0.0.0.0:0"
    tracker_usage_statistics = false

    [[udp_trackers]]
    bind_address = "0.0.0.0:0"
    tracker_usage_statistics = true

    [http_api]
    bind_address = "127.0.0.1:0"

    [http_api.access_tokens]
    admin = "MyAccessToken"

    [health_check_api]
    bind_address = "127.0.0.2:0"
"#;
}
