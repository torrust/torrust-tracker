// Scrape documentation:
//
// BEP 48. Tracker Protocol Extension: Scrape
// https://www.bittorrent.org/beps/bep_0048.html
//
// Vuze (bittorrent client) docs:
// https://wiki.vuze.com/w/Scrape

use std::net::{IpAddr, Ipv6Addr, SocketAddrV6};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use tokio::net::TcpListener;
use torrust_info_hash::InfoHash;
use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_client::http::client::Client;
use torrust_tracker_http_protocol::v1::requests::scrape_builder::QueryBuilder;
use torrust_tracker_http_protocol::v1::responses::scrape::deserialization::{self, File, ResponseBuilder};
use torrust_tracker_primitives::PeerId;
use torrust_tracker_primitives::peer::fixture::PeerBuilder;
use torrust_tracker_test_helpers::{configuration, logging};

use crate::common::fixtures::invalid_info_hashes;
use crate::server::asserts::{
    assert_cannot_parse_query_params_error_response, assert_missing_query_params_for_scrape_request_error_response,
    assert_scrape_response,
};

#[tokio::test]
#[allow(dead_code)]
async fn should_fail_when_the_request_is_empty() {
    logging::setup();

    let cfg = configuration::ephemeral_public();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;
    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .get("scrape")
        .await
        .unwrap();

    assert_missing_query_params_for_scrape_request_error_response(response).await;

    env.stop().await;
}

#[tokio::test]
async fn should_fail_when_the_info_hash_param_is_invalid() {
    logging::setup();

    let cfg = configuration::ephemeral_public();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    for invalid_value in &invalid_info_hashes() {
        let url = format!("scrape?info_hash={invalid_value}");

        let response = Client::new(env.base_url(), Duration::from_secs(5))
            .unwrap()
            .get(&url)
            .await
            .unwrap();

        assert_cannot_parse_query_params_error_response(response, "").await;
    }

    env.stop().await;
}

#[tokio::test]
async fn should_return_the_file_with_the_incomplete_peer_when_there_is_one_peer_with_bytes_pending_to_download() {
    logging::setup();

    let cfg = configuration::ephemeral_public();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

    env.add_torrent_peer(
        &info_hash,
        &PeerBuilder::default()
            .with_peer_id(&PeerId(*b"-qB00000000000000001"))
            .with_bytes_left_to_download(1)
            .build(),
    )
    .await;

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .scrape(&QueryBuilder::default().with_one_info_hash(&info_hash).query())
        .await
        .unwrap();

    let expected_scrape_response = ResponseBuilder::default()
        .add_file(
            info_hash,
            File {
                complete: 0,
                downloaded: 0,
                incomplete: 1,
            },
        )
        .build();

    assert_scrape_response(response, &expected_scrape_response).await;

    env.stop().await;
}

#[tokio::test]
async fn should_return_the_file_with_the_complete_peer_when_there_is_one_peer_with_no_bytes_pending_to_download() {
    logging::setup();

    let cfg = configuration::ephemeral_public();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

    env.add_torrent_peer(
        &info_hash,
        &PeerBuilder::default()
            .with_peer_id(&torrust_tracker_primitives::PeerId(*b"-qB00000000000000001"))
            .with_no_bytes_left_to_download()
            .build(),
    )
    .await;

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .scrape(&QueryBuilder::default().with_one_info_hash(&info_hash).query())
        .await
        .unwrap();

    let expected_scrape_response = ResponseBuilder::default()
        .add_file(
            info_hash,
            File {
                complete: 1,
                downloaded: 0,
                incomplete: 0,
            },
        )
        .build();

    assert_scrape_response(response, &expected_scrape_response).await;

    env.stop().await;
}

#[tokio::test]
async fn should_return_a_file_with_zeroed_values_when_there_are_no_peers() {
    logging::setup();

    let cfg = configuration::ephemeral_public();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .scrape(&QueryBuilder::default().with_one_info_hash(&info_hash).query())
        .await
        .unwrap();

    assert_scrape_response(response, &deserialization::Response::with_one_file(info_hash, File::zeroed())).await;

    env.stop().await;
}

#[tokio::test]
async fn should_accept_multiple_infohashes() {
    logging::setup();

    let cfg = configuration::ephemeral_public();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let info_hash1 = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237
    let info_hash2 = InfoHash::from_str("3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0").unwrap(); // DevSkim: ignore DS173237

    let response = Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .scrape(
            &QueryBuilder::default()
                .add_info_hash(&info_hash1)
                .add_info_hash(&info_hash2)
                .query(),
        )
        .await
        .unwrap();

    let expected_scrape_response = ResponseBuilder::default()
        .add_file(info_hash1, File::zeroed())
        .add_file(info_hash2, File::zeroed())
        .build();

    assert_scrape_response(response, &expected_scrape_response).await;

    env.stop().await;
}

#[tokio::test]
async fn should_increase_the_number_ot_tcp4_scrape_requests_handled_in_statistics() {
    logging::setup();

    let cfg = configuration::ephemeral_public();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

    Client::new(env.base_url(), Duration::from_secs(5))
        .unwrap()
        .scrape(&QueryBuilder::default().with_one_info_hash(&info_hash).query())
        .await
        .unwrap();

    let stats = env.container.http_tracker_core_container.stats_repository.get_stats().await;

    assert_eq!(stats.tcp4_scrapes_handled(), 1);

    drop(stats);

    env.stop().await;
}

#[tokio::test]
async fn should_increase_the_number_ot_tcp6_scrape_requests_handled_in_statistics() {
    logging::setup();

    if TcpListener::bind(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0))
        .await
        .is_err()
    {
        return; // we cannot bind to a ipv6 socket, so we will skip this test
    }

    let cfg = configuration::ephemeral_ipv6();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

    Client::bind(env.base_url(), Duration::from_secs(5), IpAddr::from_str("::1").unwrap())
        .unwrap()
        .scrape(&QueryBuilder::default().with_one_info_hash(&info_hash).query())
        .await
        .unwrap();

    let stats = env.container.http_tracker_core_container.stats_repository.get_stats().await;

    assert_eq!(stats.tcp6_scrapes_handled(), 1);

    drop(stats);

    env.stop().await;
}
