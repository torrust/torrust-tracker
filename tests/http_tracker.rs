/// Integration tests for HTTP tracker server
///
/// cargo test `http_tracker_server` -- --nocapture
mod common;
mod http;

mod http_tracker_server {

    mod for_all_config_modes {

        mod receiving_an_announce_request {
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
            use crate::http::asserts::{assert_announce_response, assert_empty_announce_response};
            use crate::http::client::Client;
            use crate::http::requests::AnnounceQueryBuilder;
            use crate::http::responses::{Announce, DictionaryPeer};
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
        }
    }
}
