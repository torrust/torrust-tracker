// adr: docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
// issue: #1453
//! UDP tracker server-wide configuration for schema v3.
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

use crate::v3_0_0::types::AtLeastU64;

/// Controls whether the UDP tracker validates the connection ID supplied by
/// clients in announce and scrape requests.
///
/// Strict validation is the secure default and matches current behaviour.
/// Disabled validation can be used for isolated compatibility listeners when
/// serving non-compliant clients that reuse expired or arbitrary connection IDs
/// is more important than anti-spoofing and replay protection.
///
/// # Security
///
/// Setting this to `Disabled` removes the narrow timestamp window that makes
/// arbitrary connection IDs unlikely to be accepted. Operators **must** isolate
/// disabled-validation listeners through external network controls and are
/// encouraged to use `Strict` wherever possible. Cookie-error metrics continue
/// to be emitted in disabled mode so operators can quantify non-compliant
/// clients.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionIdValidationPolicy {
    /// Preserve all existing connection ID validation: reject non-normal,
    /// expired, future-dated, and wrong-fingerprint values. This is the
    /// secure default.
    #[default]
    Strict,
    /// Skip connection ID validation for announce and scrape requests.
    /// The connect action continues to issue valid connection IDs.
    /// Cookie-error metrics are still emitted; IP-ban counters are not
    /// incremented.
    Disabled,
}

/// Configuration shared by every UDP tracker listener.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UdpTrackerServer {
    /// Seconds between resets of the temporary IP-ban filters.
    #[serde(default = "default_ip_bans_reset_interval_in_secs")]
    pub ip_bans_reset_interval_in_secs: IpBansResetIntervalInSecs,

    /// Connection ID validation policy for all UDP tracker listeners.
    ///
    /// This is a global setting because the ban service is shared across all
    /// UDP instances. A per-instance policy would allow one listener's traffic
    /// to pollute the shared ban counter that another listener enforces against.
    ///
    /// `strict` (default) preserves all existing validation.
    /// `disabled` skips validation so non-compliant clients that reuse
    /// expired or arbitrary connection IDs can still connect. Cookie-error
    /// metrics are still emitted in disabled mode; IP-ban counters are not
    /// incremented.
    ///
    /// **Security**: only use `disabled` on deployments where all listeners are
    /// isolated through external network controls. Always prefer `strict` in
    /// public deployments.
    ///
    /// See ADR-20260727180000 for the rationale behind shared services.
    #[serde(default)]
    pub connection_id_validation: ConnectionIdValidationPolicy,
}

impl Default for UdpTrackerServer {
    fn default() -> Self {
        Self {
            ip_bans_reset_interval_in_secs: default_ip_bans_reset_interval_in_secs(),
            connection_id_validation: ConnectionIdValidationPolicy::default(),
        }
    }
}

impl UdpTrackerServer {
    /// The minimum supported IP-ban reset interval, in seconds.
    pub const MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS: u64 = 60 * 60;

    /// The default IP-ban reset interval, in seconds.
    pub const DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS: u64 = 24 * 60 * 60;
}

/// A validated IP-ban reset interval in seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IpBansResetIntervalInSecs(AtLeastU64<{ UdpTrackerServer::MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS }>);

/// Error returned when an IP-ban reset interval is shorter than the supported minimum.
#[derive(Debug, Error, PartialEq, Eq)]
#[error(
    "The IP bans reset interval must be at least {minimum} seconds.",
    minimum = UdpTrackerServer::MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS
)]
pub struct IpBansResetIntervalTooShortError;

impl IpBansResetIntervalInSecs {
    /// Creates an interval after enforcing the domain minimum.
    ///
    /// # Errors
    ///
    /// Returns [`IpBansResetIntervalTooShortError`] when `value` is too short.
    pub fn new(value: u64) -> Result<Self, IpBansResetIntervalTooShortError> {
        AtLeastU64::new(value).map(Self).map_err(|_| IpBansResetIntervalTooShortError)
    }

    /// Returns the validated interval in seconds.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl TryFrom<u64> for IpBansResetIntervalInSecs {
    type Error = IpBansResetIntervalTooShortError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<IpBansResetIntervalInSecs> for u64 {
    fn from(value: IpBansResetIntervalInSecs) -> Self {
        value.get()
    }
}

impl Serialize for IpBansResetIntervalInSecs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IpBansResetIntervalInSecs {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

fn default_ip_bans_reset_interval_in_secs() -> IpBansResetIntervalInSecs {
    IpBansResetIntervalInSecs::new(UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS)
        .expect("the default IP-ban reset interval must satisfy its minimum")
}

#[cfg(test)]
mod tests {
    use crate::v3_0_0::udp_tracker_server::{ConnectionIdValidationPolicy, IpBansResetIntervalInSecs, UdpTrackerServer};

    #[test]
    fn it_should_default_to_a_24_hour_reset_interval() {
        assert_eq!(
            UdpTrackerServer::default().ip_bans_reset_interval_in_secs.get(),
            UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS
        );
    }

    #[test]
    fn it_should_accept_the_minimum_reset_interval() {
        assert_eq!(
            IpBansResetIntervalInSecs::new(UdpTrackerServer::MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS)
                .map(IpBansResetIntervalInSecs::get),
            Ok(UdpTrackerServer::MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS)
        );
    }

    #[test]
    fn it_should_reject_a_reset_interval_below_the_minimum() {
        let error = IpBansResetIntervalInSecs::new(UdpTrackerServer::MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS - 1)
            .expect_err("an interval below the minimum should be rejected");

        assert_eq!(
            error.to_string(),
            format!(
                "The IP bans reset interval must be at least {} seconds.",
                UdpTrackerServer::MINIMUM_IP_BANS_RESET_INTERVAL_IN_SECS
            )
        );
    }

    #[test]
    fn it_should_default_connection_id_validation_to_strict() {
        let config = UdpTrackerServer::default();
        assert_eq!(config.connection_id_validation, ConnectionIdValidationPolicy::Strict);
    }

    #[test]
    fn it_should_use_strict_when_connection_id_validation_field_is_omitted() {
        let config: UdpTrackerServer = toml::from_str("").expect("empty config should deserialize");
        assert_eq!(config.connection_id_validation, ConnectionIdValidationPolicy::Strict);
    }

    #[test]
    fn it_should_deserialize_strict_connection_id_validation() {
        let toml = r#"connection_id_validation = "strict""#;
        let config: UdpTrackerServer = toml::from_str(toml).expect("strict should deserialize");
        assert_eq!(config.connection_id_validation, ConnectionIdValidationPolicy::Strict);
    }

    #[test]
    fn it_should_deserialize_disabled_connection_id_validation() {
        let toml = r#"connection_id_validation = "disabled""#;
        let config: UdpTrackerServer = toml::from_str(toml).expect("disabled should deserialize");
        assert_eq!(config.connection_id_validation, ConnectionIdValidationPolicy::Disabled);
    }

    #[test]
    fn it_should_round_trip_strict_connection_id_validation() {
        let original = UdpTrackerServer::default();
        let serialized = toml::to_string(&original).expect("should serialize");
        let deserialized: UdpTrackerServer = toml::from_str(&serialized).expect("should deserialize");
        assert_eq!(deserialized.connection_id_validation, ConnectionIdValidationPolicy::Strict);
    }

    #[test]
    fn it_should_round_trip_disabled_connection_id_validation() {
        let original = UdpTrackerServer {
            connection_id_validation: ConnectionIdValidationPolicy::Disabled,
            ..UdpTrackerServer::default()
        };
        let serialized = toml::to_string(&original).expect("should serialize");
        let deserialized: UdpTrackerServer = toml::from_str(&serialized).expect("should deserialize");
        assert_eq!(deserialized.connection_id_validation, ConnectionIdValidationPolicy::Disabled);
    }
}
