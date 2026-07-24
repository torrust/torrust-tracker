// adr: docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
// issue: #1453
//! UDP tracker server-wide configuration for schema v3.
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

use crate::v3_0_0::types::AtLeastU64;

/// Configuration shared by every UDP tracker listener.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UdpTrackerServer {
    /// Seconds between resets of the temporary IP-ban filters.
    #[serde(default = "default_ip_bans_reset_interval_in_secs")]
    pub ip_bans_reset_interval_in_secs: IpBansResetIntervalInSecs,
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

impl Default for UdpTrackerServer {
    fn default() -> Self {
        Self {
            ip_bans_reset_interval_in_secs: default_ip_bans_reset_interval_in_secs(),
        }
    }
}

fn default_ip_bans_reset_interval_in_secs() -> IpBansResetIntervalInSecs {
    IpBansResetIntervalInSecs::new(UdpTrackerServer::DEFAULT_IP_BANS_RESET_INTERVAL_IN_SECS)
        .expect("the default IP-ban reset interval must satisfy its minimum")
}

#[cfg(test)]
mod tests {
    use crate::v3_0_0::udp_tracker_server::{IpBansResetIntervalInSecs, UdpTrackerServer};

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
}
