use std::sync::Arc;

use torrust_tracker_metrics::label::{LabelSet, LabelValue};
use torrust_tracker_metrics::{label_name, metric_name};
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::DurationSinceUnixEpoch;

use crate::event::Event;
use crate::statistics::repository::Repository;
use crate::statistics::{
    SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL, SWARM_COORDINATION_REGISTRY_PEERS_COMPLETED_STATE_REVERTED_TOTAL,
    SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL, SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL,
    SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL, SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL,
    SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL, SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL,
    SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL,
};

#[allow(clippy::too_many_lines)]
pub async fn handle_event(event: Event, stats_repository: &Arc<Repository>, now: DurationSinceUnixEpoch) {
    match event {
        // Torrent events
        Event::TorrentAdded { info_hash, .. } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent added",);

            let _unused = stats_repository
                .increment_gauge(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL),
                    &LabelSet::default(),
                    now,
                )
                .await;

            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL),
                    &LabelSet::default(),
                    now,
                )
                .await;
        }
        Event::TorrentRemoved { info_hash } => {
            tracing::debug!(info_hash = ?info_hash, "Torrent removed",);

            let _unused = stats_repository
                .decrement_gauge(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL),
                    &LabelSet::default(),
                    now,
                )
                .await;

            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL),
                    &LabelSet::default(),
                    now,
                )
                .await;
        }

        // Peer events
        Event::PeerAdded { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer added", );

            let label_set = label_set_for_peer(&peer);

            let _unused = stats_repository
                .increment_gauge(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL),
                    &label_set,
                    now,
                )
                .await;

            let _unused = stats_repository
                .increment_counter(&metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL), &label_set, now)
                .await;
        }
        Event::PeerRemoved { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer removed", );

            let label_set = label_set_for_peer(&peer);

            let _unused = stats_repository
                .decrement_gauge(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL),
                    &label_set,
                    now,
                )
                .await;

            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL),
                    &label_set,
                    now,
                )
                .await;
        }
        Event::PeerUpdated {
            info_hash,
            old_peer,
            new_peer,
        } => {
            tracing::debug!(info_hash = ?info_hash, old_peer = ?old_peer, new_peer = ?new_peer, "Peer updated", );

            // If the peer's role has changed, we need to adjust the number of
            // connections
            if old_peer.role() != new_peer.role() {
                let _unused = stats_repository
                    .increment_gauge(
                        &metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL),
                        &label_set_for_peer(&new_peer),
                        now,
                    )
                    .await;

                let _unused = stats_repository
                    .decrement_gauge(
                        &metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL),
                        &label_set_for_peer(&old_peer),
                        now,
                    )
                    .await;
            }

            // If the peer reverted from a completed state to any other state,
            // we need to increment the counter for reverted completed.
            if old_peer.is_completed() && !new_peer.is_completed() {
                let _unused = stats_repository
                    .increment_counter(
                        &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_COMPLETED_STATE_REVERTED_TOTAL),
                        &LabelSet::default(),
                        now,
                    )
                    .await;
            }

            // Regardless of the role change, we still need to increment the
            // counter for updated peers.
            let label_set = label_set_for_peer(&new_peer);

            let _unused = stats_repository
                .increment_counter(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL),
                    &label_set,
                    now,
                )
                .await;
        }
        Event::PeerDownloadCompleted { info_hash, peer } => {
            tracing::debug!(info_hash = ?info_hash, peer = ?peer, "Peer download completed", );

            let _unused: Result<(), torrust_tracker_metrics::metric_collection::Error> = stats_repository
                .increment_counter(
                    &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL),
                    &label_set_for_peer(&peer),
                    now,
                )
                .await;
        }
    }
}

/// Returns the label set to be included in the metrics for the given peer.
pub(crate) fn label_set_for_peer(peer: &Peer) -> LabelSet {
    if peer.is_seeder() {
        (label_name!("peer_role"), LabelValue::new("seeder")).into()
    } else {
        (label_name!("peer_role"), LabelValue::new("leecher")).into()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use aquatic_udp_protocol::NumberOfBytes;
    use torrust_tracker_metrics::label::LabelSet;
    use torrust_tracker_metrics::metric::MetricName;
    use torrust_tracker_primitives::peer::{Peer, PeerRole};

    use crate::statistics::repository::Repository;
    use crate::tests::{leecher, seeder};

    fn make_peer(role: PeerRole) -> Peer {
        match role {
            PeerRole::Seeder => seeder(),
            PeerRole::Leecher => leecher(),
        }
    }

    // It returns a peer with the opposite role of the given peer.
    fn make_opposite_role_peer(peer: &Peer) -> Peer {
        let mut opposite_role_peer = *peer;

        match peer.role() {
            PeerRole::Seeder => {
                opposite_role_peer.left = NumberOfBytes::new(1);
            }
            PeerRole::Leecher => {
                opposite_role_peer.left = NumberOfBytes::new(0);
            }
        }

        opposite_role_peer
    }

    pub async fn expect_counter_metric_to_be(
        stats_repository: &Arc<Repository>,
        metric_name: &MetricName,
        label_set: &LabelSet,
        expected_value: u64,
    ) {
        let value = get_counter_metric(stats_repository, metric_name, label_set).await;
        assert_eq!(value.to_string(), expected_value.to_string());
    }

    async fn get_counter_metric(stats_repository: &Arc<Repository>, metric_name: &MetricName, label_set: &LabelSet) -> u64 {
        stats_repository
            .get_metrics()
            .await
            .metric_collection
            .get_counter_value(metric_name, label_set)
            .unwrap_or_else(|| panic!("Failed to get counter value for metric name '{metric_name}' and label set '{label_set}'"))
            .value()
    }

    async fn expect_gauge_metric_to_be(
        stats_repository: &Arc<Repository>,
        metric_name: &MetricName,
        label_set: &LabelSet,
        expected_value: f64,
    ) {
        let value = get_gauge_metric(stats_repository, metric_name, label_set).await;
        assert_eq!(value.to_string(), expected_value.to_string());
    }

    async fn get_gauge_metric(stats_repository: &Arc<Repository>, metric_name: &MetricName, label_set: &LabelSet) -> f64 {
        stats_repository
            .get_metrics()
            .await
            .metric_collection
            .get_gauge_value(metric_name, label_set)
            .unwrap_or_else(|| panic!("Failed to get gauge value for metric name '{metric_name}' and label set '{label_set}'"))
            .value()
    }

    mod for_torrent_metrics {

        use std::sync::Arc;

        use torrust_tracker_clock::clock::stopped::Stopped;
        use torrust_tracker_clock::clock::{self, Time};
        use torrust_tracker_metrics::label::LabelSet;
        use torrust_tracker_metrics::metric_name;

        use crate::event::Event;
        use crate::statistics::event::handler::handle_event;
        use crate::statistics::event::handler::tests::{expect_counter_metric_to_be, expect_gauge_metric_to_be};
        use crate::statistics::repository::Repository;
        use crate::statistics::{
            SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL, SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL,
            SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL,
        };
        use crate::tests::{sample_info_hash, sample_peer};
        use crate::CurrentClock;

        #[tokio::test]
        async fn it_should_increment_the_number_of_torrents_when_a_torrent_added_event_is_received() {
            clock::Stopped::local_set_to_unix_epoch();

            let stats_repository = Arc::new(Repository::new());

            handle_event(
                Event::TorrentAdded {
                    info_hash: sample_info_hash(),
                    announcement: sample_peer(),
                },
                &stats_repository,
                CurrentClock::now(),
            )
            .await;

            expect_gauge_metric_to_be(
                &stats_repository,
                &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL),
                &LabelSet::default(),
                1.0,
            )
            .await;
        }

        #[tokio::test]
        async fn it_should_decrement_the_number_of_torrents_when_a_torrent_removed_event_is_received() {
            clock::Stopped::local_set_to_unix_epoch();

            let stats_repository = Arc::new(Repository::new());
            let metric_name = metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_TOTAL);
            let label_set = LabelSet::default();

            // Increment the gauge first to simulate a torrent being added.
            stats_repository
                .increment_gauge(&metric_name, &label_set, CurrentClock::now())
                .await
                .unwrap();

            handle_event(
                Event::TorrentRemoved {
                    info_hash: sample_info_hash(),
                },
                &stats_repository,
                CurrentClock::now(),
            )
            .await;

            expect_gauge_metric_to_be(&stats_repository, &metric_name, &label_set, 0.0).await;
        }

        #[tokio::test]
        async fn it_should_increment_the_number_of_torrents_added_when_a_torrent_added_event_is_received() {
            clock::Stopped::local_set_to_unix_epoch();

            let stats_repository = Arc::new(Repository::new());

            handle_event(
                Event::TorrentAdded {
                    info_hash: sample_info_hash(),
                    announcement: sample_peer(),
                },
                &stats_repository,
                CurrentClock::now(),
            )
            .await;

            expect_counter_metric_to_be(
                &stats_repository,
                &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_ADDED_TOTAL),
                &LabelSet::default(),
                1,
            )
            .await;
        }

        #[tokio::test]
        async fn it_should_increment_the_number_of_torrents_removed_when_a_torrent_removed_event_is_received() {
            clock::Stopped::local_set_to_unix_epoch();

            let stats_repository = Arc::new(Repository::new());

            handle_event(
                Event::TorrentRemoved {
                    info_hash: sample_info_hash(),
                },
                &stats_repository,
                CurrentClock::now(),
            )
            .await;

            expect_counter_metric_to_be(
                &stats_repository,
                &metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_REMOVED_TOTAL),
                &LabelSet::default(),
                1,
            )
            .await;
        }
    }

    mod for_peer_metrics {
        use std::sync::Arc;

        use torrust_tracker_clock::clock::stopped::Stopped;
        use torrust_tracker_clock::clock::{self, Time};
        use torrust_tracker_metrics::metric_name;

        use crate::event::Event;
        use crate::statistics::event::handler::tests::expect_counter_metric_to_be;
        use crate::statistics::event::handler::{handle_event, label_set_for_peer};
        use crate::statistics::repository::Repository;
        use crate::statistics::{
            SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL, SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL,
            SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL,
        };
        use crate::tests::{sample_info_hash, sample_peer};
        use crate::CurrentClock;

        mod peer_connections_total {

            use std::sync::Arc;

            use rstest::rstest;
            use torrust_tracker_clock::clock::stopped::Stopped;
            use torrust_tracker_clock::clock::{self, Time};
            use torrust_tracker_metrics::label::LabelValue;
            use torrust_tracker_metrics::{label_name, metric_name};
            use torrust_tracker_primitives::peer::PeerRole;

            use crate::event::Event;
            use crate::statistics::event::handler::handle_event;
            use crate::statistics::event::handler::tests::{
                expect_gauge_metric_to_be, get_gauge_metric, make_opposite_role_peer, make_peer,
            };
            use crate::statistics::repository::Repository;
            use crate::statistics::SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL;
            use crate::tests::sample_info_hash;
            use crate::CurrentClock;

            #[rstest]
            #[case("seeder")]
            #[case("leecher")]
            #[tokio::test]
            async fn it_should_increment_the_number_of_peer_connections_when_a_peer_added_event_is_received(
                #[case] role: PeerRole,
            ) {
                clock::Stopped::local_set_to_unix_epoch();

                let peer = make_peer(role);

                let stats_repository = Arc::new(Repository::new());
                let metric_name = metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL);
                let label_set = (label_name!("peer_role"), LabelValue::new(&role.to_string())).into();

                handle_event(
                    Event::PeerAdded {
                        info_hash: sample_info_hash(),
                        peer,
                    },
                    &stats_repository,
                    CurrentClock::now(),
                )
                .await;

                expect_gauge_metric_to_be(&stats_repository, &metric_name, &label_set, 1.0).await;
            }

            #[rstest]
            #[case("seeder")]
            #[case("leecher")]
            #[tokio::test]
            async fn it_should_decrement_the_number_of_peer_connections_when_a_peer_removed_event_is_received(
                #[case] role: PeerRole,
            ) {
                clock::Stopped::local_set_to_unix_epoch();

                let peer = make_peer(role);

                let stats_repository = Arc::new(Repository::new());

                let metric_name = metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL);
                let label_set = (label_name!("peer_role"), LabelValue::new(&role.to_string())).into();

                // Increment the gauge first to simulate a peer being added.
                stats_repository
                    .increment_gauge(&metric_name, &label_set, CurrentClock::now())
                    .await
                    .unwrap();

                handle_event(
                    Event::PeerRemoved {
                        info_hash: sample_info_hash(),
                        peer,
                    },
                    &stats_repository,
                    CurrentClock::now(),
                )
                .await;

                expect_gauge_metric_to_be(&stats_repository, &metric_name, &label_set, 0.0).await;
            }

            #[rstest]
            #[case("seeder")]
            #[case("leecher")]
            #[tokio::test]
            async fn it_should_adjust_the_number_of_seeders_and_leechers_when_a_peer_updated_event_is_received_and_the_peer_changed_its_role(
                #[case] old_role: PeerRole,
            ) {
                clock::Stopped::local_set_to_unix_epoch();

                let stats_repository = Arc::new(Repository::new());

                let old_peer = make_peer(old_role);
                let new_peer = make_opposite_role_peer(&old_peer);

                let metric_name = metric_name!(SWARM_COORDINATION_REGISTRY_PEER_CONNECTIONS_TOTAL);
                let old_role_label_set = (label_name!("peer_role"), LabelValue::new(&old_peer.role().to_string())).into();
                let new_role_label_set = (label_name!("peer_role"), LabelValue::new(&new_peer.role().to_string())).into();

                // Increment the gauge first by simulating a peer was added.
                handle_event(
                    Event::PeerAdded {
                        info_hash: sample_info_hash(),
                        peer: old_peer,
                    },
                    &stats_repository,
                    CurrentClock::now(),
                )
                .await;

                let old_role_total = get_gauge_metric(&stats_repository, &metric_name, &old_role_label_set).await;
                let new_role_total = 0.0;

                // The peer's role has changed, so we need to increment the new
                // role and decrement the old one.
                handle_event(
                    Event::PeerUpdated {
                        info_hash: sample_info_hash(),
                        old_peer,
                        new_peer,
                    },
                    &stats_repository,
                    CurrentClock::now(),
                )
                .await;

                // The peer's role has changed, so the new role has incremented.
                expect_gauge_metric_to_be(&stats_repository, &metric_name, &new_role_label_set, new_role_total + 1.0).await;

                // And the old role has decremented.
                expect_gauge_metric_to_be(&stats_repository, &metric_name, &old_role_label_set, old_role_total - 1.0).await;
            }
        }

        #[tokio::test]
        async fn it_should_increment_the_number_of_peers_added_when_a_peer_added_event_is_received() {
            clock::Stopped::local_set_to_unix_epoch();

            let stats_repository = Arc::new(Repository::new());

            let peer = sample_peer();

            handle_event(
                Event::PeerAdded {
                    info_hash: sample_info_hash(),
                    peer,
                },
                &stats_repository,
                CurrentClock::now(),
            )
            .await;

            expect_counter_metric_to_be(
                &stats_repository,
                &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_ADDED_TOTAL),
                &label_set_for_peer(&peer),
                1,
            )
            .await;
        }

        #[tokio::test]
        async fn it_should_increment_the_number_of_peers_removed_when_a_peer_removed_event_is_received() {
            clock::Stopped::local_set_to_unix_epoch();

            let stats_repository = Arc::new(Repository::new());

            let peer = sample_peer();

            handle_event(
                Event::PeerRemoved {
                    info_hash: sample_info_hash(),
                    peer,
                },
                &stats_repository,
                CurrentClock::now(),
            )
            .await;

            expect_counter_metric_to_be(
                &stats_repository,
                &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_REMOVED_TOTAL),
                &label_set_for_peer(&peer),
                1,
            )
            .await;
        }

        #[tokio::test]
        async fn it_should_increment_the_number_of_peers_updated_when_a_peer_updated_event_is_received() {
            clock::Stopped::local_set_to_unix_epoch();

            let stats_repository = Arc::new(Repository::new());

            let new_peer = sample_peer();

            handle_event(
                Event::PeerUpdated {
                    info_hash: sample_info_hash(),
                    old_peer: sample_peer(),
                    new_peer,
                },
                &stats_repository,
                CurrentClock::now(),
            )
            .await;

            expect_counter_metric_to_be(
                &stats_repository,
                &metric_name!(SWARM_COORDINATION_REGISTRY_PEERS_UPDATED_TOTAL),
                &label_set_for_peer(&new_peer),
                1,
            )
            .await;
        }

        mod torrent_downloads_total {

            use std::sync::Arc;

            use rstest::rstest;
            use torrust_tracker_clock::clock::stopped::Stopped;
            use torrust_tracker_clock::clock::{self, Time};
            use torrust_tracker_metrics::label::LabelValue;
            use torrust_tracker_metrics::{label_name, metric_name};
            use torrust_tracker_primitives::peer::PeerRole;

            use crate::event::Event;
            use crate::statistics::event::handler::handle_event;
            use crate::statistics::event::handler::tests::{expect_counter_metric_to_be, make_peer};
            use crate::statistics::repository::Repository;
            use crate::statistics::SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL;
            use crate::tests::sample_info_hash;
            use crate::CurrentClock;

            #[rstest]
            #[case("seeder")]
            #[case("leecher")]
            #[tokio::test]
            async fn it_should_increment_the_number_of_downloads_when_a_peer_downloaded_event_is_received(
                #[case] role: PeerRole,
            ) {
                clock::Stopped::local_set_to_unix_epoch();

                let peer = make_peer(role);

                let stats_repository = Arc::new(Repository::new());
                let metric_name = metric_name!(SWARM_COORDINATION_REGISTRY_TORRENTS_DOWNLOADS_TOTAL);
                let label_set = (label_name!("peer_role"), LabelValue::new(&role.to_string())).into();

                handle_event(
                    Event::PeerDownloadCompleted {
                        info_hash: sample_info_hash(),
                        peer,
                    },
                    &stats_repository,
                    CurrentClock::now(),
                )
                .await;

                expect_counter_metric_to_be(&stats_repository, &metric_name, &label_set, 1).await;
            }
        }
    }
}
