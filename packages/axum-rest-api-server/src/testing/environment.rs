use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;

use secrecy::ExposeSecret;
use tokio::task::JoinHandle;
use tokio_util::sync::{CancellationToken, DropGuard};
use torrust_info_hash::InfoHash;
use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_configuration::v3_0_0::{Configuration, logging};
use torrust_tracker_core::container::TrackerCoreContainer;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole, peer};
use torrust_tracker_rest_api_client::connection_info::{ConnectionInfo, Origin};
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_server::container::UdpTrackerServerContainer;

use crate::server::{self, ApiServer, CancellationRunning, Launcher};

pub type Unstarted = Environment<Stopped>;
pub type Started = Environment<Running>;

/// A test environment with no REST API server running.
pub struct Stopped {
    server: ApiServer<server::Stopped>,
}

/// A test environment whose REST API server runs on the token-aware
/// lifecycle. The environment owns the server task and its drain controller.
///
/// The environment owns no event listener, so one token stops everything it
/// started. Dropping it without `stop()` cancels that token, and the server
/// shuts down once it observes it, so the binding is released shortly after
/// the drop.
pub struct Running {
    server: CancellationRunning,
    cancel_server_on_drop: DropGuard,
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
    /// Will panic if it cannot make the TLS configuration from the provided
    /// configuration.
    #[must_use]
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        initialize_global_services(configuration);

        let container = Arc::new(EnvContainer::initialize(configuration).await);

        let bind_to = container.tracker_http_api_core_container.http_api_config.bind_address;

        let tls = if let Some(tls_config) = &container.tracker_http_api_core_container.http_api_config.tls_config {
            Some(make_rust_tls(tls_config).await.expect("tls config failed"))
        } else {
            None
        };

        let server = ApiServer::new(Launcher::new(bind_to, tls));

        Self {
            container,
            registar: Registar::default(),
            state: Stopped { server },
        }
    }

    /// Starts the test environment with a fresh cancellation token, so a
    /// stopped environment can be started again.
    ///
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

        let cancellation_token = CancellationToken::new();

        let server = self
            .state
            .server
            .start_with_cancellation(
                self.container.tracker_http_api_core_container.clone(),
                self.registar.give_form(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::RestApi, 0)).with_public_url(
                    self.container
                        .tracker_http_api_core_container
                        .http_api_config
                        .public_url
                        .as_ref()
                        .map(|url| url.as_url().clone()),
                ),
                access_tokens,
                cancellation_token.clone(),
            )
            .await
            .unwrap();

        Environment {
            container: self.container,
            registar: self.registar,
            state: Running {
                server,
                cancel_server_on_drop: cancellation_token.drop_guard(),
            },
        }
    }
}

impl Environment<Running> {
    pub async fn new(configuration: &Arc<Configuration>) -> Self {
        Environment::<Stopped>::new(configuration).await.start().await
    }

    /// Stops the test environment and returns a stopped environment.
    ///
    /// It cancels the token and joins the server task and its drain
    /// controller.
    ///
    /// # Panics
    ///
    /// Will panic, after both tasks have been joined, if either task failed or
    /// the drain timed out. The message names each failing task.
    pub async fn stop(self) -> Environment<Stopped> {
        let Running {
            server:
                CancellationRunning {
                    task,
                    shutdown_controller,
                    ..
                },
            cancel_server_on_drop,
        } = self.state;

        cancel_server_on_drop.disarm().cancel();

        let launcher = join_owned_tasks(task, shutdown_controller)
            .await
            .unwrap_or_else(|failures| panic!("Failed to stop the REST API test environment: {failures}"));

        Environment {
            container: self.container,
            registar: Registar::default(),
            state: Stopped {
                server: ApiServer::new(launcher),
            },
        }
    }

    /// # Panics
    ///
    /// Will panic if it cannot build the origin for the connection info from the
    /// server local socket address.
    #[must_use]
    pub fn get_connection_info(&self) -> ConnectionInfo {
        let origin = Origin::new(&format!("http://{}/", self.state.server.local_addr)).unwrap(); // DevSkim: ignore DS137138

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
    pub const fn bind_address(&self) -> SocketAddr {
        self.state.server.local_addr
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
        let udp_tracker_server_config = configuration.udp_tracker_server.clone();

        let http_api_config = Arc::new(configuration.http_api.clone().expect("missing HTTP API configuration"));

        let swarm_coordination_registry_container = Arc::new(SwarmCoordinationRegistryContainer::initialize(
            core_config.tracker_usage_statistics.into(),
        ));

        let tracker_core_container = Arc::new(
            TrackerCoreContainer::initialize_from(
                &core_config,
                &swarm_coordination_registry_container,
                core_config.database.as_ref(),
            )
            .await
            .expect("REST API server test initialization requires valid composition"),
        );

        let http_tracker_core_container = HttpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            &http_tracker_config,
            ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0),
        );

        let udp_tracker_core_container = UdpTrackerCoreContainer::initialize_from_tracker_core(
            &tracker_core_container,
            &udp_tracker_config,
            udp_tracker_server_config.max_connection_id_errors_per_ip,
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
}

/// Joins the server task and its drain controller and reports every failure at
/// once, so a failure of one never leaves the other detached.
async fn join_owned_tasks<E: Display>(
    server_task: JoinHandle<Result<Launcher, E>>,
    drain_controller: JoinHandle<GracefulShutdownOutcome>,
) -> Result<Launcher, String> {
    let (server_result, drain_result) = tokio::join!(server_task, drain_controller);

    let mut failures = Vec::new();

    let launcher = match server_result {
        Ok(Ok(launcher)) => Some(launcher),
        Ok(Err(error)) => {
            failures.push(format!("REST API server task failed: {error}"));
            None
        }
        Err(error) => {
            failures.push(format!("REST API server task failed to join: {error}"));
            None
        }
    };

    match drain_result {
        Ok(GracefulShutdownOutcome::Drained) => {}
        Ok(GracefulShutdownOutcome::TimedOut) => failures.push("REST API drain controller timed out".to_owned()),
        Err(error) => failures.push(format!("REST API drain controller failed to join: {error}")),
    }

    match launcher {
        Some(launcher) if failures.is_empty() => Ok(launcher),
        _ => Err(failures.join("; ")),
    }
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpListener};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::registar::Registar;
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use super::{Environment, Running, Started, Unstarted, join_owned_tasks};
    use crate::server::{CancellationRunning, Launcher};

    const TEST_DEADLINE: Duration = Duration::from_secs(10);

    async fn unstarted_environment() -> Unstarted {
        Unstarted::new(&Arc::new(ephemeral_public())).await
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

    fn a_launcher() -> Launcher {
        Launcher::new(SocketAddr::from(([127, 0, 0, 1], 0)), None)
    }

    #[tokio::test]
    async fn it_should_stop_without_a_failure_when_the_server_and_its_drain_controller_finish_through_cancellation() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;

        // Act
        let stop = tokio::spawn(stop_within_deadline(environment)).await;

        // Assert
        assert!(
            stop.is_ok(),
            "stop() should join the server and its drain controller without a failure"
        );
    }

    #[tokio::test]
    async fn it_should_release_the_rest_api_binding_when_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let binding = environment.bind_address();

        // Act
        let _stopped = stop_within_deadline(environment).await;

        // Assert
        TcpListener::bind(binding).expect("the REST API binding should be free as soon as stop() returns");
    }

    #[tokio::test]
    async fn it_should_release_the_rest_api_binding_when_dropped_without_being_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment().await).await;
        let binding = environment.bind_address();

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
        .expect("the REST API binding should be released within the test deadline after the environment is dropped");
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
            reqwest::get(format!("http://{}/api/health_check", restarted.bind_address())), // DevSkim: ignore DS137138
        )
        .await
        .expect("the health check should answer within the test deadline")
        .expect("the restarted REST API server should accept the request")
        .status();
        assert_eq!(status, reqwest::StatusCode::OK);

        stop_within_deadline(restarted).await;
    }

    #[tokio::test]
    async fn it_should_join_the_drain_controller_before_reporting_a_server_task_failure() {
        // Arrange
        let drain_controller_finished = Arc::new(AtomicBool::new(false));

        let failed_server_task = tokio::spawn(async { panic!("simulated REST API server task failure") });
        let drain_controller = tokio::spawn({
            let drain_controller_finished = drain_controller_finished.clone();
            async move {
                // Finish after the failed server task, so returning on its failure would see this unfinished.
                tokio::task::yield_now().await;
                drain_controller_finished.store(true, Ordering::SeqCst);
                GracefulShutdownOutcome::Drained
            }
        });

        // Act
        let result = tokio::time::timeout(
            TEST_DEADLINE,
            join_owned_tasks::<std::convert::Infallible>(failed_server_task, drain_controller),
        )
        .await
        .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        let failures = result
            .map(|_: Launcher| ())
            .expect_err("a failed server task should be reported");
        assert!(
            failures.contains("REST API server task failed to join"),
            "unexpected failures: {failures}"
        );
        assert!(
            drain_controller_finished.load(Ordering::SeqCst),
            "the drain controller should have finished before the failure was reported"
        );
    }

    #[tokio::test]
    async fn it_should_report_a_server_error_and_a_drain_timeout_together() {
        // Arrange
        let failed_server_task = tokio::spawn(async { Err::<Launcher, _>("simulated server error") });
        let timed_out_controller = tokio::spawn(async { GracefulShutdownOutcome::TimedOut });

        // Act
        let result = tokio::time::timeout(TEST_DEADLINE, join_owned_tasks(failed_server_task, timed_out_controller))
            .await
            .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        let failures = result.map(|_: Launcher| ()).expect_err("both failures should be reported");
        assert_eq!(
            failures,
            "REST API server task failed: simulated server error; REST API drain controller timed out"
        );
    }

    #[tokio::test]
    async fn it_should_panic_naming_each_failing_task_when_stopped_with_failed_tasks() {
        // Arrange
        let container = unstarted_environment().await.container;
        let environment: Started = Environment {
            container,
            registar: Registar::default(),
            state: Running {
                server: CancellationRunning {
                    local_addr: SocketAddr::from(([127, 0, 0, 1], 0)),
                    task: tokio::spawn(async { panic!("simulated REST API server task failure") }),
                    shutdown_controller: tokio::spawn(async { GracefulShutdownOutcome::TimedOut }),
                },
                cancel_server_on_drop: CancellationToken::new().drop_guard(),
            },
        };

        // Act
        let stop = tokio::time::timeout(TEST_DEADLINE, tokio::spawn(environment.stop()))
            .await
            .expect("stop() should finish within the test deadline");

        // Assert
        let Err(join_error) = stop else {
            panic!("stop() should panic when a task failed");
        };
        let message = *join_error
            .into_panic()
            .downcast::<String>()
            .expect("the panic should carry a message");
        assert!(
            message.contains("REST API server task failed to join") && message.contains("REST API drain controller timed out"),
            "the panic should name both failing tasks, but said: {message}"
        );
    }

    #[tokio::test]
    async fn it_should_report_a_drain_timeout_even_when_both_tasks_join() {
        // Arrange
        let finished_server_task = tokio::spawn(async { Ok::<_, std::convert::Infallible>(a_launcher()) });
        let timed_out_controller = tokio::spawn(async { GracefulShutdownOutcome::TimedOut });

        // Act
        let result = tokio::time::timeout(TEST_DEADLINE, join_owned_tasks(finished_server_task, timed_out_controller))
            .await
            .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        let failures = result.map(|_: Launcher| ()).expect_err("a drain timeout should be reported");
        assert_eq!(failures, "REST API drain controller timed out");
    }
}
