use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker::core::peer::Peer;
use torrust_tracker::core::Tracker;
use torrust_tracker::servers::udp::server::{Launcher, RunningUdpServer, StoppedUdpServer, UdpServer};
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;

use crate::common::app::setup_with_configuration;

#[allow(clippy::module_name_repetitions, dead_code)]
pub type StoppedTestEnvironment = TestEnvironment<Stopped>;
#[allow(clippy::module_name_repetitions)]
pub type RunningTestEnvironment = TestEnvironment<Running>;

pub struct TestEnvironment<S> {
    pub cfg: Arc<torrust_tracker_configuration::Configuration>,
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
    pub fn new_stopped(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let cfg = Arc::new(cfg);

        let tracker = setup_with_configuration(&cfg);

        let udp_cfg = cfg.udp_trackers[0].clone();

        let bind_to = udp_cfg
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        let udp_server = udp_server(Launcher::new(bind_to));

        Self {
            cfg,
            tracker,
            state: Stopped { udp_server },
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> TestEnvironment<Running> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker.clone(),
            state: Running {
                udp_server: self.state.udp_server.start(self.tracker).await.unwrap(),
            },
        }
    }
}

impl TestEnvironment<Running> {
    pub async fn new_running(cfg: torrust_tracker_configuration::Configuration) -> Self {
        StoppedTestEnvironment::new_stopped(cfg).start().await
    }

    #[allow(dead_code)]
    pub async fn stop(self) -> TestEnvironment<Stopped> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker,
            state: Stopped {
                udp_server: self.state.udp_server.stop().await.unwrap(),
            },
        }
    }

    pub fn bind_address(&self) -> SocketAddr {
        self.state.udp_server.state.binding
    }
}

#[allow(clippy::module_name_repetitions, dead_code)]
pub fn stopped_test_environment(cfg: torrust_tracker_configuration::Configuration) -> StoppedTestEnvironment {
    TestEnvironment::new_stopped(cfg)
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment(cfg: torrust_tracker_configuration::Configuration) -> RunningTestEnvironment {
    TestEnvironment::new_running(cfg).await
}

pub fn udp_server(launcher: Launcher) -> StoppedUdpServer {
    UdpServer::new(launcher)
}
