use core::panic;
use std::sync::Arc;

use torrust_tracker::jobs::tracker_apis;
use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker::tracker::peer::Peer;
use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time, tracker};
use torrust_tracker_configuration::{Configuration, ephemeral_configuration};

use super::connection_info::ConnectionInfo;

pub fn tracker_configuration() -> Arc<Configuration> {
    Arc::new(ephemeral_configuration())
}

pub async fn start_default_api() -> Server {
    let configuration = tracker_configuration();
    start_custom_api(configuration.clone()).await
}

pub async fn start_custom_api(configuration: Arc<Configuration>) -> Server {
    let server = start(&configuration);
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
