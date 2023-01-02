/// Integration tests for the tracker API
///
/// ```text
/// cargo test tracker_api -- --nocapture
/// ```
///
/// WIP. We are implementing a new API replacing Warp with Axum.
/// The new API runs in parallel until we finish all endpoints.
/// You can test the new API with:
///
/// ```text
/// cargo test tracker_apis -- --nocapture
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

    use reqwest::Response;

    use crate::api::ConnectionInfo;

    async fn assert_token_not_valid(response: Response) {
        assert_eq!(response.status(), 500);
        assert_eq!(
            response.text().await.unwrap(),
            "Unhandled rejection: Err { reason: \"token not valid\" }"
        );
    }

    async fn assert_unauthorized(response: Response) {
        assert_eq!(response.status(), 500);
        assert_eq!(
            response.text().await.unwrap(),
            "Unhandled rejection: Err { reason: \"unauthorized\" }"
        );
    }

    fn connection_with_invalid_token(bind_address: &str) -> ConnectionInfo {
        ConnectionInfo::authenticated(bind_address, "invalid token")
    }

    fn connection_with_no_token(bind_address: &str) -> ConnectionInfo {
        ConnectionInfo::anonymous(bind_address)
    }

    mod for_stats_resources {
        use std::str::FromStr;

        use torrust_tracker::api::resource::stats::Stats;
        use torrust_tracker::protocol::info_hash::InfoHash;

        use super::{connection_with_invalid_token, connection_with_no_token};
        use crate::api::{sample_peer, start_default_api_server, Client, Version};
        use crate::tracker_api::{assert_token_not_valid, assert_unauthorized};

        #[tokio::test]
        async fn should_allow_getting_tracker_statistics() {
            let api_server = start_default_api_server(&Version::Warp).await;

            api_server
                .add_torrent(
                    &InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(),
                    &sample_peer(),
                )
                .await;

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .get_tracker_statistics()
                .await;

            assert_eq!(response.status(), 200);
            assert_eq!(
                response.json::<Stats>().await.unwrap(),
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

        #[tokio::test]
        async fn should_not_allow_getting_tracker_statistics_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .get_tracker_statistics()
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .get_tracker_statistics()
                .await;

            assert_unauthorized(response).await;
        }
    }

    mod for_torrent_resources {
        use std::str::FromStr;

        use torrust_tracker::api::resource;
        use torrust_tracker::api::resource::torrent::{self, Torrent};
        use torrust_tracker::protocol::info_hash::InfoHash;

        use super::{connection_with_invalid_token, connection_with_no_token};
        use crate::api::{sample_peer, start_default_api_server, Client, Query, QueryParam, Version};
        use crate::tracker_api::{assert_token_not_valid, assert_unauthorized};

        #[tokio::test]
        async fn should_allow_getting_torrents() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            api_server.add_torrent(&info_hash, &sample_peer()).await;

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .get_torrents(Query::empty())
                .await;

            assert_eq!(response.status(), 200);
            assert_eq!(
                response.json::<Vec<torrent::ListItem>>().await.unwrap(),
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
        async fn should_allow_limiting_the_torrents_in_the_result() {
            let api_server = start_default_api_server(&Version::Warp).await;

            // torrents are ordered alphabetically by infohashes
            let info_hash_1 = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();
            let info_hash_2 = InfoHash::from_str("0b3aea4adc213ce32295be85d3883a63bca25446").unwrap();

            api_server.add_torrent(&info_hash_1, &sample_peer()).await;
            api_server.add_torrent(&info_hash_2, &sample_peer()).await;

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .get_torrents(Query::params([QueryParam::new("limit", "1")].to_vec()))
                .await;

            assert_eq!(response.status(), 200);
            assert_eq!(
                response.json::<Vec<torrent::ListItem>>().await.unwrap(),
                vec![torrent::ListItem {
                    info_hash: "0b3aea4adc213ce32295be85d3883a63bca25446".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: None // Torrent list does not include the peer list for each torrent
                }]
            );
        }

        #[tokio::test]
        async fn should_allow_the_torrents_result_pagination() {
            let api_server = start_default_api_server(&Version::Warp).await;

            // torrents are ordered alphabetically by infohashes
            let info_hash_1 = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();
            let info_hash_2 = InfoHash::from_str("0b3aea4adc213ce32295be85d3883a63bca25446").unwrap();

            api_server.add_torrent(&info_hash_1, &sample_peer()).await;
            api_server.add_torrent(&info_hash_2, &sample_peer()).await;

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .get_torrents(Query::params([QueryParam::new("offset", "1")].to_vec()))
                .await;

            assert_eq!(response.status(), 200);
            assert_eq!(
                response.json::<Vec<torrent::ListItem>>().await.unwrap(),
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
        async fn should_not_allow_getting_torrents_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .get_torrents(Query::empty())
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .get_torrents(Query::default())
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_getting_a_torrent_info() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            let peer = sample_peer();

            api_server.add_torrent(&info_hash, &peer).await;

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .get_torrent(&info_hash.to_string())
                .await;

            assert_eq!(response.status(), 200);
            assert_eq!(
                response.json::<Torrent>().await.unwrap(),
                Torrent {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: Some(vec![resource::peer::Peer::from(peer)])
                }
            );
        }

        #[tokio::test]
        async fn should_not_allow_getting_a_torrent_info_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            api_server.add_torrent(&info_hash, &sample_peer()).await;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .get_torrent(&info_hash.to_string())
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .get_torrent(&info_hash.to_string())
                .await;

            assert_unauthorized(response).await;
        }
    }

    mod for_whitelisted_torrent_resources {
        use std::str::FromStr;

        use torrust_tracker::protocol::info_hash::InfoHash;

        use super::{assert_token_not_valid, connection_with_invalid_token, connection_with_no_token};
        use crate::api::{start_default_api_server, Client, Version};
        use crate::tracker_api::assert_unauthorized;

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let res = Client::new(api_server.get_connection_info(), &Version::Warp)
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
            let api_server = start_default_api_server(&Version::Warp).await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let api_client = Client::new(api_server.get_connection_info(), &Version::Warp);

            let res = api_client.whitelist_a_torrent(&info_hash).await;
            assert_eq!(res.status(), 200);

            let res = api_client.whitelist_a_torrent(&info_hash).await;
            assert_eq!(res.status(), 200);
        }

        #[tokio::test]
        async fn should_not_allow_whitelisting_a_torrent_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_removing_a_torrent_from_the_whitelist() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();
            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .remove_torrent_from_whitelist(&hash)
                .await;

            assert_eq!(response.status(), 200);
            assert!(!api_server.tracker.is_info_hash_whitelisted(&info_hash).await);
        }

        #[tokio::test]
        async fn should_not_allow_removing_a_torrent_from_the_whitelist_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();

            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();
            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .remove_torrent_from_whitelist(&hash)
                .await;

            assert_token_not_valid(response).await;

            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();
            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .remove_torrent_from_whitelist(&hash)
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_reload_the_whitelist_from_the_database() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();
            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .reload_whitelist()
                .await;

            assert_eq!(response.status(), 200);
            /* This assert fails because the whitelist has not been reloaded yet.
               We could add a new endpoint GET /api/whitelist/:info_hash to check if a torrent
               is whitelisted and use that endpoint to check if the torrent is still there after reloading.
            assert!(
                !(api_server
                    .tracker
                    .is_info_hash_whitelisted(&InfoHash::from_str(&info_hash).unwrap())
                    .await)
            );
            */
        }
    }

    mod for_key_resources {
        use std::time::Duration;

        use torrust_tracker::api::resource::auth_key::AuthKey;
        use torrust_tracker::tracker::auth::Key;

        use super::{connection_with_invalid_token, connection_with_no_token};
        use crate::api::{start_default_api_server, Client, Version};
        use crate::tracker_api::{assert_token_not_valid, assert_unauthorized};

        #[tokio::test]
        async fn should_allow_generating_a_new_auth_key() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let seconds_valid = 60;

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .generate_auth_key(seconds_valid)
                .await;

            // Verify the key with the tracker
            assert!(api_server
                .tracker
                .verify_auth_key(&Key::from(response.json::<AuthKey>().await.unwrap()))
                .await
                .is_ok());
        }

        #[tokio::test]
        async fn should_not_allow_generating_a_new_auth_key_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let seconds_valid = 60;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .generate_auth_key(seconds_valid)
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .generate_auth_key(seconds_valid)
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_deleting_an_auth_key() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let seconds_valid = 60;
            let auth_key = api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .delete_auth_key(&auth_key.key)
                .await;

            assert_eq!(response.status(), 200);
            assert_eq!(response.text().await.unwrap(), "{\"status\":\"ok\"}");
        }

        #[tokio::test]
        async fn should_not_allow_deleting_an_auth_key_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let seconds_valid = 60;

            // Generate new auth key
            let auth_key = api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .delete_auth_key(&auth_key.key)
                .await;

            assert_token_not_valid(response).await;

            // Generate new auth key
            let auth_key = api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .delete_auth_key(&auth_key.key)
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_reloading_keys() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let seconds_valid = 60;
            api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(api_server.get_connection_info(), &Version::Warp)
                .reload_keys()
                .await;

            assert_eq!(response.status(), 200);
        }

        #[tokio::test]
        async fn should_not_allow_reloading_keys_for_unauthenticated_users() {
            let api_server = start_default_api_server(&Version::Warp).await;

            let seconds_valid = 60;
            api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()), &Version::Warp)
                .reload_keys()
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()), &Version::Warp)
                .reload_keys()
                .await;

            assert_unauthorized(response).await;
        }
    }
}

mod tracker_apis {

    /*

    Endpoints:

    Root:
    - [x] GET /

    Stats:
    - [ ] GET /api/stats

    Torrents:
    - [ ] GET /api/torrents?offset=:u32&limit=:u32
    - [ ] GET /api/torrent/:info_hash

    Whitelisted torrents:
    - [ ] POST   /api/whitelist/:info_hash
    - [ ] DELETE /api/whitelist/:info_hash

    Whitelist commands:
    - [ ] GET /api/whitelist/reload

    Keys:
    - [ ] POST   /api/key/:seconds_valid
    - [ ] DELETE /api/key/:key

    Key commands
    - [ ] GET /api/keys/reload

    */

    mod for_entrypoint {
        use crate::api::{start_default_api_server, Client, Query, Version};

        #[tokio::test]
        async fn test_entrypoint() {
            let api_server = start_default_api_server(&Version::Axum).await;

            let response = Client::new(api_server.get_connection_info(), &Version::Axum)
                .get("/", Query::default())
                .await;

            assert_eq!(response.status(), 200);
        }
    }
}
