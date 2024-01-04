use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use futures::executor::block_on;
use torrust_tracker::bootstrap::jobs::make_rust_tls;
use torrust_tracker::core::peer::Peer;
use torrust_tracker::core::Tracker;
use torrust_tracker::servers::apis::server::{ApiServer, Launcher, RunningApiServer, StoppedApiServer};
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;

use super::connection_info::ConnectionInfo;
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
    api_server: StoppedApiServer,
}

pub struct Running {
    api_server: RunningApiServer,
}

impl<S> TestEnvironment<S> {
    /// Add a torrent to the tracker
    pub async fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}

impl TestEnvironment<Stopped> {
    pub fn new(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let tracker = setup_with_configuration(&Arc::new(cfg));

        let config = tracker.config.http_api.clone();

        let bind_to = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        let tls = block_on(make_rust_tls(config.ssl_enabled, &config.ssl_cert_path, &config.ssl_key_path))
            .map(|tls| tls.expect("tls config failed"));

        Self::new_stopped(tracker, bind_to, tls)
    }

    pub fn new_stopped(tracker: Arc<Tracker>, bind_to: SocketAddr, tls: Option<RustlsConfig>) -> Self {
        let api_server = api_server(Launcher::new(bind_to, tls));

        Self {
            cfg: tracker.config.clone(),
            tracker,
            state: Stopped { api_server },
        }
    }

    pub async fn start(self) -> TestEnvironment<Running> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker.clone(),
            state: Running {
                api_server: self.state.api_server.start(self.tracker).await.unwrap(),
            },
        }
    }

    // pub fn config_mut(&mut self) -> &mut torrust_tracker_configuration::HttpApi {
    //     &mut self.cfg.http_api
    // }
}

impl TestEnvironment<Running> {
    pub async fn new_running(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let test_env = StoppedTestEnvironment::new(cfg);

        test_env.start().await
    }

    pub async fn stop(self) -> TestEnvironment<Stopped> {
        TestEnvironment {
            cfg: self.cfg,
            tracker: self.tracker,
            state: Stopped {
                api_server: self.state.api_server.stop().await.unwrap(),
            },
        }
    }

    pub fn get_connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            bind_address: self.state.api_server.state.binding.to_string(),
            api_token: self.cfg.http_api.access_tokens.get("admin").cloned(),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[allow(dead_code)]
pub fn stopped_test_environment(cfg: torrust_tracker_configuration::Configuration) -> StoppedTestEnvironment {
    TestEnvironment::new(cfg)
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment(cfg: torrust_tracker_configuration::Configuration) -> RunningTestEnvironment {
    TestEnvironment::new_running(cfg).await
}

pub fn api_server(launcher: Launcher) -> StoppedApiServer {
    ApiServer::new(launcher)
}
