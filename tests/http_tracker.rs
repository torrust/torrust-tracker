/// Integration tests for HTTP tracker server
///
/// cargo test `http_tracker_server` -- --nocapture
mod common;
mod http;

mod http_tracker_server {

    mod for_all_config_modes {

        mod receiving_an_announce_request {
            use std::net::{IpAddr, Ipv6Addr};
            use std::str::FromStr;

            use local_ip_address::local_ip;
            use reqwest::Response;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::{invalid_info_hashes, PeerBuilder};
            use crate::http::asserts::{
                assert_announce_response, assert_compact_announce_response, assert_empty_announce_response,
                assert_internal_server_error_response, assert_invalid_info_hash_error_response,
                assert_invalid_peer_id_error_response, assert_is_announce_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::{AnnounceQueryBuilder, Compact};
            use crate::http::responses::{self, Announce, CompactAnnounce, CompactPeer, CompactPeerList, DictionaryPeer};
            use crate::http::server::{
                start_default_http_tracker, start_http_tracker_on_reverse_proxy, start_http_tracker_with_external_ip,
                start_ipv6_http_tracker, start_public_http_tracker,
            };

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

                // Announce the new Peer 2. This new peer is non included on the response peer list
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &AnnounceQueryBuilder::default()
                            .with_info_hash(&info_hash)
                            .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                            .query(),
                    )
                    .await;

                // It should only contain teh previously announced peer
                assert_announce_response(
                    response,
                    &Announce {
                        complete: 2,
                        incomplete: 0,
                        interval: http_tracker_server.tracker.config.announce_interval,
                        min_interval: http_tracker_server.tracker.config.min_announce_interval,
                        peers: vec![DictionaryPeer::from(previously_announced_peer)],
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

                let expected_response = responses::DecodedCompactAnnounce {
                    complete: 2,
                    incomplete: 0,
                    interval: 120,
                    min_interval: 120,
                    peers: CompactPeerList::new([CompactPeer::new(&previously_announced_peer.peer_addr)].to_vec()),
                };

                assert_compact_announce_response(response, &expected_response).await;
            }

            #[tokio::test]
            async fn should_not_return_the_compact_response_by_default() {
                // code-review: the HTTP tracker does not return the compact response by default if the "compact"
                // param is not provided in the announce URL. The BEP 23 suggest to do so.

                let http_tracker_server = start_public_http_tracker().await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .into();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2 without passing the "compact" param
                // By default it should respond with the compact peer list
                // https://www.bittorrent.org/beps/bep_0023.html
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &AnnounceQueryBuilder::default()
                            .with_info_hash(&info_hash)
                            .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                            .without_compact()
                            .query(),
                    )
                    .await;

                assert!(!is_a_compact_announce_response(response).await);
            }

            async fn is_a_compact_announce_response(response: Response) -> bool {
                let bytes = response.bytes().await.unwrap();
                let compact_announce = serde_bencode::from_bytes::<CompactAnnounce>(&bytes);
                compact_announce.is_ok()
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp4_connections_handled_in_statistics() {
                let http_tracker_server = start_public_http_tracker().await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(&AnnounceQueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp4_connections_handled, 1);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp6_connections_handled_in_statistics() {
                let http_tracker_server = start_ipv6_http_tracker().await;

                Client::bind(http_tracker_server.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .announce(&AnnounceQueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_connections_handled, 1);
            }

            #[tokio::test]
            async fn should_not_increase_the_number_of_tcp6_connections_handled_if_the_client_is_not_using_an_ipv6_ip() {
                // The tracker ignores the peer address in the request param. It uses the client remote ip address.

                let http_tracker_server = start_public_http_tracker().await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &AnnounceQueryBuilder::default()
                            .with_peer_addr(&IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
                            .query(),
                    )
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_connections_handled, 0);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp4_announce_requests_handled_in_statistics() {
                let http_tracker_server = start_public_http_tracker().await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(&AnnounceQueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp4_announces_handled, 1);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp6_announce_requests_handled_in_statistics() {
                let http_tracker_server = start_ipv6_http_tracker().await;

                Client::bind(http_tracker_server.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .announce(&AnnounceQueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_announces_handled, 1);
            }

            #[tokio::test]
            async fn should_not_increase_the_number_of_tcp6_announce_requests_handled_if_the_client_is_not_using_an_ipv6_ip() {
                // The tracker ignores the peer address in the request param. It uses the client remote ip address.

                let http_tracker_server = start_public_http_tracker().await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &AnnounceQueryBuilder::default()
                            .with_peer_addr(&IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
                            .query(),
                    )
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_announces_handled, 0);
            }

            #[tokio::test]
            async fn should_assign_to_the_peer_ip_the_remote_client_ip_instead_of_the_peer_address_in_the_request_param() {
                let http_tracker_server = start_public_http_tracker().await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let client_ip = local_ip().unwrap();

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = AnnounceQueryBuilder::default()
                    .with_info_hash(&info_hash)
                    .with_peer_addr(&IpAddr::from_str("2.2.2.2").unwrap())
                    .query();

                client.announce(&announce_query).await;

                let peers = http_tracker_server.tracker.get_all_torrent_peers(&info_hash).await;
                let peer_addr = peers[0].peer_addr;

                assert_eq!(peer_addr.ip(), client_ip);
                assert_ne!(peer_addr.ip(), IpAddr::from_str("2.2.2.2").unwrap());
            }

            #[tokio::test]
            async fn when_the_client_ip_is_a_loopback_ipv4_it_should_assign_to_the_peer_ip_the_external_ip_in_the_tracker_configuration(
            ) {
                /*  We assume that both the client and tracker share the same public IP.

                    client     <-> tracker                      <-> Internet
                    127.0.0.1      external_ip = "2.137.87.41"
                */

                let http_tracker_server = start_http_tracker_with_external_ip(&IpAddr::from_str("2.137.87.41").unwrap()).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
                let client_ip = loopback_ip;

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = AnnounceQueryBuilder::default()
                    .with_info_hash(&info_hash)
                    .with_peer_addr(&IpAddr::from_str("2.2.2.2").unwrap())
                    .query();

                client.announce(&announce_query).await;

                let peers = http_tracker_server.tracker.get_all_torrent_peers(&info_hash).await;
                let peer_addr = peers[0].peer_addr;

                assert_eq!(peer_addr.ip(), http_tracker_server.tracker.config.get_ext_ip().unwrap());
                assert_ne!(peer_addr.ip(), IpAddr::from_str("2.2.2.2").unwrap());
            }

            #[tokio::test]
            async fn when_the_client_ip_is_a_loopback_ipv6_it_should_assign_to_the_peer_ip_the_external_ip_in_the_tracker_configuration(
            ) {
                /* We assume that both the client and tracker share the same public IP.

                   client     <-> tracker                                                  <-> Internet
                   ::1            external_ip = "2345:0425:2CA1:0000:0000:0567:5673:23b5"
                */

                let http_tracker_server =
                    start_http_tracker_with_external_ip(&IpAddr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap())
                        .await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
                let client_ip = loopback_ip;

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = AnnounceQueryBuilder::default()
                    .with_info_hash(&info_hash)
                    .with_peer_addr(&IpAddr::from_str("2.2.2.2").unwrap())
                    .query();

                client.announce(&announce_query).await;

                let peers = http_tracker_server.tracker.get_all_torrent_peers(&info_hash).await;
                let peer_addr = peers[0].peer_addr;

                assert_eq!(peer_addr.ip(), http_tracker_server.tracker.config.get_ext_ip().unwrap());
                assert_ne!(peer_addr.ip(), IpAddr::from_str("2.2.2.2").unwrap());
            }

            #[tokio::test]
            async fn when_the_tracker_is_behind_a_reverse_proxy_it_should_assign_to_the_peer_ip_the_ip_in_the_x_forwarded_for_http_header(
            ) {
                /*
                client          <-> http proxy                       <-> tracker                   <-> Internet
                ip:                 header:                              config:                       peer addr:
                145.254.214.256     X-Forwarded-For = 145.254.214.256    on_reverse_proxy = true       145.254.214.256
                */

                let http_tracker_server = start_http_tracker_on_reverse_proxy().await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let client = Client::new(http_tracker_server.get_connection_info());

                let announce_query = AnnounceQueryBuilder::default().with_info_hash(&info_hash).query();

                // todo: shouldn't be the the leftmost IP address?
                // THe application is taken the the rightmost IP address. See function http::filters::peer_addr
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For
                client
                    .announce_with_header(
                        &announce_query,
                        "X-Forwarded-For",
                        "203.0.113.195,2001:db8:85a3:8d3:1319:8a2e:370:7348,150.172.238.178",
                    )
                    .await;

                let peers = http_tracker_server.tracker.get_all_torrent_peers(&info_hash).await;
                let peer_addr = peers[0].peer_addr;

                assert_eq!(peer_addr.ip(), IpAddr::from_str("150.172.238.178").unwrap());
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

    mod configured_as_whitelisted {

        mod and_receiving_an_announce_request {}

        mod receiving_an_scrape_request {}
    }

    mod configured_as_private {

        mod and_receiving_an_announce_request {}

        mod receiving_an_scrape_request {}
    }

    mod configured_as_private_and_whitelisted {

        mod and_receiving_an_announce_request {}

        mod receiving_an_scrape_request {}
    }
}
