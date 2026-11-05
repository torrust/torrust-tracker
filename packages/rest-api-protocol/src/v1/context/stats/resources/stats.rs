//! API resources for the stats context.
//!
//! These types define the serialization contract for the `/api/v1/stats`
//! and `/api/v1/metrics` endpoint responses.
use serde::{Deserialize, Serialize};
use torrust_metrics::metric_collection::MetricCollection;

/// Declares [`Stats`], its field documentation and [`STATS_DESCRIPTIONS`] from one list, so the description of a field
/// exists once and cannot drift between the Rust docs and the Prometheus `# HELP` text.
///
/// Each entry is `name: type = MetricType, "one-line description";`, optionally preceded by attributes (more doc lines, serde).
/// The order is the order of the JSON fields and of the Prometheus text.
macro_rules! stats {
    ($( $(#[$attr:meta])* $name:ident : $ty:ty = $metric_type:ident, $help:literal; )*) => {
        /// Tracker statistics response for the `GET /api/v1/stats` endpoint.
        ///
        /// The Prometheus name, type and description of every field are in [`STATS_DESCRIPTIONS`].
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        #[allow(
            clippy::doc_markdown,
            reason = "the one-line HELP text names other fields in plain text, because it is also printed in the Prometheus output"
        )]
        pub struct Stats {
            $(
                #[doc = $help]
                $(#[$attr])*
                pub $name: $ty,
            )*
        }

        /// Number of fields of [`Stats`], that is, of metrics in the legacy Prometheus text.
        pub const STATS_FIELD_COUNT: usize = [$( stringify!($name) ),*].len();

        /// The single source of the Prometheus `# HELP` and `# TYPE` of every [`Stats`] field, in the order of the JSON fields.
        ///
        /// The same list declares the fields and their documentation, so the description of a field exists once. The Prometheus
        /// text renderer reads this table.
        ///
        /// The names are part of API v1 and do not follow every Prometheus naming convention (`promtool check metrics` warns about
        /// the missing `_total` suffix of counters, the abbreviated `_ns` unit and the `_total` suffix of the gauge
        /// `udp_banned_ips_total`). Renaming them is a breaking change; it is planned for the next API version (see issue #2734).
        pub const STATS_DESCRIPTIONS: [StatDescription; STATS_FIELD_COUNT] = [
            $(
                StatDescription {
                    name: stringify!($name),
                    metric_type: MetricType::$metric_type,
                    help: $help,
                },
            )*
        ];

        impl Stats {
            /// The values of all fields, in the order of [`STATS_DESCRIPTIONS`].
            ///
            /// The boolean `completed_persisted_enabled` is `1` or `0`, because a Prometheus sample value must be a number.
            #[must_use]
            pub fn values(&self) -> [u64; STATS_FIELD_COUNT] {
                [ $( u64::from(self.$name) ),* ]
            }
        }
    };
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

stats! {
        // Torrent metrics
        torrents: u64 = Gauge, "Total number of torrents.";
        seeders: u64 = Gauge, "Total number of seeders for all torrents.";
        ///
        /// Use [`Self::completed_in_session`] and [`Self::completed_persisted`] instead.
        completed: u64 = Counter, "Deprecated ambiguous completed-download total. Use completed_in_session and completed_persisted instead.";
        #[serde(default)]
        completed_in_session: u64 = Counter, "Completed downloads observed since the current tracker process started.";
        ///
        /// This is zero when persistence is disabled; use [`Self::completed_persisted_enabled`] to distinguish that state from an
        /// enabled persisted counter whose observed value is zero.
        #[serde(default)]
        completed_persisted: u64 = Counter, "Completed downloads restored from and maintained in persistent storage. Zero when persistence is disabled.";
        #[serde(default)]
        completed_persisted_enabled: bool = Gauge, "Whether completed_persisted is backed by persistent storage (1 enabled, 0 disabled).";
        leechers: u64 = Gauge, "Total number of leechers for all torrents.";
        // TCP
        tcp4_connections_handled: u64 = Counter, "Total number of TCP (HTTP tracker) connections from IPv4 peers.";
        tcp4_announces_handled: u64 = Counter, "Total number of TCP (HTTP tracker) announce requests from IPv4 peers.";
        tcp4_scrapes_handled: u64 = Counter, "Total number of TCP (HTTP tracker) scrape requests from IPv4 peers.";
        tcp6_connections_handled: u64 = Counter, "Total number of TCP (HTTP tracker) connections from IPv6 peers.";
        tcp6_announces_handled: u64 = Counter, "Total number of TCP (HTTP tracker) announce requests from IPv6 peers.";
        tcp6_scrapes_handled: u64 = Counter, "Total number of TCP (HTTP tracker) scrape requests from IPv6 peers.";
        // UDP
        udp_requests_discarded: u64 = Counter, "Total number of UDP (UDP tracker) requests discarded before processing (e.g. client source port is 0).";
        udp_requests_aborted: u64 = Counter, "Total number of UDP (UDP tracker) requests aborted.";
        udp_requests_banned: u64 = Counter, "Total number of UDP (UDP tracker) requests banned.";
        ///
        /// Banned IPs when connection ID validation is `strict`, and "would-be banned" IPs that are not blocked when it is `disabled`.
        udp_banned_ips_total: u64 = Gauge, "Number of IPs whose UDP connection ID errors exceed the limit (banned when connection ID validation is strict, would-be banned when it is disabled).";
        udp_avg_connect_processing_time_ns: u64 = Gauge, "Average rounded time spent processing UDP connect requests, in nanoseconds.";
        udp_avg_announce_processing_time_ns: u64 = Gauge, "Average rounded time spent processing UDP announce requests, in nanoseconds.";
        udp_avg_scrape_processing_time_ns: u64 = Gauge, "Average rounded time spent processing UDP scrape requests, in nanoseconds.";
        // UDPv4
        udp4_requests: u64 = Counter, "Total number of UDP (UDP tracker) requests from IPv4 peers.";
        udp4_connections_handled: u64 = Counter, "Total number of UDP (UDP tracker) connections from IPv4 peers.";
        udp4_announces_handled: u64 = Counter, "Total number of UDP (UDP tracker) announce requests from IPv4 peers.";
        udp4_scrapes_handled: u64 = Counter, "Total number of UDP (UDP tracker) scrape requests from IPv4 peers.";
        udp4_responses: u64 = Counter, "Total number of UDP (UDP tracker) responses to IPv4 peers.";
        udp4_errors_handled: u64 = Counter, "Total number of UDP (UDP tracker) errors handled for IPv4 peers.";
        // UDPv6
        udp6_requests: u64 = Counter, "Total number of UDP (UDP tracker) requests from IPv6 peers.";
        udp6_connections_handled: u64 = Counter, "Total number of UDP (UDP tracker) connection requests from IPv6 peers.";
        udp6_announces_handled: u64 = Counter, "Total number of UDP (UDP tracker) announce requests from IPv6 peers.";
        udp6_scrapes_handled: u64 = Counter, "Total number of UDP (UDP tracker) scrape requests from IPv6 peers.";
        udp6_responses: u64 = Counter, "Total number of UDP (UDP tracker) responses to IPv6 peers.";
        udp6_errors_handled: u64 = Counter, "Total number of UDP (UDP tracker) errors handled for IPv6 peers.";
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
