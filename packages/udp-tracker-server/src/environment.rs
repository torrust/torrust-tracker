use std::net::SocketAddr;
use std::sync::Arc;

use bittorrent_tracker_core::container::TrackerCoreContainer;
use bittorrent_udp_tracker_core::container::UdpTrackerCoreContainer;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::{logging, Configuration, DEFAULT_TIMEOUT};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

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
    pub udp_core_event_listener_job: Option<JoinHandle<()>>,
    pub udp_server_stats_event_listener_job: Option<JoinHandle<()>>,
    pub udp_server_banning_event_listener_job: Option<JoinHandle<()>>,
    pub cancellation_token: CancellationToken,
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
            udp_core_event_listener_job: None,
            udp_server_stats_event_listener_job: None,
            udp_server_banning_event_listener_job: None,
            cancellation_token: CancellationToken::new(),
        }
    }

    /// Starts the test environment and return a running environment.
    ///
    /// # Panics
    ///
    /// Will panic if it cannot start the server.
    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Running> {
        let cookie_lifetime = self.container.udp_tracker_core_container.udp_tracker_config.cookie_lifetime;

        // Start the UDP tracker core event listener
        let udp_core_event_listener_job = Some(bittorrent_udp_tracker_core::statistics::event::listener::run_event_listener(
            self.container.udp_tracker_core_container.event_bus.receiver(),
            self.cancellation_token.clone(),
            &self.container.udp_tracker_core_container.stats_repository,
        ));

        // Start the UDP tracker server event listener (statistics)
        let udp_server_stats_event_listener_job = Some(crate::statistics::event::listener::run_event_listener(
            self.container.udp_tracker_server_container.event_bus.receiver(),
            self.cancellation_token.clone(),
            &self.container.udp_tracker_server_container.stats_repository,
        ));

        // Start the UDP tracker server event listener (banning)
        let udp_server_banning_event_listener_job = Some(crate::banning::event::listener::run_event_listener(
            self.container.udp_tracker_server_container.event_bus.receiver(),
            self.cancellation_token.clone(),
            &self.container.udp_tracker_core_container.ban_service,
            &self.container.udp_tracker_server_container.stats_repository,
        ));

        // Start the UDP tracker server
        let server = self
            .server
            .start(
                self.container.udp_tracker_core_container.clone(),
                self.container.udp_tracker_server_container.clone(),
                self.registar.give_form(),
                cookie_lifetime,
            )
            .await
            .expect("Failed to start the UDP tracker server");

        Environment {
            container: self.container.clone(),
            registar: self.registar.clone(),
            server,
            udp_core_event_listener_job,
            udp_server_stats_event_listener_job,
            udp_server_banning_event_listener_job,
            cancellation_token: self.cancellation_token,
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
            .expect("Failed to create a UDP tracker server running environment within the timeout")
    }

    /// Stops the test environment and return a stopped environment.
    ///
    /// # Panics
    ///
    /// Will panic if it cannot stop the service within the timeout.
    #[allow(dead_code)]
    pub async fn stop(self) -> Environment<Stopped> {
        // Stop the UDP tracker core event listener
        if let Some(udp_core_event_listener_job) = self.udp_core_event_listener_job {
            // todo: send a message to the event listener to stop and wait for
            // it to finish
            udp_core_event_listener_job.abort();
        }

        // Stop the UDP tracker server event listener (statistics)
        if let Some(udp_server_stats_event_listener_job) = self.udp_server_stats_event_listener_job {
            // todo: send a message to the event listener to stop and wait for
            // it to finish
            udp_server_stats_event_listener_job.abort();
        }

        // Stop the UDP tracker server event listener (banning)
        if let Some(udp_server_banning_event_listener_job) = self.udp_server_banning_event_listener_job {
            // todo: send a message to the event listener to stop and wait for
            // it to finish
            udp_server_banning_event_listener_job.abort();
        }

        // Stop the UDP tracker server
        let server = tokio::time::timeout(DEFAULT_TIMEOUT, self.server.stop())
            .await
            .expect("Failed to stop the UDP tracker server within the timeout")
            .expect("Failed to stop the UDP tracker server");

        Environment {
            container: self.container,
            registar: Registar::default(),
            server,
            udp_core_event_listener_job: None,
            udp_server_stats_event_listener_job: None,
            udp_server_banning_event_listener_job: None,
            cancellation_token: self.cancellation_token,
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

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(TrackerCoreContainer::initialize_from(
            &core_config,
            &swarm_coordination_registry_container,
        ));

        let udp_tracker_core_container =
            UdpTrackerCoreContainer::initialize_from_tracker_core(&tracker_core_container, &udp_tracker_config);

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
