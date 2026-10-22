use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::{CancellationToken, DropGuard};
use torrust_server_lib::registar::Registar;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::udp_tracker::UdpTracker;
use torrust_tracker_configuration::v3_0_0::udp_tracker_server::{
    ConnectionIdValidationPolicy as ConfigurationConnectionIdValidationPolicy, UdpTrackerServer,
};
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_events::drain::DEFAULT_DRAIN_TIMEOUT;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_udp_core::ConnectionIdValidationPolicy;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;

use crate::container::UdpTrackerServerContainer;
use crate::server::Server;
use crate::server::spawner::Spawner;
use crate::server::states::{self, CancellationRunning};

const DEFAULT_SERVER_START_TIMEOUT: Duration = Duration::from_secs(5);

pub type Started = Environment<Running>;
pub type Unstarted = Environment<Stopped>;

/// A test environment with no UDP tracker running.
pub struct Stopped {
    server: Server<states::Stopped>,
}

/// A test environment whose UDP receive loop and event listeners run on the
/// token-aware lifecycle.
///
/// The receive loop and the listeners have independent cancellation tokens,
/// like components of the tracker application: `stop()` cancels the receive
/// loop and waits for it, and only then cancels the listeners, so that the
/// events of requests that complete while the loop drains are still processed.
///
/// The environment owns every task it started. Dropping it without `stop()`
/// cancels both tokens, but does not wait for the tasks.
pub struct Running {
    server: CancellationRunning,
    bind_to: SocketAddr,
    event_listeners: OwnedEventListeners,
    cancel_server_on_drop: DropGuard,
    cancel_listeners_on_drop: DropGuard,
}

/// The three event listeners a running environment owns.
struct OwnedEventListeners {
    core_statistics: JoinHandle<()>,
    server_statistics: JoinHandle<()>,
    server_banning: JoinHandle<()>,
}

pub struct Environment<S> {
    pub container: Arc<EnvContainer>,
    pub registar: Registar<RuntimeServiceMetadata>,
    pub connection_id_validation: ConnectionIdValidationPolicy,
    state: S,
}

impl Environment<Stopped> {
    /// Creates an environment using the global UDP server configuration.
    #[allow(dead_code)]
    #[must_use]
    pub async fn new_with_udp_tracker_server_config(
        core_config: &Arc<Core>,
        udp_tracker_config: &Arc<UdpTracker>,
        udp_tracker_server_config: &UdpTrackerServer,
    ) -> Self {
        initialize_static();

        let container = Arc::new(
            EnvContainer::initialize(
                core_config,
                udp_tracker_config,
                udp_tracker_server_config.max_connection_id_errors_per_ip,
            )
            .await,
        );

        let bind_to = container.udp_tracker_core_container.udp_tracker_config.bind_address;

        Self {
            container,
            registar: Registar::default(),
            connection_id_validation: connection_id_validation_policy(udp_tracker_server_config),
            state: Stopped {
                server: Server::new(Spawner::new(bind_to)),
            },
        }
    }

    /// Creates an environment with the default global UDP server configuration.
    #[must_use]
    pub async fn new(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>) -> Self {
        Self::new_with_udp_tracker_server_config(core_config, udp_tracker_config, &UdpTrackerServer::default()).await
    }

    /// Sets the connection ID validation policy for this test environment.
    #[must_use]
    #[allow(dead_code)]
    pub const fn with_connection_id_validation(mut self, policy: ConnectionIdValidationPolicy) -> Self {
        self.connection_id_validation = policy;
        self
    }

    /// Starts the test environment and return a running environment.
    ///
    /// Each start uses a fresh cancellation token, so a stopped environment
    /// can be started again.
    ///
    /// # Panics
    ///
    /// Will panic if it cannot start the server.
    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Running> {
        let server_token = CancellationToken::new();
        let listeners_token = CancellationToken::new();
        let bind_to = self.state.server.state.spawner.bind_to;
        let cookie_lifetime = self.container.udp_tracker_core_container.udp_tracker_config.cookie_lifetime;

        // Subscribe before the server can publish, but spawn the listeners only after a successful start,
        // so a failed start leaves no detached task behind.
        let udp_core_events = self.container.udp_tracker_core_container.event_bus.receiver();
        let udp_server_statistics_events = self.container.udp_tracker_server_container.event_bus.receiver();
        let udp_server_banning_events = self.container.udp_tracker_server_container.event_bus.receiver();

        let server = self
            .state
            .server
            .start_with_cancellation(
                self.container.udp_tracker_core_container.clone(),
                self.container.udp_tracker_server_container.clone(),
                self.registar.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0)),
                cookie_lifetime,
                self.connection_id_validation,
                server_token.clone(),
            )
            .await
            .expect("Failed to start the UDP tracker server");

        let event_listeners = OwnedEventListeners {
            core_statistics: torrust_tracker_udp_core::statistics::event::listener::run_event_listener(
                udp_core_events,
                listeners_token.clone(),
                &self.container.udp_tracker_core_container.stats_repository,
                [(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0), true)].into(),
                DEFAULT_DRAIN_TIMEOUT,
            ),
            server_statistics: crate::statistics::event::listener::run_event_listener(
                udp_server_statistics_events,
                listeners_token.clone(),
                &self.container.udp_tracker_server_container.stats_repository,
                [(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0), true)].into(),
                DEFAULT_DRAIN_TIMEOUT,
            ),
            server_banning: crate::banning::event::listener::run_event_listener(
                udp_server_banning_events,
                listeners_token.clone(),
                &self.container.udp_tracker_core_container.ban_service,
                &self.container.udp_tracker_server_container.banned_ips_publication_order,
                &self.container.udp_tracker_server_container.stats_repository,
                DEFAULT_DRAIN_TIMEOUT,
            ),
        };

        Environment {
            container: self.container,
            registar: self.registar,
            connection_id_validation: self.connection_id_validation,
            state: Running {
                server,
                bind_to,
                event_listeners,
                cancel_server_on_drop: server_token.drop_guard(),
                cancel_listeners_on_drop: listeners_token.drop_guard(),
            },
        }
    }
}

impl Environment<Running> {
    /// # Panics
    ///
    /// Will panic if it cannot start the server within the timeout.
    pub async fn new_with_udp_tracker_server_config(
        core_config: &Arc<Core>,
        udp_tracker_config: &Arc<UdpTracker>,
        udp_tracker_server_config: &UdpTrackerServer,
    ) -> Self {
        tokio::time::timeout(
            DEFAULT_SERVER_START_TIMEOUT,
            Environment::<Stopped>::new_with_udp_tracker_server_config(
                core_config,
                udp_tracker_config,
                udp_tracker_server_config,
            )
            .await
            .start(),
        )
        .await
        .expect("Failed to create a UDP tracker server running environment within the timeout")
    }

    /// Creates an environment with the default global UDP server configuration.
    #[must_use]
    pub async fn new(core_config: &Arc<Core>, udp_tracker_config: &Arc<UdpTracker>) -> Self {
        Self::new_with_udp_tracker_server_config(core_config, udp_tracker_config, &UdpTrackerServer::default()).await
    }

    /// Stops the test environment and return a stopped environment.
    ///
    /// It cancels the receive loop token and joins the receive loop, so every
    /// request it accepted has published its events. Only then it cancels the
    /// listeners token and joins the three event listeners, which process the
    /// events still queued for them (see #2410). It has no timeout of its own:
    /// the receive loop's request drain is bounded and the listeners end on
    /// cancellation.
    ///
    /// # Panics
    ///
    /// Will panic, after every owned task has been joined, if any task failed.
    /// The message names each failing task.
    pub async fn stop(self) -> Environment<Stopped> {
        let Running {
            server,
            bind_to,
            event_listeners,
            cancel_server_on_drop,
            cancel_listeners_on_drop,
        } = self.state;

        cancel_server_on_drop.disarm().cancel();

        join_owned_tasks(server.task, cancel_listeners_on_drop, event_listeners)
            .await
            .unwrap_or_else(|failures| panic!("Failed to stop the UDP test environment: {failures}"));

        Environment {
            container: self.container,
            registar: Registar::default(),
            connection_id_validation: self.connection_id_validation,
            state: Stopped {
                server: Server::new(Spawner::new(bind_to)),
            },
        }
    }

    #[must_use]
    pub const fn bind_address(&self) -> SocketAddr {
        self.state.server.local_addr
    }
}

const fn connection_id_validation_policy(policy: &UdpTrackerServer) -> ConnectionIdValidationPolicy {
    match policy.connection_id_validation {
        ConfigurationConnectionIdValidationPolicy::Strict => ConnectionIdValidationPolicy::Strict,
        ConfigurationConnectionIdValidationPolicy::Disabled => ConnectionIdValidationPolicy::Disabled,
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
    /// Panics if the persistence-required tracker-core test container cannot
    /// be composed.
    #[must_use]
    pub async fn initialize(
        core_config: &Arc<Core>,
        udp_tracker_config: &Arc<UdpTracker>,
        max_connection_id_errors_per_ip: u32,
    ) -> Self {
        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                core_config,
                &swarm_coordination_registry_container,
                core_config.database.as_ref(),
            )
            .await
            .expect("UDP server test initialization requires persistence"),
        );

        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            udp_tracker_config,
            max_connection_id_errors_per_ip,
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
}

/// Joins every task a running environment owns, in the order that protects the
/// events, and reports all failures at once, so an early failure never leaves a
/// remaining task detached.
///
/// The receive loop is joined first. Only then the listeners are cancelled and
/// joined. The guard stays armed until then, so abandoning this future before
/// the receive loop ends still cancels the listeners.
async fn join_owned_tasks(
    receive_loop: JoinHandle<Result<(), io::Error>>,
    cancel_listeners_on_drop: DropGuard,
    event_listeners: OwnedEventListeners,
) -> Result<(), String> {
    let receive_loop_result = receive_loop.await;
    cancel_listeners_on_drop.disarm().cancel();
    let (udp_core_statistics, udp_server_statistics, udp_server_banning) = tokio::join!(
        event_listeners.core_statistics,
        event_listeners.server_statistics,
        event_listeners.server_banning,
    );

    let mut failures = Vec::new();

    match receive_loop_result {
        Ok(Ok(())) => {}
        Ok(Err(error)) => failures.push(format!("UDP receive loop stopped with an error: {error}")),
        Err(error) => failures.push(format!("UDP receive loop failed to join: {error}")),
    }

    for (task, result) in [
        ("UDP core statistics event listener", udp_core_statistics),
        ("UDP server statistics event listener", udp_server_statistics),
        ("UDP server banning event listener", udp_server_banning),
    ] {
        if let Err(error) = result {
            failures.push(format!("{task} failed to join: {error}"));
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::net::{SocketAddr, UdpSocket};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_tracker_client::udp::client::UdpTrackerClient;
    use torrust_tracker_test_helpers::configuration;
    use torrust_tracker_udp_protocol::{ConnectRequest, Response, TransactionId};

    use super::{
        ConfigurationConnectionIdValidationPolicy, ConnectionIdValidationPolicy, EnvContainer, OwnedEventListeners, Started,
        UdpTrackerServer, Unstarted, connection_id_validation_policy, join_owned_tasks,
    };

    const TEST_DEADLINE: Duration = Duration::from_secs(10);

    async fn unstarted_environment() -> Unstarted {
        let configuration = configuration::ephemeral();
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_config = Arc::new(configuration.udp_trackers.expect("test configuration enables UDP")[0].clone());

        Unstarted::new(&core_config, &udp_tracker_config).await
    }

    async fn unstarted_environment_bound_to(bind_address: SocketAddr) -> Unstarted {
        let mut configuration = configuration::ephemeral();
        configuration.udp_trackers.as_mut().expect("test configuration enables UDP")[0].bind_address = bind_address;
        let core_config = Arc::new(configuration.core.clone());
        let udp_tracker_config = Arc::new(configuration.udp_trackers.expect("test configuration enables UDP")[0].clone());

        Unstarted::new(&core_config, &udp_tracker_config).await
    }

    async fn start_within_deadline(environment: Unstarted) -> Started {
        tokio::time::timeout(TEST_DEADLINE, environment.start())
            .await
            .expect("start() should finish within the test deadline")
    }

    async fn stop_within_deadline(environment: Started) -> Unstarted {
        tokio::time::timeout(TEST_DEADLINE, environment.stop())
            .await
            .expect("stop() should finish within the test deadline")
    }

    /// Holders of the shared services each event listener keeps a clone of.
    fn listener_shared_state_holders(container: &EnvContainer) -> [usize; 3] {
        [
            Arc::strong_count(&container.udp_tracker_core_container.stats_repository),
            Arc::strong_count(&container.udp_tracker_server_container.stats_repository),
            Arc::strong_count(&container.udp_tracker_core_container.ban_service),
        ]
    }

    #[test]
    fn it_should_map_strict_connection_id_validation_policy() {
        let configuration = UdpTrackerServer {
            connection_id_validation: ConfigurationConnectionIdValidationPolicy::Strict,
            ..UdpTrackerServer::default()
        };

        assert_eq!(
            ConnectionIdValidationPolicy::Strict,
            connection_id_validation_policy(&configuration)
        );
    }

    #[test]
    fn it_should_map_disabled_connection_id_validation_policy() {
        let configuration = UdpTrackerServer {
            connection_id_validation: ConfigurationConnectionIdValidationPolicy::Disabled,
            ..UdpTrackerServer::default()
        };

        assert_eq!(
            ConnectionIdValidationPolicy::Disabled,
            connection_id_validation_policy(&configuration)
        );
    }

    #[tokio::test]
    async fn it_should_stop_without_a_join_failure_when_every_owned_task_finishes_through_cancellation() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;

        // Act
        let stop = tokio::spawn(stop_within_deadline(environment)).await;

        // Assert
        assert!(
            stop.is_ok(),
            "stop() should join the receive loop and the three listeners without a join failure"
        );
    }

    #[tokio::test]
    async fn it_should_not_leave_event_listeners_running_when_the_udp_server_fails_to_start() {
        // Arrange
        let occupied_port = UdpSocket::bind("127.0.0.1:0").expect("occupy a local UDP port");
        let environment = unstarted_environment_bound_to(occupied_port.local_addr().unwrap()).await;
        // Keeping the container keeps its event buses open, so a detached listener would keep running.
        let container = environment.container.clone();
        let holders_before_start = listener_shared_state_holders(&container);

        // Act
        let start = tokio::time::timeout(TEST_DEADLINE, tokio::spawn(environment.start()))
            .await
            .expect("start() should fail within the test deadline");

        // Assert
        let Err(start_failure) = start else {
            panic!("start() should fail when its port is occupied");
        };
        let panic = start_failure.into_panic();
        let panic_message = panic.downcast_ref::<String>().map_or("", String::as_str);
        assert!(
            panic_message.starts_with("Failed to start the UDP tracker server"),
            "start() should fail because the server could not start, not for another reason: {panic_message}"
        );
        assert_eq!(
            listener_shared_state_holders(&container),
            holders_before_start,
            "an extra holder of a listener's shared state is a detached listener"
        );
    }

    #[tokio::test]
    async fn it_should_release_the_udp_socket_when_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let binding = environment.bind_address();

        // Act
        let _stopped = stop_within_deadline(environment).await;

        // Assert
        UdpSocket::bind(binding).expect("the UDP socket should be free as soon as stop() returns");
    }

    #[tokio::test]
    async fn it_should_release_the_udp_socket_when_dropped_without_being_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let binding = environment.bind_address();

        // Act
        drop(environment);

        // Assert
        // Dropping cannot await the receive loop, so the socket is released shortly after, not at once.
        tokio::time::timeout(TEST_DEADLINE, async {
            while UdpSocket::bind(binding).is_err() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("the UDP socket should be released within the test deadline after the environment is dropped");
    }

    #[tokio::test]
    async fn it_should_serve_requests_after_being_stopped_and_started_again() {
        // Arrange
        let stopped = stop_within_deadline(start_within_deadline(unstarted_environment().await).await).await;

        // Act
        let restarted = start_within_deadline(stopped).await;

        // Assert
        let client = UdpTrackerClient::new(restarted.bind_address(), TEST_DEADLINE)
            .await
            .expect("a client should reach the restarted UDP tracker");
        client
            .send(
                ConnectRequest {
                    transaction_id: TransactionId::new(1),
                }
                .into(),
            )
            .await
            .expect("the connect request should be sent");
        let response = client.receive().await.expect("the restarted UDP tracker should answer");
        assert!(matches!(response, Response::Connect(_)), "unexpected response: {response:?}");

        stop_within_deadline(restarted).await;
    }

    /// Lets every other runnable task run: on the test's current-thread runtime a task cannot
    /// make progress while the test itself runs.
    async fn let_the_other_tasks_run() {
        for _ in 0..16 {
            tokio::task::yield_now().await;
        }
    }

    /// Three listener tasks that finish when `token` is cancelled.
    fn listeners_waiting_for(token: &CancellationToken) -> OwnedEventListeners {
        let listener = || {
            let token = token.clone();
            tokio::spawn(async move { token.cancelled().await })
        };
        OwnedEventListeners {
            core_statistics: listener(),
            server_statistics: listener(),
            server_banning: listener(),
        }
    }

    #[tokio::test]
    async fn it_should_count_the_statistics_events_of_a_request_answered_just_before_stop() {
        // Arrange
        // A client has received the answer to a connect request, so the receive loop has handled it
        // and published its events.
        let environment = start_within_deadline(unstarted_environment().await).await;
        let container = environment.container.clone();
        let client = UdpTrackerClient::new(environment.bind_address(), TEST_DEADLINE)
            .await
            .expect("a client should reach the UDP tracker");
        client
            .send(
                ConnectRequest {
                    transaction_id: TransactionId::new(1),
                }
                .into(),
            )
            .await
            .expect("the connect request should be sent");
        client
            .receive()
            .await
            .expect("the UDP tracker should answer the connect request");

        // Act
        stop_within_deadline(environment).await;

        // Assert
        // The events of the answered request are counted by both statistics listeners once stop() returns.
        let connections_handled = container
            .udp_tracker_core_container
            .stats_repository
            .read_metrics(torrust_tracker_udp_core::statistics::metrics::Metrics::udp4_connections_handled)
            .await;
        let requests_received = container
            .udp_tracker_server_container
            .stats_repository
            .read_metrics(crate::statistics::metrics::Metrics::udp4_requests_received_total)
            .await;
        assert_eq!(
            connections_handled, 1,
            "the udp-core listener must have counted the connect request before stop() returned"
        );
        assert_eq!(
            requests_received, 1,
            "the udp-server listener must have counted the received request before stop() returned"
        );
    }

    #[tokio::test]
    async fn it_should_cancel_the_event_listeners_only_after_the_receive_loop_has_finished() {
        // Arrange
        // The receive loop finishes only when the test releases it, like a loop that is still draining
        // its requests. The three listeners wait for their own stop request.
        let listeners_token = CancellationToken::new();
        let (release_receive_loop, receive_loop_released) = tokio::sync::oneshot::channel::<()>();
        let receive_loop = tokio::spawn(async move {
            let _ = receive_loop_released.await;
            Ok(())
        });
        let event_listeners = listeners_waiting_for(&listeners_token);

        // Act
        let join = tokio::spawn(join_owned_tasks(
            receive_loop,
            listeners_token.clone().drop_guard(),
            event_listeners,
        ));
        let_the_other_tasks_run().await;
        let cancelled_while_the_receive_loop_runs = listeners_token.is_cancelled();
        release_receive_loop
            .send(())
            .expect("the receive loop is waiting for its release");
        let result = join.await.expect("joining must not panic");

        // Assert
        // The listeners are cancelled only once the receive loop finished, and are joined too.
        assert!(
            !cancelled_while_the_receive_loop_runs,
            "the listeners must not be cancelled while the receive loop runs"
        );
        assert!(
            listeners_token.is_cancelled(),
            "the listeners must be cancelled once the receive loop finished"
        );
        assert_eq!(result, Ok(()), "every task finished normally");
    }

    #[tokio::test]
    async fn it_should_cancel_the_event_listeners_when_the_join_is_abandoned_while_the_receive_loop_runs() {
        // Arrange
        // The receive loop never finishes. The listeners wait for their own stop request.
        let listeners_token = CancellationToken::new();
        let receive_loop = tokio::spawn(std::future::pending::<Result<(), io::Error>>());
        let event_listeners = listeners_waiting_for(&listeners_token);

        // Act
        // The caller gives up on the join, for example because its own deadline expired.
        let abandoned = tokio::time::timeout(
            Duration::from_millis(10),
            join_owned_tasks(receive_loop, listeners_token.clone().drop_guard(), event_listeners),
        )
        .await;

        // Assert
        // Abandoning the join must not leave the listeners running detached.
        assert!(abandoned.is_err(), "the join should still be waiting for the receive loop");
        assert!(
            listeners_token.is_cancelled(),
            "dropping the join before the receive loop ends must still cancel the listeners"
        );
    }

    #[tokio::test]
    async fn it_should_stop_the_event_listeners_when_dropped_without_being_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let container = environment.container.clone();
        let holders_while_running = listener_shared_state_holders(&container);

        // Act
        drop(environment);

        // Assert
        // Dropping cannot await the listeners, so they stop shortly after. While they run they hold the
        // shared services, so the holders of those services, counted before the drop, show that they ran,
        // and counted after it, whether they stopped.
        assert!(
            holders_while_running.iter().all(|holders| *holders > 1),
            "the running listeners should hold the shared services: {holders_while_running:?}"
        );
        tokio::time::timeout(TEST_DEADLINE, async {
            while listener_shared_state_holders(&container).iter().any(|holders| *holders > 1) {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("every event listener should stop within the test deadline after the environment is dropped");
    }

    #[tokio::test]
    async fn it_should_join_every_event_listener_before_reporting_a_receive_loop_failure() {
        // Arrange
        let cancellation_token = CancellationToken::new();
        let finished_listeners = Arc::new(AtomicUsize::new(0));

        let failed_receive_loop = tokio::spawn(async { Err(io::Error::other("simulated receive-loop failure")) });
        let listener_finishing_after_the_receive_loop = || {
            let cancellation_token = cancellation_token.clone();
            let finished_listeners = finished_listeners.clone();
            tokio::spawn(async move {
                cancellation_token.cancelled().await;
                // Finish after the failed receive loop, so returning on its failure would see this unfinished.
                tokio::task::yield_now().await;
                finished_listeners.fetch_add(1, Ordering::SeqCst);
            })
        };
        let event_listeners = OwnedEventListeners {
            core_statistics: listener_finishing_after_the_receive_loop(),
            server_statistics: listener_finishing_after_the_receive_loop(),
            server_banning: listener_finishing_after_the_receive_loop(),
        };

        // Act
        let result = tokio::time::timeout(
            TEST_DEADLINE,
            join_owned_tasks(failed_receive_loop, cancellation_token.drop_guard(), event_listeners),
        )
        .await
        .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        let failures = result.expect_err("a failed receive loop should be reported");
        assert_eq!(
            failures, "UDP receive loop stopped with an error: simulated receive-loop failure",
            "only the receive-loop failure should be reported"
        );
        assert_eq!(
            finished_listeners.load(Ordering::SeqCst),
            3,
            "all three event listeners should have finished before the failure was reported"
        );
    }
}
