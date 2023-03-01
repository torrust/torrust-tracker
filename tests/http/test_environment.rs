use core::panic;
use std::sync::Arc;

use torrust_tracker::http::tracker_interface::{RunningHttpServer, StoppedHttpServer, TrackerInterface, TrackerInterfaceTrait};
use torrust_tracker::protocol::info_hash::InfoHash;
use torrust_tracker::tracker::peer::Peer;
use torrust_tracker::tracker::statistics::Keeper;
use torrust_tracker::tracker::Tracker;
use torrust_tracker::{ephemeral_instance_keys, logging, static_time};
use torrust_tracker_configuration::Configuration;
use torrust_tracker_test_helpers::configuration::ephemeral;

use crate::common::tracker::{tracker_configuration, tracker_instance};

#[allow(clippy::module_name_repetitions, dead_code)]
pub type StoppedTestEnvironment<I> = TestEnvironment<Stopped<I>>;
#[allow(clippy::module_name_repetitions)]
pub type RunningTestEnvironment<I> = TestEnvironment<Running<I>>;

pub struct TestEnvironment<S> {
    pub tracker: Arc<Tracker>,
    pub state: S,
}

#[allow(dead_code)]
pub struct Stopped<I: TrackerInterfaceTrait> {
    http_server: StoppedHttpServer<I>,
}

pub struct Running<I: TrackerInterfaceTrait> {
    http_server: RunningHttpServer<I>,
}

impl<S> TestEnvironment<S> {
    /// Add a torrent to the tracker
    pub async fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &Peer) {
        self.tracker.update_torrent_with_peer_and_get_stats(info_hash, peer).await;
    }
}

impl<I: TrackerInterfaceTrait + 'static> TestEnvironment<Stopped<I>> {
    #[allow(dead_code)]
    pub fn new_stopped() -> Self {
        let cfg = tracker_configuration();

        let tracker = tracker_instance(&cfg);

        let http_server = stopped_http_server(cfg.http_trackers[0].clone());

        Self {
            tracker,
            state: Stopped { http_server },
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> TestEnvironment<Running<I>> {
        TestEnvironment {
            tracker: self.tracker.clone(),
            state: Running {
                http_server: self.state.http_server.start(self.tracker).await.unwrap(),
            },
        }
    }
}

impl<I: TrackerInterfaceTrait + 'static> TestEnvironment<Running<I>> {
    pub async fn new_running() -> Self {
        let test_env = StoppedTestEnvironment::new_stopped();

        test_env.start().await
    }

    pub async fn stop(self) -> TestEnvironment<Stopped<I>> {
        TestEnvironment {
            tracker: self.tracker,
            state: Stopped {
                http_server: self.state.http_server.stop().await.unwrap(),
            },
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub async fn running_test_environment<I: TrackerInterfaceTrait + 'static>() -> RunningTestEnvironment<I> {
    TestEnvironment::new_running().await
}

pub fn stopped_http_server<I: TrackerInterfaceTrait + 'static>(
    cfg: torrust_tracker_configuration::HttpTracker,
) -> StoppedHttpServer<I> {
    let http_server = I::new();

    TrackerInterface::new(cfg, http_server)
}

pub async fn running_http_server<I: TrackerInterfaceTrait + 'static>(
    cfg: torrust_tracker_configuration::HttpTracker,
    tracker: Arc<Tracker>,
) -> RunningHttpServer<I> {
    stopped_http_server(cfg).start(tracker).await.unwrap()
}
