//! Utilities to parse Torrust Tracker logs.
use serde::{Deserialize, Serialize};

const UDP_TRACKER_PATTERN: &str = "UDP TRACKER: Started on: udp://";
const HTTP_TRACKER_PATTERN: &str = "HTTP TRACKER: Started on: ";
const HEALTH_CHECK_PATTERN: &str = "HEALTH CHECK API: Started on: ";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RunningServices {
    pub udp_trackers: Vec<String>,
    pub http_trackers: Vec<String>,
    pub health_checks: Vec<String>,
}

impl RunningServices {
    /// It parses the tracker logs to extract the running services.
    ///
    /// For example, from this logs:
    ///
    /// ```text
    /// Loading configuration from default configuration file: `./share/default/config/tracker.development.sqlite3.toml` ...
    /// 2024-06-10T14:59:57.973525Z  INFO torrust_tracker::bootstrap::logging: logging initialized.
    /// 2024-06-10T14:59:57.974306Z  INFO UDP TRACKER: Starting on: udp://0.0.0.0:6969
    /// 2024-06-10T14:59:57.974316Z  INFO UDP TRACKER: Started on: udp://0.0.0.0:6969
    /// 2024-06-10T14:59:57.974332Z  INFO torrust_tracker::bootstrap::jobs: TLS not enabled
    /// 2024-06-10T14:59:57.974366Z  INFO HTTP TRACKER: Starting on: http://0.0.0.0:7070
    /// 2024-06-10T14:59:57.974513Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070
    /// 2024-06-10T14:59:57.974521Z  INFO torrust_tracker::bootstrap::jobs: TLS not enabled
    /// 2024-06-10T14:59:57.974615Z  INFO API: Starting on http://127.0.0.1:1212
    /// 2024-06-10T14:59:57.974618Z  INFO API: Started on http://127.0.0.1:1212
    /// 2024-06-10T14:59:57.974643Z  INFO HEALTH CHECK API: Starting on: http://127.0.0.1:1313
    /// 2024-06-10T14:59:57.974760Z  INFO HEALTH CHECK API: Started on: http://127.0.0.1:1313
    /// ```
    ///
    /// It would extract these services:
    ///
    /// ```json
    /// {
    ///   "udp_trackers": [
    ///     "127.0.0.1:6969"
    ///    ],
    ///    "http_trackers": [
    ///      "http://127.0.0.1:7070"
    ///    ],
    ///    "health_checks": [
    ///      "http://127.0.0.1:1313/health_check"
    ///    ]
    /// }
    /// ```
    ///
    /// NOTICE: Using colors in the console output could affect this method 
    /// due to the hidden control chars.
    #[must_use]
    pub fn parse_from_logs(logs: &str) -> Self {
        let mut udp_trackers: Vec<String> = Vec::new();
        let mut http_trackers: Vec<String> = Vec::new();
        let mut health_checks: Vec<String> = Vec::new();

        for line in logs.lines() {
            if let Some(address) = Self::extract_address_if_matches(line, UDP_TRACKER_PATTERN) {
                udp_trackers.push(address);
            } else if let Some(address) = Self::extract_address_if_matches(line, HTTP_TRACKER_PATTERN) {
                http_trackers.push(address);
            } else if let Some(address) = Self::extract_address_if_matches(line, HEALTH_CHECK_PATTERN) {
                health_checks.push(format!("{address}/health_check"));
            }
        }

        Self {
            udp_trackers,
            http_trackers,
            health_checks,
        }
    }

    fn extract_address_if_matches(line: &str, pattern: &str) -> Option<String> {
        line.find(pattern)
            .map(|start| Self::replace_wildcard_ip_with_localhost(line[start + pattern.len()..].trim()))
    }

    fn replace_wildcard_ip_with_localhost(address: &str) -> String {
        address.replace("0.0.0.0", "127.0.0.1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_from_logs_with_valid_logs() {
        let log = r#"
            Loading configuration from environment variable db_path = "/var/lib/torrust/tracker/database/sqlite3.db"

            [[udp_trackers]]
            enabled = true

            [[http_trackers]]
            enabled = true
            ssl_cert_path = "/var/lib/torrust/tracker/tls/localhost.crt"
            ssl_key_path = "/var/lib/torrust/tracker/tls/localhost.key"

            [http_api]
            ssl_cert_path = "/var/lib/torrust/tracker/tls/localhost.crt"
            ssl_key_path = "/var/lib/torrust/tracker/tls/localhost.key"
            ...
            Loading configuration from file: `/etc/torrust/tracker/tracker.toml` ...
            2024-06-10T15:09:54.411031Z  INFO torrust_tracker::bootstrap::logging: logging initialized.
            2024-06-10T15:09:54.415084Z  INFO UDP TRACKER: Starting on: udp://0.0.0.0:6969
            2024-06-10T15:09:54.415091Z  INFO UDP TRACKER: Started on: udp://0.0.0.0:6969
            2024-06-10T15:09:54.415104Z  INFO torrust_tracker::bootstrap::jobs: TLS not enabled
            2024-06-10T15:09:54.415130Z  INFO HTTP TRACKER: Starting on: http://0.0.0.0:7070
            2024-06-10T15:09:54.415266Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070
            2024-06-10T15:09:54.415275Z  INFO torrust_tracker::bootstrap::jobs: TLS not enabled
            2024-06-10T15:09:54.415403Z  INFO API: Starting on http://127.0.0.1:1212
            2024-06-10T15:09:54.415411Z  INFO API: Started on http://127.0.0.1:1212
            2024-06-10T15:09:54.415430Z  INFO HEALTH CHECK API: Starting on: http://127.0.0.1:1313
            2024-06-10T15:09:54.415472Z  INFO HEALTH CHECK API: Started on: http://127.0.0.1:1313
            "#;

        let running_services = RunningServices::parse_from_logs(log);

        assert_eq!(running_services.udp_trackers, vec!["127.0.0.1:6969"]);
        assert_eq!(running_services.http_trackers, vec!["http://127.0.0.1:7070"]);
        assert_eq!(running_services.health_checks, vec!["http://127.0.0.1:1313/health_check"]);
    }

    #[test]
    fn it_should_ignore_logs_with_no_matching_lines() {
        let logs = "[Other Service][INFO] Started on: 0.0.0.0:7070";
        let running_services = RunningServices::parse_from_logs(logs);

        assert!(running_services.udp_trackers.is_empty());
        assert!(running_services.http_trackers.is_empty());
        assert!(running_services.health_checks.is_empty());
    }

    #[test]
    fn it_should_replace_wildcard_ip_with_localhost() {
        let address = "0.0.0.0:8080";
        assert_eq!(RunningServices::replace_wildcard_ip_with_localhost(address), "127.0.0.1:8080");
    }
}
