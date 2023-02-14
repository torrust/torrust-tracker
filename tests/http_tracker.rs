/// Integration tests for HTTP tracker server
///
/// Warp version:
/// ```text
/// cargo test `warp_http_tracker_server` -- --nocapture
/// ```
///
/// Axum version ()WIP):
/// ```text
/// cargo test `warp_http_tracker_server` -- --nocapture
/// ```
mod common;
mod http;

mod warp_http_tracker_server {

    mod for_all_config_modes {

        mod running_on_reverse_proxy {
            use torrust_tracker::http::Version;

            use crate::http::asserts::{
                assert_could_not_find_remote_address_on_xff_header_error_response,
                assert_invalid_remote_address_on_xff_header_error_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::announce::QueryBuilder;
            use crate::http::server::start_http_tracker_on_reverse_proxy;

            #[tokio::test]
            async fn should_fail_when_the_http_request_does_not_include_the_xff_http_request_header() {
                // If the tracker is running behind a reverse proxy, the peer IP is the
                // last IP in the `X-Forwarded-For` HTTP header, which is the IP of the proxy client.

                let http_tracker_server = start_http_tracker_on_reverse_proxy(Version::Warp).await;

                let params = QueryBuilder::default().query().params();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_could_not_find_remote_address_on_xff_header_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_xff_http_request_header_contains_an_invalid_ip() {
                let http_tracker_server = start_http_tracker_on_reverse_proxy(Version::Warp).await;

                let params = QueryBuilder::default().query().params();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get_with_header(&format!("announce?{params}"), "X-Forwarded-For", "INVALID IP")
                    .await;

                assert_invalid_remote_address_on_xff_header_error_response(response).await;
            }
        }

        mod receiving_an_announce_request {

            // Announce request documentation:
            //
            // BEP 03. The BitTorrent Protocol Specification
            // https://www.bittorrent.org/beps/bep_0003.html
            //
            // BEP 23. Tracker Returns Compact Peer Lists
            // https://www.bittorrent.org/beps/bep_0023.html
            //
            // Vuze (bittorrent client) docs:
            // https://wiki.vuze.com/w/Announce

            use std::net::{IpAddr, Ipv6Addr};
            use std::str::FromStr;

            use local_ip_address::local_ip;
            use reqwest::Response;
            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::{invalid_info_hashes, PeerBuilder};
            use crate::http::asserts::{
                assert_announce_response, assert_compact_announce_response, assert_empty_announce_response,
                assert_internal_server_error_response, assert_invalid_info_hash_error_response,
                assert_invalid_peer_id_error_response, assert_is_announce_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::announce::{Compact, QueryBuilder};
            use crate::http::responses;
            use crate::http::responses::announce::{Announce, CompactPeer, CompactPeerList, DictionaryPeer};
            use crate::http::server::{
                start_default_http_tracker, start_http_tracker_on_reverse_proxy, start_http_tracker_with_external_ip,
                start_ipv6_http_tracker, start_public_http_tracker,
            };

            #[tokio::test]
            async fn should_respond_if_only_the_mandatory_fields_are_provided() {
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

                params.remove_optional_params();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_is_announce_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_url_query_component_is_empty() {
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let response = Client::new(http_tracker_server.get_connection_info()).get("announce").await;

                assert_internal_server_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_a_mandatory_field_is_missing() {
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                // Without `info_hash` param

                let mut params = QueryBuilder::default().query().params();

                params.info_hash = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_invalid_info_hash_error_response(response).await;

                // Without `peer_id` param

                let mut params = QueryBuilder::default().query().params();

                params.peer_id = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_invalid_peer_id_error_response(response).await;

                // Without `port` param

                let mut params = QueryBuilder::default().query().params();

                params.port = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_internal_server_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_info_hash_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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

                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

                params.peer_addr = Some("INVALID-IP-ADDRESS".to_string());

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_is_announce_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_downloaded_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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
                // All invalid values are ignored as if the `event` param were empty

                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_default_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
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
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .build();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2. This new peer is non included on the response peer list
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
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
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let peer = PeerBuilder::default().build();

                // Add a peer
                http_tracker_server.add_torrent(&info_hash, &peer).await;

                let announce_query = QueryBuilder::default()
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

                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .build();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2 accepting compact responses
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
                            .with_info_hash(&info_hash)
                            .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                            .with_compact(Compact::Accepted)
                            .query(),
                    )
                    .await;

                let expected_response = responses::announce::Compact {
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

                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .build();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2 without passing the "compact" param
                // By default it should respond with the compact peer list
                // https://www.bittorrent.org/beps/bep_0023.html
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
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
                let compact_announce = serde_bencode::from_bytes::<responses::announce::DeserializedCompact>(&bytes);
                compact_announce.is_ok()
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp4_connections_handled_in_statistics() {
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp4_connections_handled, 1);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp6_connections_handled_in_statistics() {
                let http_tracker_server = start_ipv6_http_tracker(Version::Warp).await;

                Client::bind(http_tracker_server.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_connections_handled, 1);
            }

            #[tokio::test]
            async fn should_not_increase_the_number_of_tcp6_connections_handled_if_the_client_is_not_using_an_ipv6_ip() {
                // The tracker ignores the peer address in the request param. It uses the client remote ip address.

                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
                            .with_peer_addr(&IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
                            .query(),
                    )
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_connections_handled, 0);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp4_announce_requests_handled_in_statistics() {
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp4_announces_handled, 1);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp6_announce_requests_handled_in_statistics() {
                let http_tracker_server = start_ipv6_http_tracker(Version::Warp).await;

                Client::bind(http_tracker_server.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_announces_handled, 1);
            }

            #[tokio::test]
            async fn should_not_increase_the_number_of_tcp6_announce_requests_handled_if_the_client_is_not_using_an_ipv6_ip() {
                // The tracker ignores the peer address in the request param. It uses the client remote ip address.

                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
                            .with_peer_addr(&IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
                            .query(),
                    )
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_announces_handled, 0);
            }

            #[tokio::test]
            async fn should_assign_to_the_peer_ip_the_remote_client_ip_instead_of_the_peer_address_in_the_request_param() {
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let client_ip = local_ip().unwrap();

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = QueryBuilder::default()
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

                let http_tracker_server =
                    start_http_tracker_with_external_ip(&IpAddr::from_str("2.137.87.41").unwrap(), Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
                let client_ip = loopback_ip;

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = QueryBuilder::default()
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

                let http_tracker_server = start_http_tracker_with_external_ip(
                    &IpAddr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap(),
                    Version::Warp,
                )
                .await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
                let client_ip = loopback_ip;

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = QueryBuilder::default()
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

                let http_tracker_server = start_http_tracker_on_reverse_proxy(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let client = Client::new(http_tracker_server.get_connection_info());

                let announce_query = QueryBuilder::default().with_info_hash(&info_hash).query();

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

            // Scrape documentation:
            //
            // BEP 48. Tracker Protocol Extension: Scrape
            // https://www.bittorrent.org/beps/bep_0048.html
            //
            // Vuze (bittorrent client) docs:
            // https://wiki.vuze.com/w/Scrape

            use std::net::IpAddr;
            use std::str::FromStr;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::{invalid_info_hashes, PeerBuilder};
            use crate::http::asserts::{assert_internal_server_error_response, assert_scrape_response};
            use crate::http::client::Client;
            use crate::http::requests;
            use crate::http::requests::scrape::QueryBuilder;
            use crate::http::responses::scrape::{self, File, ResponseBuilder};
            use crate::http::server::{start_ipv6_http_tracker, start_public_http_tracker};

            #[tokio::test]
            async fn should_fail_when_the_request_is_empty() {
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;
                let response = Client::new(http_tracker_server.get_connection_info()).get("scrape").await;

                assert_internal_server_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_info_hash_param_is_invalid() {
                let http_tracker_server = start_public_http_tracker(Version::Warp).await;

                let mut params = QueryBuilder::default().query().params();

                for invalid_value in &invalid_info_hashes() {
                    params.set_one_info_hash_param(invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    // code-review: it's not returning the invalid info hash error
                    assert_internal_server_error_response(response).await;
                }
            }

            #[tokio::test]
            async fn should_return_the_file_with_the_incomplete_peer_when_there_is_one_peer_with_bytes_pending_to_download() {
                let http_tracker = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 0,
                            downloaded: 0,
                            incomplete: 1,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            #[tokio::test]
            async fn should_return_the_file_with_the_complete_peer_when_there_is_one_peer_with_no_bytes_pending_to_download() {
                let http_tracker = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_no_bytes_pending_to_download()
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 1,
                            downloaded: 0,
                            incomplete: 0,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            #[tokio::test]
            async fn should_return_a_file_with_zeroed_values_when_there_are_no_peers() {
                let http_tracker = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                assert_scrape_response(response, &scrape::Response::with_one_file(info_hash.bytes(), File::zeroed())).await;
            }

            #[tokio::test]
            async fn should_accept_multiple_infohashes() {
                let http_tracker = start_public_http_tracker(Version::Warp).await;

                let info_hash1 = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let info_hash2 = InfoHash::from_str("3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0").unwrap();

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .add_info_hash(&info_hash1)
                            .add_info_hash(&info_hash2)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(info_hash1.bytes(), File::zeroed())
                    .add_file(info_hash2.bytes(), File::zeroed())
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            #[tokio::test]
            async fn should_increase_the_number_ot_tcp4_scrape_requests_handled_in_statistics() {
                let http_tracker = start_public_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let stats = http_tracker.tracker.get_stats().await;

                assert_eq!(stats.tcp4_scrapes_handled, 1);
            }

            #[tokio::test]
            async fn should_increase_the_number_ot_tcp6_scrape_requests_handled_in_statistics() {
                let http_tracker = start_ipv6_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                Client::bind(http_tracker.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let stats = http_tracker.tracker.get_stats().await;

                assert_eq!(stats.tcp6_scrapes_handled, 1);
            }
        }
    }

    mod configured_as_whitelisted {

        mod and_receiving_an_announce_request {
            use std::str::FromStr;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;

            use crate::http::asserts::{assert_is_announce_response, assert_torrent_not_in_whitelist_error_response};
            use crate::http::client::Client;
            use crate::http::requests::announce::QueryBuilder;
            use crate::http::server::start_whitelisted_http_tracker;

            #[tokio::test]
            async fn should_fail_if_the_torrent_is_not_in_the_whitelist() {
                let http_tracker_server = start_whitelisted_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().with_info_hash(&info_hash).query())
                    .await;

                assert_torrent_not_in_whitelist_error_response(response).await;
            }

            #[tokio::test]

            async fn should_allow_announcing_a_whitelisted_torrent() {
                let http_tracker_server = start_whitelisted_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker_server
                    .tracker
                    .add_torrent_to_whitelist(&info_hash)
                    .await
                    .expect("should add the torrent to the whitelist");

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().with_info_hash(&info_hash).query())
                    .await;

                assert_is_announce_response(response).await;
            }
        }

        mod receiving_an_scrape_request {
            use std::str::FromStr;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::PeerBuilder;
            use crate::http::asserts::assert_scrape_response;
            use crate::http::client::Client;
            use crate::http::requests;
            use crate::http::responses::scrape::{File, ResponseBuilder};
            use crate::http::server::start_whitelisted_http_tracker;

            #[tokio::test]
            async fn should_return_the_zeroed_file_when_the_requested_file_is_not_whitelisted() {
                let http_tracker = start_whitelisted_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default().add_file(info_hash.bytes(), File::zeroed()).build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            #[tokio::test]
            async fn should_return_the_file_stats_when_the_requested_file_is_whitelisted() {
                let http_tracker = start_whitelisted_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                http_tracker
                    .tracker
                    .add_torrent_to_whitelist(&info_hash)
                    .await
                    .expect("should add the torrent to the whitelist");

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 0,
                            downloaded: 0,
                            incomplete: 1,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }
        }
    }

    mod configured_as_private {

        mod and_receiving_an_announce_request {
            use std::str::FromStr;
            use std::time::Duration;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::auth::KeyId;

            use crate::http::asserts::{
                assert_invalid_authentication_key_error_response, assert_is_announce_response,
                assert_peer_not_authenticated_error_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::announce::QueryBuilder;
            use crate::http::server::start_private_http_tracker;

            #[tokio::test]
            async fn should_respond_to_authenticated_peers() {
                let http_tracker_server = start_private_http_tracker(Version::Warp).await;

                let key = http_tracker_server
                    .tracker
                    .generate_auth_key(Duration::from_secs(60))
                    .await
                    .unwrap();

                let response = Client::authenticated(http_tracker_server.get_connection_info(), key.id())
                    .announce(&QueryBuilder::default().query())
                    .await;

                assert_is_announce_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_if_the_peer_has_not_provided_the_authentication_key() {
                let http_tracker_server = start_private_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().with_info_hash(&info_hash).query())
                    .await;

                assert_peer_not_authenticated_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_if_the_peer_authentication_key_is_not_valid() {
                let http_tracker_server = start_private_http_tracker(Version::Warp).await;

                // The tracker does not have this key
                let unregistered_key_id = KeyId::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();

                let response = Client::authenticated(http_tracker_server.get_connection_info(), unregistered_key_id)
                    .announce(&QueryBuilder::default().query())
                    .await;

                assert_invalid_authentication_key_error_response(response).await;
            }
        }

        mod receiving_an_scrape_request {

            use std::str::FromStr;
            use std::time::Duration;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::auth::KeyId;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::PeerBuilder;
            use crate::http::asserts::assert_scrape_response;
            use crate::http::client::Client;
            use crate::http::requests;
            use crate::http::responses::scrape::{File, ResponseBuilder};
            use crate::http::server::start_private_http_tracker;

            #[tokio::test]
            async fn should_return_the_zeroed_file_when_the_client_is_not_authenticated() {
                let http_tracker = start_private_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default().add_file(info_hash.bytes(), File::zeroed()).build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            #[tokio::test]
            async fn should_return_the_real_file_stats_when_the_client_is_authenticated() {
                let http_tracker = start_private_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let key = http_tracker.tracker.generate_auth_key(Duration::from_secs(60)).await.unwrap();

                let response = Client::authenticated(http_tracker.get_connection_info(), key.id())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 0,
                            downloaded: 0,
                            incomplete: 1,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            #[tokio::test]
            async fn should_return_the_zeroed_file_when_the_authentication_key_provided_by_the_client_is_invalid() {
                // There is not authentication error

                let http_tracker = start_private_http_tracker(Version::Warp).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let false_key_id: KeyId = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ".parse().unwrap();

                let response = Client::authenticated(http_tracker.get_connection_info(), false_key_id)
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default().add_file(info_hash.bytes(), File::zeroed()).build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }
        }
    }

    mod configured_as_private_and_whitelisted {

        mod and_receiving_an_announce_request {}

        mod receiving_an_scrape_request {}
    }
}

mod axum_http_tracker_server {

    // WIP: migration HTTP from Warp to Axum

    use local_ip_address::local_ip;
    use torrust_tracker::http::axum_implementation::resources::ok::Ok;
    use torrust_tracker::http::Version;

    use crate::http::client::Client;
    use crate::http::server::start_default_http_tracker;

    #[tokio::test]
    async fn should_return_the_status() {
        // This is a temporary test to test the new Axum HTTP tracker server scaffolding

        let http_tracker_server = start_default_http_tracker(Version::Axum).await;

        let client_ip = local_ip().unwrap();

        let response = Client::bind(http_tracker_server.get_connection_info(), client_ip)
            .get("status")
            .await;

        let ok: Ok = serde_json::from_str(&response.text().await.unwrap()).unwrap();

        assert_eq!(
            ok,
            Ok {
                remote_client_insecure_ip: client_ip,
                remote_client_secure_ip: client_ip
            }
        );
    }

    mod should_get_the_remote_client_ip_from_the_http_request {

        // Temporary tests to test that the new Axum HTTP tracker gets the right remote client IP.
        // Once the implementation is finished, test for announce request will cover these cases.

        use std::net::IpAddr;
        use std::str::FromStr;

        use local_ip_address::local_ip;
        use torrust_tracker::http::axum_implementation::resources::ok::Ok;
        use torrust_tracker::http::Version;

        use crate::http::client::Client;
        use crate::http::server::{start_http_tracker_on_reverse_proxy, start_public_http_tracker};

        #[tokio::test]
        async fn when_the_client_ip_is_a_local_ip_it_should_assign_that_ip() {
            let http_tracker_server = start_public_http_tracker(Version::Axum).await;

            let client_ip = local_ip().unwrap();

            let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

            let response = client.get("status").await;

            let ok: Ok = serde_json::from_str(&response.text().await.unwrap()).unwrap();

            assert_eq!(
                ok,
                Ok {
                    remote_client_insecure_ip: client_ip,
                    remote_client_secure_ip: client_ip
                }
            );
        }

        #[tokio::test]
        async fn when_the_client_ip_is_a_loopback_ipv4_it_should_assign_that_ip() {
            let http_tracker_server = start_public_http_tracker(Version::Axum).await;

            let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
            let client_ip = loopback_ip;

            let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

            let response = client.get("status").await;

            let ok: Ok = serde_json::from_str(&response.text().await.unwrap()).unwrap();

            assert_eq!(
                ok,
                Ok {
                    remote_client_insecure_ip: client_ip,
                    remote_client_secure_ip: client_ip
                }
            );
        }

        #[tokio::test]
        async fn when_the_tracker_is_behind_a_reverse_proxy_it_should_assign_as_secure_ip_the_right_most_ip_in_the_x_forwarded_for_http_header(
        ) {
            /*
            client          <-> http proxy                       <-> tracker                   <-> Internet
            ip:                 header:                              config:                       remote client ip:
            145.254.214.256     X-Forwarded-For = 145.254.214.256    on_reverse_proxy = true       145.254.214.256
            */

            let http_tracker_server = start_http_tracker_on_reverse_proxy(Version::Axum).await;

            let client = Client::new(http_tracker_server.get_connection_info());

            let left_most_ip = IpAddr::from_str("203.0.113.195").unwrap();
            let right_most_ip = IpAddr::from_str("150.172.238.178").unwrap();

            let response = client
                .get_with_header(
                    "status",
                    "X-Forwarded-For",
                    &format!("{left_most_ip},2001:db8:85a3:8d3:1319:8a2e:370:7348,{right_most_ip}"),
                )
                .await;

            let ok: Ok = serde_json::from_str(&response.text().await.unwrap()).unwrap();

            assert_eq!(
                ok,
                Ok {
                    remote_client_insecure_ip: left_most_ip,
                    remote_client_secure_ip: right_most_ip
                }
            );
        }
    }

    mod for_all_config_modes {

        mod and_running_on_reverse_proxy {
            use torrust_tracker::http::Version;

            use crate::http::asserts::{
                assert_could_not_find_remote_address_on_xff_header_error_response,
                assert_invalid_remote_address_on_xff_header_error_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::announce::QueryBuilder;
            use crate::http::server::start_http_tracker_on_reverse_proxy;

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_when_the_http_request_does_not_include_the_xff_http_request_header() {
                // If the tracker is running behind a reverse proxy, the peer IP is the
                // last IP in the `X-Forwarded-For` HTTP header, which is the IP of the proxy client.

                let http_tracker_server = start_http_tracker_on_reverse_proxy(Version::Axum).await;

                let params = QueryBuilder::default().query().params();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_could_not_find_remote_address_on_xff_header_error_response(response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_when_the_xff_http_request_header_contains_an_invalid_ip() {
                let http_tracker_server = start_http_tracker_on_reverse_proxy(Version::Axum).await;

                let params = QueryBuilder::default().query().params();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get_with_header(&format!("announce?{params}"), "X-Forwarded-For", "INVALID IP")
                    .await;

                assert_invalid_remote_address_on_xff_header_error_response(response).await;
            }
        }

        mod receiving_an_announce_request {

            // Announce request documentation:
            //
            // BEP 03. The BitTorrent Protocol Specification
            // https://www.bittorrent.org/beps/bep_0003.html
            //
            // BEP 23. Tracker Returns Compact Peer Lists
            // https://www.bittorrent.org/beps/bep_0023.html
            //
            // Vuze (bittorrent client) docs:
            // https://wiki.vuze.com/w/Announce

            use std::net::{IpAddr, Ipv6Addr};
            use std::str::FromStr;

            use local_ip_address::local_ip;
            use reqwest::Response;
            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::{invalid_info_hashes, PeerBuilder};
            use crate::http::asserts::{
                assert_announce_response, assert_bad_announce_request_error_response,
                assert_cannot_parse_query_param_error_response, assert_cannot_parse_query_params_error_response,
                assert_compact_announce_response, assert_empty_announce_response, assert_internal_server_error_response,
                assert_is_announce_response, assert_missing_query_params_for_announce_request_error_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::announce::{Compact, QueryBuilder};
            use crate::http::responses;
            use crate::http::responses::announce::{Announce, CompactPeer, CompactPeerList, DictionaryPeer};
            use crate::http::server::{
                start_default_http_tracker, start_http_tracker_on_reverse_proxy, start_http_tracker_with_external_ip,
                start_ipv6_http_tracker, start_public_http_tracker,
            };

            #[tokio::test]
            async fn should_respond_if_only_the_mandatory_fields_are_provided() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

                params.remove_optional_params();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_is_announce_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_the_url_query_component_is_empty() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let response = Client::new(http_tracker_server.get_connection_info()).get("announce").await;

                assert_missing_query_params_for_announce_request_error_response(response).await;
            }

            #[tokio::test]
            async fn should_fail_when_url_query_parameters_are_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let invalid_query_param = "a=b=c";

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{invalid_query_param}"))
                    .await;

                assert_cannot_parse_query_param_error_response(response, "invalid param a=b=c").await;
            }

            #[tokio::test]
            async fn should_fail_when_a_mandatory_field_is_missing() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                // Without `info_hash` param

                let mut params = QueryBuilder::default().query().params();

                params.info_hash = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_bad_announce_request_error_response(response, "missing info_hash param").await;

                // Without `peer_id` param

                let mut params = QueryBuilder::default().query().params();

                params.peer_id = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_bad_announce_request_error_response(response, "missing peer_id param").await;

                // Without `port` param

                let mut params = QueryBuilder::default().query().params();

                params.port = None;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_bad_announce_request_error_response(response, "missing port param").await;
            }

            #[tokio::test]
            async fn should_fail_when_the_info_hash_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

                for invalid_value in &invalid_info_hashes() {
                    params.set("info_hash", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_cannot_parse_query_params_error_response(response, "").await;
                }
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_not_fail_when_the_peer_address_param_is_invalid() {
                // AnnounceQuery does not even contain the `peer_addr`
                // The peer IP is obtained in two ways:
                // 1. If tracker is NOT running `on_reverse_proxy` from the remote client IP if there.
                // 2. If tracker is     running `on_reverse_proxy` from `X-Forwarded-For` request header is tracker is running `on_reverse_proxy`.

                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

                params.peer_addr = Some("INVALID-IP-ADDRESS".to_string());

                let response = Client::new(http_tracker_server.get_connection_info())
                    .get(&format!("announce?{params}"))
                    .await;

                assert_is_announce_response(response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_when_the_downloaded_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("downloaded", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_internal_server_error_response(response).await;
                }
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_when_the_uploaded_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

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

                    assert_cannot_parse_query_params_error_response(response, "").await;
                }
            }

            #[tokio::test]
            async fn should_fail_when_the_port_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("port", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_cannot_parse_query_params_error_response(response, "").await;
                }
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_when_the_left_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

                let invalid_values = ["-1", "1.1", "a"];

                for invalid_value in invalid_values {
                    params.set("left", invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    assert_internal_server_error_response(response).await;
                }
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_not_fail_when_the_event_param_is_invalid() {
                // All invalid values are ignored as if the `event` param were empty

                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

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

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_not_fail_when_the_compact_param_is_invalid() {
                let http_tracker_server = start_default_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

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
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
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
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .build();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2. This new peer is non included on the response peer list
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
                            .with_info_hash(&info_hash)
                            .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                            .query(),
                    )
                    .await;

                // It should only contain the previously announced peer
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
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let peer = PeerBuilder::default().build();

                // Add a peer
                http_tracker_server.add_torrent(&info_hash, &peer).await;

                let announce_query = QueryBuilder::default()
                    .with_info_hash(&info_hash)
                    .with_peer_id(&peer.peer_id)
                    .query();

                assert_ne!(peer.peer_addr.ip(), announce_query.peer_addr);

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&announce_query)
                    .await;

                assert_empty_announce_response(response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_compact_response() {
                // Tracker Returns Compact Peer Lists
                // https://www.bittorrent.org/beps/bep_0023.html

                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .build();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2 accepting compact responses
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
                            .with_info_hash(&info_hash)
                            .with_peer_id(&peer::Id(*b"-qB00000000000000002"))
                            .with_compact(Compact::Accepted)
                            .query(),
                    )
                    .await;

                let expected_response = responses::announce::Compact {
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

                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                // Peer 1
                let previously_announced_peer = PeerBuilder::default()
                    .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                    .build();

                // Add the Peer 1
                http_tracker_server.add_torrent(&info_hash, &previously_announced_peer).await;

                // Announce the new Peer 2 without passing the "compact" param
                // By default it should respond with the compact peer list
                // https://www.bittorrent.org/beps/bep_0023.html
                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
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
                let compact_announce = serde_bencode::from_bytes::<responses::announce::DeserializedCompact>(&bytes);
                compact_announce.is_ok()
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp4_connections_handled_in_statistics() {
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp4_connections_handled, 1);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp6_connections_handled_in_statistics() {
                let http_tracker_server = start_ipv6_http_tracker(Version::Axum).await;

                Client::bind(http_tracker_server.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_connections_handled, 1);
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_not_increase_the_number_of_tcp6_connections_handled_if_the_client_is_not_using_an_ipv6_ip() {
                // The tracker ignores the peer address in the request param. It uses the client remote ip address.

                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
                            .with_peer_addr(&IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
                            .query(),
                    )
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_connections_handled, 0);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp4_announce_requests_handled_in_statistics() {
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp4_announces_handled, 1);
            }

            #[tokio::test]
            async fn should_increase_the_number_of_tcp6_announce_requests_handled_in_statistics() {
                let http_tracker_server = start_ipv6_http_tracker(Version::Axum).await;

                Client::bind(http_tracker_server.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .announce(&QueryBuilder::default().query())
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_announces_handled, 1);
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_not_increase_the_number_of_tcp6_announce_requests_handled_if_the_client_is_not_using_an_ipv6_ip() {
                // The tracker ignores the peer address in the request param. It uses the client remote ip address.

                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                Client::new(http_tracker_server.get_connection_info())
                    .announce(
                        &QueryBuilder::default()
                            .with_peer_addr(&IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
                            .query(),
                    )
                    .await;

                let stats = http_tracker_server.tracker.get_stats().await;

                assert_eq!(stats.tcp6_announces_handled, 0);
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_assign_to_the_peer_ip_the_remote_client_ip_instead_of_the_peer_address_in_the_request_param() {
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let client_ip = local_ip().unwrap();

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = QueryBuilder::default()
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

                let http_tracker_server =
                    start_http_tracker_with_external_ip(&IpAddr::from_str("2.137.87.41").unwrap(), Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
                let client_ip = loopback_ip;

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = QueryBuilder::default()
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

                let http_tracker_server = start_http_tracker_with_external_ip(
                    &IpAddr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap(),
                    Version::Axum,
                )
                .await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
                let client_ip = loopback_ip;

                let client = Client::bind(http_tracker_server.get_connection_info(), client_ip);

                let announce_query = QueryBuilder::default()
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

                let http_tracker_server = start_http_tracker_on_reverse_proxy(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let client = Client::new(http_tracker_server.get_connection_info());

                let announce_query = QueryBuilder::default().with_info_hash(&info_hash).query();

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

            // Scrape documentation:
            //
            // BEP 48. Tracker Protocol Extension: Scrape
            // https://www.bittorrent.org/beps/bep_0048.html
            //
            // Vuze (bittorrent client) docs:
            // https://wiki.vuze.com/w/Scrape

            use std::net::IpAddr;
            use std::str::FromStr;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::{invalid_info_hashes, PeerBuilder};
            use crate::http::asserts::{assert_internal_server_error_response, assert_scrape_response};
            use crate::http::client::Client;
            use crate::http::requests;
            use crate::http::requests::scrape::QueryBuilder;
            use crate::http::responses::scrape::{self, File, ResponseBuilder};
            use crate::http::server::{start_ipv6_http_tracker, start_public_http_tracker};

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_when_the_request_is_empty() {
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;
                let response = Client::new(http_tracker_server.get_connection_info()).get("scrape").await;

                assert_internal_server_error_response(response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_when_the_info_hash_param_is_invalid() {
                let http_tracker_server = start_public_http_tracker(Version::Axum).await;

                let mut params = QueryBuilder::default().query().params();

                for invalid_value in &invalid_info_hashes() {
                    params.set_one_info_hash_param(invalid_value);

                    let response = Client::new(http_tracker_server.get_connection_info())
                        .get(&format!("announce?{params}"))
                        .await;

                    // code-review: it's not returning the invalid info hash error
                    assert_internal_server_error_response(response).await;
                }
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_file_with_the_incomplete_peer_when_there_is_one_peer_with_bytes_pending_to_download() {
                let http_tracker = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 0,
                            downloaded: 0,
                            incomplete: 1,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_file_with_the_complete_peer_when_there_is_one_peer_with_no_bytes_pending_to_download() {
                let http_tracker = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_no_bytes_pending_to_download()
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 1,
                            downloaded: 0,
                            incomplete: 0,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_a_file_with_zeroed_values_when_there_are_no_peers() {
                let http_tracker = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                assert_scrape_response(response, &scrape::Response::with_one_file(info_hash.bytes(), File::zeroed())).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_accept_multiple_infohashes() {
                let http_tracker = start_public_http_tracker(Version::Axum).await;

                let info_hash1 = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
                let info_hash2 = InfoHash::from_str("3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0").unwrap();

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .add_info_hash(&info_hash1)
                            .add_info_hash(&info_hash2)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(info_hash1.bytes(), File::zeroed())
                    .add_file(info_hash2.bytes(), File::zeroed())
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_increase_the_number_ot_tcp4_scrape_requests_handled_in_statistics() {
                let http_tracker = start_public_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let stats = http_tracker.tracker.get_stats().await;

                assert_eq!(stats.tcp4_scrapes_handled, 1);
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_increase_the_number_ot_tcp6_scrape_requests_handled_in_statistics() {
                let http_tracker = start_ipv6_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                Client::bind(http_tracker.get_connection_info(), IpAddr::from_str("::1").unwrap())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let stats = http_tracker.tracker.get_stats().await;

                assert_eq!(stats.tcp6_scrapes_handled, 1);
            }
        }
    }

    mod configured_as_whitelisted {

        mod and_receiving_an_announce_request {
            use std::str::FromStr;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;

            use crate::http::asserts::{assert_is_announce_response, assert_torrent_not_in_whitelist_error_response};
            use crate::http::client::Client;
            use crate::http::requests::announce::QueryBuilder;
            use crate::http::server::start_whitelisted_http_tracker;

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_if_the_torrent_is_not_in_the_whitelist() {
                let http_tracker_server = start_whitelisted_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().with_info_hash(&info_hash).query())
                    .await;

                assert_torrent_not_in_whitelist_error_response(response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_allow_announcing_a_whitelisted_torrent() {
                let http_tracker_server = start_whitelisted_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker_server
                    .tracker
                    .add_torrent_to_whitelist(&info_hash)
                    .await
                    .expect("should add the torrent to the whitelist");

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().with_info_hash(&info_hash).query())
                    .await;

                assert_is_announce_response(response).await;
            }
        }

        mod receiving_an_scrape_request {
            use std::str::FromStr;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::PeerBuilder;
            use crate::http::asserts::assert_scrape_response;
            use crate::http::client::Client;
            use crate::http::requests;
            use crate::http::responses::scrape::{File, ResponseBuilder};
            use crate::http::server::start_whitelisted_http_tracker;

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_zeroed_file_when_the_requested_file_is_not_whitelisted() {
                let http_tracker = start_whitelisted_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default().add_file(info_hash.bytes(), File::zeroed()).build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_file_stats_when_the_requested_file_is_whitelisted() {
                let http_tracker = start_whitelisted_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                http_tracker
                    .tracker
                    .add_torrent_to_whitelist(&info_hash)
                    .await
                    .expect("should add the torrent to the whitelist");

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 0,
                            downloaded: 0,
                            incomplete: 1,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }
        }
    }

    mod configured_as_private {

        mod and_receiving_an_announce_request {
            use std::str::FromStr;
            use std::time::Duration;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::auth::KeyId;

            use crate::http::asserts::{
                assert_invalid_authentication_key_error_response, assert_is_announce_response,
                assert_peer_not_authenticated_error_response,
            };
            use crate::http::client::Client;
            use crate::http::requests::announce::QueryBuilder;
            use crate::http::server::start_private_http_tracker;

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_respond_to_authenticated_peers() {
                let http_tracker_server = start_private_http_tracker(Version::Axum).await;

                let key = http_tracker_server
                    .tracker
                    .generate_auth_key(Duration::from_secs(60))
                    .await
                    .unwrap();

                let response = Client::authenticated(http_tracker_server.get_connection_info(), key.id())
                    .announce(&QueryBuilder::default().query())
                    .await;

                assert_is_announce_response(response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_if_the_peer_has_not_provided_the_authentication_key() {
                let http_tracker_server = start_private_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                let response = Client::new(http_tracker_server.get_connection_info())
                    .announce(&QueryBuilder::default().with_info_hash(&info_hash).query())
                    .await;

                assert_peer_not_authenticated_error_response(response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_fail_if_the_peer_authentication_key_is_not_valid() {
                let http_tracker_server = start_private_http_tracker(Version::Axum).await;

                // The tracker does not have this key
                let unregistered_key_id = KeyId::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();

                let response = Client::authenticated(http_tracker_server.get_connection_info(), unregistered_key_id)
                    .announce(&QueryBuilder::default().query())
                    .await;

                assert_invalid_authentication_key_error_response(response).await;
            }
        }

        mod receiving_an_scrape_request {

            use std::str::FromStr;
            use std::time::Duration;

            use torrust_tracker::http::Version;
            use torrust_tracker::protocol::info_hash::InfoHash;
            use torrust_tracker::tracker::auth::KeyId;
            use torrust_tracker::tracker::peer;

            use crate::common::fixtures::PeerBuilder;
            use crate::http::asserts::assert_scrape_response;
            use crate::http::client::Client;
            use crate::http::requests;
            use crate::http::responses::scrape::{File, ResponseBuilder};
            use crate::http::server::start_private_http_tracker;

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_zeroed_file_when_the_client_is_not_authenticated() {
                let http_tracker = start_private_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let response = Client::new(http_tracker.get_connection_info())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default().add_file(info_hash.bytes(), File::zeroed()).build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_real_file_stats_when_the_client_is_authenticated() {
                let http_tracker = start_private_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let key = http_tracker.tracker.generate_auth_key(Duration::from_secs(60)).await.unwrap();

                let response = Client::authenticated(http_tracker.get_connection_info(), key.id())
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default()
                    .add_file(
                        info_hash.bytes(),
                        File {
                            complete: 0,
                            downloaded: 0,
                            incomplete: 1,
                        },
                    )
                    .build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }

            //#[tokio::test]
            #[allow(dead_code)]
            async fn should_return_the_zeroed_file_when_the_authentication_key_provided_by_the_client_is_invalid() {
                // There is not authentication error

                let http_tracker = start_private_http_tracker(Version::Axum).await;

                let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();

                http_tracker
                    .add_torrent(
                        &info_hash,
                        &PeerBuilder::default()
                            .with_peer_id(&peer::Id(*b"-qB00000000000000001"))
                            .with_bytes_pending_to_download(1)
                            .build(),
                    )
                    .await;

                let false_key_id: KeyId = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ".parse().unwrap();

                let response = Client::authenticated(http_tracker.get_connection_info(), false_key_id)
                    .scrape(
                        &requests::scrape::QueryBuilder::default()
                            .with_one_info_hash(&info_hash)
                            .query(),
                    )
                    .await;

                let expected_scrape_response = ResponseBuilder::default().add_file(info_hash.bytes(), File::zeroed()).build();

                assert_scrape_response(response, &expected_scrape_response).await;
            }
        }
    }

    mod configured_as_private_and_whitelisted {

        mod and_receiving_an_announce_request {}

        mod receiving_an_scrape_request {}
    }
}
