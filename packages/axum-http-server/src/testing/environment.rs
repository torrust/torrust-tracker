use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_info_hash::InfoHash;
use torrust_server_lib::registar::{FnSpawnServiceHeathCheck, Registar};
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker;
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_http_core::statistics::event::listener::run_event_listener;
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
pub struct Running {
    server: CancellationRunning,
    event_listener_job: JoinHandle<()>,
    cancellation_token: CancellationToken,
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
        let cancellation_token = CancellationToken::new();

        let event_listener_job = run_event_listener(
            self.container.http_tracker_core_container.event_bus.receiver(),
            cancellation_token.clone(),
            &self.container.http_tracker_core_container.stats_repository,
            [(ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0), true)].into(),
        );

        let server = self
            .state
            .server
            .start_with_cancellation_and_health_check(
                self.container.http_tracker_core_container.clone(),
                self.registar.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0)),
                cancellation_token.clone(),
                health_check,
            )
            .await
            .expect("Failed to start the HTTP tracker server");

        Environment {
            container: self.container,
            registar: self.registar,
            state: Running {
                server,
                event_listener_job,
                cancellation_token,
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
    /// It cancels the environment token once, then joins the HTTP server task,
    /// its drain controller, and the statistics event listener. Events still
    /// queued in the listener at that moment are discarded, as in the tracker
    /// application (see #2410).
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
            cancellation_token,
        } = self.state;

        cancellation_token.cancel();

        let launcher = join_owned_tasks(task, shutdown_controller, event_listener_job)
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

/// Joins every task a running environment owns and reports all failures at
/// once, so an early failure never leaves a remaining task detached.
async fn join_owned_tasks(
    server_task: JoinHandle<Launcher>,
    drain_controller: JoinHandle<GracefulShutdownOutcome>,
    event_listener: JoinHandle<()>,
) -> Result<Launcher, String> {
    let (server_result, drain_result, listener_result) = tokio::join!(server_task, drain_controller, event_listener);

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

    use tokio_util::sync::CancellationToken;
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
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

    async fn stop_within_deadline(environment: Started) -> Unstarted {
        tokio::time::timeout(TEST_DEADLINE, environment.stop())
            .await
            .expect("stop() should finish within the test deadline")
    }

    #[tokio::test]
    async fn it_should_finish_the_event_listener_through_cancellation_when_stopped() {
        // Arrange
        let environment = unstarted_environment().await.start().await;

        // Act
        let stop = tokio::spawn(stop_within_deadline(environment)).await;

        // Assert
        assert!(
            stop.is_ok(),
            "stop() should join every owned task, including the listener, without a join failure"
        );
    }

    #[tokio::test]
    async fn it_should_release_the_http_binding_when_stopped() {
        // Arrange
        let environment = unstarted_environment().await.start().await;
        let binding: SocketAddr = *environment.bind_address();

        // Act
        let _stopped = stop_within_deadline(environment).await;

        // Assert
        TcpListener::bind(binding).expect("the HTTP binding should be free as soon as stop() returns");
    }

    #[tokio::test]
    async fn it_should_serve_requests_after_being_stopped_and_started_again() {
        // Arrange
        let stopped = stop_within_deadline(unstarted_environment().await.start().await).await;

        // Act
        let restarted = stopped.start().await;

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
                tokio::task::yield_now().await;
                listener_finished.store(true, Ordering::SeqCst);
            }
        });
        cancellation_token.cancel();

        // Act
        let result = tokio::time::timeout(
            TEST_DEADLINE,
            join_owned_tasks(failed_server_task, drained_controller, event_listener),
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

    #[tokio::test]
    async fn it_should_report_a_drain_timeout_even_when_every_task_joins() {
        // Arrange
        let finished_server_task = tokio::spawn(async { Launcher::new(SocketAddr::from(([127, 0, 0, 1], 0)), None, false) });
        let timed_out_controller = tokio::spawn(async { GracefulShutdownOutcome::TimedOut });
        let finished_event_listener = tokio::spawn(async {});

        // Act
        let result = tokio::time::timeout(
            TEST_DEADLINE,
            join_owned_tasks(finished_server_task, timed_out_controller, finished_event_listener),
        )
        .await
        .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        let failures = result.map(|_: Launcher| ()).expect_err("a drain timeout should be reported");
        assert_eq!(failures, "HTTP drain controller timed out");
    }
}
