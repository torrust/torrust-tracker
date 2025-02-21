use std::sync::Arc;

use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use bittorrent_tracker_core::authentication::handler::KeysHandler;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_tracker_core::torrent::repository::in_memory::InMemoryTorrentRepository;
use bittorrent_tracker_core::whitelist::manager::WhitelistManager;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use bittorrent_udp_tracker_core::services::banning::BanService;
use bittorrent_udp_tracker_core::{self};
use tokio::sync::RwLock;
use torrust_tracker_configuration::{Core, HttpApi, HttpTracker, UdpTracker};

pub struct TrackerHttpApiCoreContainer {
    // todo: replace with TrackerCoreContainer
    pub core_config: Arc<Core>,
    pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    pub keys_handler: Arc<KeysHandler>,
    pub whitelist_manager: Arc<WhitelistManager>,

    // todo: replace with HttpTrackerCoreContainer
    pub http_stats_repository: Arc<bittorrent_http_tracker_core::statistics::repository::Repository>,

    // todo: replace with UdpTrackerCoreContainer
    pub ban_service: Arc<RwLock<BanService>>,
    pub udp_stats_repository: Arc<bittorrent_udp_tracker_core::statistics::repository::Repository>,

    pub http_api_config: Arc<HttpApi>,
}

impl TrackerHttpApiCoreContainer {
    #[must_use]
    pub fn initialize(
        core_config: &Arc<Core>,
        http_tracker_config: &Arc<HttpTracker>,
        udp_tracker_config: &Arc<UdpTracker>,
        http_api_config: &Arc<HttpApi>,
    ) -> Arc<TrackerHttpApiCoreContainer> {
        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(core_config));
        let http_tracker_core_container = HttpTrackerCoreContainer::initialize_from(&tracker_core_container, http_tracker_config);
        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from(&tracker_core_container, udp_tracker_config);

        Self::initialize_from(
            &tracker_core_container,
            &http_tracker_core_container,
            &udp_tracker_core_container,
            http_api_config,
        )
    }

    #[must_use]
    pub fn initialize_from(
        tracker_core_container: &Arc<TrackerCoreContainer>,
        http_tracker_core_container: &Arc<HttpTrackerCoreContainer>,
        udp_tracker_core_container: &Arc<UdpTrackerCoreContainer>,
        http_api_config: &Arc<HttpApi>,
    ) -> Arc<TrackerHttpApiCoreContainer> {
        Arc::new(TrackerHttpApiCoreContainer {
            core_config: tracker_core_container.core_config.clone(),
            in_memory_torrent_repository: tracker_core_container.in_memory_torrent_repository.clone(),
            keys_handler: tracker_core_container.keys_handler.clone(),
            whitelist_manager: tracker_core_container.whitelist_manager.clone(),

            http_stats_repository: http_tracker_core_container.http_stats_repository.clone(),

            ban_service: udp_tracker_core_container.ban_service.clone(),
            udp_stats_repository: udp_tracker_core_container.udp_stats_repository.clone(),

            http_api_config: http_api_config.clone(),
        })
    }
}
