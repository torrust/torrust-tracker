use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker::tracker::peer::Peer;
use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::tracker::Tracker;
use torrust_tracker::udp::server::{RunningUdpServer, StoppedUdpServer, UdpServer};
use torrust_tracker::{ephemeral_instance_keys, logging, static_time};
use torrust_tracker_configuration::Configuration;
use torrust_tracker_test_helpers::configuration::ephemeral;

fn tracker_configuration() -> Arc<Configuration> {
    Arc::new(ephemeral())
}

#[allow(clippy::module_name_repetitions, dead_code)]
pub type StoppedTestEnvironment = TestEnvironment<Stopped>;
#[allow(clippy::module_name_repetitions)]
pub type RunningTestEnvironment = TestEnvironment<Running>;

pub struct TestEnvironment<S> {
    pub tracker: Arc<Tracker>,
    pub state: S,
}

#[allow(dead_code)]
pub struct Stopped {
    api_server: StoppedUdpServer,
}

pub struct Running {
    api_server: RunningUdpServer,
}

impl<S> TestEnvironment<S> {
    /// Add a torrent to the tracker
    #[allow(dead_code)]
    pub async fn add_torrent(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}

impl TestEnvironment<Stopped> {
    #[allow(dead_code)]
    pub fn new_stopped() -> Self {
        let udp_server = udp_server();

        Self {
            tracker: udp_server.tracker.clone(),
            state: Stopped { api_server: udp_server },
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> TestEnvironment<Running> {
        TestEnvironment {
            tracker: self.tracker,
            state: Running {
                api_server: self.state.api_server.start().await.unwrap(),
            },
        }
    }
}

impl TestEnvironment<Running> {
    pub async fn new_running() -> Self {
        let udp_server = running_udp_server().await;

        Self {
            tracker: udp_server.tracker.clone(),
            state: Running { api_server: udp_server },
        }
    }

    #[allow(dead_code)]
    pub async fn stop(self) -> TestEnvironment<Stopped> {
        TestEnvironment {
            tracker: self.tracker,
            state: Stopped {
                api_server: self.state.api_server.stop().await.unwrap(),
            },
        }
    }

    pub fn bind_address(&self) -> SocketAddr {
        self.state.api_server.state.bind_address
    }
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment() -> RunningTestEnvironment {
    TestEnvironment::new_running().await
}

// TODO: Move to test-helpers crate once `Tracker` is isolated.
pub fn tracker_instance(configuration: &Arc<Configuration>) -> Arc<Tracker> {
    // Set the time of Torrust app starting
    lazy_static::initialize(&static_time::TIME_AT_APP_START);

    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize stats tracker
    let (stats_event_sender, stats_repository) = Keeper::new_active_instance();

    // Initialize Torrust tracker
    let tracker = match Tracker::new(configuration, Some(stats_event_sender), stats_repository) {
        Ok(tracker) => Arc::new(tracker),
        Err(error) => {
            panic!("{}", error)
        }
    };

    // Initialize logging
    logging::setup(configuration);

    tracker
}

pub fn udp_server() -> StoppedUdpServer {
    let config = tracker_configuration();

    let tracker = tracker_instance(&config);

    UdpServer::new(config.udp_trackers[0].clone(), tracker)
}

pub async fn running_udp_server() -> RunningUdpServer {
    udp_server().start().await.unwrap()
}
