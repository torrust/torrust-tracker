use std::sync::Arc;

use crate::tracker::statistics::Metrics;
use crate::tracker::{TorrentsMetrics, Tracker};

#[derive(Debug, PartialEq)]
pub struct TrackerMetrics {
    pub torrents_metrics: TorrentsMetrics,
    pub protocol_metrics: Metrics,
}

pub async fn get_metrics(tracker: Arc<Tracker>) -> TrackerMetrics {
    let torrents_metrics = tracker.get_torrents_metrics().await;
    let stats = tracker.get_stats().await;

    TrackerMetrics {
        torrents_metrics,
        protocol_metrics: Metrics {
            tcp4_connections_handled: stats.tcp4_connections_handled,
            tcp4_announces_handled: stats.tcp4_announces_handled,
            tcp4_scrapes_handled: stats.tcp4_scrapes_handled,
            tcp6_connections_handled: stats.tcp6_connections_handled,
            tcp6_announces_handled: stats.tcp6_announces_handled,
            tcp6_scrapes_handled: stats.tcp6_scrapes_handled,
            udp4_connections_handled: stats.udp4_connections_handled,
            udp4_announces_handled: stats.udp4_announces_handled,
            udp4_scrapes_handled: stats.udp4_scrapes_handled,
            udp6_connections_handled: stats.udp6_connections_handled,
            udp6_announces_handled: stats.udp6_announces_handled,
            udp6_scrapes_handled: stats.udp6_scrapes_handled,
        },
    }
}
