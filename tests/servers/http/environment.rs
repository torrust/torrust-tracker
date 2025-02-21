use std::sync::Arc;

use bittorrent_http_tracker_core::container::HttpTrackerCoreContainer;
use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use futures::executor::block_on;
use torrust_axum_http_tracker_server::server::{HttpServer, Launcher, Running, Stopped};
use torrust_axum_server::tsl::make_rust_tls;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::Configuration;
use torrust_tracker_lib::bootstrap::app::initialize_global_services;
use torrust_tracker_primitives::peer;

pub struct Environment<S> {
    pub container: Arc<EnvContainer>,
    pub registar: Registar,
    pub server: HttpServer<S>,
}

impl<S> Environment<S> {
    /// Add a torrent to the tracker
    pub fn add_torrent_peer(&self, info_hash: &InfoHash, peer: &peer::Peer) {
        let () = self
            .container
            .tracker_core_container
            .in_memory_torrent_repository
            .upsert_peer(info_hash, peer);
    }
}

impl Environment<Stopped> {
    #[allow(dead_code)]
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
        }
    }

    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Running> {
        Environment {
            container: self.container.clone(),
            registar: self.registar.clone(),
            server: self
                .server
                .start(self.container.http_tracker_core_container.clone(), self.registar.give_form())
                .await
                .unwrap(),
        }
    }
}

impl Environment<Running> {
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        Environment::<Stopped>::new(configuration).start().await
    }

    pub async fn stop(self) -> Environment<Stopped> {
        Environment {
            container: self.container,
            registar: Registar::default(),
            server: self.server.stop().await.unwrap(),
        }
    }

    pub fn bind_address(&self) -> &std::net::SocketAddr {
        &self.server.state.binding
    }
}

pub struct EnvContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub http_tracker_core_container: Arc<HttpTrackerCoreContainer>,
}

impl EnvContainer {
    pub fn initialize(configuration: &Configuration) -> Self {
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = configuration
            .http_trackers
            .clone()
            .expect("missing HTTP tracker configuration");
        let http_tracker_config = Arc::new(http_tracker_config[0].clone());

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(&core_config));
        let http_tracker_container = HttpTrackerCoreContainer::initialize_from(&tracker_core_container, &http_tracker_config);

        Self {
            tracker_core_container,
            http_tracker_core_container: http_tracker_container,
        }
    }
}
