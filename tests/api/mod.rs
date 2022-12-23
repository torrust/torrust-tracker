use core::panic;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use reqwest::Response;
use torrust_tracker::api::resource::auth_key::AuthKey;
use torrust_tracker::api::resource::stats::Stats;
use torrust_tracker::api::resource::torrent::{self, Torrent};
use torrust_tracker::config::Configuration;
use torrust_tracker::jobs::tracker_api;
use torrust_tracker::protocol::clock::DurationSinceUnixEpoch;
use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker::tracker::peer::{self, Peer};
use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time, tracker};

use crate::common::ephemeral_random_port;

pub fn sample_peer() -> peer::Peer {
    peer::Peer {
        peer_id: peer::Id(*b"-qB00000000000000000"),
        peer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(126, 0, 0, 1)), 8080),
        updated: DurationSinceUnixEpoch::new(1_669_397_478_934, 0),
        uploaded: NumberOfBytes(0),
        downloaded: NumberOfBytes(0),
        left: NumberOfBytes(0),
        event: AnnounceEvent::Started,
    }
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

pub async fn start_default_api_server() -> Server {
    let configuration = tracker_configuration();
    start_custom_api_server(configuration.clone()).await
}

pub async fn start_custom_api_server(configuration: Arc<Configuration>) -> Server {
    start(configuration).await
}

async fn start(configuration: Arc<Configuration>) -> Server {
    let connection_info = ConnectionInfo::new(
        &configuration.http_api.bind_address.clone(),
        &configuration.http_api.access_tokens.get_key_value("admin").unwrap().1.clone(),
    );

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

    // Initialize logging
    logging::setup(&configuration);

    // Start the HTTP API job
    tracker_api::start_job(&configuration.http_api, tracker.clone()).await;

    Server {
        tracker,
        connection_info,
    }
}

pub struct Server {
    pub tracker: Arc<tracker::Tracker>,
    pub connection_info: ConnectionInfo,
}

impl Server {
    pub fn get_connection_info(&self) -> ConnectionInfo {
        self.connection_info.clone()
    }

    /// Add a torrent to the tracker
    pub async fn add_torrent(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
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
        self.post(&format!("key/{}", &seconds_valid)).await.json().await.unwrap()
    }

    pub async fn whitelist_a_torrent(&self, info_hash: &str) -> Response {
        self.post(&format!("whitelist/{}", &info_hash)).await
    }

    pub async fn get_torrent(&self, info_hash: &str) -> Torrent {
        self.get(&format!("torrent/{}", &info_hash))
            .await
            .json::<Torrent>()
            .await
            .unwrap()
    }

    pub async fn get_torrents(&self) -> Vec<torrent::ListItem> {
        self.get("torrents").await.json::<Vec<torrent::ListItem>>().await.unwrap()
    }

    pub async fn get_tracker_statistics(&self) -> Stats {
        self.get("stats").await.json::<Stats>().await.unwrap()
    }

    async fn get(&self, path: &str) -> Response {
        reqwest::Client::builder()
            .build()
            .unwrap()
            .get(self.url(path))
            .send()
            .await
            .unwrap()
    }

    async fn post(&self, path: &str) -> Response {
        reqwest::Client::new().post(self.url(path).clone()).send().await.unwrap()
    }

    fn url(&self, path: &str) -> String {
        format!(
            "http://{}/api/{path}?token={}",
            &self.connection_info.bind_address, &self.connection_info.api_token
        )
    }
}
