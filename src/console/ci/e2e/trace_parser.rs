//! Utilities to parse Torrust Tracker traces.
use serde::{Deserialize, Serialize};

const UDP_TRACKER_PATTERN: &str = "UDP TRACKER: Starting on: udp://";
const HTTP_TRACKER_PATTERN: &str = "HTTP TRACKER: Starting on: ";
const HEALTH_CHECK_PATTERN: &str = "HEALTH CHECK API: Starting on: ";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RunningServices {
    pub udp_trackers: Vec<String>,
    pub http_trackers: Vec<String>,
    pub health_checks: Vec<String>,
}

impl RunningServices {
    /// It parses the tracker traces to extract the running services.
    ///
    /// For example, from this traces:
    ///
    /// ```text
    /// Loading configuration file: `./storage/tracker/etc/tracker.toml` ...
    /// 2024-04-10T02:28:58.219529Z  INFO UDP TRACKER: Starting on: udp://0.0.0.0:6969
    /// 2024-04-10T02:28:58.219563Z  INFO torrust_tracker::servers::udp::server: Running UDP Tracker on Socket: 0.0.0.0:6969
    /// 2024-04-10T02:28:58.219578Z  INFO torrust_tracker::bootstrap::jobs: TLS not enabled
    /// 2024-04-10T02:28:58.219612Z  INFO HTTP TRACKER: Starting on: http://0.0.0.0:7070
    /// 2024-04-10T02:28:58.219775Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070
    /// 2024-04-10T02:28:58.219787Z  INFO torrust_tracker::bootstrap::jobs: TLS not enabled
    /// 2024-04-10T02:28:58.219929Z  INFO API: Starting on http://127.0.0.1:1212
    /// 2024-04-10T02:28:58.219936Z  INFO API: Started on http://127.0.0.1:1212
    /// 2024-04-10T02:28:58.219967Z  INFO HEALTH CHECK API: Starting on: http://127.0.0.1:1313
    /// 2024-04-10T02:28:58.220019Z  INFO HEALTH CHECK API: Started on: http://127.0.0.1:1313
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
    pub fn parse_from_traces(traces: &str) -> Self {
        let mut udp_trackers: Vec<String> = Vec::new();
        let mut http_trackers: Vec<String> = Vec::new();
        let mut health_checks: Vec<String> = Vec::new();

        for line in traces.lines() {
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
    fn it_should_parse_from_traces_with_valid_traces() {
        let traces = "\
        2024-04-10T02:28:58.219529Z  INFO UDP TRACKER: Starting on: udp://0.0.0.0:6969\n\
        2024-04-10T02:28:58.219612Z  INFO HTTP TRACKER: Starting on: https://0.0.0.0:7070\n\
        2024-04-10T02:28:58.219929Z  INFO API: Starting on http://127.0.0.1:1212\n\
        2024-04-10T02:28:58.219967Z  INFO HEALTH CHECK API: Starting on: http://127.0.0.1:1313";
        let running_services = RunningServices::parse_from_traces(traces);

        assert_eq!(running_services.udp_trackers, vec!["127.0.0.1:6969"]);
        assert_eq!(running_services.http_trackers, vec!["https://127.0.0.1:7070"]);
        assert_eq!(running_services.health_checks, vec!["http://127.0.0.1:1313/health_check"]);
    }

    #[test]
    fn it_should_ignore_traces_with_no_matching_lines() {
        let traces = "2024-04-10T02:28:58.219967Z  INFO OTHER SERVICE: Starting on: http://127.0.0.1:1313";
        let running_services = RunningServices::parse_from_traces(traces);

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
