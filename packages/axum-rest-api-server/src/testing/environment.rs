use std::net::SocketAddr;
use std::sync::Arc;

use secrecy::ExposeSecret;
use torrust_info_hash::InfoHash;
use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_configuration::{Configuration, logging};
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole, peer};
use torrust_tracker_rest_api_client::connection_info::{ConnectionInfo, Origin};
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_server::container::UdpTrackerServerContainer;

use crate::server::{ApiServer, Launcher, Running, Stopped};

pub type Started = Environment<Running>;

pub struct Environment<S>
where
    S: std::fmt::Debug + std::fmt::Display,
{
    pub container: Arc<EnvContainer>,
    pub registar: Registar<RuntimeServiceMetadata>,
    pub server: ApiServer<S>,
}

impl<S> Environment<S>
where
    S: std::fmt::Debug + std::fmt::Display,
{
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
    /// Will panic if it cannot make the TLS configuration from the provided
    /// configuration.
    #[must_use]
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        initialize_global_services(configuration);

        let container = Arc::new(EnvContainer::initialize(configuration).await);

        let bind_to = container.tracker_http_api_core_container.http_api_config.bind_address;

        let tls = if let Some(tls_config) = &container.tracker_http_api_core_container.http_api_config.tsl_config {
            Some(make_rust_tls(tls_config).await.expect("tls config failed"))
        } else {
            None
        };

        let server = ApiServer::new(Launcher::new(bind_to, tls));

        Self {
            container,
            registar: Registar::default(),
            server,
        }
    }

    /// # Panics
    ///
    /// Will panic if the server cannot be started.
    pub async fn start(self) -> Environment<Running> {
        let access_tokens = Arc::new(
            self.container
                .tracker_http_api_core_container
                .http_api_config
                .access_tokens
                .clone(),
        );

        Environment {
            container: self.container.clone(),
            registar: self.registar.clone(),
            server: self
                .server
                .start(
                    self.container.tracker_http_api_core_container.clone(),
                    self.registar.give_form(),
                    RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)),
                    access_tokens,
                )
                .await
                .unwrap(),
        }
    }
}

impl Environment<Running> {
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        Environment::<Stopped>::new(configuration).await.start().await
    }

    /// # Panics
    ///
    /// Will panic if the server cannot be stopped.
    pub async fn stop(self) -> Environment<Stopped> {
        Environment {
            container: self.container,
            registar: Registar::default(),
            server: self.server.stop().await.unwrap(),
        }
    }

    /// # Panics
    ///
    /// Will panic if it cannot build the origin for the connection info from the
    /// server local socket address.
    #[must_use]
    pub fn get_connection_info(&self) -> ConnectionInfo {
        let origin = Origin::new(&format!("http://{}/", self.server.state.local_addr)).unwrap(); // DevSkim: ignore DS137138

        ConnectionInfo {
            origin,
            api_token: self
                .container
                .tracker_http_api_core_container
                .http_api_config
                .access_tokens
                .get("admin")
                .map(|token| token.expose_secret().to_string()),
        }
    }

    #[must_use]
    pub fn bind_address(&self) -> SocketAddr {
        self.server.state.local_addr
    }
}

pub struct EnvContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub tracker_http_api_core_container: Arc<TrackerHttpApiCoreContainer>,
}

impl EnvContainer {
    /// # Panics
    ///
    /// Will panic if:
    ///
    /// - The configuration does not contain a HTTP tracker configuration.
    /// - The configuration does not contain a UDP tracker configuration.
    /// - The configuration does not contain a HTTP API configuration.
    #[must_use]
    pub async fn initialize(configuration: &Configuration) -> Self {
        let core_config = Arc::new(configuration.core.clone());

        let http_tracker_config = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP tracker configuration");
        let http_tracker_config = Arc::new(http_tracker_config[0].clone());

        let udp_tracker_configurations = configuration.udp_trackers.clone().expect("missing UDP tracker configuration");
        let udp_tracker_config = Arc::new(udp_tracker_configurations[0].clone());

        let http_api_config = Arc::new(
            configuration
                .http_api
                .clone()
                .expect("missing HTTP API configuration")
                .clone(),
        );

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container =
            Arc::new(TrackerCoreContainer::initialize_from(&core_config, &swarm_coordination_registry_container).await);

        let http_tracker_core_container = HttpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            &http_tracker_config,
            ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
        );

        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            &udp_tracker_config,
            ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
        );

        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

        let tracker_http_api_core_container = TrackerHttpApiCoreContainer::initialize_from(
            &swarm_coordination_registry_container,
            &tracker_core_container,
            &http_tracker_core_container,
            &udp_tracker_core_container,
            &udp_tracker_server_container,
            &http_api_config,
        );

        Self {
            tracker_core_container,
            tracker_http_api_core_container,
        }
    }
}

fn initialize_global_services(configuration: &Configuration) {
    initialize_static();
    logging::setup(&configuration.logging);
}

fn initialize_static() {
    torrust_clock::initialize_static();
    torrust_tracker_udp_core::initialize_static();
}
