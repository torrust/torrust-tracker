/// Integration tests for the tracker API
///
/// ```text
/// cargo test tracker_api -- --nocapture
/// ```
extern crate rand;

mod api;
mod common;

mod tracker_api {

    /*

    Endpoints:

    Stats:
    GET /api/stats

    Torrents:
    GET /api/torrents?offset=:u32&limit=:u32
    GET /api/torrent/:info_hash

    Whitelisted torrents:
    POST   /api/whitelist/:info_hash
    DELETE /api/whitelist/:info_hash

    Whitelist command:
    GET    /api/whitelist/reload

    Keys:
    POST   /api/key/:seconds_valid
    GET    /api/keys/reload
    DELETE /api/key/:key

    */

    mod for_stats_resources {
        use std::str::FromStr;

        use torrust_tracker::api::resource::stats::Stats;
        use torrust_tracker::protocol::info_hash::InfoHash;

        use crate::api::{sample_torrent_peer, Client, Server};

        #[tokio::test]
        async fn should_allow_getting_tracker_statistics() {
            let api_server = Server::new_running_instance().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            let (peer, _peer_resource) = sample_torrent_peer();

            let api_connection_info = api_server.get_connection_info().unwrap();

            // Add a torrent to the tracker
            api_server
                .tracker
                .unwrap()
                .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
                .await;

            let stats_resource = Client::new(api_connection_info).get_tracker_statistics().await;

            assert_eq!(
                stats_resource,
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
                }
            );
        }
    }

    mod for_torrent_resources {
        #[tokio::test]
        async fn should_allow_getting_torrents() {
            let api_server = Server::new_running_instance().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            let (peer, _peer_resource) = sample_torrent_peer();

            let api_connection_info = api_server.get_connection_info().unwrap();

            // Add a torrent to the tracker
            api_server
                .tracker
                .unwrap()
                .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
                .await;

            let torrent_resources = Client::new(api_connection_info).get_torrents().await;

            assert_eq!(
                torrent_resources,
                vec![torrent::ListItem {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: None // Torrent list does not include peer list
                }]
            );
        }

        #[tokio::test]
        async fn should_allow_getting_a_torrent_info() {
            let api_server = Server::new_running_instance().await;
            let api_connection_info = api_server.get_connection_info().unwrap();

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            let (peer, peer_resource) = sample_torrent_peer();

            // Add a torrent to the tracker
            api_server
                .tracker
                .unwrap()
                .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
                .await;

            let torrent_resource = Client::new(api_connection_info).get_torrent(&info_hash.to_string()).await;

            assert_eq!(
                torrent_resource,
                Torrent {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: Some(vec![peer_resource])
                }
            );
        }

        use std::str::FromStr;

        use torrust_tracker::api::resource::torrent::{self, Torrent};
        use torrust_tracker::protocol::info_hash::InfoHash;

        use crate::api::{sample_torrent_peer, Client, Server};

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent() {
            let api_server = Server::new_running_instance().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let res = Client::new(api_server.get_connection_info().unwrap())
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_eq!(res.status(), 200);
            assert!(
                api_server
                    .tracker
                    .unwrap()
                    .is_info_hash_whitelisted(&InfoHash::from_str(&info_hash).unwrap())
                    .await
            );
        }
    }

    mod for_whitelisted_torrent_resources {
        use std::str::FromStr;

        use torrust_tracker::protocol::info_hash::InfoHash;

        use crate::api::{Client, Server};

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent() {
            let api_server = Server::new_running_instance().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let res = Client::new(api_server.get_connection_info().unwrap())
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_eq!(res.status(), 200);
            assert!(
                api_server
                    .tracker
                    .unwrap()
                    .is_info_hash_whitelisted(&InfoHash::from_str(&info_hash).unwrap())
                    .await
            );
        }

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent_that_has_been_already_whitelisted() {
            let api_server = Server::new_running_instance().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let api_client = Client::new(api_server.get_connection_info().unwrap());

            let res = api_client.whitelist_a_torrent(&info_hash).await;
            assert_eq!(res.status(), 200);

            let res = api_client.whitelist_a_torrent(&info_hash).await;
            assert_eq!(res.status(), 200);
        }
    }

    mod for_key_resources {
        use torrust_tracker::tracker::auth;

        use crate::api::{Client, Server};

        #[tokio::test]
        async fn should_allow_generating_a_new_auth_key() {
            let api_server = Server::new_running_instance().await;

            let seconds_valid = 60;

            let auth_key = Client::new(api_server.get_connection_info().unwrap())
                .generate_auth_key(seconds_valid)
                .await;

            // Verify the key with the tracker
            assert!(api_server
                .tracker
                .unwrap()
                .verify_auth_key(&auth::Key::from(auth_key))
                .await
                .is_ok());
        }
    }
}
