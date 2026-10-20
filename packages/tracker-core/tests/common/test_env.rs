use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::{JoinHandle, yield_now};
use tokio::time::{Instant, timeout_at};
use tokio_util::sync::{CancellationToken, DropGuard};
use torrust_clock::DurationSinceUnixEpoch;
use torrust_info_hash::InfoHash;
use torrust_metrics::label::LabelSet;
use torrust_metrics::metric::MetricName;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_core::announce_handler::PeersWanted;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_core::statistics::persisted::load_persisted_metrics;
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::{AnnounceData, AnnounceEvent, ScrapeData};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

/// Time `TestEnv::stop` allows, in total, for every listener to finish after cancellation.
const LISTENERS_STOP_TIMEOUT: Duration = Duration::from_secs(5);

pub struct TestEnv {
    pub swarm_coordination_registry_container: Arc<SwarmCoordinationRegistryContainer>,
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    listeners: Listeners,
}

/// The listener tasks a [`TestEnv`] runs and the token that stops them.
///
/// Dropping a `CancellationToken` does not cancel it, so the token is held through a
/// `DropGuard`: an environment dropped without `stop()` still cancels its listeners.
struct Listeners {
    cancellation_token: CancellationToken,
    _cancel_on_drop: DropGuard,
    handles: Vec<(&'static str, JoinHandle<()>)>,
}

impl Listeners {
    fn new() -> Self {
        let cancellation_token = CancellationToken::new();

        Self {
            _cancel_on_drop: cancellation_token.clone().drop_guard(),
            cancellation_token,
            handles: Vec::new(),
        }
    }

    fn token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    fn add(&mut self, name: &'static str, handle: JoinHandle<()>) {
        self.handles.push((name, handle));
    }

    /// Cancels every listener, then joins all of them before the shared deadline.
    ///
    /// Every handle is joined before any failure is reported, so one listener that
    /// panics or hangs does not leave the others running.
    async fn stop(&mut self) -> Result<(), String> {
        self.cancellation_token.cancel();

        let deadline = Instant::now() + LISTENERS_STOP_TIMEOUT;
        let mut failures = Vec::new();

        for (name, mut handle) in std::mem::take(&mut self.handles) {
            match timeout_at(deadline, &mut handle).await {
                Ok(Ok(())) => {}
                Ok(Err(error)) => failures.push(format!("{name} listener failed: {error}")),
                Err(_elapsed) => {
                    handle.abort();
                    failures.push(format!(
                        "{name} listener did not stop within {LISTENERS_STOP_TIMEOUT:?} of cancellation"
                    ));
                }
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures.join("; "))
        }
    }
}

impl TestEnv {
    #[must_use]
    pub async fn started(core_config: Core) -> Self {
        let mut test_env = Self::new(core_config).await;
        test_env.start().await;
        test_env
    }

    #[must_use]
    pub async fn new(core_config: Core) -> Self {
        let core_config = Arc::new(core_config);

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                &core_config,
                &swarm_coordination_registry_container,
                core_config.database.as_ref(),
            )
            .await
            .expect("tracker core test environment requires valid composition"),
        );

        Self {
            swarm_coordination_registry_container,
            tracker_core_container,
            listeners: Listeners::new(),
        }
    }

    pub async fn start(&mut self) {
        let now = DurationSinceUnixEpoch::from_secs(0);
        if self
            .tracker_core_container
            .core_config
            .tracker_policy
            .persistent_torrent_completed_stat
        {
            self.load_persisted_metrics(now).await;
        }
        self.run_jobs().await;
    }

    async fn load_persisted_metrics(&self, now: DurationSinceUnixEpoch) {
        load_persisted_metrics(
            &self.tracker_core_container.stats_repository,
            &self
                .tracker_core_container
                .persistence
                .as_ref()
                .expect("tracker core test environment requires persistence")
                .db_downloads_metric_repository,
            now,
        )
        .await
        .unwrap();
    }

    async fn run_jobs(&mut self) {
        let swarm_statistics_listener =
            torrust_tracker_swarm_coordination_registry::statistics::event::listener::run_event_listener(
                self.swarm_coordination_registry_container.event_bus.receiver(),
                self.listeners.token(),
                &self.swarm_coordination_registry_container.stats_repository,
            );
        self.listeners.add("swarm statistics", swarm_statistics_listener);

        let in_memory_statistics_listener = torrust_tracker_core::statistics::event::listener::run_in_memory_event_listener(
            self.swarm_coordination_registry_container.event_bus.receiver(),
            self.listeners.token(),
            &self.tracker_core_container.stats_repository,
        );
        self.listeners.add("in-memory statistics", in_memory_statistics_listener);

        if self
            .tracker_core_container
            .core_config
            .tracker_policy
            .persistent_torrent_completed_stat
        {
            let persistent_statistics_listener =
                torrust_tracker_core::statistics::event::listener::run_persistent_completed_statistics_event_listener(
                    self.swarm_coordination_registry_container.event_bus.receiver(),
                    self.listeners.token(),
                    &self
                        .tracker_core_container
                        .persistence
                        .as_ref()
                        .expect("tracker core test environment requires persistence")
                        .db_downloads_metric_repository,
                    &self.tracker_core_container.stats_repository,
                );
            self.listeners
                .add("persistent completed statistics", persistent_statistics_listener);
        }

        // Give the event listeners some time to start
        // todo: they should notify when they are ready
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    /// Cancels the event listeners and waits for all of them to finish.
    ///
    /// Dropping the environment without calling this still cancels them, but cannot
    /// wait for them.
    ///
    /// # Errors
    ///
    /// Returns every listener that panicked or did not finish within the stop timeout.
    pub async fn stop(&mut self) -> Result<(), String> {
        self.listeners.stop().await
    }

    pub async fn announce_peer_started(&self, mut peer: Peer, remote_client_ip: &IpAddr, info_hash: &InfoHash) -> AnnounceData {
        peer.event = AnnounceEvent::Started;

        let announce_data = self
            .tracker_core_container
            .announce_handler
            .handle_announcement(info_hash, &mut peer, remote_client_ip, None, &PeersWanted::AsManyAsPossible)
            .await
            .unwrap();

        // Give time to the event listeners to process the event
        yield_now().await;

        announce_data
    }

    pub async fn announce_peer_completed(&self, mut peer: Peer, remote_client_ip: &IpAddr, info_hash: &InfoHash) -> AnnounceData {
        peer.event = AnnounceEvent::Completed;

        let announce_data = self
            .tracker_core_container
            .announce_handler
            .handle_announcement(info_hash, &mut peer, remote_client_ip, None, &PeersWanted::AsManyAsPossible)
            .await
            .unwrap();

        // Give time to the event listeners to process the event
        yield_now().await;

        announce_data
    }

    pub async fn scrape(&self, info_hash: &InfoHash) -> ScrapeData {
        self.tracker_core_container
            .scrape_handler
            .handle_scrape(&vec![*info_hash])
            .await
            .unwrap()
    }

    pub async fn increase_number_of_downloads(&self, peer: Peer, remote_client_ip: &IpAddr, info_hash: &InfoHash) {
        let _announce_data = self.announce_peer_started(peer, remote_client_ip, info_hash).await;
        let announce_data = self.announce_peer_completed(peer, remote_client_ip, info_hash).await;

        assert_eq!(announce_data.stats.downloads(), 1);
    }

    pub async fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata> {
        self.swarm_coordination_registry_container
            .swarms
            .get_swarm_metadata(info_hash)
            .await
    }

    /// Waits until the global download count in the database reaches `expected`, with a 5-second
    /// timeout. Used in tests to avoid a race between the event listener persisting to the
    /// database and the creation of a new `TestEnv` that reads from that same database.
    pub async fn wait_for_global_downloads_persisted(&self, expected: u64) {
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                if let Ok(Some(downloads)) = self
                    .tracker_core_container
                    .persistence
                    .as_ref()
                    .expect("tracker core test environment requires persistence")
                    .database_stores
                    .torrent_metrics_store
                    .load_global_downloads()
                    .await
                    && u64::from(downloads) >= expected
                {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
        .await
        .expect("Timed out waiting for global downloads to be persisted to the database");
    }

    /// Writes a torrent's downloads count straight into the database, as a
    /// previous tracker run would have left it.
    pub async fn persist_torrent_downloads(&self, info_hash: &InfoHash, downloads: u32) {
        self.tracker_core_container
            .persistence
            .as_ref()
            .expect("tracker core test environment requires persistence")
            .database_stores
            .torrent_metrics_store
            .save_torrent_downloads(info_hash, downloads)
            .await
            .unwrap();
    }

    pub async fn remove_swarm(&self, info_hash: &InfoHash) {
        self.swarm_coordination_registry_container
            .swarms
            .remove(info_hash)
            .await
            .unwrap();
    }

    pub async fn get_counter_value(&self, metric_name: &str) -> u64 {
        self.tracker_core_container
            .stats_repository
            .read_metrics(|metrics| {
                metrics
                    .metric_collection()
                    .get_counter_value(&MetricName::new(metric_name), &LabelSet::default())
                    .unwrap()
                    .value()
            })
            .await
    }
}
