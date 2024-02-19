//! Utilities to parse Torrust Tracker logs.
use serde::{Deserialize, Serialize};

const UDP_TRACKER_PATTERN: &str = "[UDP Tracker][INFO] Starting on: udp://";
const HTTP_TRACKER_PATTERN: &str = "[HTTP Tracker][INFO] Starting on: ";
const HEALTH_CHECK_PATTERN: &str = "[HEALTH CHECK API][INFO] Starting on: ";

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
    /// Loading default configuration file: `./share/default/config/tracker.development.sqlite3.toml` ...
    /// 2024-01-24T16:36:14.614898789+00:00 [torrust_tracker::bootstrap::logging][INFO] logging initialized.
    /// 2024-01-24T16:36:14.615586025+00:00 [UDP Tracker][INFO] Starting on: udp://0.0.0.0:6969
    /// 2024-01-24T16:36:14.615623705+00:00 [torrust_tracker::bootstrap::jobs][INFO] TLS not enabled
    /// 2024-01-24T16:36:14.615694484+00:00 [HTTP Tracker][INFO] Starting on: http://0.0.0.0:7070
    /// 2024-01-24T16:36:14.615710534+00:00 [HTTP Tracker][INFO] Started on: http://0.0.0.0:7070
    /// 2024-01-24T16:36:14.615716574+00:00 [torrust_tracker::bootstrap::jobs][INFO] TLS not enabled
    /// 2024-01-24T16:36:14.615764904+00:00 [API][INFO] Starting on http://127.0.0.1:1212
    /// 2024-01-24T16:36:14.615767264+00:00 [API][INFO] Started on http://127.0.0.1:1212
    /// 2024-01-24T16:36:14.615777574+00:00 [HEALTH CHECK API][INFO] Starting on: http://127.0.0.1:1313
    /// 2024-01-24T16:36:14.615791124+00:00 [HEALTH CHECK API][INFO] Started on: http://127.0.0.1:1313
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
        let logs = "\
            [UDP Tracker][INFO] Starting on: udp://0.0.0.0:8080\n\
            [HTTP Tracker][INFO] Starting on: 0.0.0.0:9090\n\
            [HEALTH CHECK API][INFO] Starting on: 0.0.0.0:10010";
        let running_services = RunningServices::parse_from_logs(logs);

        assert_eq!(running_services.udp_trackers, vec!["127.0.0.1:8080"]);
        assert_eq!(running_services.http_trackers, vec!["127.0.0.1:9090"]);
        assert_eq!(running_services.health_checks, vec!["127.0.0.1:10010/health_check"]);
    }

    #[test]
    fn it_should_ignore_logs_with_no_matching_lines() {
        let logs = "[Other Service][INFO] Starting on: 0.0.0.0:7070";
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
