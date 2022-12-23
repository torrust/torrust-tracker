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

        use crate::api::{sample_peer, start_default_api_server, Client};

        #[tokio::test]
        async fn should_allow_getting_tracker_statistics() {
            let api_server = start_default_api_server().await;

            api_server
                .add_torrent(
                    &InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(),
                    &sample_peer(),
                )
                .await;

            let stats_resource = Client::new(api_server.get_connection_info()).get_tracker_statistics().await;

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
            let api_server = start_default_api_server().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            api_server.add_torrent(&info_hash, &sample_peer()).await;

            let torrent_resources = Client::new(api_server.get_connection_info()).get_torrents().await;

            assert_eq!(
                torrent_resources,
                vec![torrent::ListItem {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: None // Torrent list does not include the peer list for each torrent
                }]
            );
        }

        #[tokio::test]
        async fn should_allow_getting_a_torrent_info() {
            let api_server = start_default_api_server().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            let peer = sample_peer();

            api_server.add_torrent(&info_hash, &peer).await;

            let torrent_resource = Client::new(api_server.get_connection_info())
                .get_torrent(&info_hash.to_string())
                .await;

            assert_eq!(
                torrent_resource,
                Torrent {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: Some(vec![resource::peer::Peer::from(peer)])
                }
            );
        }

        use std::str::FromStr;

        use torrust_tracker::api::resource;
        use torrust_tracker::api::resource::torrent::{self, Torrent};
        use torrust_tracker::protocol::info_hash::InfoHash;

        use crate::api::{sample_peer, start_default_api_server, Client};

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent() {
            let api_server = start_default_api_server().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let res = Client::new(api_server.get_connection_info())
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_eq!(res.status(), 200);
            assert!(
                api_server
                    .tracker
                    .is_info_hash_whitelisted(&InfoHash::from_str(&info_hash).unwrap())
                    .await
            );
        }
    }

    mod for_whitelisted_torrent_resources {
        use std::str::FromStr;

        use torrust_tracker::protocol::info_hash::InfoHash;

        use crate::api::{start_default_api_server, Client};

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent() {
            let api_server = start_default_api_server().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let res = Client::new(api_server.get_connection_info())
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_eq!(res.status(), 200);
            assert!(
                api_server
                    .tracker
                    .is_info_hash_whitelisted(&InfoHash::from_str(&info_hash).unwrap())
                    .await
            );
        }

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent_that_has_been_already_whitelisted() {
            let api_server = start_default_api_server().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let api_client = Client::new(api_server.get_connection_info());

            let res = api_client.whitelist_a_torrent(&info_hash).await;
            assert_eq!(res.status(), 200);

            let res = api_client.whitelist_a_torrent(&info_hash).await;
            assert_eq!(res.status(), 200);
        }
    }

    mod for_key_resources {
        use torrust_tracker::tracker::auth;

        use crate::api::{start_default_api_server, Client};

        #[tokio::test]
        async fn should_allow_generating_a_new_auth_key() {
            let api_server = start_default_api_server().await;

            let seconds_valid = 60;

            let auth_key = Client::new(api_server.get_connection_info())
                .generate_auth_key(seconds_valid)
                .await;

            // Verify the key with the tracker
            assert!(api_server.tracker.verify_auth_key(&auth::Key::from(auth_key)).await.is_ok());
        }
    }
}
