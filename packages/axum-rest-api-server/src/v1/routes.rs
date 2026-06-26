//! Route initialization for the v1 API.
use std::sync::Arc;

use axum::Router;
use torrust_tracker_rest_api_application::use_cases::torrent::TorrentApiService;
use torrust_tracker_rest_api_application::use_cases::whitelist::WhitelistApiService;
use torrust_tracker_rest_api_core::container::TrackerHttpApiCoreContainer;
use torrust_tracker_rest_api_runtime_adapter::adapters::torrent::TrackerTorrentQueryAdapter;
use torrust_tracker_rest_api_runtime_adapter::adapters::whitelist::TrackerWhitelistAdapter;

use super::context::{auth_key, stats, torrent, whitelist};

/// Add the routes for the v1 API.
pub fn add(prefix: &str, router: Router, http_api_container: &Arc<TrackerHttpApiCoreContainer>) -> Router {
    let v1_prefix = format!("{prefix}/v1");

    let router = auth_key::routes::add(
        &v1_prefix,
        router,
        &http_api_container.tracker_core_container.keys_handler.clone(),
    );
    let router = stats::routes::add(&v1_prefix, router, http_api_container);

    let whitelist_adapter = TrackerWhitelistAdapter::new(&http_api_container.tracker_core_container.whitelist_manager);
    let whitelist_service = Arc::new(WhitelistApiService::new(Box::new(whitelist_adapter)));
    let router = whitelist::routes::add(&v1_prefix, router, &whitelist_service);

    let tracker_adapter =
        TrackerTorrentQueryAdapter::new(&http_api_container.tracker_core_container.in_memory_torrent_repository);
    let torrent_service = Arc::new(TorrentApiService::new(Box::new(tracker_adapter)));

    torrent::routes::add(&v1_prefix, router, &torrent_service)
}
