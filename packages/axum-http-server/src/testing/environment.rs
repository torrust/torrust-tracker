use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::{CancellationToken, DropGuard};
use torrust_info_hash::InfoHash;
use torrust_server_lib::registar::{FnSpawnServiceHeathCheck, Registar};
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_configuration::v3_0_0::shutdown::Shutdown;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_http_core::statistics::event::listener::{DEFAULT_DRAIN_TIMEOUT, run_event_listener};
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole, peer};
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::server::{self, CancellationRunning, HttpServer, Launcher};

pub type Unstarted = Environment<Stopped>;
pub type Started = Environment<Running>;

/// A test environment with no HTTP tracker running.
pub struct Stopped {
    server: HttpServer<server::Stopped>,
}

/// A test environment whose HTTP tracker and statistics event listener run
/// on the token-aware lifecycle. The environment owns every task it started.
///
/// The server and the listener have independent cancellation tokens, like
/// components of the tracker application: `stop()` cancels the server and waits
/// for it, and only then cancels the listener, so that the events of requests
/// that complete while the server drains are still processed.
///
/// Dropping it without `stop()` cancels both tokens, and the server shuts
/// down once it observes it, so the binding is released shortly after the drop.
pub struct Running {
    server: CancellationRunning,
    event_listener_job: JoinHandle<()>,
    cancel_server_on_drop: DropGuard,
    cancel_listener_on_drop: DropGuard,
}

pub struct Environment<S> {
    pub container: Arc<EnvContainer>,
    pub registar: Registar<RuntimeServiceMetadata>,
    state: S,
}

impl<S: Sync> Environment<S> {
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

        let tls = if let Some(tls_config) = &container.http_tracker_core_container.http_tracker_config.tls_config {
            Some(make_rust_tls(tls_config).await.expect("tls config failed"))
        } else {
            None
        };

        let server = HttpServer::new(Launcher::new(
            bind_to,
            tls,
            container.http_tracker_core_container.http_tracker_config.network.ipv6_v6only,
            Shutdown::default().http_drain_in_secs.as_duration(),
        ));

        Self {
            container,
            registar: Registar::default(),
            state: Stopped { server },
        }
    }

    /// Starts the test environment and return a running environment.
    ///
    /// # Panics
    ///
    /// Will panic if the server fails to start.    
    #[allow(dead_code)]
    pub async fn start(self) -> Environment<Running> {
        self.start_with_health_check(crate::server::check_fn).await
    }

    /// Starts the environment with the supplied health-check callback.
    ///
    /// Each start uses a fresh cancellation token, so a stopped environment
    /// can be started again.
    ///
    /// # Panics
    ///
    /// Panics if the HTTP tracker server fails to start or register with the
    /// test registry.
    pub async fn start_with_health_check(self, health_check: FnSpawnServiceHeathCheck) -> Environment<Running> {
        let server_token = CancellationToken::new();
        let listener_token = CancellationToken::new();

        // Subscribe before the server can publish, but spawn the listener only after a successful start,
        // so a failed start leaves no detached task behind.
        let event_receiver = self.container.http_tracker_core_container.event_bus.receiver();

        let server = self
            .state
            .server
            .start_with_cancellation_and_health_check(
                self.container.http_tracker_core_container.clone(),
                self.registar.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0)),
                server_token.clone(),
                health_check,
            )
            .await
            .expect("Failed to start the HTTP tracker server");

        let event_listener_job = run_event_listener(
            event_receiver,
            listener_token.clone(),
            &self.container.http_tracker_core_container.stats_repository,
            [(ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0), true)].into(),
            DEFAULT_DRAIN_TIMEOUT,
        );

        Environment {
            container: self.container,
            registar: self.registar,
            state: Running {
                server,
                event_listener_job,
                cancel_server_on_drop: server_token.drop_guard(),
                cancel_listener_on_drop: listener_token.drop_guard(),
            },
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
    /// It cancels the server token and joins the HTTP server task and its drain
    /// controller, so every request the server accepted has published its
    /// events. Only then it cancels the listener token and joins the statistics
    /// event listener, which processes the events still queued for it (see
    /// #2410).
    ///
    /// # Panics
    ///
    /// Will panic, after every owned task has been joined, if any task failed
    /// or the HTTP drain timed out. The message names each failing task.
    pub async fn stop(self) -> Environment<Stopped> {
        let Running {
            server:
                CancellationRunning {
                    task,
                    shutdown_controller,
                    ..
                },
            event_listener_job,
            cancel_server_on_drop,
            cancel_listener_on_drop,
        } = self.state;

        cancel_server_on_drop.disarm().cancel();

        let launcher = join_owned_tasks(task, shutdown_controller, cancel_listener_on_drop, event_listener_job)
            .await
            .unwrap_or_else(|failures| panic!("Failed to stop the HTTP test environment: {failures}"));

        Environment {
            container: self.container,
            registar: Registar::default(),
            state: Stopped {
                server: HttpServer::new(launcher),
            },
        }
    }

    #[must_use]
    pub const fn bind_address(&self) -> &std::net::SocketAddr {
        &self.state.server.binding
    }

    /// Returns the base URL for the HTTP tracker.
    ///
    /// # Panics
    ///
    /// Will panic if the socket address cannot be parsed into a URL.
    #[must_use]
    pub fn base_url(&self) -> reqwest::Url {
        reqwest::Url::parse(&format!("http://{}/", self.bind_address())).unwrap() // DevSkim: ignore DS137138
    }
}

pub struct EnvContainer {
    pub tracker_core_container: Arc<TrackerCoreContainer>,
    pub http_tracker_core_container: Arc<HttpTrackerCoreContainer>,
}

impl EnvContainer {
    /// # Panics
    ///
    /// Panics if the persistence-required tracker-core test container cannot
    /// be composed.
    #[must_use]
    pub async fn initialize(core_config: &Arc<Core>, http_tracker_config: &Arc<HttpTracker>) -> Self {
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
            .expect("HTTP server test initialization requires persistence"),
        );

        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let http_tracker_container = HttpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            http_tracker_config,
            configuration_instance_id,
        );

        Self {
            tracker_core_container,
            http_tracker_core_container: http_tracker_container,
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
/// The server and its drain controller are joined first. Only then the
/// listener is cancelled and joined. The guard stays armed until then, so
/// abandoning this future before the server stage ends still cancels the
/// listener.
async fn join_owned_tasks(
    server_task: JoinHandle<Launcher>,
    drain_controller: JoinHandle<GracefulShutdownOutcome>,
    cancel_listener_on_drop: DropGuard,
    event_listener: JoinHandle<()>,
) -> Result<Launcher, String> {
    let (server_result, drain_result) = tokio::join!(server_task, drain_controller);
    cancel_listener_on_drop.disarm().cancel();
    let listener_result = event_listener.await;

    let mut failures = Vec::new();

    match drain_result {
        Ok(GracefulShutdownOutcome::Drained) => {}
        Ok(GracefulShutdownOutcome::TimedOut) => failures.push("HTTP drain controller timed out".to_owned()),
        Err(error) => failures.push(format!("HTTP drain controller failed to join: {error}")),
    }

    if let Err(error) = listener_result {
        failures.push(format!("HTTP statistics event listener failed to join: {error}"));
    }

    match server_result {
        Ok(launcher) if failures.is_empty() => Ok(launcher),
        Ok(_) => Err(failures.join("; ")),
        Err(error) => {
            failures.insert(0, format!("HTTP server task failed to join: {error}"));
            Err(failures.join("; "))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpListener};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_configuration::v3_0_0::shutdown::Shutdown;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use super::{Started, Unstarted, join_owned_tasks};
    use crate::server::Launcher;

    const TEST_DEADLINE: Duration = Duration::from_secs(10);

    async fn unstarted_environment() -> Unstarted {
        let configuration = ephemeral_public();
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = Arc::new(configuration.http_trackers.expect("test configuration enables HTTP")[0].clone());

        Unstarted::new(&core_config, &http_tracker_config).await
    }

    async fn unstarted_environment_bound_to(bind_address: SocketAddr) -> Unstarted {
        let mut configuration = ephemeral_public();
        configuration.http_trackers.as_mut().expect("test configuration enables HTTP")[0].bind_address = bind_address;
        let core_config = Arc::new(configuration.core.clone());
        let http_tracker_config = Arc::new(configuration.http_trackers.expect("test configuration enables HTTP")[0].clone());

        Unstarted::new(&core_config, &http_tracker_config).await
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

    #[tokio::test]
    async fn it_should_stop_without_a_join_failure_when_every_owned_task_finishes_through_cancellation() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;

        // Act
        let stop = tokio::spawn(stop_within_deadline(environment)).await;

        // Assert
        assert!(
            stop.is_ok(),
            "stop() should join every owned task, including the listener, without a join failure"
        );
    }

    #[tokio::test]
    async fn it_should_not_leave_the_statistics_listener_running_when_the_http_server_fails_to_start() {
        // Arrange
        let occupied_port = TcpListener::bind("127.0.0.1:0").expect("occupy a local TCP port");
        let environment = unstarted_environment_bound_to(occupied_port.local_addr().unwrap()).await;
        // Keeping the container keeps its event bus open, so a detached listener would keep running.
        let container = environment.container.clone();

        // Act
        let start = tokio::time::timeout(TEST_DEADLINE, tokio::spawn(environment.start()))
            .await
            .expect("start() should fail within the test deadline");

        // Assert
        assert!(
            start.is_err_and(|error| error.is_panic()),
            "start() should fail when its port is occupied"
        );
        assert_eq!(
            Arc::strong_count(&container.http_tracker_core_container.stats_repository),
            1,
            "only the container should hold the statistics repository; a second holder is a detached listener"
        );
    }

    #[tokio::test]
    async fn it_should_release_the_http_binding_when_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let binding: SocketAddr = *environment.bind_address();

        // Act
        let _stopped = stop_within_deadline(environment).await;

        // Assert
        TcpListener::bind(binding).expect("the HTTP binding should be free as soon as stop() returns");
    }

    #[tokio::test]
    async fn it_should_release_the_http_binding_when_dropped_without_being_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let binding: SocketAddr = *environment.bind_address();

        // Act
        drop(environment);

        // Assert
        // Dropping cannot await the server task, so the binding is released shortly after, not at once.
        tokio::time::timeout(TEST_DEADLINE, async {
            while TcpListener::bind(binding).is_err() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("the HTTP binding should be released within the test deadline after the environment is dropped");
    }

    #[tokio::test]
    async fn it_should_serve_requests_after_being_stopped_and_started_again() {
        // Arrange
        let stopped = stop_within_deadline(start_within_deadline(unstarted_environment().await).await).await;

        // Act
        let restarted = start_within_deadline(stopped).await;

        // Assert
        let status = tokio::time::timeout(
            TEST_DEADLINE,
            reqwest::get(restarted.base_url().join("health_check").unwrap()),
        )
        .await
        .expect("the health check should answer within the test deadline")
        .expect("the restarted HTTP tracker should accept the request")
        .status();
        assert_eq!(status, reqwest::StatusCode::OK);

        stop_within_deadline(restarted).await;
    }

    #[tokio::test]
    async fn it_should_join_the_event_listener_before_reporting_an_http_server_task_failure() {
        // Arrange
        let cancellation_token = CancellationToken::new();
        let listener_finished = Arc::new(AtomicBool::new(false));

        let failed_server_task = tokio::spawn(async { panic!("simulated HTTP server task failure") });
        let drained_controller = tokio::spawn(async { GracefulShutdownOutcome::Drained });
        let event_listener = tokio::spawn({
            let cancellation_token = cancellation_token.clone();
            let listener_finished = listener_finished.clone();
            async move {
                cancellation_token.cancelled().await;
                // Finish after the failed server task, so returning on its failure would see this unfinished.
                tokio::task::yield_now().await;
                listener_finished.store(true, Ordering::SeqCst);
            }
        });

        // Act
        let result = tokio::time::timeout(
            TEST_DEADLINE,
            join_owned_tasks(
                failed_server_task,
                drained_controller,
                cancellation_token.drop_guard(),
                event_listener,
            ),
        )
        .await
        .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        let failures = result
            .map(|_: Launcher| ())
            .expect_err("a failed server task should be reported");
        assert!(
            failures.contains("HTTP server task failed to join"),
            "unexpected failures: {failures}"
        );
        assert!(
            listener_finished.load(Ordering::SeqCst),
            "the event listener should have finished before the failure was reported"
        );
    }

    /// An announce request whose header block is not terminated yet: the server keeps the connection
    /// open and waits for the rest, so the request is still in flight while the environment stops.
    const UNFINISHED_COMPLETED_ANNOUNCE: &[u8] = b"GET /announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%01&peer_id=-qB00000000000000001&port=17548&uploaded=0&downloaded=0&left=0&event=completed HTTP/1.1\r\nHost: 127.0.0.1\r\n";

    /// Waits until the server no longer accepts connections, which only happens once it started to shut down.
    async fn wait_until_the_server_refuses_new_connections(address: SocketAddr) {
        tokio::time::timeout(TEST_DEADLINE, async {
            while tokio::net::TcpStream::connect(address).await.is_ok() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("the server should stop accepting connections once stop() asked it to shut down");
    }

    #[tokio::test]
    async fn it_should_count_the_statistics_event_of_a_request_that_completes_while_stop_drains_the_server() {
        // Arrange
        // A client has sent every header of a `completed` announce except the final blank line, so the
        // request is still in flight. The environment starts to stop, and the server keeps the connection
        // open while it drains.
        let environment = start_within_deadline(unstarted_environment().await).await;
        let address = *environment.bind_address();
        let container = environment.container.clone();
        let mut client = tokio::net::TcpStream::connect(address)
            .await
            .expect("the running server should accept the client");
        client
            .write_all(UNFINISHED_COMPLETED_ANNOUNCE)
            .await
            .expect("the client should send the first part of the request");
        let stop = tokio::spawn(environment.stop());
        wait_until_the_server_refuses_new_connections(address).await;

        // Act
        // The client finishes the request while the server is draining, and `stop()` returns afterwards.
        client
            .write_all(b"\r\n")
            .await
            .expect("the draining server should still read the rest of the request");
        let mut response = [0_u8; 12];
        client
            .read_exact(&mut response)
            .await
            .expect("the draining server should answer the request");
        tokio::time::timeout(TEST_DEADLINE, stop)
            .await
            .expect("stop() should finish within the test deadline")
            .expect("stop() should not panic");

        // Assert
        // The announce was answered, and its event, published during the drain, was counted before stop() returned.
        assert_eq!(&response, b"HTTP/1.1 200", "the in-flight announce should be answered");
        let counted = container
            .http_tracker_core_container
            .stats_repository
            .read_metrics(torrust_tracker_http_core::statistics::metrics::Metrics::tcp4_announces_handled)
            .await;
        assert_eq!(
            counted, 1,
            "the announce completed while stop() was draining the server, so its event must be counted before stop() returns"
        );
    }

    /// Lets every other runnable task run: on the test's current-thread runtime a task cannot
    /// make progress while the test itself runs.
    async fn let_the_other_tasks_run() {
        for _ in 0..16 {
            tokio::task::yield_now().await;
        }
    }

    #[tokio::test]
    async fn it_should_cancel_the_event_listener_only_after_the_server_and_its_drain_controller_have_finished() {
        // Arrange
        // The server task and its drain controller each finish only when the test releases them, like a
        // server that is still answering requests. The listener waits for its own stop request.
        let listener_token = CancellationToken::new();
        let (release_server, server_released) = tokio::sync::oneshot::channel::<()>();
        let (release_controller, controller_released) = tokio::sync::oneshot::channel::<()>();
        let server_task = tokio::spawn(async move {
            let _ = server_released.await;
            Launcher::new(
                SocketAddr::from(([127, 0, 0, 1], 0)),
                None,
                false,
                Shutdown::default().http_drain_in_secs.as_duration(),
            )
        });
        let drain_controller = tokio::spawn(async move {
            let _ = controller_released.await;
            GracefulShutdownOutcome::Drained
        });
        let event_listener = tokio::spawn({
            let listener_token = listener_token.clone();
            async move { listener_token.cancelled().await }
        });

        // Act
        // The environment joins its tasks; the server finishes first, and then its drain controller.
        let join = tokio::spawn(join_owned_tasks(
            server_task,
            drain_controller,
            listener_token.clone().drop_guard(),
            event_listener,
        ));
        let_the_other_tasks_run().await;
        let cancelled_while_the_server_runs = listener_token.is_cancelled();
        release_server.send(()).expect("the server task is waiting for its release");
        let_the_other_tasks_run().await;
        let cancelled_while_the_controller_runs = listener_token.is_cancelled();
        release_controller
            .send(())
            .expect("the drain controller is waiting for its release");
        let result = join.await.expect("joining must not panic");

        // Assert
        // The listener is cancelled only once both server-side tasks have finished, and is joined too.
        assert!(
            !cancelled_while_the_server_runs,
            "the listener must not be cancelled while the server runs"
        );
        assert!(
            !cancelled_while_the_controller_runs,
            "the listener must not be cancelled while the drain controller runs"
        );
        assert!(
            listener_token.is_cancelled(),
            "the listener must be cancelled once the server and its drain controller finished"
        );
        assert!(result.is_ok(), "every task finished normally: {result:?}");
    }

    #[tokio::test]
    async fn it_should_cancel_the_event_listener_when_the_join_is_abandoned_while_the_server_runs() {
        // Arrange
        // The server task never finishes. The listener waits for its own stop request.
        let listener_token = CancellationToken::new();
        let server_task = tokio::spawn(std::future::pending::<Launcher>());
        let drain_controller = tokio::spawn(std::future::pending::<GracefulShutdownOutcome>());
        let event_listener = tokio::spawn({
            let listener_token = listener_token.clone();
            async move { listener_token.cancelled().await }
        });

        // Act
        // The caller gives up on the join, for example because its own deadline expired.
        let abandoned = tokio::time::timeout(
            Duration::from_millis(10),
            join_owned_tasks(
                server_task,
                drain_controller,
                listener_token.clone().drop_guard(),
                event_listener,
            ),
        )
        .await;

        // Assert
        // Abandoning the join must not leave a listener that was never asked to stop. It is cancelled, not joined.
        assert!(abandoned.is_err(), "the join should still be waiting for the server");
        assert!(
            listener_token.is_cancelled(),
            "dropping the join before the server stage ends must still cancel the listener"
        );
    }

    #[tokio::test]
    async fn it_should_stop_the_event_listener_when_dropped_without_being_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let container = environment.container.clone();
        let holders_while_running = Arc::strong_count(&container.http_tracker_core_container.stats_repository);

        // Act
        drop(environment);

        // Assert
        // Dropping cannot await the listener, so it stops shortly after. While it runs it holds the
        // statistics repository, so the repository's holders show whether it stopped.
        assert!(
            holders_while_running > 1,
            "the running listener should hold the statistics repository"
        );
        tokio::time::timeout(TEST_DEADLINE, async {
            while Arc::strong_count(&container.http_tracker_core_container.stats_repository) > 1 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("the event listener should stop within the test deadline after the environment is dropped");
    }

    #[tokio::test]
    async fn it_should_report_a_drain_timeout_even_when_every_task_joins() {
        // Arrange
        let finished_server_task = tokio::spawn(async {
            Launcher::new(
                SocketAddr::from(([127, 0, 0, 1], 0)),
                None,
                false,
                Shutdown::default().http_drain_in_secs.as_duration(),
            )
        });
        let timed_out_controller = tokio::spawn(async { GracefulShutdownOutcome::TimedOut });
        let finished_event_listener = tokio::spawn(async {});

        // Act
        let result = tokio::time::timeout(
            TEST_DEADLINE,
            join_owned_tasks(
                finished_server_task,
                timed_out_controller,
                CancellationToken::new().drop_guard(),
                finished_event_listener,
            ),
        )
        .await
        .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        let failures = result.map(|_: Launcher| ()).expect_err("a drain timeout should be reported");
        assert_eq!(failures, "HTTP drain controller timed out");
    }
}
