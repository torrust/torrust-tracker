//! Probes and parses tracker health-check endpoints without owning process or I/O task resources.
//!
//! Readiness policy and deadlines remain with the running tracker lifecycle.

use std::net::SocketAddr;

use torrust_tracker_axum_health_check_api_server::resources::Report;

const HEALTH_CHECK_STARTUP_PREFIX: &str = "Started on: http://";
const HEALTH_CHECK_LOG_TARGET: &str = "HEALTH CHECK API";

/// A deadline-bounded client for the tracker health-check endpoint.
pub(super) struct HealthCheckClient {
    pub(super) address: SocketAddr,
    client: reqwest::Client,
}

impl HealthCheckClient {
    pub(super) fn new(address: SocketAddr) -> Self {
        Self {
            address,
            client: reqwest::Client::new(),
        }
    }

    pub(super) async fn probe(&self, deadline: tokio::time::Instant) -> Result<HealthCheckProbe, HealthCheckProbeError> {
        let health_check_url = format!("http://{}/health_check", self.address); // DevSkim: ignore DS137138
        let response = match tokio::time::timeout_at(deadline, self.client.get(health_check_url).send()).await {
            Ok(Ok(response)) => response,
            Ok(Err(_)) => return Ok(HealthCheckProbe::Unavailable),
            Err(_) => return Err(HealthCheckProbeError::TimedOut),
        };

        if !response.status().is_success() {
            return Err(HealthCheckProbeError::UnexpectedHttpStatus(response.status()));
        }

        let report = match tokio::time::timeout_at(deadline, response.json::<Report>()).await {
            Ok(Ok(report)) => report,
            Ok(Err(error)) => return Err(HealthCheckProbeError::InvalidReport(error.to_string())),
            Err(_) => return Err(HealthCheckProbeError::TimedOut),
        };

        Ok(HealthCheckProbe::Report(report))
    }
}

pub(super) enum HealthCheckProbe {
    Unavailable,
    Report(Report),
}

pub(super) enum HealthCheckProbeError {
    TimedOut,
    UnexpectedHttpStatus(reqwest::StatusCode),
    InvalidReport(String),
}

pub(super) fn parse_health_check_address(line: &str) -> Option<SocketAddr> {
    if !line.contains(HEALTH_CHECK_LOG_TARGET) {
        return None;
    }
    let address = line.split_once(HEALTH_CHECK_STARTUP_PREFIX)?.1;
    address.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::parse_health_check_address;

    #[test]
    fn it_should_extract_the_assigned_health_check_address_from_its_startup_log() {
        // Arrange
        let line = "2026-09-02T10:20:22Z  INFO HEALTH CHECK API: Started on: http://127.0.0.1:43210";

        // Act
        let address = parse_health_check_address(line);

        // Assert
        assert_eq!(
            address.expect("health-check address should parse").to_string(),
            "127.0.0.1:43210"
        );
    }

    #[test]
    fn it_should_reject_non_health_check_startup_logs() {
        // Arrange
        let lines = [
            "2026-09-02T10:20:22Z  INFO HTTP TRACKER: Started on: http://127.0.0.1:43210",
            "2026-09-02T10:20:22Z  INFO HEALTH CHECK API: Listening on: http://127.0.0.1:43210",
            "2026-09-02T10:20:22Z  INFO HEALTH CHECK API: Started on: http://not-an-address", // DevSkim: ignore DS137138
        ];

        // Act and Assert
        for line in lines {
            assert_eq!(
                parse_health_check_address(line),
                None,
                "line should not provide a health-check address: {line}"
            );
        }
    }
}
