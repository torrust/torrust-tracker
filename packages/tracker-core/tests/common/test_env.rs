use std::net::IpAddr;
use std::sync::Arc;

use aquatic_udp_protocol::AnnounceEvent;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::announce_handler::PeersWanted;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use tokio::task::yield_now;
use torrust_tracker_configuration::Core;
use torrust_tracker_primitives::core::{AnnounceData, ScrapeData};
use torrust_tracker_primitives::peer::Peer;
use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;
use torrust_tracker_torrent_repository::container::TorrentRepositoryContainer;

pub struct TestEnv {
    pub torrent_repository_container: Arc<TorrentRepositoryContainer>,
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

        let torrent_repository_container = Arc::new(TorrentRepositoryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            &core_config,
            &torrent_repository_container,
        ));

        Self {
            torrent_repository_container,
            tracker_core_container,
        }
    }

    pub async fn start(&self) {
        let mut jobs = vec![];

        let job = torrust_tracker_torrent_repository::statistics::event::listener::run_event_listener(
            self.torrent_repository_container.event_bus.receiver(),
            &self.torrent_repository_container.stats_repository,
        );

        jobs.push(job);

        let job = bittorrent_tracker_core::statistics::event::listener::run_event_listener(
            self.torrent_repository_container.event_bus.receiver(),
            &self.tracker_core_container.stats_repository,
            &self.tracker_core_container.db_torrent_repository,
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
        self.torrent_repository_container
            .swarms
            .get_swarm_metadata(info_hash)
            .await
            .unwrap()
    }

    pub async fn remove_swarm(&self, info_hash: &InfoHash) {
        self.torrent_repository_container.swarms.remove(info_hash).await.unwrap();
    }
}
