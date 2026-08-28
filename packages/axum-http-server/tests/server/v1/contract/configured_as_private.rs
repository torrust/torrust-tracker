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
            .persistence
            .as_ref()
            .expect("private tracker test requires persistence")
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
                    "announce/{invalid_key}?info_hash=%81%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&ip=2.137.87.41&downloaded=0&uploaded=0&peer_id=-qB00000000000000001&port=17548&left=0&event=completed&compact=0"
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
            .persistence
            .as_ref()
            .expect("private tracker test requires persistence")
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
