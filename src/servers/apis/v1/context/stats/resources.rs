use serde::{Deserialize, Serialize};

use crate::tracker::services::statistics::TrackerMetrics;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Stats {
    pub torrents: u64,
    pub seeders: u64,
    pub completed: u64,
    pub leechers: u64,
    pub tcp4_connections_handled: u64,
    pub tcp4_announces_handled: u64,
    pub tcp4_scrapes_handled: u64,
    pub tcp6_connections_handled: u64,
    pub tcp6_announces_handled: u64,
    pub tcp6_scrapes_handled: u64,
    pub udp4_connections_handled: u64,
    pub udp4_announces_handled: u64,
    pub udp4_scrapes_handled: u64,
    pub udp6_connections_handled: u64,
    pub udp6_announces_handled: u64,
    pub udp6_scrapes_handled: u64,
}

impl From<TrackerMetrics> for Stats {
    fn from(metrics: TrackerMetrics) -> Self {
        Self {
            torrents: metrics.torrents_metrics.torrents,
            seeders: metrics.torrents_metrics.seeders,
            completed: metrics.torrents_metrics.completed,
            leechers: metrics.torrents_metrics.leechers,
            tcp4_connections_handled: metrics.protocol_metrics.tcp4_connections_handled,
            tcp4_announces_handled: metrics.protocol_metrics.tcp4_announces_handled,
            tcp4_scrapes_handled: metrics.protocol_metrics.tcp4_scrapes_handled,
            tcp6_connections_handled: metrics.protocol_metrics.tcp6_connections_handled,
            tcp6_announces_handled: metrics.protocol_metrics.tcp6_announces_handled,
            tcp6_scrapes_handled: metrics.protocol_metrics.tcp6_scrapes_handled,
            udp4_connections_handled: metrics.protocol_metrics.udp4_connections_handled,
            udp4_announces_handled: metrics.protocol_metrics.udp4_announces_handled,
            udp4_scrapes_handled: metrics.protocol_metrics.udp4_scrapes_handled,
            udp6_connections_handled: metrics.protocol_metrics.udp6_connections_handled,
            udp6_announces_handled: metrics.protocol_metrics.udp6_announces_handled,
            udp6_scrapes_handled: metrics.protocol_metrics.udp6_scrapes_handled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stats;
    use crate::tracker::services::statistics::TrackerMetrics;
    use crate::tracker::statistics::Metrics;
    use crate::tracker::TorrentsMetrics;

    #[test]
    fn stats_resource_should_be_converted_from_tracker_metrics() {
        assert_eq!(
            Stats::from(TrackerMetrics {
                torrents_metrics: TorrentsMetrics {
                    seeders: 1,
                    completed: 2,
                    leechers: 3,
                    torrents: 4
                },
                protocol_metrics: Metrics {
                    tcp4_connections_handled: 5,
                    tcp4_announces_handled: 6,
                    tcp4_scrapes_handled: 7,
                    tcp6_connections_handled: 8,
                    tcp6_announces_handled: 9,
                    tcp6_scrapes_handled: 10,
                    udp4_connections_handled: 11,
                    udp4_announces_handled: 12,
                    udp4_scrapes_handled: 13,
                    udp6_connections_handled: 14,
                    udp6_announces_handled: 15,
                    udp6_scrapes_handled: 16
                }
            }),
            Stats {
                torrents: 4,
                seeders: 1,
                completed: 2,
                leechers: 3,
                tcp4_connections_handled: 5,
                tcp4_announces_handled: 6,
                tcp4_scrapes_handled: 7,
                tcp6_connections_handled: 8,
                tcp6_announces_handled: 9,
                tcp6_scrapes_handled: 10,
                udp4_connections_handled: 11,
                udp4_announces_handled: 12,
                udp4_scrapes_handled: 13,
                udp6_connections_handled: 14,
                udp6_announces_handled: 15,
                udp6_scrapes_handled: 16
            }
        );
    }
}
