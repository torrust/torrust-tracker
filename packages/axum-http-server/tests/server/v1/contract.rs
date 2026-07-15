use std::sync::Arc;

use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_test_helpers::{configuration, logging};

#[tokio::test]
async fn environment_should_be_started_and_stopped() {
    logging::setup();

    let cfg = configuration::ephemeral();
    let core_config = Arc::new(cfg.core.clone());
    let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
    let env = Started::new(&core_config, &http_tracker_config).await;

    env.stop().await;
}

mod for_all_config_modes {

    use std::sync::Arc;
    use std::time::Duration;

    use torrust_tracker_axum_http_server::testing::environment::Started;
    use torrust_tracker_axum_http_server::v1::handlers::health_check::{Report, Status};
    use torrust_tracker_client::http::client::Client;
    use torrust_tracker_test_helpers::{configuration, logging};

    #[tokio::test]
    async fn health_check_endpoint_should_return_ok_if_the_http_tracker_is_running() {
        logging::setup();

        let cfg = configuration::ephemeral_with_reverse_proxy();
        let core_config = Arc::new(cfg.core.clone());
        let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
        let env = Started::new(&core_config, &http_tracker_config).await;

        let response = Client::new(env.base_url(), Duration::from_secs(5))
            .unwrap()
            .health_check()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
        assert_eq!(response.json::<Report>().await.unwrap(), Report { status: Status::Ok });

        env.stop().await;
    }

    mod and_running_on_reverse_proxy {
        use std::sync::Arc;
        use std::time::Duration;

        use torrust_tracker_axum_http_server::testing::environment::Started;
        use torrust_tracker_client::http::client::Client;
        use torrust_tracker_http_protocol::v1::requests::announce::AnnounceBuilder;
        use torrust_tracker_test_helpers::{configuration, logging};

        use crate::server::asserts::assert_could_not_find_remote_address_on_x_forwarded_for_header_error_response;

        #[tokio::test]
        async fn should_fail_when_the_http_request_does_not_include_the_xff_http_request_header() {
            logging::setup();

            // If the tracker is running behind a reverse proxy, the peer IP is the
            // right most IP in the `X-Forwarded-For` HTTP header, which is the IP of the proxy's client.

            let cfg = configuration::ephemeral_with_reverse_proxy();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let params = AnnounceBuilder::default().query().to_string();

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&format!("announce?{params}"))
                .await
                .unwrap();

            assert_could_not_find_remote_address_on_x_forwarded_for_header_error_response(response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_xff_http_request_header_contains_an_invalid_ip() {
            logging::setup();

            let cfg = configuration::ephemeral_with_reverse_proxy();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let params = AnnounceBuilder::default().query().to_string();

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get_with_header(&format!("announce?{params}"), "X-Forwarded-For", "INVALID IP")
                .await
                .unwrap();

            assert_could_not_find_remote_address_on_x_forwarded_for_header_error_response(response).await;

            env.stop().await;
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

        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6};
        use std::str::FromStr;
        use std::sync::Arc;
        use std::time::Duration;

        use local_ip_address::local_ip;
        use reqwest::{Response, StatusCode};
        use tokio::net::TcpListener;
        use torrust_info_hash::InfoHash;
        use torrust_peer_id::PeerId;
        use torrust_tracker_axum_http_server::testing::environment::Started;
        use torrust_tracker_client::http::client::Client;
        use torrust_tracker_http_protocol::percent_encoding::percent_encode_byte_array;
        use torrust_tracker_http_protocol::v1::requests::announce::{AnnounceBuilder, Compact};
        use torrust_tracker_http_protocol::v1::responses::announce::deserialization::{
            CompactPeer, CompactPeerList, DeserializedNormal, DictionaryPeer,
        };
        use torrust_tracker_primitives::PeerId as DomainPeerId;
        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_test_helpers::{configuration, logging};

        use crate::common::fixtures::invalid_info_hashes;
        use crate::server::asserts::{
            assert_announce_response, assert_bad_announce_request_error_response, assert_cannot_parse_query_param_error_response,
            assert_cannot_parse_query_params_error_response, assert_compact_announce_response, assert_is_announce_response,
            assert_missing_query_params_for_announce_request_error_response,
        };

        #[tokio::test]
        async fn it_should_start_and_stop() {
            logging::setup();

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;
            env.stop().await;
        }

        #[tokio::test]
        async fn should_respond_if_only_the_mandatory_fields_are_provided() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            // Build a URL with only mandatory fields (info_hash, peer_id, port)
            let params = format!(
                "info_hash={}&peer_id={}&port={}",
                percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes()),
                percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0),
                AnnounceBuilder::default().query().port,
            );

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&format!("announce?{params}"))
                .await
                .unwrap();

            assert_is_announce_response(response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_url_query_component_is_empty() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get("announce")
                .await
                .unwrap();

            assert_missing_query_params_for_announce_request_error_response(response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_url_query_parameters_are_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let invalid_query_param = "a=b=c";

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&format!("announce?{invalid_query_param}"))
                .await
                .unwrap();

            assert_cannot_parse_query_param_error_response(response, "invalid param a=b=c").await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_a_mandatory_field_is_missing() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            // Without `info_hash` param
            let params = format!(
                "peer_id={}&port={}",
                percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0),
                AnnounceBuilder::default().query().port,
            );

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&format!("announce?{params}"))
                .await
                .unwrap();

            assert_bad_announce_request_error_response(response, "missing param info_hash").await;

            // Without `peer_id` param
            let params = format!(
                "info_hash={}&port={}",
                percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes()),
                AnnounceBuilder::default().query().port,
            );

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&format!("announce?{params}"))
                .await
                .unwrap();

            assert_bad_announce_request_error_response(response, "missing param peer_id").await;

            // Without `port` param
            let params = format!(
                "info_hash={}&peer_id={}",
                percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes()),
                percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0),
            );

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&format!("announce?{params}"))
                .await
                .unwrap();

            assert_bad_announce_request_error_response(response, "missing param port").await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_info_hash_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            for invalid_value in &invalid_info_hashes() {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&event=started&compact=0",
                    invalid_value,
                    percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0),
                    AnnounceBuilder::default().query().port,
                    "192.168.1.88",
                );

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
        async fn should_not_fail_when_the_peer_address_param_is_invalid() {
            logging::setup();

            // AnnounceQuery does not even contain the `peer_addr`
            // The peer IP is obtained in two ways:
            // 1. If tracker is NOT running `on_reverse_proxy` from the remote client IP.
            // 2. If tracker is     running `on_reverse_proxy` from `X-Forwarded-For` request HTTP header.

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let url = format!(
                "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&event=started&compact=0",
                percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes()),
                percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0),
                AnnounceBuilder::default().query().port,
                "INVALID-IP-ADDRESS",
            );

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&url)
                .await
                .unwrap();

            assert_is_announce_response(response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_downloaded_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_peer_id = percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0);
            let default_port = AnnounceBuilder::default().query().port;

            let invalid_values = ["-1", "1.1", "a"];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&downloaded={}&event=started&compact=0",
                    default_info_hash, default_peer_id, default_port, "192.168.1.88", invalid_value,
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_uploaded_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_peer_id = percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0);
            let default_port = AnnounceBuilder::default().query().port;

            let invalid_values = ["-1", "1.1", "a"];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&uploaded={}&event=started&compact=0",
                    default_info_hash, default_peer_id, default_port, "192.168.1.88", invalid_value,
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_peer_id_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_port = AnnounceBuilder::default().query().port;

            let invalid_values = [
                "0",
                "-1",
                "1.1",
                "a",
                "-qB0000000000000000",   // 19 bytes
                "-qB000000000000000000", // 21 bytes
            ];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&event=started&compact=0",
                    default_info_hash, invalid_value, default_port, "192.168.1.88",
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_port_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_peer_id = percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0);

            let invalid_values = ["-1", "1.1", "a"];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&event=started&compact=0",
                    default_info_hash, default_peer_id, invalid_value, "192.168.1.88",
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_left_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_peer_id = percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0);
            let default_port = AnnounceBuilder::default().query().port;

            let invalid_values = ["-1", "1.1", "a"];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&left={}&event=started&compact=0",
                    default_info_hash, default_peer_id, default_port, "192.168.1.88", invalid_value,
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_event_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_peer_id = percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0);
            let default_port = AnnounceBuilder::default().query().port;

            let invalid_values = [
                "0",
                "-1",
                "1.1",
                "a",
                "Started",   // It should be lowercase to be valid: `started`
                "Stopped",   // It should be lowercase to be valid: `stopped`
                "Completed", // It should be lowercase to be valid: `completed`
            ];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&event={}&compact=0",
                    default_info_hash, default_peer_id, default_port, "192.168.1.88", invalid_value,
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_compact_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_peer_id = percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0);
            let default_port = AnnounceBuilder::default().query().port;

            let invalid_values = ["-1", "1.1", "a"];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&event=started&compact={}",
                    default_info_hash, default_peer_id, default_port, "192.168.1.88", invalid_value,
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_when_the_numwant_param_is_invalid() {
            logging::setup();

            let cfg = configuration::ephemeral();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let default_info_hash = percent_encode_byte_array(&AnnounceBuilder::default().query().info_hash.bytes());
            let default_peer_id = percent_encode_byte_array(&AnnounceBuilder::default().query().peer_id.0);
            let default_port = AnnounceBuilder::default().query().port;

            let invalid_values = ["-1", "1.1", "a"];

            for invalid_value in invalid_values {
                let url = format!(
                    "announce?info_hash={}&peer_id={}&port={}&peer_addr={}&event=started&compact=0&numwant={}",
                    default_info_hash, default_peer_id, default_port, "192.168.1.88", invalid_value,
                );

                let response = Client::new(env.base_url(), Duration::from_secs(5))
                    .unwrap()
                    .get(&url)
                    .await
                    .unwrap();

                assert_bad_announce_request_error_response(response, "invalid param value").await;
            }

            env.stop().await;
        }

        #[tokio::test]
        async fn should_return_no_peers_if_the_announced_peer_is_the_first_one() {
            logging::setup();

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(
                    &AnnounceBuilder::default()
                        .with_info_hash(&InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap()) // DevSkim: ignore DS173237
                        .query(),
                )
                .await
                .unwrap();

            let announce_policy = env.container.tracker_core_container.core_config.announce_policy;

            assert_announce_response(
                response,
                &DeserializedNormal {
                    complete: 1, // the peer for this test
                    incomplete: 0,
                    interval: announce_policy.interval,
                    min_interval: announce_policy.interval_min,
                    peers: vec![],
                },
            )
            .await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_return_the_list_of_previously_announced_peers() {
            logging::setup();

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            // Peer 1
            let previously_announced_peer = PeerBuilder::default()
                .with_peer_id(&DomainPeerId(*b"-qB00000000000000001"))
                .build();

            // Add the Peer 1
            env.add_torrent_peer(&info_hash, &previously_announced_peer).await;

            // Announce the new Peer 2. This new peer is non included on the response peer list
            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(
                    &AnnounceBuilder::default()
                        .with_info_hash(&info_hash)
                        .with_peer_id(&PeerId(*b"-qB00000000000000002"))
                        .query(),
                )
                .await
                .unwrap();

            let announce_policy = env.container.tracker_core_container.core_config.announce_policy;

            // It should only contain the previously announced peer
            assert_announce_response(
                response,
                &DeserializedNormal {
                    complete: 2,
                    incomplete: 0,
                    interval: announce_policy.interval,
                    min_interval: announce_policy.interval_min,
                    peers: vec![DictionaryPeer {
                        peer_id: previously_announced_peer.peer_id.as_bytes().to_vec(),
                        ip: previously_announced_peer.peer_addr.ip().to_string(),
                        port: previously_announced_peer.peer_addr.port(),
                    }],
                },
            )
            .await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_return_the_list_of_previously_announced_peers_including_peers_using_ipv4_and_ipv6() {
            logging::setup();

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            // Announce a peer using IPV4
            let peer_using_ipv4 = PeerBuilder::default()
                .with_peer_id(&DomainPeerId(*b"-qB00000000000000001"))
                .with_peer_addr(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0x69, 0x69, 0x69, 0x69)), 8080))
                .build();
            env.add_torrent_peer(&info_hash, &peer_using_ipv4).await;

            // Announce a peer using IPV6
            let peer_using_ipv6 = PeerBuilder::default()
                .with_peer_id(&DomainPeerId(*b"-qB00000000000000002"))
                .with_peer_addr(&SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::new(0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969, 0x6969)),
                    8080,
                ))
                .build();
            env.add_torrent_peer(&info_hash, &peer_using_ipv6).await;

            // Announce the new Peer.
            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(
                    &AnnounceBuilder::default()
                        .with_info_hash(&info_hash)
                        .with_peer_id(&PeerId(*b"-qB00000000000000003"))
                        .query(),
                )
                .await
                .unwrap();

            let announce_policy = env.container.tracker_core_container.core_config.announce_policy;

            // The newly announced peer is not included on the response peer list,
            // but all the previously announced peers should be included regardless the IP version they are using.
            assert_announce_response(
                response,
                &DeserializedNormal {
                    complete: 3,
                    incomplete: 0,
                    interval: announce_policy.interval,
                    min_interval: announce_policy.interval_min,
                    peers: vec![
                        DictionaryPeer {
                            peer_id: peer_using_ipv4.peer_id.as_bytes().to_vec(),
                            ip: peer_using_ipv4.peer_addr.ip().to_string(),
                            port: peer_using_ipv4.peer_addr.port(),
                        },
                        DictionaryPeer {
                            peer_id: peer_using_ipv6.peer_id.as_bytes().to_vec(),
                            ip: peer_using_ipv6.peer_addr.ip().to_string(),
                            port: peer_using_ipv6.peer_addr.port(),
                        },
                    ],
                },
            )
            .await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_consider_two_peers_to_be_the_same_when_they_have_the_same_socket_address_even_if_the_peer_id_is_different()
         {
            logging::setup();

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237
            let peer = PeerBuilder::default().build();

            let announce_query_1 = AnnounceBuilder::default()
                .with_info_hash(&info_hash)
                .with_peer_id(&PeerId(peer.peer_id.0))
                .with_peer_addr(peer.peer_addr.ip())
                .with_port(peer.peer_addr.port())
                .query();

            let announce_query_2 = AnnounceBuilder::default()
                .with_info_hash(&info_hash)
                .with_peer_id(&PeerId(*b"-qB00000000000000002")) // Different peer ID
                .with_peer_addr(peer.peer_addr.ip())
                .with_port(peer.peer_addr.port())
                .query();

            // Same peer socket address
            assert_eq!(announce_query_1.peer_addr, announce_query_2.peer_addr);
            assert_eq!(announce_query_1.port, announce_query_2.port);

            // Different peer ID
            assert_ne!(announce_query_1.peer_id, announce_query_2.peer_id);

            let _response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(&announce_query_1)
                .await
                .unwrap();
            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(&announce_query_2)
                .await
                .unwrap();

            let announce_policy = env.container.tracker_core_container.core_config.announce_policy;

            // The response should contain only the first peer.
            assert_announce_response(
                response,
                &DeserializedNormal {
                    complete: 1,
                    incomplete: 0,
                    interval: announce_policy.interval,
                    min_interval: announce_policy.interval_min,
                    peers: vec![],
                },
            )
            .await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_return_the_compact_response() {
            logging::setup();

            // Tracker Returns Compact Peer Lists
            // https://www.bittorrent.org/beps/bep_0023.html

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            // Peer 1
            let previously_announced_peer = PeerBuilder::default()
                .with_peer_id(&DomainPeerId(*b"-qB00000000000000001"))
                .build();

            // Add the Peer 1
            env.add_torrent_peer(&info_hash, &previously_announced_peer).await;

            // Announce the new Peer 2 accepting compact responses
            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(
                    &AnnounceBuilder::default()
                        .with_info_hash(&info_hash)
                        .with_peer_id(&PeerId(*b"-qB00000000000000002"))
                        .with_compact(Compact::Accepted)
                        .query(),
                )
                .await
                .unwrap();

            let expected_response =
                torrust_tracker_http_protocol::v1::responses::announce::deserialization::DeserializedCompactParsed {
                    complete: 2,
                    incomplete: 0,
                    interval: 120,
                    min_interval: 120,
                    peers: CompactPeerList::new([CompactPeer::new(&previously_announced_peer.peer_addr)].to_vec()),
                };

            assert_compact_announce_response(response, &expected_response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_not_return_the_compact_response_by_default() {
            logging::setup();

            // code-review: the HTTP tracker does not return the compact response by default if the "compact"
            // param is not provided in the announce URL. The BEP 23 suggest to do so.

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            // Peer 1
            let previously_announced_peer = PeerBuilder::default()
                .with_peer_id(&DomainPeerId(*b"-qB00000000000000001"))
                .build();

            // Add the Peer 1
            env.add_torrent_peer(&info_hash, &previously_announced_peer).await;

            // Announce the new Peer 2 without passing the "compact" param
            // By default it should respond with the compact peer list
            // https://www.bittorrent.org/beps/bep_0023.html
            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(
                    &AnnounceBuilder::default()
                        .with_info_hash(&info_hash)
                        .with_peer_id(&PeerId(*b"-qB00000000000000002"))
                        .without_compact()
                        .query(),
                )
                .await
                .unwrap();

            assert!(!is_a_compact_announce_response(response).await);

            env.stop().await;
        }

        async fn is_a_compact_announce_response(response: Response) -> bool {
            let bytes = response.bytes().await.unwrap();
            let compact_announce = serde_bencode::from_bytes::<
                torrust_tracker_http_protocol::v1::responses::announce::deserialization::DeserializedCompact,
            >(&bytes);
            compact_announce.is_ok()
        }

        #[tokio::test]
        async fn should_increase_the_number_of_tcp4_announce_requests_handled_in_statistics() {
            logging::setup();

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(&AnnounceBuilder::default().query())
                .await
                .unwrap();

            let stats = env.container.http_tracker_core_container.stats_repository.get_stats().await;

            assert_eq!(stats.tcp4_announces_handled(), 1);

            drop(stats);

            env.stop().await;
        }

        #[tokio::test]
        async fn should_increase_the_number_of_tcp6_announce_requests_handled_in_statistics() {
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

            Client::bind(env.base_url(), Duration::from_secs(5), IpAddr::from_str("::1").unwrap())
                .unwrap()
                .announce(&AnnounceBuilder::default().query())
                .await
                .unwrap();

            let stats = env.container.http_tracker_core_container.stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_announces_handled(), 1);

            drop(stats);

            env.stop().await;
        }

        #[tokio::test]
        async fn should_not_increase_the_number_of_tcp6_announce_requests_handled_if_the_client_is_not_using_an_ipv6_ip() {
            logging::setup();

            // The tracker ignores the peer address in the request param. It uses the client remote ip address.

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(
                    &AnnounceBuilder::default()
                        .with_peer_addr(IpAddr::V6(Ipv6Addr::LOCALHOST))
                        .query(),
                )
                .await
                .unwrap();

            let stats = env.container.http_tracker_core_container.stats_repository.get_stats().await;

            assert_eq!(stats.tcp6_announces_handled(), 0);

            drop(stats);

            env.stop().await;
        }

        #[tokio::test]
        async fn should_assign_to_the_peer_ip_the_remote_client_ip_instead_of_the_peer_address_in_the_request_param() {
            logging::setup();

            let cfg = configuration::ephemeral_public();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237
            let client_ip = local_ip().unwrap();

            let announce_query = AnnounceBuilder::default()
                .with_info_hash(&info_hash)
                .with_peer_addr(IpAddr::from_str("2.2.2.2").unwrap())
                .query();

            {
                let client = Client::bind(env.base_url(), Duration::from_secs(5), client_ip).unwrap();
                let status = client.announce(&announce_query).await.unwrap().status();

                assert_eq!(status, StatusCode::OK);
            }

            let peers = env
                .container
                .tracker_core_container
                .in_memory_torrent_repository
                .get_torrent_peers(&info_hash, usize::MAX)
                .await;
            let peer_addr = peers[0].peer_addr;

            assert_eq!(peer_addr.ip(), client_ip);
            assert_ne!(peer_addr.ip(), IpAddr::from_str("2.2.2.2").unwrap());

            env.stop().await;
        }

        #[tokio::test]
        async fn when_the_client_ip_is_a_loopback_ipv4_it_should_assign_to_the_peer_ip_the_external_ip_in_the_tracker_configuration()
         {
            logging::setup();

            /*  We assume that both the client and tracker share the same public IP.

                client     <-> tracker                      <-> Internet
                127.0.0.1      external_ip = "2.137.87.41"
            */
            let cfg = configuration::ephemeral_with_external_ip(IpAddr::from_str("2.137.87.41").unwrap());
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237
            let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
            let client_ip = loopback_ip;

            let announce_query = AnnounceBuilder::default()
                .with_info_hash(&info_hash)
                .with_peer_addr(IpAddr::from_str("2.2.2.2").unwrap())
                .query();

            {
                let client = Client::bind(env.base_url(), Duration::from_secs(5), client_ip).unwrap();
                let status = client.announce(&announce_query).await.unwrap().status();

                assert_eq!(status, StatusCode::OK);
            }

            let peers = env
                .container
                .tracker_core_container
                .in_memory_torrent_repository
                .get_torrent_peers(&info_hash, usize::MAX)
                .await;
            let peer_addr = peers[0].peer_addr;

            let ext_ip: IpAddr = env
                .container
                .tracker_core_container
                .core_config
                .net
                .external_ip
                .unwrap()
                .into();
            assert_eq!(peer_addr.ip(), ext_ip);
            assert_ne!(peer_addr.ip(), IpAddr::from_str("2.2.2.2").unwrap());

            env.stop().await;
        }

        #[tokio::test]
        async fn when_the_client_ip_is_a_loopback_ipv6_it_should_assign_to_the_peer_ip_the_external_ip_in_the_tracker_configuration()
         {
            logging::setup();

            /* We assume that both the client and tracker share the same public IP.

               client     <-> tracker                                                  <-> Internet
               ::1            external_ip = "2345:0425:2CA1:0000:0000:0567:5673:23b5"
            */

            let cfg =
                configuration::ephemeral_with_external_ip(IpAddr::from_str("2345:0425:2CA1:0000:0000:0567:5673:23b5").unwrap());
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237
            let loopback_ip = IpAddr::from_str("127.0.0.1").unwrap();
            let client_ip = loopback_ip;

            let announce_query = AnnounceBuilder::default()
                .with_info_hash(&info_hash)
                .with_peer_addr(IpAddr::from_str("2.2.2.2").unwrap())
                .query();

            {
                let client = Client::bind(env.base_url(), Duration::from_secs(5), client_ip).unwrap();
                let status = client.announce(&announce_query).await.unwrap().status();

                assert_eq!(status, StatusCode::OK);
            }

            let peers = env
                .container
                .tracker_core_container
                .in_memory_torrent_repository
                .get_torrent_peers(&info_hash, usize::MAX)
                .await;
            let peer_addr = peers[0].peer_addr;

            let ext_ip: IpAddr = env
                .container
                .tracker_core_container
                .core_config
                .net
                .external_ip
                .unwrap()
                .into();
            assert_eq!(peer_addr.ip(), ext_ip);
            assert_ne!(peer_addr.ip(), IpAddr::from_str("2.2.2.2").unwrap());

            env.stop().await;
        }

        #[tokio::test]
        async fn when_the_tracker_is_behind_a_reverse_proxy_it_should_assign_to_the_peer_ip_the_ip_in_the_x_forwarded_for_http_header()
         {
            logging::setup();

            /*
            client          <-> http proxy                       <-> tracker                   <-> Internet
            ip:                 header:                              config:                       peer addr:
            145.254.214.256     X-Forwarded-For = 145.254.214.256    on_reverse_proxy = true       145.254.214.256
            */

            let cfg = configuration::ephemeral_with_reverse_proxy();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            let announce_query = AnnounceBuilder::default().with_info_hash(&info_hash).query();

            {
                let client = Client::new(env.base_url(), Duration::from_secs(5)).unwrap();
                let status = client
                    .announce_with_header(
                        &announce_query,
                        "X-Forwarded-For",
                        "203.0.113.195,2001:db8:85a3:8d3:1319:8a2e:370:7348,150.172.238.178",
                    )
                    .await
                    .unwrap()
                    .status();

                assert_eq!(status, StatusCode::OK);
            }

            let peers = env
                .container
                .tracker_core_container
                .in_memory_torrent_repository
                .get_torrent_peers(&info_hash, usize::MAX)
                .await;
            let peer_addr = peers[0].peer_addr;

            assert_eq!(peer_addr.ip(), IpAddr::from_str("150.172.238.178").unwrap());

            env.stop().await;
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
    }
}

mod configured_as_whitelisted {

    mod and_receiving_an_announce_request {
        use std::str::FromStr;
        use std::sync::Arc;
        use std::time::Duration;

        use torrust_info_hash::InfoHash;
        use torrust_tracker_axum_http_server::testing::environment::Started;
        use torrust_tracker_client::http::client::Client;
        use torrust_tracker_http_protocol::v1::requests::announce::AnnounceBuilder;
        use torrust_tracker_test_helpers::logging::logs_contains_a_line_with;
        use torrust_tracker_test_helpers::{configuration, logging};
        use uuid::Uuid;

        use crate::common::fixtures::random_info_hash;
        use crate::server::asserts::{assert_is_announce_response, assert_torrent_not_in_whitelist_error_response};

        #[tokio::test]
        async fn should_fail_if_the_torrent_is_not_in_the_whitelist() {
            logging::setup();

            let cfg = configuration::ephemeral_listed();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let request_id = Uuid::new_v4();
            let info_hash = random_info_hash();

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce_with_header(
                    &AnnounceBuilder::default().with_info_hash(&info_hash).query(),
                    "x-request-id",
                    &request_id.to_string(),
                )
                .await
                .unwrap();

            assert_torrent_not_in_whitelist_error_response(response).await;

            assert!(
                logs_contains_a_line_with(&["ERROR", &format!("{info_hash}"), "is not whitelisted"]),
                "Expected logs to contain: ERROR ... {info_hash} is not whitelisted"
            );

            env.stop().await;
        }

        #[tokio::test]
        async fn should_allow_announcing_a_whitelisted_torrent() {
            logging::setup();

            let cfg = configuration::ephemeral_listed();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            env.container
                .tracker_core_container
                .whitelist_manager
                .add_torrent_to_whitelist(&info_hash)
                .await
                .expect("should add the torrent to the whitelist");

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(&AnnounceBuilder::default().with_info_hash(&info_hash).query())
                .await
                .unwrap();

            assert_is_announce_response(response).await;

            env.stop().await;
        }
    }

    mod receiving_an_scrape_request {
        use std::str::FromStr;
        use std::sync::Arc;
        use std::time::Duration;

        use torrust_info_hash::InfoHash;
        use torrust_tracker_axum_http_server::testing::environment::Started;
        use torrust_tracker_client::http::client::Client;
        use torrust_tracker_http_protocol::v1::requests::scrape_builder::QueryBuilder;
        use torrust_tracker_http_protocol::v1::responses::scrape::deserialization::{File, ResponseBuilder};
        use torrust_tracker_primitives::PeerId;
        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_test_helpers::logging::logs_contains_a_line_with;
        use torrust_tracker_test_helpers::{configuration, logging};

        use crate::common::fixtures::random_info_hash;
        use crate::server::asserts::assert_scrape_response;

        #[tokio::test]
        async fn should_return_the_zeroed_file_when_the_requested_file_is_not_whitelisted() {
            logging::setup();

            let cfg = configuration::ephemeral_listed();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = random_info_hash();

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

            let expected_scrape_response = ResponseBuilder::default().add_file(info_hash, File::zeroed()).build();

            assert_scrape_response(response, &expected_scrape_response).await;

            assert!(
                logs_contains_a_line_with(&["ERROR", &format!("{info_hash}"), "is not whitelisted"]),
                "Expected logs to contain: ERROR ... {info_hash} is not whitelisted"
            );

            env.stop().await;
        }

        #[tokio::test]
        async fn should_return_the_file_stats_when_the_requested_file_is_whitelisted() {
            logging::setup();

            let cfg = configuration::ephemeral_listed();
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

            env.container
                .tracker_core_container
                .whitelist_manager
                .add_torrent_to_whitelist(&info_hash)
                .await
                .expect("should add the torrent to the whitelist");

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
    }
}

mod configured_as_private {

    mod and_receiving_an_announce_request {
        use std::str::FromStr;
        use std::sync::Arc;
        use std::time::Duration;

        use torrust_info_hash::InfoHash;
        use torrust_tracker_axum_http_server::testing::environment::Started;
        use torrust_tracker_client::http::client::{Client, Key as TrackerClientKey};
        use torrust_tracker_core::authentication::Key;
        use torrust_tracker_http_protocol::v1::requests::announce::AnnounceBuilder;
        use torrust_tracker_test_helpers::{configuration, logging};

        use crate::server::asserts::{
            assert_authentication_error_response, assert_is_announce_response, assert_tracker_core_authentication_error_response,
        };

        #[tokio::test]
        async fn should_respond_to_authenticated_peers() {
            logging::setup();

            let cfg = configuration::ephemeral_private();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let expiring_key = env
                .container
                .tracker_core_container
                .keys_handler
                .generate_expiring_peer_key(Some(Duration::from_secs(60)))
                .await
                .unwrap();

            let response = Client::authenticated(
                env.base_url(),
                Duration::from_secs(5),
                TrackerClientKey::new(expiring_key.key().value()),
            )
            .unwrap()
            .announce(&AnnounceBuilder::default().query())
            .await
            .unwrap();

            assert_is_announce_response(response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_if_the_peer_has_not_provided_the_authentication_key() {
            logging::setup();

            let cfg = configuration::ephemeral_private();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .announce(&AnnounceBuilder::default().with_info_hash(&info_hash).query())
                .await
                .unwrap();

            assert_tracker_core_authentication_error_response(response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_fail_if_the_key_query_param_cannot_be_parsed() {
            logging::setup();

            let cfg = configuration::ephemeral_private();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let invalid_key = "INVALID_KEY";

            let response = Client::new(env.base_url(), Duration::from_secs(5)).unwrap()
                    .get(&format!(
                        "announce/{invalid_key}?info_hash=%81%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_addr=2.137.87.41&downloaded=0&uploaded=0&peer_id=-qB00000000000000001&port=17548&left=0&event=completed&compact=0"
                    ))
                    .await.unwrap();

            assert_authentication_error_response(response).await;
        }

        #[tokio::test]
        async fn should_fail_if_the_peer_cannot_be_authenticated_with_the_provided_key() {
            logging::setup();

            let cfg = configuration::ephemeral_private();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            // The tracker does not have this key
            let unregistered_key = Key::from_str("YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ").unwrap();

            let response = Client::authenticated(
                env.base_url(),
                Duration::from_secs(5),
                TrackerClientKey::new(unregistered_key.value()),
            )
            .unwrap()
            .announce(&AnnounceBuilder::default().query())
            .await
            .unwrap();

            assert_tracker_core_authentication_error_response(response).await;

            env.stop().await;
        }
    }

    mod receiving_an_scrape_request {

        use std::str::FromStr;
        use std::sync::Arc;
        use std::time::Duration;

        use torrust_info_hash::InfoHash;
        use torrust_tracker_axum_http_server::testing::environment::Started;
        use torrust_tracker_client::http::client::{Client, Key as TrackerClientKey};
        use torrust_tracker_core::authentication::Key;
        use torrust_tracker_http_protocol::v1::requests::scrape_builder::QueryBuilder;
        use torrust_tracker_http_protocol::v1::responses::scrape::deserialization::{File, ResponseBuilder};
        use torrust_tracker_primitives::PeerId;
        use torrust_tracker_primitives::peer::fixture::PeerBuilder;
        use torrust_tracker_test_helpers::{configuration, logging};

        use crate::server::asserts::{assert_authentication_error_response, assert_scrape_response};

        #[tokio::test]
        async fn should_fail_if_the_key_query_param_cannot_be_parsed() {
            logging::setup();

            let cfg = configuration::ephemeral_private();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let invalid_key = "INVALID_KEY";

            let response = Client::new(env.base_url(), Duration::from_secs(5))
                .unwrap()
                .get(&format!(
                    "scrape/{invalid_key}?info_hash=%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0"
                ))
                .await
                .unwrap();

            assert_authentication_error_response(response).await;
        }

        #[tokio::test]
        async fn should_return_the_zeroed_file_when_the_client_is_not_authenticated() {
            logging::setup();

            let cfg = configuration::ephemeral_private();
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

            let expected_scrape_response = ResponseBuilder::default().add_file(info_hash, File::zeroed()).build();

            assert_scrape_response(response, &expected_scrape_response).await;

            env.stop().await;
        }

        #[tokio::test]
        async fn should_return_the_real_file_stats_when_the_client_is_authenticated() {
            logging::setup();

            let cfg = configuration::ephemeral_private();
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

            let expiring_key = env
                .container
                .tracker_core_container
                .keys_handler
                .generate_expiring_peer_key(Some(Duration::from_secs(60)))
                .await
                .unwrap();

            let response = Client::authenticated(
                env.base_url(),
                Duration::from_secs(5),
                TrackerClientKey::new(expiring_key.key().value()),
            )
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
        async fn should_return_the_zeroed_file_when_the_authentication_key_provided_by_the_client_is_invalid() {
            logging::setup();

            // There is not authentication error
            // code-review: should this really be this way?

            let cfg = configuration::ephemeral_private();
            let core_config = Arc::new(cfg.core.clone());
            let http_tracker_config = Arc::new(cfg.http_trackers.unwrap()[0].clone());
            let env = Started::new(&core_config, &http_tracker_config).await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(); // DevSkim: ignore DS173237

            env.add_torrent_peer(
                &info_hash,
                &PeerBuilder::default()
                    .with_peer_id(&torrust_tracker_primitives::PeerId(*b"-qB00000000000000001"))
                    .with_bytes_left_to_download(1)
                    .build(),
            )
            .await;

            let false_key: Key = "YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ".parse().unwrap();

            let response = Client::authenticated(
                env.base_url(),
                Duration::from_secs(5),
                TrackerClientKey::new(false_key.value()),
            )
            .unwrap()
            .scrape(&QueryBuilder::default().with_one_info_hash(&info_hash).query())
            .await
            .unwrap();

            let expected_scrape_response = ResponseBuilder::default().add_file(info_hash, File::zeroed()).build();

            assert_scrape_response(response, &expected_scrape_response).await;

            env.stop().await;
        }
    }
}

mod configured_as_private_and_whitelisted {

    mod and_receiving_an_announce_request {}

    mod receiving_an_scrape_request {}
}

mod using_ipv6_v6only {
    use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    use std::sync::Arc;
    use std::time::Duration;

    use torrust_tracker_axum_http_server::testing::environment::Started;
    use torrust_tracker_client::http::client::Client;
    use torrust_tracker_test_helpers::{configuration, logging};

    #[tokio::test]
    async fn should_accept_ipv6_connections_with_ipv6_v6only_enabled() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let mut http_tracker_config = cfg.http_trackers.unwrap()[0].clone();
        http_tracker_config.bind_address = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);
        http_tracker_config.ipv6_v6only = true;
        let http_tracker_config = Arc::new(http_tracker_config);
        let env = Started::new(&core_config, &http_tracker_config).await;

        let client = Client::bind(env.base_url(), Duration::from_secs(5), IpAddr::V6(Ipv6Addr::UNSPECIFIED)).unwrap();

        let response = client.health_check().await.unwrap();

        assert_eq!(response.status(), 200);

        env.stop().await;
    }
}
