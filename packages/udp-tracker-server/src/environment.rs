use std::net::SocketAddr;
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::{logging, Configuration, DEFAULT_TIMEOUT};
use torrust_tracker_primitives::peer;

use crate::container::UdpTrackerServerContainer;
use crate::server::spawner::Spawner;
use crate::server::states::{Running, Stopped};
use crate::server::Server;

pub type Started = Environment<Running>;

pub struct Environment<S>
where
    S: std::fmt::Debug + std::fmt::Display,
{
    pub container: Arc<EnvContainer>,
    pub registar: Registar,
    pub server: Server<S>,
}

impl<S> Environment<S>
where
    S: std::fmt::Debug + std::fmt::Display,
{
    /// Add a torrent to the tracker
    #[allow(dead_code)]
    pub fn add_torrent(&self, info_hash: &InfoHash, peer: &peer::Peer) {
        let _number_of_downloads_increased = self
            .container
            .tracker_core_container
            .in_memory_torrent_repository
            .upsert_peer(info_hash, peer);
    }
}

impl Environment<Stopped> {
    #[allow(dead_code)]
    #[must_use]
    pub fn new(configuration: &Arc<Configuration>) -> Self {
        initialize_global_services(configuration);

        let container = Arc::new(EnvContainer::initialize(configuration));

        let bind_to = container.udp_tracker_core_container.udp_tracker_config.bind_address;

        let server = Server::new(Spawner::new(bind_to));

        Self {
            container,
            registar: Registar::default(),
            server,
        }
    }

    /// # Panics
    ///
    /// Will panic if it cannot start the server.
    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Running> {
        let cookie_lifetime = self.container.udp_tracker_core_container.udp_tracker_config.cookie_lifetime;

        Environment {
            container: self.container.clone(),
            registar: self.registar.clone(),
            server: self
                .server
                .start(
                    self.container.udp_tracker_core_container.clone(),
                    self.container.udp_tracker_server_container.clone(),
                    self.registar.give_form(),
                    cookie_lifetime,
                )
                .await
                .unwrap(),
        }
    }
}

impl Environment<Running> {
    /// # Panics
    ///
    /// Will panic if it cannot start the server within the timeout.
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        tokio::time::timeout(DEFAULT_TIMEOUT, Environment::<Stopped>::new(configuration).start())
            .await
            .expect("it should create an environment within the timeout")
    }

    /// # Panics
    ///
    /// Will panic if it cannot stop the service within the timeout.
    #[allow(dead_code)]
    pub async fn stop(self) -> Environment<Stopped> {
        let stopped = tokio::time::timeout(DEFAULT_TIMEOUT, self.server.stop())
            .await
            .expect("it should stop the environment within the timeout");

        Environment {
            container: self.container,
            registar: Registar::default(),
            server: stopped.expect("it should stop the udp tracker service"),
        }
    }

    #[must_use]
    pub fn bind_address(&self) -> SocketAddr {
        self.server.state.local_addr
    }
}

pub struct EnvContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
    pub udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
}

impl EnvContainer {
    /// # Panics
    ///
    /// Will panic if the configuration is missing the UDP tracker configuration.
    #[must_use]
    pub fn initialize(configuration: &Configuration) -> Self {
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_configurations = configuration.udp_trackers.clone().expect("missing UDP tracker configuration");
        let udp_tracker_config = Arc::new(udp_tracker_configurations[0].clone());

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(&core_config));
        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from(&tracker_core_container, &udp_tracker_config);
        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

        Self {
            tracker_core_container,
            udp_tracker_core_container,
            udp_tracker_server_container,
        }
    }
}

fn initialize_global_services(configuration: &Configuration) {
    initialize_static();
    logging::setup(&configuration.logging);
}

fn initialize_static() {
    torrust_tracker_clock::initialize_static();
    bittorrent_udp_tracker_core::initialize_static();
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;
    use torrust_tracker_test_helpers::{configuration, logging};

    use crate::environment::Started;

    #[tokio::test]
    async fn it_should_make_and_stop_udp_server() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;
        sleep(Duration::from_secs(1)).await;
        env.stop().await;
        sleep(Duration::from_secs(1)).await;
    }
}
