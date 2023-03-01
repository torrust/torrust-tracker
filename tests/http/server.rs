use core::panic;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use torrust_tracker::http::Version;
use torrust_tracker::jobs::http_tracker;
use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker::tracker::peer::Peer;
use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time, tracker};
use torrust_tracker_configuration::Configuration;
use torrust_tracker_primitives::TrackerMode;

use super::connection_info::ConnectionInfo;

/// Starts a HTTP tracker with mode "public" in settings
pub async fn start_public_http_tracker(version: Version) -> Server {
    let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral();
    configuration.mode = TrackerMode::Public;
    start_custom_http_tracker(Arc::new(configuration), version).await
}

/// Starts a HTTP tracker with mode "listed" in settings
pub async fn start_whitelisted_http_tracker(version: Version) -> Server {
    let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral();
    configuration.mode = TrackerMode::Listed;
    start_custom_http_tracker(Arc::new(configuration), version).await
}

/// Starts a HTTP tracker with mode "private" in settings
pub async fn start_private_http_tracker(version: Version) -> Server {
    let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral();
    configuration.mode = TrackerMode::Private;
    start_custom_http_tracker(Arc::new(configuration), version).await
}

/// Starts a HTTP tracker with a wildcard IPV6 address.
/// The configuration in the `config.toml` file would be like this:
///
/// ```text
/// [[http_trackers]]
/// bind_address = "[::]:7070"
/// ```
pub async fn start_ipv6_http_tracker(version: Version) -> Server {
    let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral();

    // Change socket address to "wildcard address" (unspecified address which means any IP address)
    // but keeping the random port generated with the ephemeral configuration.
    let socket_addr: SocketAddr = configuration.http_trackers[0].bind_address.parse().unwrap();
    let new_ipv6_socket_address = format!("[::]:{}", socket_addr.port());
    configuration.http_trackers[0].bind_address = new_ipv6_socket_address;

    start_custom_http_tracker(Arc::new(configuration), version).await
}

/// Starts a HTTP tracker with an specific `external_ip`.
/// The configuration in the `config.toml` file would be like this:
///
/// ```text
/// external_ip = "2.137.87.41"
/// ```
pub async fn start_http_tracker_with_external_ip(external_ip: &IpAddr, version: Version) -> Server {
    let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral();
    configuration.external_ip = Some(external_ip.to_string());
    start_custom_http_tracker(Arc::new(configuration), version).await
}

/// Starts a HTTP tracker `on_reverse_proxy`.
/// The configuration in the `config.toml` file would be like this:
///
/// ```text
/// on_reverse_proxy = true
/// ```
pub async fn start_http_tracker_on_reverse_proxy(version: Version) -> Server {
    let mut configuration = torrust_tracker_test_helpers::configuration::ephemeral();
    configuration.on_reverse_proxy = true;
    start_custom_http_tracker(Arc::new(configuration), version).await
}

pub async fn start_default_http_tracker(version: Version) -> Server {
    let configuration = tracker_configuration();
    start_custom_http_tracker(configuration.clone(), version).await
}

pub fn tracker_configuration() -> Arc<Configuration> {
    Arc::new(torrust_tracker_test_helpers::configuration::ephemeral())
}

pub async fn start_custom_http_tracker(configuration: Arc<Configuration>, version: Version) -> Server {
    let server = start(&configuration);
    http_tracker::start_job(&configuration.http_trackers[0], server.tracker.clone(), version).await;
    server
}

fn start(configuration: &Arc<Configuration>) -> Server {
    let connection_info = ConnectionInfo::anonymous(&configuration.http_trackers[0].bind_address.clone());

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

    pub async fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}
