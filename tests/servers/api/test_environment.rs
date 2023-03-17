use std::sync::Arc;

use torrust_tracker::servers::apis::server::{ApiServer, RunningApiServer, StoppedApiServer};
use torrust_tracker::shared::bit_torrent::info_hash::InfoHash;
use torrust_tracker::tracker::peer::Peer;
use torrust_tracker::tracker::Tracker;

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
    pub fn new_stopped(cfg: torrust_tracker_configuration::Configuration) -> Self {
        let cfg = Arc::new(cfg);

        let tracker = setup_with_configuration(&cfg);

        let api_server = api_server(cfg.http_api.clone());

        Self {
            cfg,
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

    pub fn config_mut(&mut self) -> &mut torrust_tracker_configuration::HttpApi {
        &mut self.state.api_server.cfg
    }
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
                api_server: self.state.api_server.stop().await.unwrap(),
            },
        }
    }

    pub fn get_connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            bind_address: self.state.api_server.state.bind_addr.to_string(),
            api_token: self.state.api_server.cfg.access_tokens.get("admin").cloned(),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn stopped_test_environment(cfg: torrust_tracker_configuration::Configuration) -> StoppedTestEnvironment {
    TestEnvironment::new_stopped(cfg)
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment(cfg: torrust_tracker_configuration::Configuration) -> RunningTestEnvironment {
    TestEnvironment::new_running(cfg).await
}

pub fn api_server(cfg: torrust_tracker_configuration::HttpApi) -> StoppedApiServer {
    ApiServer::new(cfg)
}
