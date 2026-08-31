//! Route initialization for the v1 API.
use std::sync::Arc;

use axum::Router;
use torrust_tracker_rest_api_application::v1::use_cases::auth_key::AuthKeyApiService;
use torrust_tracker_rest_api_application::v1::use_cases::stats::StatsApiService;
use torrust_tracker_rest_api_application::v1::use_cases::torrent::TorrentApiService;
use torrust_tracker_rest_api_application::v1::use_cases::whitelist::WhitelistApiService;
use torrust_tracker_rest_api_runtime_adapter::v1::adapters::auth_key::TrackerAuthKeyAdapter;
use torrust_tracker_rest_api_runtime_adapter::v1::adapters::stats::TrackerStatsAdapter;
use torrust_tracker_rest_api_runtime_adapter::v1::adapters::torrent::TrackerTorrentQueryAdapter;
use torrust_tracker_rest_api_runtime_adapter::v1::adapters::whitelist::TrackerWhitelistAdapter;
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;

use super::context::{auth_key, stats, torrent, whitelist};

/// Add the routes for the v1 API.
pub fn add(prefix: &str, router: Router, http_api_container: &Arc<TrackerHttpApiCoreContainer>) -> Router {
    let v1_prefix = format!("{prefix}/v1");

    let auth_key_service = if http_api_container.tracker_core_container.core_config.private {
        http_api_container
            .tracker_core_container
            .persistence
            .as_ref()
            .map(|persistence| {
                let auth_key_adapter = TrackerAuthKeyAdapter::new(&persistence.keys_handler);
                Arc::new(AuthKeyApiService::new(Box::new(auth_key_adapter)))
            })
    } else {
        None
    };
    let router = auth_key::routes::add(&v1_prefix, router, auth_key_service.as_ref());

    let stats_adapter = TrackerStatsAdapter::new(
        &http_api_container.tracker_core_container.in_memory_torrent_repository,
        &http_api_container.swarm_coordination_registry_container.stats_repository,
        &http_api_container.tracker_core_container.stats_repository,
        &http_api_container.http_stats_repository,
        &http_api_container.udp_core_stats_repository,
        &http_api_container.udp_server_stats_repository,
    );
    let stats_service = Arc::new(StatsApiService::new(Box::new(stats_adapter)));
    let router = stats::routes::add(&v1_prefix, router, &stats_service);

    let whitelist_service = if http_api_container.tracker_core_container.core_config.listed {
        http_api_container
            .tracker_core_container
            .persistence
            .as_ref()
            .map(|persistence| {
                let whitelist_adapter = TrackerWhitelistAdapter::new(&persistence.whitelist_manager);
                Arc::new(WhitelistApiService::new(Box::new(whitelist_adapter)))
            })
    } else {
        None
    };
    let router = whitelist::routes::add(&v1_prefix, router, whitelist_service.as_ref());

    let tracker_adapter =
        TrackerTorrentQueryAdapter::new(&http_api_container.tracker_core_container.in_memory_torrent_repository);
    let torrent_service = Arc::new(TorrentApiService::new(Box::new(tracker_adapter)));

    torrent::routes::add(&v1_prefix, router, &torrent_service)
}
