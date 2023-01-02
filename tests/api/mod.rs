use core::panic;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use aquatic_udp_protocol::{AnnounceEvent, NumberOfBytes};
use reqwest::Response;
use torrust_tracker::config::Configuration;
use torrust_tracker::jobs::{tracker_api, tracker_apis};
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
    pub api_token: Option<String>,
}

impl ConnectionInfo {
    pub fn authenticated(bind_address: &str, api_token: &str) -> Self {
        Self {
            bind_address: bind_address.to_string(),
            api_token: Some(api_token.to_string()),
        }
    }

    pub fn anonymous(bind_address: &str) -> Self {
        Self {
            bind_address: bind_address.to_string(),
            api_token: None,
        }
    }
}

pub async fn start_default_api_server(version: &Version) -> Server {
    let configuration = tracker_configuration();
    start_custom_api_server(configuration.clone(), version).await
}

pub async fn start_custom_api_server(configuration: Arc<Configuration>, version: &Version) -> Server {
    match &version {
        Version::Warp => start_warp_api(configuration).await,
        Version::Axum => start_axum_api(configuration).await,
    }
}

async fn start_warp_api(configuration: Arc<Configuration>) -> Server {
    let server = start(&configuration);

    // Start the HTTP API job
    tracker_api::start_job(&configuration.http_api, server.tracker.clone()).await;

    server
}

async fn start_axum_api(configuration: Arc<Configuration>) -> Server {
    let server = start(&configuration);

    // Start HTTP APIs server (multiple API versions)
    // Temporarily run the new API on a port number after the current API port
    tracker_apis::start_job(&configuration.http_api, server.tracker.clone()).await;

    server
}

fn start(configuration: &Arc<Configuration>) -> Server {
    let connection_info = ConnectionInfo::authenticated(
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
    let tracker = match tracker::Tracker::new(configuration, Some(stats_event_sender), stats_repository) {
        Ok(tracker) => Arc::new(tracker),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Initialize logging
    logging::setup(configuration);

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

    pub fn get_bind_address(&self) -> String {
        self.connection_info.bind_address.clone()
    }

    /// Add a torrent to the tracker
    pub async fn add_torrent(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}

pub struct Client {
    connection_info: ConnectionInfo,
    base_path: String,
}

type ReqwestQuery = Vec<ReqwestQueryParam>;
type ReqwestQueryParam = (String, String);

#[derive(Default, Debug)]
pub struct Query {
    params: Vec<QueryParam>,
}

impl Query {
    pub fn empty() -> Self {
        Self { params: vec![] }
    }

    pub fn params(params: Vec<QueryParam>) -> Self {
        Self { params }
    }

    pub fn add_param(&mut self, param: QueryParam) {
        self.params.push(param);
    }

    fn with_token(token: &str) -> Self {
        Self {
            params: vec![QueryParam::new("token", token)],
        }
    }
}

impl From<Query> for ReqwestQuery {
    fn from(url_search_params: Query) -> Self {
        url_search_params
            .params
            .iter()
            .map(|param| ReqwestQueryParam::from((*param).clone()))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct QueryParam {
    name: String,
    value: String,
}

impl QueryParam {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<QueryParam> for ReqwestQueryParam {
    fn from(param: QueryParam) -> Self {
        (param.name, param.value)
    }
}

pub enum Version {
    Warp,
    Axum,
}

impl Client {
    pub fn new(connection_info: ConnectionInfo, version: &Version) -> Self {
        Self {
            connection_info,
            base_path: match version {
                Version::Warp => "/api/".to_string(),
                Version::Axum => String::new(),
            },
        }
    }

    pub async fn generate_auth_key(&self, seconds_valid: i32) -> Response {
        self.post(&format!("key/{}", &seconds_valid)).await
    }

    pub async fn delete_auth_key(&self, key: &str) -> Response {
        self.delete(&format!("key/{}", &key)).await
    }

    pub async fn reload_keys(&self) -> Response {
        self.get("keys/reload", Query::default()).await
    }

    pub async fn whitelist_a_torrent(&self, info_hash: &str) -> Response {
        self.post(&format!("whitelist/{}", &info_hash)).await
    }

    pub async fn remove_torrent_from_whitelist(&self, info_hash: &str) -> Response {
        self.delete(&format!("whitelist/{}", &info_hash)).await
    }

    pub async fn reload_whitelist(&self) -> Response {
        self.get("whitelist/reload", Query::default()).await
    }

    pub async fn get_torrent(&self, info_hash: &str) -> Response {
        self.get(&format!("torrent/{}", &info_hash), Query::default()).await
    }

    pub async fn get_torrents(&self, params: Query) -> Response {
        self.get("torrents", params).await
    }

    pub async fn get_tracker_statistics(&self) -> Response {
        self.get("stats", Query::default()).await
    }

    pub async fn get(&self, path: &str, params: Query) -> Response {
        let mut query: Query = params;

        if let Some(token) = &self.connection_info.api_token {
            query.add_param(QueryParam::new("token", token));
        };

        reqwest::Client::builder()
            .build()
            .unwrap()
            .get(self.base_url(path))
            .query(&ReqwestQuery::from(query))
            .send()
            .await
            .unwrap()
    }

    async fn post(&self, path: &str) -> Response {
        reqwest::Client::new()
            .post(self.base_url(path).clone())
            .query(&ReqwestQuery::from(self.query_with_token()))
            .send()
            .await
            .unwrap()
    }

    async fn delete(&self, path: &str) -> Response {
        reqwest::Client::new()
            .delete(self.base_url(path).clone())
            .query(&ReqwestQuery::from(self.query_with_token()))
            .send()
            .await
            .unwrap()
    }

    fn base_url(&self, path: &str) -> String {
        format!("http://{}{}{path}", &self.connection_info.bind_address, &self.base_path)
    }

    fn query_with_token(&self) -> Query {
        match &self.connection_info.api_token {
            Some(token) => Query::with_token(token),
            None => Query::default(),
        }
    }
}
