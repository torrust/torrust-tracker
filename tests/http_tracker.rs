/// Integration tests for HTTP tracker server
///
/// cargo test `http_tracker_server` -- --nocapture
mod common;
mod http;

mod http_tracker_server {

    mod for_all_config_modes {

        mod receiving_an_announce_request {
            use crate::common::fixtures::invalid_info_hashes;
            use crate::http::asserts::{
                assert_internal_server_error_response, assert_invalid_info_hash_error_response,
                assert_invalid_peer_id_error_response, assert_is_announce_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::AnnounceQueryBuilder;
            use crate::http::server::start_default_http_tracker;

            #[tokio::test]
            async fn should_respond_when_only_the_mandatory_fields_are_provided() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                params.remove_optional_params();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_is_announce_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_request_is_empty() {
                let http_tracker_server = start_default_http_tracker().await;

                let response = Client::new(http_tracker_server.get_connection_info()).get("announce").await;

                assert_internal_server_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_a_mandatory_field_is_missing() {
                let http_tracker_server = start_default_http_tracker().await;

                // Without `info_hash` param

                let mut params = AnnounceQueryBuilder::default().query().params();

                params.info_hash = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_invalid_info_hash_error_response(response).await;

                // Without `peer_id` param

                let mut params = AnnounceQueryBuilder::default().query().params();

                params.peer_id = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_invalid_peer_id_error_response(response).await;

                // Without `port` param

                let mut params = AnnounceQueryBuilder::default().query().params();

                params.port = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_internal_server_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_info_hash_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                for invalid_value in &invalid_info_hashes() {
                    params.set("info_hash", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_invalid_info_hash_error_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_not_fail_when_the_peer_address_param_is_invalid() {
                // AnnounceQuery does not even contain the `peer_addr`
                // The peer IP is obtained in two ways:
                // 1. If tracker is NOT running `on_reverse_proxy` from the remote client IP if there.
                // 2. If tracker is     running `on_reverse_proxy` from `X-Forwarded-For` request header is tracker is running `on_reverse_proxy`.

                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                params.peer_addr = Some("INVALID-IP-ADDRESS".to_string());

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_is_announce_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_downloaded_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("downloaded", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_internal_server_error_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_fail_when_the_uploaded_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("uploaded", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_internal_server_error_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_fail_when_the_peer_id_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                let invalid_values = [
                    "0",
                    "-1",
                    "1.1",
                    "a",
                    "-qB0000000000000000",   // 19 bytes
                    "-qB000000000000000000", // 21 bytes
                ];

                for invalid_value in invalid_values {
                    params.set("peer_id", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_invalid_peer_id_error_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_fail_when_the_port_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("port", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_internal_server_error_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_fail_when_the_left_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("left", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_internal_server_error_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_not_fail_when_the_event_param_is_invalid() {
                // All invalid values are ignored as if the `event` param was empty

                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                let invalid_values = [
                    "0",
                    "-1",
                    "1.1",
                    "a",
                    "Started",   // It should be lowercase
                    "Stopped",   // It should be lowercase
                    "Completed", // It should be lowercase
                ];

                for invalid_value in invalid_values {
                    params.set("event", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_is_announce_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_not_fail_when_the_compact_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker().await;

                let mut params = AnnounceQueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("compact", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_internal_server_error_response(response).await;
                }
            }
        }

        mod receiving_an_scrape_request {
            use crate::http::asserts::assert_internal_server_error_response;
            use crate::http::client::Client;
            use crate::http::server::start_default_http_tracker;

            #[tokio::test]
            async fn should_fail_when_the_request_is_empty() {
                let http_tracker_server = start_default_http_tracker().await;

                let response = Client::new(http_tracker_server.get_connection_info()).get("scrape").await;

                assert_internal_server_error_response(response).await;
            }
        }
    }

    mod configured_as_public {

        mod receiving_an_announce_request {
            use std::str::FromStr;

            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::PeerBuilder;
            use crate::http::asserts::{
                assert_announce_response, assert_compact_announce_response, assert_empty_announce_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::{AnnounceQueryBuilder, Compact};
            use crate::http::responses::{Announce, CompactPeer, CompactPeerList, DecodedCompactAnnounce, DictionaryPeer};
            use crate::http::server::start_public_http_tracker;

            #[tokio::test]
            async fn should_return_no_peers_if_the_announced_peer_is_the_first_one() {
                let http_tracker_server = start_public_http_tracker().await;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &AnnounceQueryBuilder::default()
                            .with_info_hash(&InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap())
                            .query(),
                    )
                    .await;

                assert_announce_response(
                    response,
                    &Announce {
                        complete: 1, // the peer for this test
                        incomplete: 0,
                        interval: http_tracker_server.tracker.config.announce_interval,
                        min_interval: http_tracker_server.tracker.config.min_announce_interval,
                        peers: vec![],
                    },
                )
                .await;
            }

            #[tokio::test]
            async fn should_return_the_list_of_previously_announced_peers() {
                let http_tracker_server = start_public_http_tracker().await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .into();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &AnnounceQueryBuilder::default()
                            .with_info_hash(&info_hash)
                            .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                            .query(),
                    )
                    .await;

                let expected_peer = DictionaryPeer {
                    peer_id: previously_announced_peer.peer_id.to_string(),
                    ip: previously_announced_peer.peer_addr.ip().to_string(),
                    port: previously_announced_peer.peer_addr.port(),
                };

                // This new peer is non included on the response peer list
                assert_announce_response(
                    response,
                    &Announce {
                        complete: 2,
                        incomplete: 0,
                        interval: http_tracker_server.tracker.config.announce_interval,
                        min_interval: http_tracker_server.tracker.config.min_announce_interval,
                        peers: vec![expected_peer],
                    },
                )
                .await;
            }

            #[tokio::test]
            async fn should_consider_two_peers_to_be_the_same_when_they_have_the_same_peer_id_even_if_the_ip_is_different() {
                let http_tracker_server = start_public_http_tracker().await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let peer = PeerBuilder::default().into();

                // Add a peer
                http_tracker_server.add_torrent(&info_hash, &peer).await;

                let announce_query = AnnounceQueryBuilder::default()
                    .with_info_hash(&info_hash)
                    .with_peer_id(&peer.peer_id)
                    .query();

                assert_ne!(peer.peer_addr.ip(), announce_query.peer_addr);

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&announce_query)
                    .await;

                assert_empty_announce_response(response).await;
            }

            #[tokio::test]
            async fn should_return_the_compact_response() {
                // Tracker Returns Compact Peer Lists
                // https://www.bittorrent.org/beps/bep_0023.html

                let http_tracker_server = start_public_http_tracker().await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .into();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2 accepting compact responses
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &AnnounceQueryBuilder::default()
                            .with_info_hash(&info_hash)
                            .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                            .with_compact(Compact::Accepted)
                            .query(),
                    )
                    .await;

                let expected_response = DecodedCompactAnnounce {
                    complete: 2,
                    incomplete: 0,
                    interval: 120,
                    min_interval: 120,
                    peers: CompactPeerList::new([CompactPeer::new(&previously_announced_peer.peer_addr)].to_vec()),
                };

                assert_compact_announce_response(response, &expected_response).await;
            }
        }
    }
}
