//! Statistics services.
//!
//! It includes:
//!
//! - A [`factory`](crate::statistics::setup::factory) function to build the structs needed to collect the tracker metrics.
//! - A [`get_metrics`] service to get the tracker [`metrics`](crate::statistics::metrics::Metrics).
//!
//! Tracker metrics are collected using a Publisher-Subscribe pattern.
//!
//! The factory function builds two structs:
//!
//! - An statistics event [`Sender`](crate::statistics::event::sender::Sender)
//! - An statistics [`Repository`]
//!
//! ```text
//! let (stats_event_sender, stats_repository) = factory(tracker_usage_statistics);
//! ```
//!
//! The statistics repository is responsible for storing the metrics in memory.
//! The statistics event sender allows sending events related to metrics.
//! There is an event listener that is receiving all the events and processing them with an event handler.
//! Then, the event handler updates the metrics depending on the received event.
use std::sync::Arc;

use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use torrust_tracker_primitives::swarm_metadata::AggregateSwarmMetadata;

use crate::statistics::metrics::Metrics;
use crate::statistics::repository::Repository;

/// All the metrics collected by the tracker.
#[derive(Debug, PartialEq)]
pub struct TrackerMetrics {
    /// Domain level metrics.
    ///
    /// General metrics for all torrents (number of seeders, leechers, etcetera)
    pub torrents_metrics: AggregateSwarmMetadata,

    /// Application level metrics. Usage statistics/metrics.
    ///
    /// Metrics about how the tracker is been used (number of  number of http scrape requests, etcetera)
    pub protocol_metrics: Metrics,
}

/// It returns all the [`TrackerMetrics`]
pub async fn get_metrics(
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    stats_repository: Arc<Repository>,
) -> TrackerMetrics {
    let torrents_metrics = in_memory_torrent_repository.get_torrents_metrics();
    let stats = stats_repository.get_stats().await;

    TrackerMetrics {
        torrents_metrics,
        protocol_metrics: Metrics {
            // TCPv4
            tcp4_announces_handled: stats.tcp4_announces_handled,
            tcp4_scrapes_handled: stats.tcp4_scrapes_handled,
            // TCPv6
            tcp6_announces_handled: stats.tcp6_announces_handled,
            tcp6_scrapes_handled: stats.tcp6_scrapes_handled,
            // Samples
            metric_collection: stats.metric_collection.clone(),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use bittorrent_tracker_core::{self};
    use torrust_tracker_configuration::Configuration;
    use torrust_tracker_primitives::swarm_metadata::AggregateSwarmMetadata;
    use torrust_tracker_test_helpers::configuration;

    use crate::statistics::event::listener::run_event_listener;
    use crate::statistics::services::{get_metrics, TrackerMetrics};
    use crate::statistics::{self, describe_metrics};

    pub fn tracker_configuration() -> Configuration {
        configuration::ephemeral()
    }

    #[tokio::test]
    async fn the_statistics_service_should_return_the_tracker_metrics() {
        let config = tracker_configuration();

        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

        // HTTP core stats
        let (http_stats_keeper, http_stats_repository) = statistics::setup::factory(config.core.tracker_usage_statistics);

        if config.core.tracker_usage_statistics {
            let _unused = run_event_listener(http_stats_keeper.receiver(), &http_stats_repository);
        }

        let tracker_metrics = get_metrics(in_memory_torrent_repository.clone(), http_stats_repository).await;

        assert_eq!(
            tracker_metrics,
            TrackerMetrics {
                torrents_metrics: AggregateSwarmMetadata::default(),
                protocol_metrics: describe_metrics(),
            }
        );
    }
}
