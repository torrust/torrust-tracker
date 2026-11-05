//! API resources for the stats context.
//!
//! These types define the serialization contract for the `/api/v1/stats`
//! and `/api/v1/metrics` endpoint responses.
use serde::{Deserialize, Serialize};
use torrust_metrics::metric_collection::MetricCollection;

/// Tracker statistics response for the `GET /api/v1/stats` endpoint.
///
/// The Prometheus name, type and description of every field are in [`STATS_DESCRIPTIONS`].
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Stats {
    // Torrent metrics
    /// Total number of torrents.
    pub torrents: u64,
    /// Total number of seeders for all torrents.
    pub seeders: u64,
    /// Deprecated ambiguous completed-download total. Use
    /// [`Self::completed_in_session`] and [`Self::completed_persisted`] instead.
    pub completed: u64,
    /// Completed downloads observed since the current tracker process started.
    #[serde(default)]
    pub completed_in_session: u64,
    /// Completed downloads restored from and maintained in persistent storage.
    ///
    /// This is zero when persistence is disabled; use
    /// [`Self::completed_persisted_enabled`] to distinguish that state from an
    /// enabled persisted counter whose observed value is zero.
    #[serde(default)]
    pub completed_persisted: u64,
    /// Whether [`Self::completed_persisted`] is backed by persistent storage.
    #[serde(default)]
    pub completed_persisted_enabled: bool,
    /// Total number of leechers for all torrents.
    pub leechers: u64,

    // Protocol metrics
    /// Total number of TCP (HTTP tracker) connections from IPv4 peers.
    pub tcp4_connections_handled: u64,
    /// Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers.
    pub tcp4_announces_handled: u64,
    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv4 peers.
    pub tcp4_scrapes_handled: u64,
    /// Total number of TCP (HTTP tracker) connections from IPv6 peers.
    pub tcp6_connections_handled: u64,
    /// Total number of TCP (HTTP tracker) `announce` requests from IPv6 peers.
    pub tcp6_announces_handled: u64,
    /// Total number of TCP (HTTP tracker) `scrape` requests from IPv6 peers.
    pub tcp6_scrapes_handled: u64,

    // UDP
    /// Total number of UDP (UDP tracker) requests discarded before processing (e.g. client source port is 0).
    pub udp_requests_discarded: u64,
    /// Total number of UDP (UDP tracker) requests aborted.
    pub udp_requests_aborted: u64,
    /// Total number of UDP (UDP tracker) requests banned.
    pub udp_requests_banned: u64,
    /// Number of IPs whose UDP connection ID errors exceed the limit: banned IPs
    /// when connection ID validation is `strict`, and "would-be banned" IPs that
    /// are not blocked when it is `disabled`.
    pub udp_banned_ips_total: u64,
    /// Average rounded time spent processing UDP connect requests.
    pub udp_avg_connect_processing_time_ns: u64,
    /// Average rounded time spent processing UDP announce requests.
    pub udp_avg_announce_processing_time_ns: u64,
    /// Average rounded time spent processing UDP scrape requests.
    pub udp_avg_scrape_processing_time_ns: u64,

    // UDPv4
    /// Total number of UDP (UDP tracker) requests from IPv4 peers.
    pub udp4_requests: u64,
    /// Total number of UDP (UDP tracker) connections from IPv4 peers.
    pub udp4_connections_handled: u64,
    /// Total number of UDP (UDP tracker) `announce` requests from IPv4 peers.
    pub udp4_announces_handled: u64,
    /// Total number of UDP (UDP tracker) `scrape` requests from IPv4 peers.
    pub udp4_scrapes_handled: u64,
    /// Total number of UDP (UDP tracker) responses from IPv4 peers.
    pub udp4_responses: u64,
    /// Total number of UDP (UDP tracker) errors handled from IPv4 peers.
    pub udp4_errors_handled: u64,

    // UDPv6
    /// Total number of UDP (UDP tracker) requests from IPv6 peers.
    pub udp6_requests: u64,
    /// Total number of UDP (UDP tracker) `connection` requests from IPv6 peers.
    pub udp6_connections_handled: u64,
    /// Total number of UDP (UDP tracker) `announce` requests from IPv6 peers.
    pub udp6_announces_handled: u64,
    /// Total number of UDP (UDP tracker) `scrape` requests from IPv6 peers.
    pub udp6_scrapes_handled: u64,
    /// Total number of UDP (UDP tracker) responses from IPv6 peers.
    pub udp6_responses: u64,
    /// Total number of UDP (UDP tracker) errors handled from IPv6 peers.
    pub udp6_errors_handled: u64,
}

/// Prometheus metric type of a [`Stats`] field in the `?format=prometheus` text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetricType {
    /// A value that only grows while the tracker runs (or is restored from storage at startup).
    Counter,
    /// A value that can go up and down.
    Gauge,
}

impl MetricType {
    /// The word used in the `# TYPE` line of the Prometheus text format.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Gauge => "gauge",
        }
    }
}

/// Prometheus name, type and description of one [`Stats`] field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatDescription {
    /// Metric name; the same as the JSON field name (part of API v1).
    pub name: &'static str,
    /// Metric type printed in the `# TYPE` line.
    pub metric_type: MetricType,
    /// One-line description printed in the `# HELP` line.
    pub help: &'static str,
}

/// Number of fields of [`Stats`], that is, of metrics in the legacy Prometheus text.
pub const STATS_FIELD_COUNT: usize = 32;

/// The single source of the Prometheus `# HELP` and `# TYPE` of every [`Stats`] field, in the order of the JSON fields.
///
/// The Prometheus text renderer reads this table, so the description of a field exists once. When a field is added to [`Stats`],
/// add it here and to [`Stats::values`] in the same position; a test checks the names and the order against the JSON.
pub const STATS_DESCRIPTIONS: [StatDescription; STATS_FIELD_COUNT] = [
    StatDescription {
        name: "torrents",
        metric_type: MetricType::Gauge,
        help: "Total number of torrents.",
    },
    StatDescription {
        name: "seeders",
        metric_type: MetricType::Gauge,
        help: "Total number of seeders for all torrents.",
    },
    StatDescription {
        name: "completed",
        metric_type: MetricType::Counter,
        help: "Deprecated ambiguous completed-download total. Use completed_in_session and completed_persisted instead.",
    },
    StatDescription {
        name: "completed_in_session",
        metric_type: MetricType::Counter,
        help: "Completed downloads observed since the current tracker process started.",
    },
    StatDescription {
        name: "completed_persisted",
        metric_type: MetricType::Counter,
        help: "Completed downloads restored from and maintained in persistent storage. Zero when persistence is disabled.",
    },
    StatDescription {
        name: "completed_persisted_enabled",
        metric_type: MetricType::Gauge,
        help: "Whether completed_persisted is backed by persistent storage (1 enabled, 0 disabled).",
    },
    StatDescription {
        name: "leechers",
        metric_type: MetricType::Gauge,
        help: "Total number of leechers for all torrents.",
    },
    StatDescription {
        name: "tcp4_connections_handled",
        metric_type: MetricType::Counter,
        help: "Total number of TCP (HTTP tracker) connections from IPv4 peers.",
    },
    StatDescription {
        name: "tcp4_announces_handled",
        metric_type: MetricType::Counter,
        help: "Total number of TCP (HTTP tracker) announce requests from IPv4 peers.",
    },
    StatDescription {
        name: "tcp4_scrapes_handled",
        metric_type: MetricType::Counter,
        help: "Total number of TCP (HTTP tracker) scrape requests from IPv4 peers.",
    },
    StatDescription {
        name: "tcp6_connections_handled",
        metric_type: MetricType::Counter,
        help: "Total number of TCP (HTTP tracker) connections from IPv6 peers.",
    },
    StatDescription {
        name: "tcp6_announces_handled",
        metric_type: MetricType::Counter,
        help: "Total number of TCP (HTTP tracker) announce requests from IPv6 peers.",
    },
    StatDescription {
        name: "tcp6_scrapes_handled",
        metric_type: MetricType::Counter,
        help: "Total number of TCP (HTTP tracker) scrape requests from IPv6 peers.",
    },
    StatDescription {
        name: "udp_requests_discarded",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) requests discarded before processing (e.g. client source port is 0).",
    },
    StatDescription {
        name: "udp_requests_aborted",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) requests aborted.",
    },
    StatDescription {
        name: "udp_requests_banned",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) requests banned.",
    },
    StatDescription {
        name: "udp_banned_ips_total",
        metric_type: MetricType::Gauge,
        help: "Number of IPs whose UDP connection ID errors exceed the limit (banned when connection ID validation is strict, would-be banned when it is disabled).",
    },
    StatDescription {
        name: "udp_avg_connect_processing_time_ns",
        metric_type: MetricType::Gauge,
        help: "Average rounded time spent processing UDP connect requests, in nanoseconds.",
    },
    StatDescription {
        name: "udp_avg_announce_processing_time_ns",
        metric_type: MetricType::Gauge,
        help: "Average rounded time spent processing UDP announce requests, in nanoseconds.",
    },
    StatDescription {
        name: "udp_avg_scrape_processing_time_ns",
        metric_type: MetricType::Gauge,
        help: "Average rounded time spent processing UDP scrape requests, in nanoseconds.",
    },
    StatDescription {
        name: "udp4_requests",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) requests from IPv4 peers.",
    },
    StatDescription {
        name: "udp4_connections_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) connections from IPv4 peers.",
    },
    StatDescription {
        name: "udp4_announces_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) announce requests from IPv4 peers.",
    },
    StatDescription {
        name: "udp4_scrapes_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) scrape requests from IPv4 peers.",
    },
    StatDescription {
        name: "udp4_responses",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) responses to IPv4 peers.",
    },
    StatDescription {
        name: "udp4_errors_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) errors handled for IPv4 peers.",
    },
    StatDescription {
        name: "udp6_requests",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) requests from IPv6 peers.",
    },
    StatDescription {
        name: "udp6_connections_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) connection requests from IPv6 peers.",
    },
    StatDescription {
        name: "udp6_announces_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) announce requests from IPv6 peers.",
    },
    StatDescription {
        name: "udp6_scrapes_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) scrape requests from IPv6 peers.",
    },
    StatDescription {
        name: "udp6_responses",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) responses to IPv6 peers.",
    },
    StatDescription {
        name: "udp6_errors_handled",
        metric_type: MetricType::Counter,
        help: "Total number of UDP (UDP tracker) errors handled for IPv6 peers.",
    },
];

impl Stats {
    /// The values of all fields, in the order of [`STATS_DESCRIPTIONS`].
    ///
    /// The boolean `completed_persisted_enabled` is `1` or `0`, because a Prometheus sample value must be a number.
    #[allow(deprecated)]
    #[must_use]
    pub fn values(&self) -> [u64; STATS_FIELD_COUNT] {
        [
            self.torrents,
            self.seeders,
            self.completed,
            self.completed_in_session,
            self.completed_persisted,
            u64::from(self.completed_persisted_enabled),
            self.leechers,
            self.tcp4_connections_handled,
            self.tcp4_announces_handled,
            self.tcp4_scrapes_handled,
            self.tcp6_connections_handled,
            self.tcp6_announces_handled,
            self.tcp6_scrapes_handled,
            self.udp_requests_discarded,
            self.udp_requests_aborted,
            self.udp_requests_banned,
            self.udp_banned_ips_total,
            self.udp_avg_connect_processing_time_ns,
            self.udp_avg_announce_processing_time_ns,
            self.udp_avg_scrape_processing_time_ns,
            self.udp4_requests,
            self.udp4_connections_handled,
            self.udp4_announces_handled,
            self.udp4_scrapes_handled,
            self.udp4_responses,
            self.udp4_errors_handled,
            self.udp6_requests,
            self.udp6_connections_handled,
            self.udp6_announces_handled,
            self.udp6_scrapes_handled,
            self.udp6_responses,
            self.udp6_errors_handled,
        ]
    }
}

/// Extendable metrics response for the `GET /api/v1/metrics` endpoint.
///
/// Contains structured labeled metrics that can be serialized to JSON
/// or Prometheus format.
#[derive(Serialize, Debug, PartialEq)]
pub struct LabeledStats {
    /// The labeled metrics collection from all tracker subsystems.
    pub metrics: MetricCollection,
}

#[cfg(test)]
mod tests {
    use super::{STATS_DESCRIPTIONS, Stats};

    #[test]
    fn it_should_deserialize_a_legacy_stats_response() {
        // Arrange
        let payload = r#"{"torrents":0,"seeders":0,"completed":0,"leechers":0,"tcp4_connections_handled":0,"tcp4_announces_handled":0,"tcp4_scrapes_handled":0,"tcp6_connections_handled":0,"tcp6_announces_handled":0,"tcp6_scrapes_handled":0,"udp_requests_discarded":0,"udp_requests_aborted":0,"udp_requests_banned":0,"udp_banned_ips_total":0,"udp_avg_connect_processing_time_ns":0,"udp_avg_announce_processing_time_ns":0,"udp_avg_scrape_processing_time_ns":0,"udp4_requests":0,"udp4_connections_handled":0,"udp4_announces_handled":0,"udp4_scrapes_handled":0,"udp4_responses":0,"udp4_errors_handled":0,"udp6_requests":0,"udp6_connections_handled":0,"udp6_announces_handled":0,"udp6_scrapes_handled":0,"udp6_responses":0,"udp6_errors_handled":0}"#;

        // Act
        let stats: Stats = serde_json::from_str(payload).unwrap();

        // Assert
        assert_eq!(stats.completed_in_session, 0);
        assert_eq!(stats.completed_persisted, 0);
        assert!(!stats.completed_persisted_enabled);
    }

    #[test]
    fn it_should_describe_every_field_once_in_the_order_of_the_json_fields() {
        let stats = Stats {
            torrents: 1,
            seeders: 2,
            completed: 3,
            completed_in_session: 4,
            completed_persisted: 5,
            completed_persisted_enabled: true,
            leechers: 7,
            tcp4_connections_handled: 8,
            tcp4_announces_handled: 9,
            tcp4_scrapes_handled: 10,
            tcp6_connections_handled: 11,
            tcp6_announces_handled: 12,
            tcp6_scrapes_handled: 13,
            udp_requests_discarded: 14,
            udp_requests_aborted: 15,
            udp_requests_banned: 16,
            udp_banned_ips_total: 17,
            udp_avg_connect_processing_time_ns: 18,
            udp_avg_announce_processing_time_ns: 19,
            udp_avg_scrape_processing_time_ns: 20,
            udp4_requests: 21,
            udp4_connections_handled: 22,
            udp4_announces_handled: 23,
            udp4_scrapes_handled: 24,
            udp4_responses: 25,
            udp4_errors_handled: 26,
            udp6_requests: 27,
            udp6_connections_handled: 28,
            udp6_announces_handled: 29,
            udp6_scrapes_handled: 30,
            udp6_responses: 31,
            udp6_errors_handled: 32,
        };

        // The text serialization keeps the declaration order of the fields.
        let text = serde_json::to_string(&stats).unwrap();
        let json: serde_json::Value = serde_json::from_str(&text).unwrap();
        let values = stats.values();

        assert_eq!(json.as_object().unwrap().len(), STATS_DESCRIPTIONS.len());

        let mut position = 0;

        for (index, description) in STATS_DESCRIPTIONS.iter().enumerate() {
            let found = text[position..]
                .find(&format!("\"{}\":", description.name))
                .unwrap_or_else(|| panic!("{} is missing or out of order in the JSON", description.name));
            position += found + description.name.len();

            assert!(!description.help.is_empty(), "{} has no help", description.name);
            // The boolean is the only field that is not a number in the JSON.
            let value = &json[description.name];
            let expected = value.as_u64().unwrap_or_else(|| u64::from(value.as_bool() == Some(true)));
            assert_eq!(values[index], expected, "{}", description.name);
        }
    }
}
