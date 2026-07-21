//! Per-tracker network topology configuration for schema v3.
//!
//! See `docs/adrs/20260721000000_make_network_configuration_per_tracker_instance.md`.

use std::convert::TryFrom;
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Network {
    /// The external IP address of the tracker. If the client is using a
    /// loopback IP address, this IP address will be used instead. If the peer
    /// is using a loopback IP address, the tracker assumes that the peer is
    /// in the same network as the tracker and will use the tracker's IP
    /// address instead.
    #[serde(default = "Network::default_external_ip")]
    pub external_ip: Option<ExternalIp>,

    /// Whether the tracker is behind a reverse proxy or not.
    /// If the tracker is behind a reverse proxy, the `X-Forwarded-For` header
    /// sent from the proxy will be used to get the client's IP address.
    #[serde(default = "Network::default_on_reverse_proxy")]
    pub on_reverse_proxy: bool,

    /// Whether to set `IPV6_V6ONLY=1` on IPv6 sockets.
    ///
    /// When `true` (IPv6-only), the tracker must also bind an IPv4 socket
    /// (for example, `0.0.0.0:<port>`) to accept IPv4 connections. When
    /// `false` (the default), the socket option is not overridden and the OS
    /// default applies.
    ///
    /// On OpenBSD, `IPV6_V6ONLY` is always `1` and cannot be disabled; setting
    /// this to `false` is a no-op.
    #[serde(default = "Network::default_ipv6_v6only")]
    pub ipv6_v6only: bool,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            external_ip: Self::default_external_ip(),
            on_reverse_proxy: Self::default_on_reverse_proxy(),
            ipv6_v6only: Self::default_ipv6_v6only(),
        }
    }
}

impl Network {
    fn default_external_ip() -> Option<ExternalIp> {
        None
    }

    fn default_on_reverse_proxy() -> bool {
        false
    }

    fn default_ipv6_v6only() -> bool {
        false
    }
}
/// A validated external IP address that is guaranteed not to be a wildcard
/// address (`0.0.0.0` or `::`).
///
/// Wildcard addresses are never valid external IPs. This type enforces that
/// constraint at construction time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct ExternalIp(IpAddr);

impl TryFrom<IpAddr> for ExternalIp {
    type Error = &'static str;

    fn try_from(ip: IpAddr) -> Result<Self, Self::Error> {
        if ip.is_unspecified() {
            Err("wildcard/unspecified IP address is not a valid external IP")
        } else {
            Ok(Self(ip))
        }
    }
}

impl FromStr for ExternalIp {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip: IpAddr = s.parse().map_err(|_| "invalid IP address format")?;
        ExternalIp::try_from(ip)
    }
}

impl From<ExternalIp> for IpAddr {
    fn from(ip: ExternalIp) -> Self {
        ip.0
    }
}

impl fmt::Display for ExternalIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Custom deserialize to reject unspecified addresses
impl<'de> Deserialize<'de> for ExternalIp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ip = IpAddr::deserialize(deserializer)?;
        ExternalIp::try_from(ip).map_err(serde::de::Error::custom)
    }
}
