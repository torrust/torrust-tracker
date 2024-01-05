use std::sync::Arc;

use futures::executor::block_on;
use torrust_tracker::bootstrap::jobs::make_rust_tls;
use torrust_tracker::core::peer::Peer;
use torrust_tracker::core::Tracker;
use torrust_tracker::servers::http::server::{HttpServer, Launcher, RunningHttpServer, StoppedHttpServer};
use torrust_tracker::servers::registar::Registar;
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
    http_server: StoppedHttpServer,
}

pub struct Running {
    http_server: RunningHttpServer,
}

impl<S> TestEnvironment<S> {
    /// Add a torrent to the tracker
    pub async fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}

impl TestEnvironment<Stopped> {
    #[allow(dead_code)]
    pub fn new_stopped(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let cfg = Arc::new(cfg);

        let tracker = setup_with_configuration(&cfg);

        let config = cfg.http_trackers[0].clone();

        let bind_to = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        let tls = block_on(make_rust_tls(config.ssl_enabled, &config.ssl_cert_path, &config.ssl_key_path))
            .map(|tls| tls.expect("tls config failed"));

        let http_server = HttpServer::new(Launcher::new(bind_to, tls));

        Self {
            cfg,
            tracker,
            state: Stopped { http_server },
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> TestEnvironment<Running> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker.clone(),
            state: Running {
                http_server: self
                    .state
                    .http_server
                    .start(self.tracker, Registar::default().give_form())
                    .await
                    .unwrap(),
            },
        }
    }

    // #[allow(dead_code)]
    // pub fn config(&self) -> &torrust_tracker_configuration::HttpTracker {
    //     &self.state.http_server.cfg
    // }

    // #[allow(dead_code)]
    // pub fn config_mut(&mut self) -> &mut torrust_tracker_configuration::HttpTracker {
    //     &mut self.state.http_server.cfg
    // }
}

impl TestEnvironment<Running> {
    pub async fn new_running(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let test_env = StoppedTestEnvironment::new_stopped(cfg);

        test_env.start().await
    }

    pub async fn stop(self) -> TestEnvironment<Stopped> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker,
            state: Stopped {
                http_server: self.state.http_server.stop().await.unwrap(),
            },
        }
    }

    pub fn bind_address(&self) -> &std::net::SocketAddr {
        &self.state.http_server.state.binding
    }

    // #[allow(dead_code)]
    // pub fn config(&self) -> &torrust_tracker_configuration::HttpTracker {
    //     &self.state.http_server.cfg
    // }
}

#[allow(clippy::module_name_repetitions, dead_code)]
pub fn stopped_test_environment(cfg: torrust_tracker_configuration::Configuration) -> StoppedTestEnvironment {
    TestEnvironment::new_stopped(cfg)
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment(cfg: torrust_tracker_configuration::Configuration) -> RunningTestEnvironment {
    TestEnvironment::new_running(cfg).await
}

#[allow(dead_code)]
pub fn http_server(launcher: Launcher) -> StoppedHttpServer {
    HttpServer::new(launcher)
}
