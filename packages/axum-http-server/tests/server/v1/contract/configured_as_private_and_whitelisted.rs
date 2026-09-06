use std::sync::Arc;
use std::time::Duration;

use torrust_info_hash::InfoHash;
use torrust_tracker_axum_http_server::testing::environment::Started;
use torrust_tracker_client::http::client::{Client, Key as TrackerClientKey};
use torrust_tracker_primitives::PeerId;
use torrust_tracker_primitives::peer::fixture::PeerBuilder;
use torrust_tracker_test_helpers::{configuration, logging};

const CLIENT_TIMEOUT: Duration = Duration::from_secs(5);

struct PrivateListedTracker {
    environment: Started,
}

impl PrivateListedTracker {
    async fn start() -> Self {
        logging::setup();

        let cfg = configuration::ephemeral_private_and_listed();
        let core_config = Arc::new(cfg.core.clone());
        let http_tracker_config =
            Arc::new(cfg.http_trackers.expect("private listed tracker requires an HTTP tracker")[0].clone());

        Self {
            environment: Started::new(&core_config, &http_tracker_config).await,
        }
    }

    fn unauthenticated_client(&self) -> Client {
        Client::new(self.environment.base_url(), CLIENT_TIMEOUT).expect("should create an unauthenticated tracker client")
    }

    async fn authenticated_client(&self) -> Client {
        let authentication_key = self
            .environment
            .container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("private listed tracker requires persistence")
            .keys_handler
            .generate_expiring_peer_key(Some(Duration::from_secs(60)))
            .await
            .expect("should generate an authentication key");

        Client::authenticated(
            self.environment.base_url(),
            CLIENT_TIMEOUT,
            TrackerClientKey::new(authentication_key.key().value()),
        )
        .expect("should create an authenticated tracker client")
    }

    async fn list_torrent(&self, info_hash: &InfoHash) {
        self.environment
            .container
            .tracker_core_container
            .persistence
            .as_ref()
            .expect("private listed tracker requires persistence")
            .whitelist_manager
            .add_torrent_to_whitelist(info_hash)
            .await
            .expect("should add the torrent to the whitelist");
    }

    async fn add_incomplete_peer(&self, info_hash: &InfoHash) {
        self.environment
            .add_torrent_peer(
                info_hash,
                &PeerBuilder::default()
                    .with_peer_id(&PeerId(*b"-qB00000000000000001"))
                    .with_bytes_left_to_download(1)
                    .build(),
            )
            .await;
    }

    async fn stop(self) {
        self.environment.stop().await;
    }
}

mod and_receiving_an_announce_request {
    use torrust_tracker_http_protocol::v1::requests::announce::AnnounceBuilder;

    use super::PrivateListedTracker;
    use crate::common::fixtures::random_info_hash;
    use crate::server::asserts::{
        assert_authentication_error_response, assert_is_announce_response, assert_torrent_not_in_whitelist_error_response,
    };

    #[tokio::test]
    async fn it_should_return_an_authentication_error_for_an_unauthenticated_peer_announcing_an_unlisted_torrent() {
        // Arrange
        let tracker = PrivateListedTracker::start().await;
        let unlisted_torrent = random_info_hash();
        let client = tracker.unauthenticated_client();
        let announce = AnnounceBuilder::default().with_info_hash(&unlisted_torrent).query();

        // Act
        let response = client.announce(&announce).await.unwrap();

        // Assert
        assert_authentication_error_response(response).await;
        tracker.stop().await;
    }

    #[tokio::test]
    async fn it_should_return_a_whitelist_error_for_an_authenticated_peer_announcing_an_unlisted_torrent() {
        // Arrange
        let tracker = PrivateListedTracker::start().await;
        let unlisted_torrent = random_info_hash();
        let client = tracker.authenticated_client().await;
        let announce = AnnounceBuilder::default().with_info_hash(&unlisted_torrent).query();

        // Act
        let response = client.announce(&announce).await.unwrap();

        // Assert
        assert_torrent_not_in_whitelist_error_response(response).await;
        tracker.stop().await;
    }

    #[tokio::test]
    async fn it_should_respond_to_an_authenticated_peer_announcing_a_listed_torrent() {
        // Arrange
        let tracker = PrivateListedTracker::start().await;
        let listed_torrent = random_info_hash();
        tracker.list_torrent(&listed_torrent).await;
        let client = tracker.authenticated_client().await;
        let announce = AnnounceBuilder::default().with_info_hash(&listed_torrent).query();

        // Act
        let response = client.announce(&announce).await.unwrap();

        // Assert
        assert_is_announce_response(response).await;
        tracker.stop().await;
    }
}

mod receiving_an_scrape_request {
    use torrust_tracker_http_protocol::v1::requests::scrape_builder::QueryBuilder;
    use torrust_tracker_http_protocol::v1::responses::scrape::deserialization::{File, ResponseBuilder};

    use super::PrivateListedTracker;
    use crate::common::fixtures::random_info_hash;
    use crate::server::asserts::assert_scrape_response;

    #[tokio::test]
    async fn it_should_hide_listed_torrent_stats_from_an_unauthenticated_peer() {
        // Arrange
        let tracker = PrivateListedTracker::start().await;
        let listed_torrent = random_info_hash();
        tracker.add_incomplete_peer(&listed_torrent).await;
        tracker.list_torrent(&listed_torrent).await;
        let client = tracker.unauthenticated_client();
        let scrape = QueryBuilder::default().with_one_info_hash(&listed_torrent).query();

        // Act
        let response = client.scrape(&scrape).await.unwrap();

        // Assert
        let expected_response = ResponseBuilder::default().add_file(listed_torrent, File::zeroed()).build();
        assert_scrape_response(response, &expected_response).await;
        tracker.stop().await;
    }

    #[tokio::test]
    async fn it_should_hide_unlisted_torrent_stats_from_an_authenticated_peer() {
        // Arrange
        let tracker = PrivateListedTracker::start().await;
        let unlisted_torrent = random_info_hash();
        tracker.add_incomplete_peer(&unlisted_torrent).await;
        let client = tracker.authenticated_client().await;
        let scrape = QueryBuilder::default().with_one_info_hash(&unlisted_torrent).query();

        // Act
        let response = client.scrape(&scrape).await.unwrap();

        // Assert
        let expected_response = ResponseBuilder::default().add_file(unlisted_torrent, File::zeroed()).build();
        assert_scrape_response(response, &expected_response).await;
        tracker.stop().await;
    }

    #[tokio::test]
    async fn it_should_return_listed_torrent_stats_to_an_authenticated_peer() {
        // Arrange
        let tracker = PrivateListedTracker::start().await;
        let listed_torrent = random_info_hash();
        tracker.add_incomplete_peer(&listed_torrent).await;
        tracker.list_torrent(&listed_torrent).await;
        let client = tracker.authenticated_client().await;
        let scrape = QueryBuilder::default().with_one_info_hash(&listed_torrent).query();

        // Act
        let response = client.scrape(&scrape).await.unwrap();

        // Assert
        let expected_response = ResponseBuilder::default()
            .add_file(
                listed_torrent,
                File {
                    complete: 0,
                    downloaded: 0,
                    incomplete: 1,
                },
            )
            .build();
        assert_scrape_response(response, &expected_response).await;
        tracker.stop().await;
    }
}
