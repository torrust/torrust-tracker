use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker::tracker::peer::Peer;
use torrust_tracker::tracker::Tracker;
use torrust_tracker::udp::server::{RunningUdpServer, StoppedUdpServer, UdpServer};
use torrust_tracker_test_helpers::configuration;

use crate::common::tracker::new_tracker;

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
    udp_server: StoppedUdpServer,
}

pub struct Running {
    udp_server: RunningUdpServer,
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
            state: Stopped { udp_server: udp_server },
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> TestEnvironment<Running> {
        TestEnvironment {
            tracker: self.tracker,
            state: Running {
                udp_server: self.state.udp_server.start().await.unwrap(),
            },
        }
    }
}

impl TestEnvironment<Running> {
    pub async fn new_running() -> Self {
        let udp_server = running_udp_server().await;

        Self {
            tracker: udp_server.tracker.clone(),
            state: Running { udp_server: udp_server },
        }
    }

    #[allow(dead_code)]
    pub async fn stop(self) -> TestEnvironment<Stopped> {
        TestEnvironment {
            tracker: self.tracker,
            state: Stopped {
                udp_server: self.state.udp_server.stop().await.unwrap(),
            },
        }
    }

    pub fn bind_address(&self) -> SocketAddr {
        self.state.udp_server.state.bind_address
    }
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment() -> RunningTestEnvironment {
    TestEnvironment::new_running().await
}

pub fn udp_server() -> StoppedUdpServer {
    let config = Arc::new(configuration::ephemeral());

    let tracker = new_tracker(config.clone());

    UdpServer::new(config.udp_trackers[0].clone(), tracker)
}

pub async fn running_udp_server() -> RunningUdpServer {
    udp_server().start().await.unwrap()
}
