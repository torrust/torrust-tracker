/// Integration tests for the tracker API
///
/// cargo test tracker_api -- --nocapture
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
    use tokio::task::JoinHandle;
    use torrust_tracker::api::resources::auth_key_resource::AuthKeyResource;
    use torrust_tracker::api::resources::stats_resource::StatsResource;
    use torrust_tracker::api::resources::torrent_resource::{TorrentPeerResource, TorrentResource};
    use torrust_tracker::jobs::tracker_api;
    use torrust_tracker::peer::TorrentPeer;
    use torrust_tracker::protocol::clock::DurationSinceUnixEpoch;
    use torrust_tracker::tracker::key::AuthKey;
    use torrust_tracker::tracker::statistics::StatsTracker;
    use torrust_tracker::tracker::TorrentTracker;
    use torrust_tracker::{ephemeral_instance_keys, logging, static_time, Configuration, InfoHash, PeerId};

    use crate::common::ephemeral_random_port;

    #[tokio::test]
    async fn should_allow_generating_a_new_auth_key() {
        let configuration = tracker_configuration();
        let api_server = new_running_api_server(configuration.clone()).await;

        let bind_address = api_server.bind_address.unwrap().clone();
        let seconds_valid = 60;
        let api_token = configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone();

        let url = format!("http://{}/api/key/{}?token={}", &bind_address, &seconds_valid, &api_token);

        let auth_key: AuthKeyResource = reqwest::Client::new().post(url).send().await.unwrap().json().await.unwrap();

        // Verify the key with the tracker
        assert!(api_server
            .tracker
            .unwrap()
            .verify_auth_key(&AuthKey::from(auth_key))
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn should_allow_whitelisting_a_torrent() {
        let configuration = tracker_configuration();
        let api_server = new_running_api_server(configuration.clone()).await;

        let bind_address = api_server.bind_address.unwrap().clone();
        let api_token = configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone();
        let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

        let url = format!("http://{}/api/whitelist/{}?token={}", &bind_address, &info_hash, &api_token);

        let res = reqwest::Client::new().post(url.clone()).send().await.unwrap();

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
        let configuration = tracker_configuration();
        let api_server = new_running_api_server(configuration.clone()).await;

        let bind_address = api_server.bind_address.unwrap().clone();
        let api_token = configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone();
        let info_hash = "9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d".to_owned();

        let url = format!("http://{}/api/whitelist/{}?token={}", &bind_address, &info_hash, &api_token);

        // First whitelist request
        let res = reqwest::Client::new().post(url.clone()).send().await.unwrap();
        assert_eq!(res.status(), 200);

        // Second whitelist request
        let res = reqwest::Client::new().post(url.clone()).send().await.unwrap();
        assert_eq!(res.status(), 200);
    }

    fn sample_torrent_peer() -> (TorrentPeer, TorrentPeerResource) {
        let torrent_peer = TorrentPeer {
            peer_id: PeerId(*b"-qB00000000000000000"),
            peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
            updated: DurationSinceUnixEpoch::new(1669397478934, 0),
            uploaded: NumberOfBytes(0),
            downloaded: NumberOfBytes(0),
            left: NumberOfBytes(0),
            event: AnnounceEvent::Started,
        };
        let torrent_peer_resource = TorrentPeerResource::from(torrent_peer);

        (torrent_peer, torrent_peer_resource)
    }

    #[tokio::test]
    async fn should_allow_getting_a_torrent_info() {
        let configuration = tracker_configuration();
        let api_server = new_running_api_server(configuration.clone()).await;

        let bind_address = api_server.bind_address.unwrap().clone();
        let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();
        let api_token = configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone();

        let (peer, peer_resource) = sample_torrent_peer();

        // Add a torrent to the tracker
        api_server
            .tracker
            .unwrap()
            .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
            .await;

        let url = format!("http://{}/api/torrent/{}?token={}", &bind_address, &info_hash, &api_token);

        let torrent_resource = reqwest::Client::builder()
            .build()
            .unwrap()
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<TorrentResource>()
            .await
            .unwrap();

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
        let configuration = tracker_configuration();
        let api_server = new_running_api_server(configuration.clone()).await;

        let bind_address = api_server.bind_address.unwrap().clone();
        let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();
        let api_token = configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone();

        let (peer, _peer_resource) = sample_torrent_peer();

        // Add a torrent to the tracker
        api_server
            .tracker
            .unwrap()
            .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
            .await;

        let url = format!("http://{}/api/torrents?token={}", &bind_address, &api_token);

        let torrent_resources = reqwest::Client::builder()
            .build()
            .unwrap()
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<Vec<TorrentResource>>()
            .await
            .unwrap();

        assert_eq!(
            torrent_resources,
            vec![TorrentResource {
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
        let configuration = tracker_configuration();
        let api_server = new_running_api_server(configuration.clone()).await;

        let bind_address = api_server.bind_address.unwrap().clone();
        let info_hash = InfoHash::from_str("9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d").unwrap();
        let api_token = configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone();

        let (peer, _peer_resource) = sample_torrent_peer();

        // Add a torrent to the tracker
        api_server
            .tracker
            .unwrap()
            .update_torrent_with_peer_and_get_stats(&info_hash, &peer)
            .await;

        let url = format!("http://{}/api/stats?token={}", &bind_address, &api_token);

        let stats = reqwest::Client::builder()
            .build()
            .unwrap()
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<StatsResource>()
            .await
            .unwrap();

        assert_eq!(
            stats,
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

    async fn new_running_api_server(configuration: Arc<Configuration>) -> ApiServer {
        let mut api_server = ApiServer::new();
        api_server.start(configuration).await;
        api_server
    }

    pub struct ApiServer {
        pub started: AtomicBool,
        pub job: Option<JoinHandle<()>>,
        pub bind_address: Option<String>,
        pub tracker: Option<Arc<TorrentTracker>>,
    }

    impl ApiServer {
        pub fn new() -> Self {
            Self {
                started: AtomicBool::new(false),
                job: None,
                bind_address: None,
                tracker: None,
            }
        }

        pub async fn start(&mut self, configuration: Arc<Configuration>) {
            if !self.started.load(Ordering::Relaxed) {
                self.bind_address = Some(configuration.http_api.bind_address.clone());

                // Set the time of Torrust app starting
                lazy_static::initialize(&static_time::TIME_AT_APP_START);

                // Initialize the Ephemeral Instance Random Seed
                lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

                // Initialize stats tracker
                let (stats_event_sender, stats_repository) = StatsTracker::new_active_instance();

                // Initialize Torrust tracker
                let tracker = match TorrentTracker::new(configuration.clone(), Some(stats_event_sender), stats_repository) {
                    Ok(tracker) => Arc::new(tracker),
                    Err(error) => {
                        panic!("{}", error)
                    }
                };
                self.tracker = Some(tracker.clone());

                // Initialize logging
                logging::setup_logging(&configuration);

                // Start the HTTP API job
                self.job = Some(tracker_api::start_job(&configuration, tracker).await);

                self.started.store(true, Ordering::Relaxed);
            }
        }
    }
}
