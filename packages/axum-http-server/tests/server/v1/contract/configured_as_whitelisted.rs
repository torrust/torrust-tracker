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
            .persistence
            .as_ref()
            .expect("listed tracker test requires persistence")
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
            .persistence
            .as_ref()
            .expect("listed tracker test requires persistence")
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
