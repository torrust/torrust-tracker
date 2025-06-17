use std::net::IpAddr;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceEvent;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::PeersWanted;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_tracker_core::statistics::persisted::load_persisted_metrics;
use tokio::task::yield_now;
use tokio_util::sync::CancellationToken;
use torrust_tracker_configuration::Core;
use torrust_tracker_metrics::label::LabelSet;
use torrust_tracker_metrics::metric::MetricName;
use torrust_tracker_primitives::core::{AnnounceData, ScrapeData};
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_primitives::DurationSinceUnixEpoch;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

pub struct TestEnv {
    pub swarm_coordination_registry_container: Arc<SwarmCoordinationRegistryContainer>,
    pub tracker_core_container: Arc<TrackerCoreContainer>,
}

impl TestEnv {
    #[must_use]
    pub async fn started(core_config: Core) -> Self {
        let test_env = TestEnv::new(core_config);
        test_env.start().await;
        test_env
    }

    #[must_use]
    pub fn new(core_config: Core) -> Self {
        let core_config = Arc::new(core_config);

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            &core_config,
            &swarm_coordination_registry_container,
        ));

        Self {
            swarm_coordination_registry_container,
            tracker_core_container,
        }
    }

    pub async fn start(&self) {
        let now = DurationSinceUnixEpoch::from_secs(0);
        self.load_persisted_metrics(now).await;
        self.run_jobs().await;
    }

    async fn load_persisted_metrics(&self, now: DurationSinceUnixEpoch) {
        load_persisted_metrics(
            &self.tracker_core_container.stats_repository,
            &self.tracker_core_container.db_downloads_metric_repository,
            now,
        )
        .await
        .unwrap();
    }

    async fn run_jobs(&self) {
        let mut jobs = vec![];
        let cancellation_token = CancellationToken::new();

        let job = torrust_tracker_swarm_coordination_registry::statistics::event::listener::run_event_listener(
            self.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token.clone(),
            &self.swarm_coordination_registry_container.stats_repository,
        );

        jobs.push(job);

        let job = bittorrent_tracker_core::statistics::event::listener::run_event_listener(
            self.swarm_coordination_registry_container.event_bus.receiver(),
            cancellation_token.clone(),
            &self.tracker_core_container.stats_repository,
            &self.tracker_core_container.db_downloads_metric_repository,
            self.tracker_core_container
                .core_config
                .tracker_policy
                .persistent_torrent_completed_stat,
        );
        jobs.push(job);

        // Give the event listeners some time to start
        // todo: they should notify when they are ready
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    pub async fn announce_peer_started(
        &mut self,
        mut peer: Peer,
        remote_client_ip: &IpAddr,
        info_hash: &InfoHash,
    ) -> AnnounceData {
        peer.event = AnnounceEvent::Started;

        let announce_data = self
            .tracker_core_container
            .announce_handler
            .handle_announcement(info_hash, &mut peer, remote_client_ip, &PeersWanted::AsManyAsPossible)
            .await
            .unwrap();

        // Give time to the event listeners to process the event
        yield_now().await;

        announce_data
    }

    pub async fn announce_peer_completed(
        &mut self,
        mut peer: Peer,
        remote_client_ip: &IpAddr,
        info_hash: &InfoHash,
    ) -> AnnounceData {
        peer.event = AnnounceEvent::Completed;

        let announce_data = self
            .tracker_core_container
            .announce_handler
            .handle_announcement(info_hash, &mut peer, remote_client_ip, &PeersWanted::AsManyAsPossible)
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

    pub async fn increase_number_of_downloads(&mut self, peer: Peer, remote_client_ip: &IpAddr, info_hash: &InfoHash) {
        let _announce_data = self.announce_peer_started(peer, remote_client_ip, info_hash).await;
        let announce_data = self.announce_peer_completed(peer, remote_client_ip, info_hash).await;

        assert_eq!(announce_data.stats.downloads(), 1);
    }

    pub async fn get_swarm_metadata(&self, info_hash: &InfoHash) -> Option<SwarmMetadata> {
        self.swarm_coordination_registry_container
            .swarms
            .get_swarm_metadata(info_hash)
            .await
            .unwrap()
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
            .get_metrics()
            .await
            .metric_collection
            .get_counter_value(&MetricName::new(metric_name), &LabelSet::default())
            .unwrap()
            .value()
    }
}
