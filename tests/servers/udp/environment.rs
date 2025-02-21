use std::net::SocketAddr;
use std::sync::Arc;

use bittorrent_primitives::info_hash::InfoHash;
use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::{Configuration, DEFAULT_TIMEOUT};
use torrust_tracker_lib::bootstrap::app::initialize_global_services;
use torrust_tracker_lib::servers::udp::server::spawner::Spawner;
use torrust_tracker_lib::servers::udp::server::states::{Running, Stopped};
use torrust_tracker_lib::servers::udp::server::Server;
use torrust_tracker_primitives::peer;

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

        let bind_to = container.udp_tracker_core_container.udp_tracker_config.bind_address;

        let server = Server::new(Spawner::new(bind_to));

        Self {
            container,
            registar: Registar::default(),
            server,
        }
    }

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
                    self.registar.give_form(),
                    cookie_lifetime,
                )
                .await
                .unwrap(),
        }
    }
}

impl Environment<Running> {
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        tokio::time::timeout(DEFAULT_TIMEOUT, Environment::<Stopped>::new(configuration).start())
            .await
            .expect("it should create an environment within the timeout")
    }

    #[allow(dead_code)]
    pub async fn stop(self) -> Environment<Stopped> {
        let stopped = tokio::time::timeout(DEFAULT_TIMEOUT, self.server.stop())
            .await
            .expect("it should stop the environment within the timeout");

        Environment {
            container: self.container,
            registar: Registar::default(),
            server: stopped.expect("it stop the udp tracker service"),
        }
    }

    pub fn bind_address(&self) -> SocketAddr {
        self.server.state.local_addr
    }
}

pub struct EnvContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
}

impl EnvContainer {
    pub fn initialize(configuration: &Configuration) -> Self {
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_configurations = configuration.udp_trackers.clone().expect("missing UDP tracker configuration");
        let udp_tracker_config = Arc::new(udp_tracker_configurations[0].clone());

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize(&core_config));
        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from(&tracker_core_container, &udp_tracker_config);

        Self {
            tracker_core_container,
            udp_tracker_core_container,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;
    use torrust_tracker_test_helpers::configuration;

    use crate::common::logging;
    use crate::servers::udp::Started;

    #[tokio::test]
    async fn it_should_make_and_stop_udp_server() {
        logging::setup();

        let env = Started::new(&configuration::ephemeral().into()).await;
        sleep(Duration::from_secs(1)).await;
        env.stop().await;
        sleep(Duration::from_secs(1)).await;
    }
}
