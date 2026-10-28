//! Route initialization for the v1 API.
//!
//! Completed-download persistence availability is derived here from validated
//! configuration as defined by ADR
//! [`20260901113500_define_completed_download_metric_retention_names`](../../../../docs/adrs/20260901113500_define_completed_download_metric_retention_names.md).
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
        http_api_container
            .tracker_core_container
            .core_config
            .tracker_policy
            .persistent_torrent_completed_stat,
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

#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::{Body, to_bytes};
    use axum::http::header::CONTENT_TYPE;
    use axum::http::{Method, Request, StatusCode};
    use rstest::rstest;
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_test_helpers::configuration::ephemeral;
    use tower::ServiceExt;

    use super::add;
    use crate::test_support::{MAX_BODY_BYTES, api_container};

    const KEY_PATH: &str = "/api/v1/key/60";
    const WHITELIST_RELOAD_PATH: &str = "/api/v1/whitelist/reload";
    const INFO_HASH: &str = "443c7602b4fde83d1154d6d9da48808418b181b6";

    fn private_tracker() -> Configuration {
        let mut configuration = ephemeral();
        configuration.core.private = true;
        configuration
    }

    fn listed_tracker() -> Configuration {
        let mut configuration = ephemeral();
        configuration.core.listed = true;
        configuration
    }

    fn without_database(mut configuration: Configuration) -> Configuration {
        configuration.core.database = None;
        configuration
    }

    /// Sends one request to the v1 routes composed for the configuration and returns the status, the content type
    /// and the body.
    async fn call(configuration: &Configuration, method: Method, uri: &str) -> (StatusCode, Option<String>, String) {
        let container = api_container(configuration).await;
        let router = add("/api", Router::new(), &container);
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .expect("the request is well formed");

        let response = router.oneshot(request).await.expect("the router never fails");

        let status = response.status();
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .map(|value| value.to_str().expect("the content type is text").to_string());
        let body = to_bytes(response.into_body(), MAX_BODY_BYTES)
            .await
            .expect("the body is readable and within the limit");

        (
            status,
            content_type,
            String::from_utf8(body.to_vec()).expect("the body is text"),
        )
    }

    #[tokio::test]
    async fn it_should_serve_the_auth_key_routes_when_the_tracker_is_private_and_has_persistence() {
        let configuration = private_tracker();

        let (status, _content_type, _body) = call(&configuration, Method::POST, KEY_PATH).await;

        assert_eq!(status, StatusCode::OK);
    }

    #[rstest]
    #[case::a_public_tracker_with_persistence(ephemeral())]
    #[case::a_private_tracker_without_persistence_as_the_second_line_of_defense(without_database(private_tracker()))]
    #[tokio::test]
    async fn it_should_answer_conflict_on_the_auth_key_routes_when_the_private_capability_is_not_available(
        #[case] configuration: Configuration,
    ) {
        let (status, content_type, body) = call(&configuration, Method::POST, KEY_PATH).await;

        assert_eq!(
            (status, content_type, body),
            (
                StatusCode::CONFLICT,
                Some("application/json".to_string()),
                r#"{"status":"err","reason":"private capability is disabled by configuration"}"#.to_string(),
            )
        );
    }

    #[tokio::test]
    async fn it_should_serve_the_whitelist_routes_when_the_tracker_is_listed_and_has_persistence() {
        let configuration = listed_tracker();

        let (status, content_type, body) = call(&configuration, Method::GET, WHITELIST_RELOAD_PATH).await;

        assert_eq!(
            (status, content_type, body),
            (
                StatusCode::OK,
                Some("application/json".to_string()),
                r#"{"status":"ok"}"#.to_string()
            )
        );
    }

    #[rstest]
    #[case::a_public_tracker_with_persistence(ephemeral())]
    #[case::a_listed_tracker_without_persistence_as_the_second_line_of_defense(without_database(listed_tracker()))]
    #[tokio::test]
    async fn it_should_answer_conflict_on_the_whitelist_routes_when_the_listed_capability_is_not_available(
        #[case] configuration: Configuration,
    ) {
        let (status, content_type, body) = call(&configuration, Method::GET, WHITELIST_RELOAD_PATH).await;

        assert_eq!(
            (status, content_type, body),
            (
                StatusCode::CONFLICT,
                Some("application/json".to_string()),
                r#"{"status":"err","reason":"listed capability is disabled by configuration"}"#.to_string(),
            )
        );
    }

    #[rstest]
    #[case::the_statistics("/api/v1/stats")]
    #[case::the_torrent_list("/api/v1/torrents")]
    #[tokio::test]
    async fn it_should_serve_the_read_routes_even_when_the_tracker_has_no_persistence(#[case] uri: &str) {
        let configuration = without_database(ephemeral());

        let (status, _content_type, _body) = call(&configuration, Method::GET, uri).await;

        assert_eq!(status, StatusCode::OK);
    }

    #[rstest]
    #[case::generate_a_key(Method::POST, "/api/v1/key/60")]
    #[case::delete_a_key(Method::DELETE, "/api/v1/key/abc")]
    #[case::add_a_key(Method::POST, "/api/v1/keys")]
    #[case::reload_the_keys(Method::GET, "/api/v1/keys/reload")]
    #[case::whitelist_a_torrent(Method::POST, &format!("/api/v1/whitelist/{INFO_HASH}"))]
    #[case::remove_a_torrent_from_the_whitelist(Method::DELETE, &format!("/api/v1/whitelist/{INFO_HASH}"))]
    #[case::reload_the_whitelist(Method::GET, "/api/v1/whitelist/reload")]
    #[case::get_the_statistics(Method::GET, "/api/v1/stats")]
    #[case::get_the_metrics(Method::GET, "/api/v1/metrics")]
    #[case::get_the_torrents(Method::GET, "/api/v1/torrents")]
    #[case::get_a_torrent(Method::GET, &format!("/api/v1/torrent/{INFO_HASH}"))]
    #[tokio::test]
    async fn it_should_register_every_route_of_the_four_contexts_even_for_the_least_capable_tracker(
        #[case] method: Method,
        #[case] uri: &str,
    ) {
        let configuration = without_database(ephemeral());

        let (status, _content_type, _body) = call(&configuration, method, uri).await;

        assert!(
            status != StatusCode::NOT_FOUND && status != StatusCode::METHOD_NOT_ALLOWED,
            "the route is not registered: {status}"
        );
    }
}
