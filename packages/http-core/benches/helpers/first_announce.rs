use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use tokio_util::sync::CancellationToken;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric_collection::aggregate::sum::SumCounter;
use torrust_metrics::metric_name;
use torrust_net_primitives::service_binding::ServiceBinding;
use torrust_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use torrust_tracker_events::drain::DEFAULT_DRAIN_TIMEOUT;
use torrust_tracker_http_core::services::announce::AnnounceService;
use torrust_tracker_http_protocol::v1::requests::announce::Announce;
use torrust_tracker_http_protocol::v1::services::peer_ip_resolver::ClientIpSources;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::{AnnounceData, AnnounceEvent};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_swarm_coordination_registry::statistics::event::listener::run_event_listener;
use torrust_tracker_swarm_coordination_registry::statistics::repository::Repository;
use torrust_tracker_test_helpers::configuration;

use crate::helpers::util::{
    announce_service, initialize_core_tracker_services_with_torrent_repository, sample_announce_request_for_peer,
    sample_info_hash, sample_peer, sample_server_service_binding,
};

const PEERS_ADDED_TOTAL: &str = "swarm_coordination_registry_peers_added_total";
const PEERS_UPDATED_TOTAL: &str = "swarm_coordination_registry_peers_updated_total";
const PEERS_REMOVED_TOTAL: &str = "swarm_coordination_registry_peers_removed_total";

/// Capacity of the swarm event bus (`CHANNEL_CAPACITY` in `packages/events/src/broadcaster.rs`, which
/// is private). A listener that falls further behind skips events and never counts them.
const SWARM_EVENT_CHANNEL_CAPACITY: u64 = 65_536;

/// The torrent-added event the registry sends when the first peer of the torrent arrives.
const TORRENT_ADDED_EVENTS: u64 = 1;

const LISTENER_WAIT_DEADLINE: Duration = Duration::from_secs(10);
const LISTENER_POLL_INTERVAL: Duration = Duration::from_millis(10);
const LISTENER_IDLE_POLLS: u32 = 20;

/// Peer events counted by the swarm statistics listener.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PeerEventCounts {
    added: u64,
    updated: u64,
    removed: u64,
}

impl PeerEventCounts {
    const fn total(self) -> u64 {
        self.added + self.updated + self.removed
    }
}

/// An announce service over a swarm of exactly `swarm_size` other peers, and the announces of one
/// new peer into it.
///
/// [`Self::first_announce`] announces `started` for a peer that is not in the swarm: it inserts the
/// peer and returns up to `max_peers_per_announce` of the other peers. [`Self::depart`] announces
/// `stopped` for the same peer, which puts the swarm back to `swarm_size` peers. A benchmark times
/// only the first announce, so every timed announce is an insert into a swarm of the same size,
/// however many iterations Criterion runs.
///
/// Unlike [`super::announce::AnnounceBenchmarkContext`], the swarm registry sends its events to a
/// statistics listener running on the benchmark's Tokio runtime, as in production. The listener's
/// metrics let [`Self::assert_swarm_unchanged_by_run`] check that every announce was an insert or a
/// removal and never an update.
pub struct FirstAnnounceBenchmarkContext {
    announce_service: AnnounceService,
    started_request: Announce,
    stopped_request: Announce,
    client_ip_sources: ClientIpSources,
    server_service_binding: ServiceBinding,
    in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    swarm_stats_repository: Arc<Repository>,
    swarm_size: usize,
    first_announces: AtomicU64,
    departures: AtomicU64,
}

impl FirstAnnounceBenchmarkContext {
    /// Must be awaited inside the Tokio runtime that runs the benchmark, because it spawns the
    /// swarm statistics listener on it.
    ///
    /// Before returning, it makes one first announce and one departure outside any measured time,
    /// and checks that the first announce returned `min(swarm_size, max_peers_per_announce)` peers.
    ///
    /// # Panics
    ///
    /// Panics if that announce returns a different number of peers, or if an announce is rejected.
    pub async fn new(swarm_size: usize) -> Self {
        let config = configuration::ephemeral_public();

        let swarm_container = SwarmCoordinationRegistryContainer::initialize(config.core.tracker_usage_statistics.into());

        // A broadcast receiver sees only the events sent after it subscribes, so the listener
        // subscribes before the swarm is pre-filled.
        let _listener = run_event_listener(
            swarm_container.event_bus.receiver(),
            CancellationToken::new(),
            &swarm_container.stats_repository,
            DEFAULT_DRAIN_TIMEOUT,
        );

        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::new(swarm_container.swarms.clone()));

        let (core_tracker_services, core_http_tracker_services) =
            initialize_core_tracker_services_with_torrent_repository(&config, in_memory_torrent_repository.clone()).await;

        let max_peers_per_announce = core_tracker_services.core_config.announce_policy.max_peers_per_announce;

        let info_hash = sample_info_hash();
        for index in 0..swarm_size {
            in_memory_torrent_repository
                .handle_announcement(&info_hash, &other_peer(index), None)
                .await;
        }

        let new_peer = sample_peer();
        let (started_request, client_ip_sources) = sample_announce_request_for_peer(new_peer);
        let (stopped_request, _) = sample_announce_request_for_peer(Peer {
            event: AnnounceEvent::Stopped,
            ..new_peer
        });

        let context = Self {
            announce_service: announce_service(core_tracker_services, core_http_tracker_services),
            started_request,
            stopped_request,
            client_ip_sources,
            server_service_binding: sample_server_service_binding(),
            in_memory_torrent_repository,
            swarm_stats_repository: swarm_container.stats_repository.clone(),
            swarm_size,
            first_announces: AtomicU64::new(0),
            departures: AtomicU64::new(0),
        };

        let announce_data = context.first_announce().await;
        assert_eq!(
            announce_data.peers.len(),
            swarm_size.min(max_peers_per_announce),
            "the first announce into a swarm of {swarm_size} peers should return the other peers, up to the announce limit"
        );
        context.depart().await;

        context
    }

    /// Announces `started` for a peer that is not in the swarm.
    ///
    /// # Panics
    ///
    /// Panics if the announce is rejected, which would mean the benchmark measures an error path
    /// instead of an announce.
    pub async fn first_announce(&self) -> AnnounceData {
        self.first_announces.fetch_add(1, Ordering::Relaxed);

        self.announce(&self.started_request).await
    }

    /// Announces `stopped` for the peer of [`Self::first_announce`], which removes it from the
    /// swarm.
    ///
    /// # Panics
    ///
    /// Panics if the announce is rejected.
    pub async fn depart(&self) {
        self.departures.fetch_add(1, Ordering::Relaxed);

        self.announce(&self.stopped_request).await;
    }

    /// Checks, outside any measured time, that the swarm holds exactly the pre-filled peers and
    /// that the swarm statistics listener counted only inserts and removals.
    ///
    /// The exact counts are asserted only when the context sent no more swarm events than the event
    /// bus can hold, so that no event can have been dropped. That always holds under
    /// `cargo test --benches`, which runs one iteration. After a longer run, dropped events can only
    /// lower a count, so it asserts that no update was counted and that no count is above its
    /// expected value.
    ///
    /// # Panics
    ///
    /// Panics if the swarm size or the counted events are not as expected.
    pub async fn assert_swarm_unchanged_by_run(&self, benchmark_id: &str) {
        let swarm_size = self.swarm_size;

        let peers_in_swarm = self.in_memory_torrent_repository.count_peers().await;
        assert_eq!(
            peers_in_swarm, swarm_size,
            "benchmark `{benchmark_id}`: the swarm should have exactly {swarm_size} peers after the run, but it has {peers_in_swarm}; \
             every first announce should be followed by the departure of its peer"
        );

        let first_announces = self.first_announces.load(Ordering::Relaxed);
        let departures = self.departures.load(Ordering::Relaxed);
        let expected = PeerEventCounts {
            added: u64::try_from(swarm_size).expect("the swarm size should fit in u64") + first_announces,
            updated: 0,
            removed: departures,
        };
        let swarm_events_sent = TORRENT_ADDED_EVENTS + expected.added + expected.removed;
        let no_event_can_have_been_dropped = swarm_events_sent <= SWARM_EVENT_CHANNEL_CAPACITY;

        let counted = self.wait_for_listener(expected, no_event_can_have_been_dropped).await;

        let context_message = format!(
            "benchmark `{benchmark_id}` (swarm of {swarm_size} peers, {first_announces} first announces, {departures} departures)"
        );

        if no_event_can_have_been_dropped {
            assert_eq!(
                counted, expected,
                "{context_message}: every announce should be an insert or a removal, never an update"
            );
        } else {
            assert_eq!(
                counted.updated, 0,
                "{context_message}: no announce should update an existing peer; counted {counted:?}"
            );
            assert!(
                counted.added <= expected.added && counted.removed <= expected.removed,
                "{context_message}: no count should exceed its expected value {expected:?}; counted {counted:?}"
            );
        }
    }

    async fn announce(&self, request: &Announce) -> AnnounceData {
        self.announce_service
            .handle_announce(request, &self.client_ip_sources, &self.server_service_binding, None)
            .await
            .expect("the benchmark announce should be accepted")
    }

    /// Waits for the asynchronous listener to count the events already sent.
    ///
    /// It returns once the listener has counted as many events as expected, or after
    /// `LISTENER_WAIT_DEADLINE`. When events may have been dropped the expected total can never be
    /// reached, so it also returns once the counts have not moved for `LISTENER_IDLE_POLLS` polls.
    async fn wait_for_listener(&self, expected: PeerEventCounts, exact: bool) -> PeerEventCounts {
        let deadline = Instant::now() + LISTENER_WAIT_DEADLINE;
        let mut counted = self.counted_peer_events().await;
        let mut idle_polls = 0;

        while counted.total() < expected.total() && Instant::now() < deadline && (exact || idle_polls < LISTENER_IDLE_POLLS) {
            tokio::time::sleep(LISTENER_POLL_INTERVAL).await;

            let latest = self.counted_peer_events().await;
            idle_polls = if latest == counted { idle_polls + 1 } else { 0 };
            counted = latest;
        }

        counted
    }

    async fn counted_peer_events(&self) -> PeerEventCounts {
        self.swarm_stats_repository
            .read_metrics(|metrics| {
                let collection = metrics.metric_collection();
                let count = |name| {
                    collection
                        .sum_counter(&name, &LabelSet::empty())
                        .expect("the swarm statistics should describe the peer counters")
                };

                PeerEventCounts {
                    added: count(metric_name!(PEERS_ADDED_TOTAL)),
                    updated: count(metric_name!(PEERS_UPDATED_TOTAL)),
                    removed: count(metric_name!(PEERS_REMOVED_TOTAL)),
                }
            })
            .await
    }
}

/// A seeder other than the one the benchmark announces, with an address of its own.
fn other_peer(index: usize) -> Peer {
    let index = u32::try_from(index).expect("the swarm size should fit in u32");

    Peer {
        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::from(0x0A00_0000 + index)), 6881),
        ..sample_peer()
    }
}
