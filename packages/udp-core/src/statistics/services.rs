//! Statistics services.
//!
//! It includes a [`get_metrics`] service to get the tracker [`metrics`](crate::statistics::metrics::Metrics).
//!
//! Tracker metrics are collected using a Publisher-Subscribe pattern.
//!
//! [`UdpTrackerCoreServices::initialize_from`](crate::container::UdpTrackerCoreServices::initialize_from)
//! builds the two structs involved:
//!
//! - An event [`Sender`](crate::event::sender::Sender), taken from the services' event bus
//! - An statistics [`Repository`]
//!
//! The statistics repository is responsible for storing the metrics in memory.
//! The statistics event sender allows sending events related to metrics.
//! There is an event listener that is receiving all the events and processing them with an event handler.
//! Then, the event handler updates the metrics depending on the received event.
//!
//! For example, if you send the event [`Event::UdpConnect`](crate::event::Event::UdpConnect):
//!
//! ```text
//! let result = event_sender.send_event(Event::UdpConnect { connection }).await;
//! ```
//!
//! Eventually the counter for UDP connections from IPv4 peers will be increased.
//!
//! ```rust,no_run
//! pub struct Metrics {
//!     // ...
//!     pub udp4_connections_handled: u64,  // This will be incremented
//!     // ...
//! }
//! ```
use std::sync::Arc;

use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata;
use torrust_tracker_swarm_coordination_registry::swarm::registry::Error as SwarmRegistryError;

use crate::statistics::metrics::Metrics;
use crate::statistics::repository::Repository;

/// All the metrics collected by the tracker.
#[derive(Debug, PartialEq)]
pub struct TrackerMetrics {
    /// Domain level metrics.
    ///
    /// General metrics for all torrents (number of seeders, leechers, etcetera)
    pub torrents_metrics: AggregateActiveSwarmMetadata,

    /// Application level metrics. Usage statistics/metrics.
    ///
    /// Metrics about how the tracker is been used (number of udp announce requests, etcetera)
    pub protocol_metrics: Metrics,
}

/// It returns all the [`TrackerMetrics`]
///
/// # Errors
///
/// Propagates the swarm registry error from the torrent repository.
pub async fn get_metrics(
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    stats_repository: Arc<Repository>,
) -> Result<TrackerMetrics, SwarmRegistryError> {
    let torrents_metrics = in_memory_torrent_repository.get_aggregate_swarm_metadata().await?;
    let stats = stats_repository.get_stats().await;

    Ok(TrackerMetrics {
        torrents_metrics,
        protocol_metrics: Metrics {
            metric_collection: stats.metric_collection.clone(),
        },
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
    use torrust_tracker_core::{self};
    use torrust_tracker_primitives::swarm_metadata::AggregateActiveSwarmMetadata;

    use crate::statistics::describe_metrics;
    use crate::statistics::repository::Repository;
    use crate::statistics::services::{TrackerMetrics, get_metrics};

    #[tokio::test]
    async fn the_statistics_service_should_return_the_tracker_metrics() {
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());

        let repository = Arc::new(Repository::new());

        let tracker_metrics = get_metrics(in_memory_torrent_repository.clone(), repository.clone())
            .await
            .unwrap();

        assert_eq!(
            tracker_metrics,
            TrackerMetrics {
                torrents_metrics: AggregateActiveSwarmMetadata::default(),
                protocol_metrics: describe_metrics(),
            }
        );
    }
}
