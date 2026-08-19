use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::{Core, UdpTracker};
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;

use crate::container::UdpTrackerServerContainer;
use crate::server::Server;
use crate::server::spawner::Spawner;
use crate::server::states::{Running, Stopped};

const DEFAULT_SERVER_LIFECYCLE_TIMEOUT: Duration = Duration::from_secs(5);

pub type Started = Environment<Running>;
pub type Unstarted = Environment<Stopped>;

pub struct Environment<S>
where
    S: std::fmt::Debug + std::fmt::Display,
{
    pub container: Arc<EnvContainer>,
    pub registar: Registar<RuntimeServiceMetadata>,
    pub server: Server<S>,
    pub udp_core_event_listener_job: Option<JoinHandle<()>>,
    pub udp_server_stats_event_listener_job: Option<JoinHandle<()>>,
    pub udp_server_banning_event_listener_job: Option<JoinHandle<()>>,
    pub cancellation_token: CancellationToken,
    pub connection_id_validation: ConnectionIdValidationPolicy,
}

impl Environment<Stopped> {
    #[allow(dead_code)]
    #[must_use]
    pub async fn new(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>) -> Self {
        initialize_static();

        let container = Arc::new(EnvContainer::initialize(core_config, udp_tracker_config).await);

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
            connection_id_validation: ConnectionIdValidationPolicy::Strict,
            // TODO(#1980): remove this hardcoded fallback once schema v3 is the
            // default. The v3 `UdpTrackerServer` config carries `connection_id_validation`
            // natively; the policy should come from there, not from a separate
            // Environment field.
        }
    }

    /// Sets the connection ID validation policy for this test environment.
    #[must_use]
    #[allow(dead_code)]
    pub fn with_connection_id_validation(mut self, policy: ConnectionIdValidationPolicy) -> Self {
        self.connection_id_validation = policy;
        self
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
        let udp_core_event_listener_job = Some(torrust_tracker_udp_core::statistics::event::listener::run_event_listener(
            self.container.udp_tracker_core_container.event_bus.receiver(),
            self.cancellation_token.clone(),
            &self.container.udp_tracker_core_container.stats_repository,
            [(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0), true)].into(),
        ));

        // Start the UDP tracker server event listener (statistics)
        let udp_server_stats_event_listener_job = Some(crate::statistics::event::listener::run_event_listener(
            self.container.udp_tracker_server_container.event_bus.receiver(),
            self.cancellation_token.clone(),
            &self.container.udp_tracker_server_container.stats_repository,
            [(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0), true)].into(),
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
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0)),
                cookie_lifetime,
                self.connection_id_validation,
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
            connection_id_validation: self.connection_id_validation,
        }
    }
}

impl Environment<Running> {
    /// # Panics
    ///
    /// Will panic if it cannot start the server within the timeout.
    pub async fn new(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>) -> Self {
        tokio::time::timeout(
            DEFAULT_SERVER_LIFECYCLE_TIMEOUT,
            Environment::<Stopped>::new(core_config, udp_tracker_config).await.start(),
        )
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
        let server = tokio::time::timeout(DEFAULT_SERVER_LIFECYCLE_TIMEOUT, self.server.stop())
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
            connection_id_validation: self.connection_id_validation,
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
    #[must_use]
    pub async fn initialize(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>) -> Self {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container =
            Arc::new(TrackerCoreContainer::initialize_from(core_config, &swarm_coordination_registry_container).await);

        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            udp_tracker_config,
            torrust_tracker_primitives::ConfigurationInstanceId::new(torrust_tracker_primitives::ServiceRole::UdpTracker, 0),
        );

        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(core_config);

        Self {
            tracker_core_container,
            udp_tracker_core_container,
            udp_tracker_server_container,
        }
    }
}

fn initialize_static() {
    torrust_clock::initialize_static();
    torrust_tracker_udp_core::initialize_static();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::time::sleep;
    use torrust_tracker_test_helpers::{configuration, logging};

    use super::Started;

    #[tokio::test]
    async fn it_should_make_and_stop_udp_server() {
        logging::setup();

        let cfg = configuration::ephemeral();
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.unwrap()[0].clone());

        let env = Started::new(&core_config, &udp_tracker_config).await;
        sleep(Duration::from_secs(1)).await;
        env.stop().await;
        sleep(Duration::from_secs(1)).await;
    }
}
