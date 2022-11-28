/// Integration tests for the tracker API
///
/// cargo test `tracker_api` -- --nocapture
extern crate rand;

mod common;

mod tracker_api {
    use core::panic;
    use std::env;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::str::FromStr;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
    use reqwest::Response;
    use tokio::task::JoinHandle;
    use torrust_tracker::api::resources::auth_key_resource::AuthKey;
    use torrust_tracker::api::resources::stats_resource::StatsResource;
    use torrust_tracker::api::resources::torrent_resource::{TorrentListItemResource, TorrentPeerResource, TorrentResource};
    use torrust_tracker::config::Configuration;
    use torrust_tracker::jobs::tracker_api;
    use torrust_tracker::protocol::clock::DurationSinceUnixEpoch;
    use torrust_tracker::protocol::info_hash::InfoHash;
    use torrust_tracker::tracker::key::Auth;
    use torrust_tracker::tracker::peer::{self, TorrentPeer};
    use torrust_tracker::tracker::statistics::Keeper;
    use torrust_tracker::{ephemeral_instance_keys, logging, static_time, tracker};

    use crate::common::ephemeral_random_port;

    #[tokio::test]
    async fn should_allow_generating_a_new_auth_key() {
        let api_server = ApiServer::new_running_instance().await;

        let seconds_valid = 60;

        let auth_key = ApiClient::new(api_server.get_connection_info().unwrap())
            .generate_auth_key(seconds_valid)
            .await;

        // Verify the key with the tracker
        assert!(api_server
            .tracker
            .unwrap()
            .verify_auth_key(&Auth::from(auth_key))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn should_allow_whitelisting_a_torrent() {
        let api_server = ApiServer::new_running_instance().await;

        let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

        let res = ApiClient::new(api_server.get_connection_info().unwrap())
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
        let api_server = ApiServer::new_running_instance().await;

        let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

        let api_client = ApiClient::new(api_server.get_connection_info().unwrap());

        let res = api_client.whitelist_a_torrent(&info_hash).await;
        assert_eq!(res.status(), 200);

        let res = api_client.whitelist_a_torrent(&info_hash).await;
        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn should_allow_getting_a_torrent_info() {
        let api_server = ApiServer::new_running_instance().await;
        let api_connection_info = api_server.get_connection_info().unwrap();

        let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

        let (peer, peer_resource) = sample_torrent_peer();

        // Add a torrent to the tracker
        api_server
            .tracker
            .unwrap()
            .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
            .await;

        let torrent_resource = ApiClient::new(api_connection_info).get_torrent(&info_hash.to_string()).await;

        assert_eq!(
            torrent_resource,
            TorrentResource {
                info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                seeders: 1,
                completed: 0,
                leechers: 0,
                peers: Some(vec![peer_resource])
            }
        );
    }

    #[tokio::test]
    async fn should_allow_getting_torrents() {
        let api_server = ApiServer::new_running_instance().await;

        let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

        let (peer, _peer_resource) = sample_torrent_peer();

        let api_connection_info = api_server.get_connection_info().unwrap();

        // Add a torrent to the tracker
        api_server
            .tracker
            .unwrap()
            .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
            .await;

        let torrent_resources = ApiClient::new(api_connection_info).get_torrents().await;

        assert_eq!(
            torrent_resources,
            vec![TorrentListItemResource {
                info_hash: "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_string(),
                seeders: 1,
                completed: 0,
                leechers: 0,
                peers: None // Torrent list does not include peer list
            }]
        );
    }

    #[tokio::test]
    async fn should_allow_getting_tracker_statistics() {
        let api_server = ApiServer::new_running_instance().await;

        let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();

        let (peer, _peer_resource) = sample_torrent_peer();

        let api_connection_info = api_server.get_connection_info().unwrap();

        // Add a torrent to the tracker
        api_server
            .tracker
            .unwrap()
            .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
            .await;

        let stats_resource = ApiClient::new(api_connection_info).get_tracker_statistics().await;

        assert_eq!(
            stats_resource,
            StatsResource {
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

    fn sample_torrent_peer() -> (TorrentPeer, TorrentPeerResource) {
        let torrent_peer = TorrentPeer {
            peer_id: peer::Id(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
            uploaded: NumberOfBytes(0),
            downloaded: NumberOfBytes(0),
            left: NumberOfBytes(0),
            event: AnnounceEvent::Started,
        };
        let torrent_peer_resource = TorrentPeerResource::from(torrent_peer);

        (torrent_peer, torrent_peer_resource)
    }

    fn tracker_configuration() -> Arc<Configuration> {
        let mut config = Configuration::default();
        config.log_level = Some("off".to_owned());

        // Ephemeral socket address
        let port = ephemeral_random_port();
        config.http_api.bind_address = format!("127.0.0.1:{}", &port);

        // Ephemeral database
        let temp_directory = env::temp_dir();
        let temp_file = temp_directory.join(format!("data_{}.db", &port));
        config.db_path = temp_file.to_str().unwrap().to_owned();

        Arc::new(config)
    }

    #[derive(Clone)]
    struct ApiConnectionInfo {
        pub bind_address: String,
        pub api_token: String,
    }

    impl ApiConnectionInfo {
        pub fn new(bind_address: &str, api_token: &str) -> Self {
            Self {
                bind_address: bind_address.to_string(),
                api_token: api_token.to_string(),
            }
        }
    }

    struct ApiServer {
        pub started: AtomicBool,
        pub job: Option<JoinHandle<()>>,
        pub tracker: Option<Arc<tracker::Tracker>>,
        pub connection_info: Option<ApiConnectionInfo>,
    }

    impl ApiServer {
        pub fn new() -> Self {
            Self {
                started: AtomicBool::new(false),
                job: None,
                tracker: None,
                connection_info: None,
            }
        }

        pub async fn new_running_instance() -> ApiServer {
            let configuration = tracker_configuration();
            ApiServer::new_running_custom_instance(configuration.clone()).await
        }

        async fn new_running_custom_instance(configuration: Arc<Configuration>) -> ApiServer {
            let mut api_server = ApiServer::new();
            api_server.start(configuration).await;
            api_server
        }

        pub async fn start(&mut self, configuration: Arc<Configuration>) {
            if !self.started.load(Ordering::Relaxed) {
                self.connection_info = Some(ApiConnectionInfo::new(
                    &configuration.http_api.bind_address.clone(),
                    &configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone(),
                ));

                // Set the time of Torrust app starting
                lazy_static::initialize(&static_time::TIME_AT_APP_START);

                // Initialize the Ephemeral Instance Random Seed
                lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

                // Initialize stats tracker
                let (stats_event_sender, stats_repository) = Keeper::new_active_instance();

                // Initialize Torrust tracker
                let tracker = match tracker::Tracker::new(&configuration.clone(), Some(stats_event_sender), stats_repository) {
                    Ok(tracker) => Arc::new(tracker),
                    Err(error) => {
                        panic!("{}", error)
                    }
                };
                self.tracker = Some(tracker.clone());

                // Initialize logging
                logging::setup(&configuration);

                // Start the HTTP API job
                self.job = Some(tracker_api::start_job(&configuration, tracker).await);

                self.started.store(true, Ordering::Relaxed);
            }
        }

        pub fn get_connection_info(&self) -> Option<ApiConnectionInfo> {
            self.connection_info.clone()
        }
    }

    struct ApiClient {
        connection_info: ApiConnectionInfo,
    }

    impl ApiClient {
        pub fn new(connection_info: ApiConnectionInfo) -> Self {
            Self { connection_info }
        }

        pub async fn generate_auth_key(&self, seconds_valid: i32) -> AuthKey {
            let url = format!(
                "http://{}/api/key/{}?token={}",
                &self.connection_info.bind_address, &seconds_valid, &self.connection_info.api_token
            );
            reqwest::Client::new().post(url).send().await.unwrap().json().await.unwrap()
        }

        pub async fn whitelist_a_torrent(&self, info_hash: &str) -> Response {
            let url = format!(
                "http://{}/api/whitelist/{}?token={}",
                &self.connection_info.bind_address, &info_hash, &self.connection_info.api_token
            );
            reqwest::Client::new().post(url.clone()).send().await.unwrap()
        }

        pub async fn get_torrent(&self, info_hash: &str) -> TorrentResource {
            let url = format!(
                "http://{}/api/torrent/{}?token={}",
                &self.connection_info.bind_address, &info_hash, &self.connection_info.api_token
            );
            reqwest::Client::builder()
                .build()
                .unwrap()
                .get(url)
                .send()
                .await
                .unwrap()
                .json::<TorrentResource>()
                .await
                .unwrap()
        }

        pub async fn get_torrents(&self) -> Vec<TorrentListItemResource> {
            let url = format!(
                "http://{}/api/torrents?token={}",
                &self.connection_info.bind_address, &self.connection_info.api_token
            );
            reqwest::Client::builder()
                .build()
                .unwrap()
                .get(url)
                .send()
                .await
                .unwrap()
                .json::<Vec<TorrentListItemResource>>()
                .await
                .unwrap()
        }

        pub async fn get_tracker_statistics(&self) -> StatsResource {
            let url = format!(
                "http://{}/api/stats?token={}",
                &self.connection_info.bind_address, &self.connection_info.api_token
            );
            reqwest::Client::builder()
                .build()
                .unwrap()
                .get(url)
                .send()
                .await
                .unwrap()
                .json::<StatsResource>()
                .await
                .unwrap()
        }
    }
}
