//! Module to handle the UDP server instances.
use std::fmt::Debug;

use derive_more::derive::Display;
use thiserror::Error;

pub mod bound_socket;
pub mod launcher;
pub mod processor;
pub mod receiver;
pub mod request_buffer;
pub mod spawner;
pub mod states;

/// Error that can occur when starting or stopping the UDP server.
///
/// Some errors triggered while starting the server are:
///
/// - The server cannot bind to the given address.
/// - It cannot get the bound address.
///
/// Some errors triggered while stopping the server are:
///
/// - The [`Server`] cannot send the shutdown signal to the spawned UDP service thread.
#[derive(Debug, Error)]
pub enum UdpError {
    #[error("could not bind UDP tracker listener: {source}")]
    Bind { source: std::io::Error },

    #[error("UDP tracker startup notification was not received: {source}")]
    StartupNotification { source: tokio::sync::oneshot::error::RecvError },

    #[error("UDP tracker launcher failed during startup: {source}")]
    Launcher { source: std::io::Error },

    #[error("could not register UDP tracker service: {source}")]
    Registration {
        source: torrust_server_lib::registar::RegistrationError,
    },

    #[error("Any error to do with starting or stopping the sever")]
    FailedToStartOrStopServer(String),
}

/// A UDP server.
///
/// It's an state machine. Configurations cannot be changed. This struct
/// represents concrete configuration and state. It allows to start and stop the
/// server but always keeping the same configuration.
///
/// > **NOTICE**: if the configurations changes after running the server it will
/// > reset to the initial value after stopping the server. This struct is not
/// > intended to persist configurations between runs.
#[allow(
    clippy::module_name_repetitions,
    reason = "public type is the UDP server module's canonical state controller"
)]
#[derive(Debug, Display)]
pub struct Server<S>
where
    S: std::fmt::Debug + std::fmt::Display,
{
    /// The state of the server: `running` or `stopped`.
    pub state: S,
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, UdpSocket};
    use std::sync::Arc;
    use std::time::Duration;

    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{Registar, RegistrationError, ServiceRegistration};
    use torrust_tracker_configuration::v3_0_0::{Configuration, logging};
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::configuration::ephemeral_public;
    use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;

    use super::spawner::Spawner;
    use super::{Server, UdpError};
    use crate::container::UdpTrackerServerContainer;

    fn initialize_global_services(configuration: &Configuration) {
        initialize_static();
        logging::setup(&configuration.logging);
    }

    fn initialize_static() {
        torrust_clock::initialize_static();
        torrust_tracker_udp_core::initialize_static();
    }

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_public());
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(
            cfg.udp_trackers
                .clone()
                .expect("no UDP services array config provided")
                .first()
                .expect("no UDP test service config provided")
                .clone(),
        );

        initialize_global_services(&cfg);

        let udp_trackers = cfg.udp_trackers.clone().expect("missing UDP trackers configuration");
        let config = &udp_trackers[0];
        let bind_to = config.bind_address;
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let register = &Registar::<RuntimeServiceMetadata>::default();

        let stopped = Server::new(Spawner::new(bind_to));

        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize(
            &core_config,
            &udp_tracker_config,
            cfg.udp_tracker_server.max_connection_id_errors_per_ip,
            configuration_instance_id,
        )
        .await;
        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

        let started = stopped
            .start(
                udp_tracker_core_container,
                udp_tracker_server_container,
                register.give_form(),
                RuntimeServiceMetadata::new(configuration_instance_id),
                config.cookie_lifetime,
                torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
            )
            .await
            .expect("it should start the server");

        let stopped = started.stop().await.expect("it should stop the server");

        tokio::time::sleep(Duration::from_secs(1)).await;

        assert_eq!(stopped.state.spawner.bind_to, bind_to);
    }

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop_with_wait() {
        let cfg = Arc::new(ephemeral_public());
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(
            cfg.udp_trackers
                .clone()
                .expect("no UDP services array config provided")
                .first()
                .expect("no UDP test service config provided")
                .clone(),
        );

        initialize_global_services(&cfg);

        let bind_to = udp_tracker_config.bind_address;
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let register = &Registar::<RuntimeServiceMetadata>::default();

        let stopped = Server::new(Spawner::new(bind_to));

        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize(
            &core_config,
            &udp_tracker_config,
            cfg.udp_tracker_server.max_connection_id_errors_per_ip,
            configuration_instance_id,
        )
        .await;
        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

        let started = stopped
            .start(
                udp_tracker_core_container,
                udp_tracker_server_container,
                register.give_form(),
                RuntimeServiceMetadata::new(configuration_instance_id),
                udp_tracker_config.cookie_lifetime,
                torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
            )
            .await
            .expect("it should start the server");

        tokio::time::sleep(Duration::from_secs(1)).await;

        let stopped = started.stop().await.expect("it should stop the server");

        tokio::time::sleep(Duration::from_secs(1)).await;

        assert_eq!(stopped.state.spawner.bind_to, bind_to);
    }

    #[tokio::test]
    async fn it_should_preserve_registration_error_and_release_listener_when_registration_fails() {
        // Arrange
        let mut cfg = ephemeral_public();
        let reserved_socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("reserve UDP listener address");
        let bind_to = reserved_socket.local_addr().expect("read UDP listener address");
        drop(reserved_socket);
        cfg.udp_trackers.as_mut().expect("test configuration enables UDP")[0].bind_address = bind_to;
        let cfg = Arc::new(cfg);
        let core_config = Arc::new(cfg.core.clone());
        let udp_tracker_config = Arc::new(cfg.udp_trackers.as_ref().expect("test configuration enables UDP")[0].clone());
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);
        let registar = Registar::default();
        registar
            .give_form()
            .register(ServiceRegistration::new(
                ServiceBinding::new(Protocol::UDP, bind_to).expect("UDP service binding should be valid"),
                RuntimeServiceMetadata::new(configuration_instance_id),
                None,
            ))
            .await
            .expect("reserve the UDP service registration");
        initialize_global_services(&cfg);
        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize(
            &core_config,
            &udp_tracker_config,
            cfg.udp_tracker_server.max_connection_id_errors_per_ip,
            configuration_instance_id,
        )
        .await;
        let udp_tracker_server_container = UdpTrackerServerContainer::initialize(&core_config);

        // Act
        let result = Server::new(Spawner::new(bind_to))
            .start(
                udp_tracker_core_container,
                udp_tracker_server_container,
                registar.give_form(),
                RuntimeServiceMetadata::new(configuration_instance_id),
                udp_tracker_config.cookie_lifetime,
                torrust_tracker_udp_core::ConnectionIdValidationPolicy::Strict,
            )
            .await;

        // Assert
        let UdpError::Registration {
            source: RegistrationError::DuplicateBinding(binding),
        } = result.expect_err("duplicate registration should fail")
        else {
            panic!("UDP starter should retain the registration failure source");
        };
        assert_eq!(binding.bind_address(), bind_to);
        UdpSocket::bind(bind_to).expect("UDP listener should be released after registration failure");
    }

    mod token_aware_start {
        use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
        use std::sync::Arc;
        use std::time::Duration;

        use tokio_util::sync::CancellationToken;
        use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
        use torrust_server_lib::registar::{Registar, RegistrationError, ServiceRegistration};
        use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
        use torrust_tracker_test_helpers::configuration::ephemeral_public;
        use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
        use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;

        use super::initialize_global_services;
        use crate::container::UdpTrackerServerContainer;
        use crate::server::spawner::Spawner;
        use crate::server::states::CancellationRunning;
        use crate::server::{Server, UdpError};

        const TEST_COMPLETION_TIMEOUT: Duration = Duration::from_secs(5);

        fn available_udp_address() -> SocketAddr {
            let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("select available UDP address");
            socket.local_addr().expect("read available UDP address")
        }

        fn udp_tracker_metadata() -> RuntimeServiceMetadata {
            RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0))
        }

        /// Everything the UDP server needs to start on a known, currently free address.
        struct StoppedUdpServerOnFreeAddress {
            bind_to: SocketAddr,
            udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
            udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
            cookie_lifetime: Duration,
        }

        impl StoppedUdpServerOnFreeAddress {
            async fn new() -> Self {
                let bind_to = available_udp_address();
                let mut configuration = ephemeral_public();
                configuration.udp_trackers.as_mut().expect("test configuration enables UDP")[0].bind_address = bind_to;
                initialize_global_services(&configuration);
                let core_config = Arc::new(configuration.core.clone());
                let udp_tracker_config =
                    Arc::new(configuration.udp_trackers.as_ref().expect("test configuration enables UDP")[0].clone());
                let udp_tracker_core_container = UdpTrackerCoreContainer::initialize(
                    &core_config,
                    &udp_tracker_config,
                    configuration.udp_tracker_server.max_connection_id_errors_per_ip,
                    ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0),
                )
                .await;

                Self {
                    bind_to,
                    udp_tracker_core_container,
                    udp_tracker_server_container: UdpTrackerServerContainer::initialize(&core_config),
                    cookie_lifetime: udp_tracker_config.cookie_lifetime,
                }
            }

            async fn start(
                self,
                registar: &Registar<RuntimeServiceMetadata>,
                cancellation_token: CancellationToken,
            ) -> Result<CancellationRunning, UdpError> {
                Server::new(Spawner::new(self.bind_to))
                    .start_with_cancellation(
                        self.udp_tracker_core_container,
                        self.udp_tracker_server_container,
                        registar.give_form(),
                        udp_tracker_metadata(),
                        self.cookie_lifetime,
                        ConnectionIdValidationPolicy::Strict,
                        cancellation_token,
                    )
                    .await
            }
        }

        #[tokio::test]
        async fn it_should_stop_the_receive_loop_and_release_the_socket_when_its_cancellation_token_is_cancelled() {
            // Arrange
            let server = StoppedUdpServerOnFreeAddress::new().await;
            let bind_to = server.bind_to;
            let cancellation_token = CancellationToken::new();
            let running = server
                .start(&Registar::default(), cancellation_token.clone())
                .await
                .expect("the token-aware UDP server should start");

            // Act
            cancellation_token.cancel();
            let loop_result = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, running.task)
                .await
                .expect("the receive loop should stop after token cancellation")
                .expect("the receive loop should not panic");

            // Assert
            assert!(
                loop_result.is_ok(),
                "a cancelled receive loop should stop without an error: {loop_result:?}"
            );
            UdpSocket::bind(bind_to).expect("the UDP socket should be released after the receive loop stops");
        }

        #[tokio::test]
        async fn it_should_register_the_service_with_a_health_check_that_reaches_the_running_server() {
            // Arrange
            let server = StoppedUdpServerOnFreeAddress::new().await;
            let registar = Registar::default();
            let cancellation_token = CancellationToken::new();

            // Act
            let running = server
                .start(&registar, cancellation_token.clone())
                .await
                .expect("the token-aware UDP server should start");

            // Assert
            let services = registar.services().await;
            assert_eq!(services.len(), 1, "exactly the started UDP tracker should be registered");
            assert_eq!(services[0].service_binding().bind_address(), running.local_addr);
            let health_check = services[0]
                .spawn_check()
                .expect("the UDP tracker registration should include a health check");
            let health = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, health_check.job)
                .await
                .expect("the health check should finish within the test deadline")
                .expect("the health check task should not panic");
            assert!(
                health.is_ok(),
                "the health check should reach the running UDP server: {health:?}"
            );

            cancellation_token.cancel();
            drop(tokio::time::timeout(TEST_COMPLETION_TIMEOUT, running.task).await);
        }

        #[tokio::test]
        async fn it_should_release_the_socket_when_registration_fails() {
            // Arrange
            let server = StoppedUdpServerOnFreeAddress::new().await;
            let bind_to = server.bind_to;
            let registar = Registar::default();
            registar
                .give_form()
                .register(ServiceRegistration::new(
                    ServiceBinding::new(Protocol::UDP, bind_to).expect("UDP service binding should be valid"),
                    udp_tracker_metadata(),
                    None,
                ))
                .await
                .expect("reserve the UDP service registration");

            // Act
            let result = server.start(&registar, CancellationToken::new()).await;

            // Assert
            let Err(UdpError::Registration {
                source: RegistrationError::DuplicateBinding(binding),
            }) = result
            else {
                panic!("the token-aware starter should retain the registration failure source");
            };
            assert_eq!(binding.bind_address(), bind_to);
            UdpSocket::bind(bind_to).expect("the UDP socket should be released after registration failure");
        }
    }
}

/// Todo: submit test to tokio documentation.
#[cfg(test)]
mod test_tokio {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::sync::Barrier;
    use tokio::task::JoinSet;

    #[tokio::test]
    async fn test_barrier_with_aborted_tasks() {
        // Create a barrier that requires 10 tasks to proceed.
        let barrier = Arc::new(Barrier::new(10));
        let mut tasks = JoinSet::default();
        let mut handles = Vec::default();

        // Set Barrier to 9/10.
        for _ in 0..9 {
            let c = barrier.clone();
            handles.push(tasks.spawn(async move {
                c.wait().await;
            }));
        }

        // Abort two tasks: Barrier: 7/10.
        for _ in 0..2 {
            if let Some(handle) = handles.pop() {
                handle.abort();
            }
        }

        // Spawn a single task: Barrier 8/10.
        let c = barrier.clone();
        handles.push(tasks.spawn(async move {
            c.wait().await;
        }));

        // give a chance fro the barrier to release.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // assert that the barrier isn't removed, i.e. 8, not 10.
        for h in &handles {
            assert!(!h.is_finished());
        }

        // Spawn two more tasks to trigger the barrier release: Barrier 10/10.
        for _ in 0..2 {
            let c = barrier.clone();
            handles.push(tasks.spawn(async move {
                c.wait().await;
            }));
        }

        // give a chance fro the barrier to release.
        tokio::time::sleep(Duration::from_millis(50)).await;

        // assert that the barrier has been triggered
        for h in &handles {
            assert!(h.is_finished());
        }

        tasks.shutdown().await;
    }
}
