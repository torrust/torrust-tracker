/// Integration tests for the tracker API
///
/// ```text
/// cargo test tracker_apis -- --nocapture
/// ```
extern crate rand;

mod api;
mod common;

mod tracker_apis {

    use crate::common::fixtures::invalid_info_hashes;

    // When these infohashes are used in URL path params
    // the response is a custom response returned in the handler
    fn invalid_infohashes_returning_bad_request() -> Vec<String> {
        invalid_info_hashes()
    }

    // When these infohashes are used in URL path params
    // the response is an Axum response returned in the handler
    fn invalid_infohashes_returning_not_found() -> Vec<String> {
        [String::new(), " ".to_string()].to_vec()
    }

    mod authentication {
        use crate::api::asserts::{assert_token_not_valid, assert_unauthorized};
        use crate::api::client::Client;
        use crate::api::server::start_default_api;
        use crate::common::http::{Query, QueryParam};

        #[tokio::test]
        async fn should_authenticate_requests_by_using_a_token_query_param() {
            let api_server = start_default_api().await;

            let token = api_server.get_connection_info().api_token.unwrap();

            let response = Client::new(api_server.get_connection_info())
                .get_request_with_query("stats", Query::params([QueryParam::new("token", &token)].to_vec()))
                .await;

            assert_eq!(response.status(), 200);
        }

        #[tokio::test]
        async fn should_not_authenticate_requests_when_the_token_is_missing() {
            let api_server = start_default_api().await;

            let response = Client::new(api_server.get_connection_info())
                .get_request_with_query("stats", Query::default())
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_not_authenticate_requests_when_the_token_is_empty() {
            let api_server = start_default_api().await;

            let response = Client::new(api_server.get_connection_info())
                .get_request_with_query("stats", Query::params([QueryParam::new("token", "")].to_vec()))
                .await;

            assert_token_not_valid(response).await;
        }

        #[tokio::test]
        async fn should_not_authenticate_requests_when_the_token_is_invalid() {
            let api_server = start_default_api().await;

            let response = Client::new(api_server.get_connection_info())
                .get_request_with_query("stats", Query::params([QueryParam::new("token", "INVALID TOKEN")].to_vec()))
                .await;

            assert_token_not_valid(response).await;
        }

        #[tokio::test]
        async fn should_allow_the_token_query_param_to_be_at_any_position_in_the_url_query() {
            let api_server = start_default_api().await;

            let token = api_server.get_connection_info().api_token.unwrap();

            // At the beginning of the query component
            let response = Client::new(api_server.get_connection_info())
                .get_request(&format!("torrents?token={}&limit=1", &token))
                .await;

            assert_eq!(response.status(), 200);

            // At the end of the query component
            let response = Client::new(api_server.get_connection_info())
                .get_request(&format!("torrents?limit=1&token={}", &token))
                .await;

            assert_eq!(response.status(), 200);
        }
    }

    mod for_stats_resources {
        use std::str::FromStr;

        use torrust_tracker::apis::resources::stats::Stats;
        use torrust_tracker::protocol::info_hash::InfoHash;

        use crate::api::asserts::{assert_stats, assert_token_not_valid, assert_unauthorized};
        use crate::api::client::Client;
        use crate::api::connection_info::{connection_with_invalid_token, connection_with_no_token};
        use crate::api::server::start_default_api;
        use crate::common::fixtures::PeerBuilder;

        #[tokio::test]
        async fn should_allow_getting_tracker_statistics() {
            let api_server = start_default_api().await;

            api_server
                .add_torrent_peer(
                    &InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap(),
                    &PeerBuilder::default().into(),
                )
                .await;

            let response = Client::new(api_server.get_connection_info()).get_tracker_statistics().await;

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
        }

        #[tokio::test]
        async fn should_not_allow_getting_tracker_statistics_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .get_tracker_statistics()
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .get_tracker_statistics()
                .await;

            assert_unauthorized(response).await;
        }
    }

    mod for_torrent_resources {
        use std::str::FromStr;

        use torrust_tracker::apis::resources::torrent::Torrent;
        use torrust_tracker::apis::resources::{self, torrent};
        use torrust_tracker::protocol::info_hash::InfoHash;

        use super::{invalid_infohashes_returning_bad_request, invalid_infohashes_returning_not_found};
        use crate::api::asserts::{
            assert_bad_request, assert_invalid_infohash_param, assert_not_found, assert_token_not_valid, assert_torrent_info,
            assert_torrent_list, assert_torrent_not_known, assert_unauthorized,
        };
        use crate::api::client::Client;
        use crate::api::connection_info::{connection_with_invalid_token, connection_with_no_token};
        use crate::api::server::start_default_api;
        use crate::common::fixtures::PeerBuilder;
        use crate::common::http::{Query, QueryParam};

        #[tokio::test]
        async fn should_allow_getting_torrents() {
            let api_server = start_default_api().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            api_server.add_torrent_peer(&info_hash, &PeerBuilder::default().into()).await;

            let response = Client::new(api_server.get_connection_info())
                .get_torrents(Query::empty())
                .await;

            assert_torrent_list(
                response,
                vec![torrent::ListItem {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: None, // Torrent list does not include the peer list for each torrent
                }],
            )
            .await;
        }

        #[tokio::test]
        async fn should_allow_limiting_the_torrents_in_the_result() {
            let api_server = start_default_api().await;

            // torrents are ordered alphabetically by infohashes
            let info_hash_1 = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();
            let info_hash_2 = InfoHash::from_str("0b3aea4adc213ce32295be85d3883a63bca25446").unwrap();

            api_server
                .add_torrent_peer(&info_hash_1, &PeerBuilder::default().into())
                .await;
            api_server
                .add_torrent_peer(&info_hash_2, &PeerBuilder::default().into())
                .await;

            let response = Client::new(api_server.get_connection_info())
                .get_torrents(Query::params([QueryParam::new("limit", "1")].to_vec()))
                .await;

            assert_torrent_list(
                response,
                vec![torrent::ListItem {
                    info_hash: "0b3aea4adc213ce32295be85d3883a63bca25446".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: None, // Torrent list does not include the peer list for each torrent
                }],
            )
            .await;
        }

        #[tokio::test]
        async fn should_allow_the_torrents_result_pagination() {
            let api_server = start_default_api().await;

            // torrents are ordered alphabetically by infohashes
            let info_hash_1 = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();
            let info_hash_2 = InfoHash::from_str("0b3aea4adc213ce32295be85d3883a63bca25446").unwrap();

            api_server
                .add_torrent_peer(&info_hash_1, &PeerBuilder::default().into())
                .await;
            api_server
                .add_torrent_peer(&info_hash_2, &PeerBuilder::default().into())
                .await;

            let response = Client::new(api_server.get_connection_info())
                .get_torrents(Query::params([QueryParam::new("offset", "1")].to_vec()))
                .await;

            assert_torrent_list(
                response,
                vec![torrent::ListItem {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: None, // Torrent list does not include the peer list for each torrent
                }],
            )
            .await;
        }

        #[tokio::test]
        async fn should_fail_getting_torrents_when_the_offset_query_parameter_cannot_be_parsed() {
            let api_server = start_default_api().await;

            let invalid_offsets = [" ", "-1", "1.1", "INVALID OFFSET"];

            for invalid_offset in &invalid_offsets {
                let response = Client::new(api_server.get_connection_info())
                    .get_torrents(Query::params([QueryParam::new("offset", invalid_offset)].to_vec()))
                    .await;

                assert_bad_request(response, "Failed to deserialize query string: invalid digit found in string").await;
            }
        }

        #[tokio::test]
        async fn should_fail_getting_torrents_when_the_limit_query_parameter_cannot_be_parsed() {
            let api_server = start_default_api().await;

            let invalid_limits = [" ", "-1", "1.1", "INVALID LIMIT"];

            for invalid_limit in &invalid_limits {
                let response = Client::new(api_server.get_connection_info())
                    .get_torrents(Query::params([QueryParam::new("limit", invalid_limit)].to_vec()))
                    .await;

                assert_bad_request(response, "Failed to deserialize query string: invalid digit found in string").await;
            }
        }

        #[tokio::test]
        async fn should_not_allow_getting_torrents_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .get_torrents(Query::empty())
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .get_torrents(Query::default())
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_getting_a_torrent_info() {
            let api_server = start_default_api().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            let peer = PeerBuilder::default().into();

            api_server.add_torrent_peer(&info_hash, &peer).await;

            let response = Client::new(api_server.get_connection_info())
                .get_torrent(&info_hash.to_string())
                .await;

            assert_torrent_info(
                response,
                Torrent {
                    info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                    seeders: 1,
                    completed: 0,
                    leechers: 0,
                    peers: Some(vec![resources::peer::Peer::from(peer)]),
                },
            )
            .await;
        }

        #[tokio::test]
        async fn should_fail_while_getting_a_torrent_info_when_the_torrent_does_not_exist() {
            let api_server = start_default_api().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            let response = Client::new(api_server.get_connection_info())
                .get_torrent(&info_hash.to_string())
                .await;

            assert_torrent_not_known(response).await;
        }

        #[tokio::test]
        async fn should_fail_getting_a_torrent_info_when_the_provided_infohash_is_invalid() {
            let api_server = start_default_api().await;

            for invalid_infohash in &invalid_infohashes_returning_bad_request() {
                let response = Client::new(api_server.get_connection_info())
                    .get_torrent(invalid_infohash)
                    .await;

                assert_invalid_infohash_param(response, invalid_infohash).await;
            }

            for invalid_infohash in &invalid_infohashes_returning_not_found() {
                let response = Client::new(api_server.get_connection_info())
                    .get_torrent(invalid_infohash)
                    .await;

                assert_not_found(response).await;
            }
        }

        #[tokio::test]
        async fn should_not_allow_getting_a_torrent_info_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

            api_server.add_torrent_peer(&info_hash, &PeerBuilder::default().into()).await;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .get_torrent(&info_hash.to_string())
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .get_torrent(&info_hash.to_string())
                .await;

            assert_unauthorized(response).await;
        }
    }

    mod for_whitelisted_torrent_resources {
        use std::str::FromStr;

        use torrust_tracker::protocol::info_hash::InfoHash;

        use super::{invalid_infohashes_returning_bad_request, invalid_infohashes_returning_not_found};
        use crate::api::asserts::{
            assert_failed_to_reload_whitelist, assert_failed_to_remove_torrent_from_whitelist,
            assert_failed_to_whitelist_torrent, assert_invalid_infohash_param, assert_not_found, assert_ok,
            assert_token_not_valid, assert_unauthorized,
        };
        use crate::api::client::Client;
        use crate::api::connection_info::{connection_with_invalid_token, connection_with_no_token};
        use crate::api::force_database_error;
        use crate::api::server::start_default_api;

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent() {
            let api_server = start_default_api().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let response = Client::new(api_server.get_connection_info())
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_ok(response).await;
            assert!(
                api_server
                    .tracker
                    .is_info_hash_whitelisted(&InfoHash::from_str(&info_hash).unwrap())
                    .await
            );
        }

        #[tokio::test]
        async fn should_allow_whitelisting_a_torrent_that_has_been_already_whitelisted() {
            let api_server = start_default_api().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let api_client = Client::new(api_server.get_connection_info());

            let response = api_client.whitelist_a_torrent(&info_hash).await;
            assert_ok(response).await;

            let response = api_client.whitelist_a_torrent(&info_hash).await;
            assert_ok(response).await;
        }

        #[tokio::test]
        async fn should_not_allow_whitelisting_a_torrent_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_fail_when_the_torrent_cannot_be_whitelisted() {
            let api_server = start_default_api().await;

            let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            force_database_error(&api_server.tracker);

            let response = Client::new(api_server.get_connection_info())
                .whitelist_a_torrent(&info_hash)
                .await;

            assert_failed_to_whitelist_torrent(response).await;
        }

        #[tokio::test]
        async fn should_fail_whitelisting_a_torrent_when_the_provided_infohash_is_invalid() {
            let api_server = start_default_api().await;

            for invalid_infohash in &invalid_infohashes_returning_bad_request() {
                let response = Client::new(api_server.get_connection_info())
                    .whitelist_a_torrent(invalid_infohash)
                    .await;

                assert_invalid_infohash_param(response, invalid_infohash).await;
            }

            for invalid_infohash in &invalid_infohashes_returning_not_found() {
                let response = Client::new(api_server.get_connection_info())
                    .whitelist_a_torrent(invalid_infohash)
                    .await;

                assert_not_found(response).await;
            }
        }

        #[tokio::test]
        async fn should_allow_removing_a_torrent_from_the_whitelist() {
            let api_server = start_default_api().await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();
            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();

            let response = Client::new(api_server.get_connection_info())
                .remove_torrent_from_whitelist(&hash)
                .await;

            assert_ok(response).await;
            assert!(!api_server.tracker.is_info_hash_whitelisted(&info_hash).await);
        }

        #[tokio::test]
        async fn should_not_fail_trying_to_remove_a_non_whitelisted_torrent_from_the_whitelist() {
            let api_server = start_default_api().await;

            let non_whitelisted_torrent_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

            let response = Client::new(api_server.get_connection_info())
                .remove_torrent_from_whitelist(&non_whitelisted_torrent_hash)
                .await;

            assert_ok(response).await;
        }

        #[tokio::test]
        async fn should_fail_removing_a_torrent_from_the_whitelist_when_the_provided_infohash_is_invalid() {
            let api_server = start_default_api().await;

            for invalid_infohash in &invalid_infohashes_returning_bad_request() {
                let response = Client::new(api_server.get_connection_info())
                    .remove_torrent_from_whitelist(invalid_infohash)
                    .await;

                assert_invalid_infohash_param(response, invalid_infohash).await;
            }

            for invalid_infohash in &invalid_infohashes_returning_not_found() {
                let response = Client::new(api_server.get_connection_info())
                    .remove_torrent_from_whitelist(invalid_infohash)
                    .await;

                assert_not_found(response).await;
            }
        }

        #[tokio::test]
        async fn should_fail_when_the_torrent_cannot_be_removed_from_the_whitelist() {
            let api_server = start_default_api().await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();
            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();

            force_database_error(&api_server.tracker);

            let response = Client::new(api_server.get_connection_info())
                .remove_torrent_from_whitelist(&hash)
                .await;

            assert_failed_to_remove_torrent_from_whitelist(response).await;
        }

        #[tokio::test]
        async fn should_not_allow_removing_a_torrent_from_the_whitelist_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();

            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();
            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .remove_torrent_from_whitelist(&hash)
                .await;

            assert_token_not_valid(response).await;

            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();
            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .remove_torrent_from_whitelist(&hash)
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_reload_the_whitelist_from_the_database() {
            let api_server = start_default_api().await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();
            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();

            let response = Client::new(api_server.get_connection_info()).reload_whitelist().await;

            assert_ok(response).await;
            /* todo: this assert fails because the whitelist has not been reloaded yet.
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

        #[tokio::test]
        async fn should_fail_when_the_whitelist_cannot_be_reloaded_from_the_database() {
            let api_server = start_default_api().await;

            let hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();
            let info_hash = InfoHash::from_str(&hash).unwrap();
            api_server.tracker.add_torrent_to_whitelist(&info_hash).await.unwrap();

            force_database_error(&api_server.tracker);

            let response = Client::new(api_server.get_connection_info()).reload_whitelist().await;

            assert_failed_to_reload_whitelist(response).await;
        }
    }

    mod for_key_resources {
        use std::time::Duration;

        use torrust_tracker::tracker::auth::Key;

        use crate::api::asserts::{
            assert_auth_key_utf8, assert_failed_to_delete_key, assert_failed_to_generate_key, assert_failed_to_reload_keys,
            assert_invalid_auth_key_param, assert_invalid_key_duration_param, assert_ok, assert_token_not_valid,
            assert_unauthorized,
        };
        use crate::api::client::Client;
        use crate::api::connection_info::{connection_with_invalid_token, connection_with_no_token};
        use crate::api::force_database_error;
        use crate::api::server::start_default_api;

        #[tokio::test]
        async fn should_allow_generating_a_new_auth_key() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;

            let response = Client::new(api_server.get_connection_info())
                .generate_auth_key(seconds_valid)
                .await;

            let auth_key_resource = assert_auth_key_utf8(response).await;

            // Verify the key with the tracker
            assert!(api_server
                .tracker
                .verify_auth_key(&auth_key_resource.key.parse::<Key>().unwrap())
                .await
                .is_ok());
        }

        #[tokio::test]
        async fn should_not_allow_generating_a_new_auth_key_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .generate_auth_key(seconds_valid)
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .generate_auth_key(seconds_valid)
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_fail_generating_a_new_auth_key_when_the_key_duration_is_invalid() {
            let api_server = start_default_api().await;

            let invalid_key_durations = [
                // "", it returns 404
                // " ", it returns 404
                "-1", "text",
            ];

            for invalid_key_duration in &invalid_key_durations {
                let response = Client::new(api_server.get_connection_info())
                    .post(&format!("key/{}", &invalid_key_duration))
                    .await;

                assert_invalid_key_duration_param(response, invalid_key_duration).await;
            }
        }

        #[tokio::test]
        async fn should_fail_when_the_auth_key_cannot_be_generated() {
            let api_server = start_default_api().await;

            force_database_error(&api_server.tracker);

            let seconds_valid = 60;
            let response = Client::new(api_server.get_connection_info())
                .generate_auth_key(seconds_valid)
                .await;

            assert_failed_to_generate_key(response).await;
        }

        #[tokio::test]
        async fn should_allow_deleting_an_auth_key() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;
            let auth_key = api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(api_server.get_connection_info())
                .delete_auth_key(&auth_key.key.to_string())
                .await;

            assert_ok(response).await;
        }

        #[tokio::test]
        async fn should_fail_deleting_an_auth_key_when_the_key_id_is_invalid() {
            let api_server = start_default_api().await;

            let invalid_auth_key_ids = [
                // "", it returns a 404
                // " ", it returns a 404
                "0",
                "-1",
                "INVALID AUTH KEY ID",
                "IrweYtVuQPGbG9Jzx1DihcPmJGGpVy8",   // 32 char key cspell:disable-line
                "IrweYtVuQPGbG9Jzx1DihcPmJGGpVy8zs", // 34 char key cspell:disable-line
            ];

            for invalid_auth_key_id in &invalid_auth_key_ids {
                let response = Client::new(api_server.get_connection_info())
                    .delete_auth_key(invalid_auth_key_id)
                    .await;

                assert_invalid_auth_key_param(response, invalid_auth_key_id).await;
            }
        }

        #[tokio::test]
        async fn should_fail_when_the_auth_key_cannot_be_deleted() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;
            let auth_key = api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            force_database_error(&api_server.tracker);

            let response = Client::new(api_server.get_connection_info())
                .delete_auth_key(&auth_key.key.to_string())
                .await;

            assert_failed_to_delete_key(response).await;
        }

        #[tokio::test]
        async fn should_not_allow_deleting_an_auth_key_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;

            // Generate new auth key
            let auth_key = api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .delete_auth_key(&auth_key.key.to_string())
                .await;

            assert_token_not_valid(response).await;

            // Generate new auth key
            let auth_key = api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .delete_auth_key(&auth_key.key.to_string())
                .await;

            assert_unauthorized(response).await;
        }

        #[tokio::test]
        async fn should_allow_reloading_keys() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;
            api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(api_server.get_connection_info()).reload_keys().await;

            assert_ok(response).await;
        }

        #[tokio::test]
        async fn should_fail_when_keys_cannot_be_reloaded() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;
            api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            force_database_error(&api_server.tracker);

            let response = Client::new(api_server.get_connection_info()).reload_keys().await;

            assert_failed_to_reload_keys(response).await;
        }

        #[tokio::test]
        async fn should_not_allow_reloading_keys_for_unauthenticated_users() {
            let api_server = start_default_api().await;

            let seconds_valid = 60;
            api_server
                .tracker
                .generate_auth_key(Duration::from_secs(seconds_valid))
                .await
                .unwrap();

            let response = Client::new(connection_with_invalid_token(&api_server.get_bind_address()))
                .reload_keys()
                .await;

            assert_token_not_valid(response).await;

            let response = Client::new(connection_with_no_token(&api_server.get_bind_address()))
                .reload_keys()
                .await;

            assert_unauthorized(response).await;
        }
    }
}
