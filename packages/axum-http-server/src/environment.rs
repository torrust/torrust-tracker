use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_server::tsl::make_rust_tls;
use torrust_tracker_configuration::{Core, HttpTracker};
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_tracker_core::container::HttpTrackerCoreContainer;
use torrust_tracker_http_tracker_core::statistics::event::listener::run_event_listener;
use torrust_tracker_primitives::peer;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::server::{HttpServer, Launcher, Running, Stopped};

pub type Started = Environment<Running>;

pub struct Environment<S> {
    pub container: Arc<EnvContainer>,
    pub registar: Registar,
    pub server: HttpServer<S>,
    pub event_listener_job: Option<JoinHandle<()>>,
    pub cancellation_token: CancellationToken,
}

impl<S> Environment<S> {
    /// Add a torrent to the tracker
    pub async fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &peer::Peer) {
        self.container
            .tracker_core_container
            .in_memory_torrent_repository
            .handle_announcement(info_hash, peer, None)
            .await;
    }
}

impl Environment<Stopped> {
    /// # Panics
    ///
    /// Will panic if it fails to build the TLS config from the `tsl_config` field of `http_tracker_config`.
    #[allow(dead_code)]
    #[must_use]
    pub async fn new(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>) -> Self {
        initialize_static();

        let container = Arc::new(EnvContainer::initialize(core_config, http_tracker_config).await);

        let bind_to = container.http_tracker_core_container.http_tracker_config.bind_address;

        let tls = if let Some(tls_config) = &container.http_tracker_core_container.http_tracker_config.tsl_config {
            Some(make_rust_tls(tls_config).await.expect("tls config failed"))
        } else {
            None
        };

        let server = HttpServer::new(Launcher::new(bind_to, tls));

        Self {
            container,
            registar: Registar::default(),
            server,
            event_listener_job: None,
            cancellation_token: CancellationToken::new(),
        }
    }

    /// Starts the test environment and return a running environment.
    ///
    /// # Panics
    ///
    /// Will panic if the server fails to start.    
    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Running> {
        // Start the event listener
        let event_listener_job = run_event_listener(
            self.container.http_tracker_core_container.event_bus.receiver(),
            self.cancellation_token.clone(),
            &self.container.http_tracker_core_container.stats_repository,
        );

        // Start the server
        let server = self
            .server
            .start(self.container.http_tracker_core_container.clone(), self.registar.give_form())
            .await
            .expect("Failed to start the HTTP tracker server");

        Environment {
            container: self.container.clone(),
            registar: self.registar.clone(),
            server,
            event_listener_job: Some(event_listener_job),
            cancellation_token: self.cancellation_token,
        }
    }
}

impl Environment<Running> {
    pub async fn new(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>) -> Self {
        Environment::<Stopped>::new(core_config, http_tracker_config)
            .await
            .start()
            .await
    }

    /// Stops the test environment and return a stopped environment.
    ///
    /// # Panics
    ///
    /// Will panic if the server fails to stop.
    pub async fn stop(self) -> Environment<Stopped> {
        // Stop the event listener
        if let Some(event_listener_job) = self.event_listener_job {
            // todo: send a message to the event listener to stop and wait for
            // it to finish
            event_listener_job.abort();
        }

        // Stop the server
        let server = self.server.stop().await.expect("Failed to stop the HTTP tracker server");

        Environment {
            container: self.container,
            registar: Registar::default(),
            server,
            event_listener_job: None,
            cancellation_token: self.cancellation_token,
        }
    }

    #[must_use]
    pub fn bind_address(&self) -> &std::net::SocketAddr {
        &self.server.state.binding
    }
}

pub struct EnvContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub http_tracker_core_container: Arc<HttpTrackerCoreContainer>,
}

impl EnvContainer {
    #[must_use]
    pub async fn initialize(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>) -> Self {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container =
            Arc::new(TrackerCoreContainer::initialize_from(core_config, &swarm_coordination_registry_container).await);

        let http_tracker_container =
            HttpTrackerCoreContainer::initialize_from_tracker_core(&tracker_core_container, http_tracker_config);

        Self {
            tracker_core_container,
            http_tracker_core_container: http_tracker_container,
        }
    }
}

fn initialize_static() {
    torrust_clock::initialize_static();
}
