use std::sync::Arc;

use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use bittorrent_http_tracker_core::statistics::event::listener::run_event_listener;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use futures::executor::block_on;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_axum_server::tsl::make_rust_tls;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::{logging, Configuration};
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
    /// Will panic if it fails to make the TSL config from the configuration.
    #[allow(dead_code)]
    #[must_use]
    pub fn new(configuration: &Arc<Configuration>) -> Self {
        initialize_global_services(configuration);

        let container = Arc::new(EnvContainer::initialize(configuration));

        let bind_to = container.http_tracker_core_container.http_tracker_config.bind_address;

        let tls = block_on(make_rust_tls(
            &container.http_tracker_core_container.http_tracker_config.tsl_config,
        ))
        .map(|tls| tls.expect("tls config failed"));

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
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        Environment::<Stopped>::new(configuration).start().await
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
    /// # Panics
    ///
    /// Will panic if the configuration is missing the HTTP tracker configuration.
    #[must_use]
    pub fn initialize(configuration: &Configuration) -> Self {
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP tracker configuration");
        let http_tracker_config = Arc::new(http_tracker_config[0].clone());

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            configuration.core.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            &core_config,
            &swarm_coordination_registry_container,
        ));

        let http_tracker_container =
            HttpTrackerCoreContainer::initialize_from_tracker_core(&tracker_core_container, &http_tracker_config);

        Self {
            tracker_core_container,
            http_tracker_core_container: http_tracker_container,
        }
    }
}

fn initialize_global_services(configuration: &Configuration) {
    initialize_static();
    logging::setup(&configuration.logging);
}

fn initialize_static() {
    torrust_tracker_clock::initialize_static();
}
