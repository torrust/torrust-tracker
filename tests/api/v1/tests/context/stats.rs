use std::str::FromStr;

use torrust_tracker::apis::v1::context::stats::resources::Stats;
use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker_test_helpers::configuration;

use crate::api::connection_info::{connection_with_invalid_token, connection_with_no_token};
use crate::api::test_environment::running_test_environment;
use crate::api::v1::asserts::{assert_stats, assert_token_not_valid, assert_unauthorized};
use crate::api::v1::client::Client;
use crate::common::fixtures::PeerBuilder;

#[tokio::test]
async fn should_allow_getting_tracker_statistics() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    test_env
        .add_torrent_peer(
            &InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(),
            &PeerBuilder::default().into(),
        )
        .await;

    let response = Client::new(test_env.get_connection_info()).get_tracker_statistics().await;

    assert_stats(
        response,
        Stats {
            torrents: 1,
            seeders: 1,
            completed: 0,
            leechers: 0,
            tcp4_connections_handled: 0,
            tcp4_announces_handled: 0,
            tcp4_scrapes_handled: 0,
            tcp6_connections_handled: 0,
            tcp6_announces_handled: 0,
            tcp6_scrapes_handled: 0,
            udp4_connections_handled: 0,
            udp4_announces_handled: 0,
            udp4_scrapes_handled: 0,
            udp6_connections_handled: 0,
            udp6_announces_handled: 0,
            udp6_scrapes_handled: 0,
        },
    )
    .await;

    test_env.stop().await;
}

#[tokio::test]
async fn should_not_allow_getting_tracker_statistics_for_unauthenticated_users() {
    let test_env = running_test_environment(configuration::ephemeral()).await;

    let response = Client::new(connection_with_invalid_token(
        test_env.get_connection_info().bind_address.as_str(),
    ))
    .get_tracker_statistics()
    .await;

    assert_token_not_valid(response).await;

    let response = Client::new(connection_with_no_token(test_env.get_connection_info().bind_address.as_str()))
        .get_tracker_statistics()
        .await;

    assert_unauthorized(response).await;

    test_env.stop().await;
}
