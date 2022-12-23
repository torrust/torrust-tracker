use core::panic;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use reqwest::Response;
use tokio::task::JoinHandle;
use torrust_tracker::api::resource;
use torrust_tracker::api::resource::auth_key::AuthKey;
use torrust_tracker::api::resource::stats::Stats;
use torrust_tracker::api::resource::torrent::{self, Torrent};
use torrust_tracker::config::Configuration;
use torrust_tracker::jobs::tracker_api;
use torrust_tracker::protocol::clock::DurationSinceUnixEpoch;
use torrust_tracker::tracker::peer;
use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time, tracker};

use crate::common::ephemeral_random_port;

pub fn sample_torrent_peer() -> (peer::Peer, resource::peer::Peer) {
    let torrent_peer = peer::Peer {
        peer_id: peer::Id(*b"-qB00000000000000000"),
        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
        updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
        uploaded: NumberOfBytes(0),
        downloaded: NumberOfBytes(0),
        left: NumberOfBytes(0),
        event: AnnounceEvent::Started,
    };
    let torrent_peer_resource = resource::peer::Peer::from(torrent_peer);

    (torrent_peer, torrent_peer_resource)
}

pub fn tracker_configuration() -> Arc<Configuration> {
    let mut config = Configuration {
        log_level: Some("off".to_owned()),
        ..Default::default()
    };

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
pub struct ConnectionInfo {
    pub bind_address: String,
    pub api_token: String,
}

impl ConnectionInfo {
    pub fn new(bind_address: &str, api_token: &str) -> Self {
        Self {
            bind_address: bind_address.to_string(),
            api_token: api_token.to_string(),
        }
    }
}

pub struct Server {
    pub started: AtomicBool,
    pub job: Option<JoinHandle<()>>,
    pub tracker: Option<Arc<tracker::Tracker>>,
    pub connection_info: Option<ConnectionInfo>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            started: AtomicBool::new(false),
            job: None,
            tracker: None,
            connection_info: None,
        }
    }

    pub async fn new_running_instance() -> Self {
        let configuration = tracker_configuration();
        Self::new_running_custom_instance(configuration.clone()).await
    }

    async fn new_running_custom_instance(configuration: Arc<Configuration>) -> Self {
        let mut api_server = Self::new();
        api_server.start(configuration).await;
        api_server
    }

    pub async fn start(&mut self, configuration: Arc<Configuration>) {
        if !self.started.load(Ordering::Relaxed) {
            self.connection_info = Some(ConnectionInfo::new(
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
            self.job = Some(tracker_api::start_job(&configuration.http_api, tracker).await);

            self.started.store(true, Ordering::Relaxed);
        }
    }

    pub fn get_connection_info(&self) -> Option<ConnectionInfo> {
        self.connection_info.clone()
    }
}

pub struct Client {
    connection_info: ConnectionInfo,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo) -> Self {
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

    pub async fn get_torrent(&self, info_hash: &str) -> Torrent {
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
            .json::<Torrent>()
            .await
            .unwrap()
    }

    pub async fn get_torrents(&self) -> Vec<torrent::ListItem> {
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
            .json::<Vec<torrent::ListItem>>()
            .await
            .unwrap()
    }

    pub async fn get_tracker_statistics(&self) -> Stats {
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
            .json::<Stats>()
            .await
            .unwrap()
    }
}
