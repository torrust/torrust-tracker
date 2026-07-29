use std::fmt;

use serde::{Deserialize, Serialize};

// issue: #2036
/// A tracker application service role.
///
/// This role identifies the application behavior implemented by a listener.
/// It does not identify its transport or socket binding: HTTP and HTTPS both
/// use [`Self::HttpTracker`] and differ through their service binding.
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ServiceRole {
    /// A `BitTorrent` HTTP or HTTPS tracker service.
    HttpTracker,
    /// A `BitTorrent` UDP tracker service.
    UdpTracker,
    /// The tracker management REST API service.
    #[serde(rename = "tracker_rest_api")]
    RestApi,
    /// The tracker health-check API service.
    HealthCheckApi,
}

impl ServiceRole {
    /// Returns the stable tracker-owned identifier for this role.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HttpTracker => "http_tracker",
            Self::UdpTracker => "udp_tracker",
            Self::RestApi => "tracker_rest_api",
            Self::HealthCheckApi => "health_check_api",
        }
    }
}

impl fmt::Display for ServiceRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::ServiceRole;

    #[test]
    fn it_should_return_the_canonical_identifier_for_each_role() {
        // Arrange
        let roles = [
            (ServiceRole::HttpTracker, "http_tracker"),
            (ServiceRole::UdpTracker, "udp_tracker"),
            (ServiceRole::RestApi, "tracker_rest_api"),
            (ServiceRole::HealthCheckApi, "health_check_api"),
        ];

        // Act and Assert
        for (service_role, identifier) in roles {
            assert_eq!(service_role.as_str(), identifier);
        }
    }

    #[test]
    fn it_should_display_the_canonical_identifier_for_each_role() {
        // Arrange
        let roles = [
            (ServiceRole::HttpTracker, "http_tracker"),
            (ServiceRole::UdpTracker, "udp_tracker"),
            (ServiceRole::RestApi, "tracker_rest_api"),
            (ServiceRole::HealthCheckApi, "health_check_api"),
        ];

        // Act and Assert
        for (service_role, identifier) in roles {
            assert_eq!(service_role.to_string(), identifier);
        }
    }

    #[test]
    fn it_should_serialize_each_role_to_its_canonical_identifier() {
        // Arrange
        let roles = [
            (ServiceRole::HttpTracker, r#""http_tracker""#),
            (ServiceRole::UdpTracker, r#""udp_tracker""#),
            (ServiceRole::RestApi, r#""tracker_rest_api""#),
            (ServiceRole::HealthCheckApi, r#""health_check_api""#),
        ];

        // Act and Assert
        for (service_role, identifier) in roles {
            assert_eq!(serde_json::to_string(&service_role).unwrap(), identifier);
        }
    }
}
