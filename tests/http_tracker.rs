/// Integration tests for HTTP tracker server
///
/// cargo test `http_tracker_server` -- --nocapture
mod common;
mod http;

mod http_tracker_server {

    mod receiving_an_announce_request {
        use crate::http::asserts::assert_internal_server_error;
        use crate::http::client::Client;
        use crate::http::server::start_default_http_tracker;

        #[tokio::test]
        async fn should_fail_when_the_request_is_empty() {
            let http_tracker_server = start_default_http_tracker().await;

            let response = Client::new(http_tracker_server.get_connection_info()).get("announce").await;

            assert_internal_server_error(response).await;
        }
    }

    mod receiving_an_scrape_request {
        use crate::http::asserts::assert_internal_server_error;
        use crate::http::client::Client;
        use crate::http::server::start_default_http_tracker;

        #[tokio::test]
        async fn should_fail_when_the_request_is_empty() {
            let http_tracker_server = start_default_http_tracker().await;

            let response = Client::new(http_tracker_server.get_connection_info()).get("scrape").await;

            assert_internal_server_error(response).await;
        }
    }
}

mod public_http_tracker_server {

    mod receiving_an_announce_request {
        use std::net::{IpAddr, Ipv4Addr};
        use std::str::FromStr;

        use torrust_tracker::protocol::info_hash::InfoHash;
        use torrust_tracker::tracker::peer;

        use crate::common::fixtures::sample_peer;
        use crate::http::asserts::assert_announce_response;
        use crate::http::client::Client;
        use crate::http::requests::{AnnounceQuery, Compact, Event};
        use crate::http::responses::{Announce, DictionaryPeer};
        use crate::http::server::start_default_http_tracker;

        fn sample_announce_query(info_hash: &InfoHash) -> AnnounceQuery {
            AnnounceQuery {
                info_hash: info_hash.0,
                peer_addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 88)),
                downloaded: 0,
                uploaded: 0,
                peer_id: peer::Id(*b"-qB00000000000000001").0,
                port: 17548,
                left: 0,
                event: Some(Event::Completed),
                compact: Some(Compact::NotAccepted),
            }
        }

        #[tokio::test]
        async fn should_return_no_peers_if_the_announced_peer_is_the_first_one() {
            let http_tracker_server = start_default_http_tracker().await;

            let response = Client::new(http_tracker_server.get_connection_info())
                .announce(&sample_announce_query(
                    &InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap(),
                ))
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
            let http_tracker_server = start_default_http_tracker().await;

            let info_hash = InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap();
            let peer = sample_peer();

            // Add a peer
            http_tracker_server.add_torrent(&info_hash, &peer).await;

            let announce_query = sample_announce_query(&info_hash);

            assert_ne!(
                announce_query.peer_id, peer.peer_id.0,
                "the new peer id must be different from the previously announced peer otherwise the peer previously added peer in not included in the list"
            );

            // Announce the new peer. This new peer is non included the response peers list
            let response = Client::new(http_tracker_server.get_connection_info())
                .announce(&announce_query)
                .await;

            assert_announce_response(
                response,
                &Announce {
                    complete: 2,
                    incomplete: 0,
                    interval: 120,
                    min_interval: 120,
                    peers: vec![DictionaryPeer {
                        ip: peer.peer_addr.ip().to_string(),
                        peer_id: String::new(),
                        port: peer.peer_addr.port(),
                    }],
                },
            )
            .await;
        }
    }
}
