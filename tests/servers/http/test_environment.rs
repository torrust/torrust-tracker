use std::sync::Arc;

use torrust_tracker::core::peer::Peer;
use torrust_tracker::core::Tracker;
use torrust_tracker::servers::http::server::{HttpServer, HttpServerLauncher, RunningHttpServer, StoppedHttpServer};
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;

use crate::common::app::setup_with_configuration;

#[allow(clippy::module_name_repetitions, dead_code)]
pub type StoppedTestEnvironment<I> = TestEnvironment<Stopped<I>>;
#[allow(clippy::module_name_repetitions)]
pub type RunningTestEnvironment<I> = TestEnvironment<Running<I>>;

pub struct TestEnvironment<S> {
    pub cfg: Arc<torrust_tracker_configuration::Configuration>,
    pub tracker: Arc<Tracker>,
    pub state: S,
}

#[allow(dead_code)]
pub struct Stopped<I: HttpServerLauncher> {
    http_server: StoppedHttpServer<I>,
}

pub struct Running<I: HttpServerLauncher> {
    http_server: RunningHttpServer<I>,
}

impl<S> TestEnvironment<S> {
    /// Add a torrent to the tracker
    pub async fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}

impl<I: HttpServerLauncher + 'static> TestEnvironment<Stopped<I>> {
    #[allow(dead_code)]
    pub fn new_stopped(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let cfg = Arc::new(cfg);

        let tracker = setup_with_configuration(&cfg);

        let http_server = http_server(cfg.http_trackers[0].clone());

        Self {
            cfg,
            tracker,
            state: Stopped { http_server },
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> TestEnvironment<Running<I>> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker.clone(),
            state: Running {
                http_server: self.state.http_server.start(self.tracker).await.unwrap(),
            },
        }
    }

    #[allow(dead_code)]
    pub fn config(&self) -> &torrust_tracker_configuration::HttpTracker {
        &self.state.http_server.cfg
    }

    #[allow(dead_code)]
    pub fn config_mut(&mut self) -> &mut torrust_tracker_configuration::HttpTracker {
        &mut self.state.http_server.cfg
    }
}

impl<I: HttpServerLauncher + 'static> TestEnvironment<Running<I>> {
    pub async fn new_running(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let test_env = StoppedTestEnvironment::new_stopped(cfg);

        test_env.start().await
    }

    pub async fn stop(self) -> TestEnvironment<Stopped<I>> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker,
            state: Stopped {
                http_server: self.state.http_server.stop().await.unwrap(),
            },
        }
    }

    pub fn bind_address(&self) -> &std::net::SocketAddr {
        &self.state.http_server.state.bind_addr
    }

    #[allow(dead_code)]
    pub fn config(&self) -> &torrust_tracker_configuration::HttpTracker {
        &self.state.http_server.cfg
    }
}

#[allow(clippy::module_name_repetitions, dead_code)]
pub fn stopped_test_environment<I: HttpServerLauncher + 'static>(
    cfg: torrust_tracker_configuration::Configuration,
) -> StoppedTestEnvironment<I> {
    TestEnvironment::new_stopped(cfg)
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment<I: HttpServerLauncher + 'static>(
    cfg: torrust_tracker_configuration::Configuration,
) -> RunningTestEnvironment<I> {
    TestEnvironment::new_running(cfg).await
}

pub fn http_server<I: HttpServerLauncher + 'static>(cfg: torrust_tracker_configuration::HttpTracker) -> StoppedHttpServer<I> {
    let http_server = I::new();

    HttpServer::new(cfg, http_server)
}
