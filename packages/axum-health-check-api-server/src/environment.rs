use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::{CancellationToken, DropGuard};
use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
use torrust_tracker_configuration::v3_0_0::health_check_api::HealthCheckApi;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};

use crate::server::{self, CancellationRunning};

pub type Unstarted = Environment<Stopped>;
pub type Started = Environment<Running>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Error(String),
}

/// A test environment with no health check API server running.
pub struct Stopped {
    bind_to: SocketAddr,
}

/// A test environment whose health check API server runs on the token-aware
/// lifecycle. The environment owns the server task and its drain controller.
///
/// The environment owns no event listener, so one token stops everything it
/// started. Dropping it without `stop()` cancels that token, and the server
/// shuts down once it observes it, so the binding is released shortly after
/// the drop. A drop cannot await the registry lock, so only `stop()` removes
/// the environment's registration from the `registar`.
pub struct Running {
    server: CancellationRunning,
    bind_to: SocketAddr,
    cancel_server_on_drop: DropGuard,
}

pub struct Environment<S> {
    pub registar: Registar<RuntimeServiceMetadata>,
    state: S,
}

impl Environment<Stopped> {
    #[must_use]
    pub fn new(config: &Arc<HealthCheckApi>, registar: Registar<RuntimeServiceMetadata>) -> Self {
        Self {
            registar,
            state: Stopped {
                bind_to: config.bind_address,
            },
        }
    }

    /// Starts the test environment with a fresh cancellation token, so a
    /// stopped environment can be started again, also on a fixed port.
    ///
    /// Starting registers the health check API in the environment's
    /// `registar`; `stop()` removes that registration again.
    ///
    /// # Panics
    ///
    /// Will panic if the server cannot be started.
    pub async fn start(self) -> Environment<Running> {
        let cancellation_token = CancellationToken::new();

        let server = server::start_with_cancellation(
            self.state.bind_to,
            self.registar.clone(),
            RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HealthCheckApi, 0)),
            cancellation_token.clone(),
        )
        .await
        .expect("it should start the health check service");

        Environment {
            registar: self.registar,
            state: Running {
                server,
                bind_to: self.state.bind_to,
                cancel_server_on_drop: cancellation_token.drop_guard(),
            },
        }
    }
}

impl Environment<Running> {
    pub async fn new(config: &Arc<HealthCheckApi>, registar: Registar<RuntimeServiceMetadata>) -> Self {
        Environment::<Stopped>::new(config, registar).start().await
    }

    /// The address the server is listening on.
    #[must_use]
    pub const fn bind_address(&self) -> SocketAddr {
        self.state.server.local_addr
    }

    /// Stops the test environment and returns a stopped environment.
    ///
    /// It cancels the token, joins the server task and its drain controller,
    /// and removes the environment's own registration from the `registar`,
    /// leaving the services of other owners in place.
    ///
    /// # Errors
    ///
    /// Will return an error, after both tasks have been joined and the
    /// registration removed, if either task failed or the drain timed out. The
    /// message names each failing task.
    pub async fn stop(self) -> Result<Environment<Stopped>, Error> {
        let Running {
            server:
                CancellationRunning {
                    task,
                    shutdown_controller,
                    service_binding,
                    ..
                },
            bind_to,
            cancel_server_on_drop,
        } = self.state;

        cancel_server_on_drop.disarm().cancel();

        let joined = join_owned_tasks(task, shutdown_controller).await;
        self.registar.deregister(&service_binding).await;
        joined.map_err(Error::Error)?;

        Ok(Environment {
            registar: self.registar,
            state: Stopped { bind_to },
        })
    }
}

/// Joins the server task and its drain controller and reports every failure at
/// once, so a failure of one never leaves the other detached.
async fn join_owned_tasks<E: Display>(
    server_task: JoinHandle<Result<(), E>>,
    drain_controller: JoinHandle<GracefulShutdownOutcome>,
) -> Result<(), String> {
    let (server_result, drain_result) = tokio::join!(server_task, drain_controller);

    let mut failures = Vec::new();

    match server_result {
        Ok(Ok(())) => {}
        Ok(Err(error)) => failures.push(format!("health check API server task failed: {error}")),
        Err(error) => failures.push(format!("health check API server task failed to join: {error}")),
    }

    match drain_result {
        Ok(GracefulShutdownOutcome::Drained) => {}
        Ok(GracefulShutdownOutcome::TimedOut) => failures.push("health check API drain controller timed out".to_owned()),
        Err(error) => failures.push(format!("health check API drain controller failed to join: {error}")),
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_net_primitives::service_binding::{Protocol, ServiceBinding};
    use torrust_server_lib::registar::{Registar, ServiceRegistration};
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_configuration::v3_0_0::health_check_api::HealthCheckApi;
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};

    use super::{Environment, Error, Running, Started, Unstarted, join_owned_tasks};
    use crate::server::CancellationRunning;

    const TEST_DEADLINE: Duration = Duration::from_secs(10);

    fn unstarted_environment() -> Unstarted {
        let config = HealthCheckApi {
            bind_address: "127.0.0.1:0".parse().unwrap(),
        };
        Unstarted::new(&Arc::new(config), Registar::default())
    }

    async fn start_within_deadline(environment: Unstarted) -> Started {
        tokio::time::timeout(TEST_DEADLINE, environment.start())
            .await
            .expect("start() should finish within the test deadline")
    }

    async fn stop_within_deadline(environment: Started) -> Result<Unstarted, Error> {
        tokio::time::timeout(TEST_DEADLINE, environment.stop())
            .await
            .expect("stop() should finish within the test deadline")
    }

    #[tokio::test]
    async fn it_should_stop_without_a_failure_when_the_server_and_its_drain_controller_finish_through_cancellation() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment()).await;

        // Act
        let stop = stop_within_deadline(environment).await;

        // Assert
        assert!(
            stop.is_ok(),
            "stop() should join the server and its drain controller without a failure"
        );
    }

    #[tokio::test]
    async fn it_should_release_the_health_check_api_binding_when_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment()).await;
        let binding = environment.bind_address();

        // Act
        let _stopped = stop_within_deadline(environment).await.expect("stop() should succeed");

        // Assert
        TcpListener::bind(binding).expect("the health check API binding should be free as soon as stop() returns");
    }

    #[tokio::test]
    async fn it_should_release_the_health_check_api_binding_when_dropped_without_being_stopped() {
        // Arrange
        let environment = start_within_deadline(unstarted_environment()).await;
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
        .expect("the health check API binding should be released within the test deadline after the environment is dropped");
    }

    #[tokio::test]
    async fn it_should_serve_requests_after_being_stopped_and_started_again() {
        // Arrange
        let stopped = stop_within_deadline(start_within_deadline(unstarted_environment()).await)
            .await
            .expect("the first stop should succeed");

        // Act
        let restarted = start_within_deadline(stopped).await;

        // Assert
        let status = tokio::time::timeout(
            TEST_DEADLINE,
            reqwest::get(format!("http://{}/health_check", restarted.bind_address())), // DevSkim: ignore DS137138
        )
        .await
        .expect("the health check should answer within the test deadline")
        .expect("the restarted health check API server should accept the request")
        .status();
        assert_eq!(status, reqwest::StatusCode::OK);

        stop_within_deadline(restarted).await.expect("the second stop should succeed");
    }

    #[tokio::test]
    async fn it_should_serve_requests_after_being_stopped_and_started_again_on_a_fixed_port() {
        // Arrange
        let fixed_address = {
            let probe = TcpListener::bind("127.0.0.1:0").unwrap();
            probe.local_addr().unwrap()
        };
        let config = HealthCheckApi {
            bind_address: fixed_address,
        };
        let stopped = stop_within_deadline(start_within_deadline(Unstarted::new(&Arc::new(config), Registar::default())).await)
            .await
            .expect("the first stop should succeed");

        // Act
        let restarted = start_within_deadline(stopped).await;

        // Assert
        assert_eq!(restarted.bind_address(), fixed_address);
        let status = tokio::time::timeout(
            TEST_DEADLINE,
            reqwest::get(format!("http://{fixed_address}/health_check")), // DevSkim: ignore DS137138
        )
        .await
        .expect("the health check should answer within the test deadline")
        .expect("the restarted health check API server should accept the request")
        .status();
        assert_eq!(status, reqwest::StatusCode::OK);

        stop_within_deadline(restarted).await.expect("the second stop should succeed");
    }

    #[tokio::test]
    async fn it_should_remove_only_its_own_registration_from_the_registar_when_stopped() {
        // Arrange
        let other_service_binding = ServiceBinding::new(Protocol::UDP, "127.0.0.1:6969".parse().unwrap()).unwrap();
        let registar = Registar::default();
        registar
            .give_form()
            .register(ServiceRegistration::new(
                other_service_binding.clone(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0)),
                None,
            ))
            .await
            .expect("the other service should register");
        let config = HealthCheckApi {
            bind_address: "127.0.0.1:0".parse().unwrap(),
        };
        let environment = start_within_deadline(Unstarted::new(&Arc::new(config), registar.clone())).await;
        assert_eq!(
            registar.services().await.len(),
            2,
            "the environment should register its service"
        );

        // Act
        let _stopped = stop_within_deadline(environment).await.expect("stop() should succeed");

        // Assert
        let remaining: Vec<_> = registar
            .services()
            .await
            .into_iter()
            .map(|service| service.service_binding().clone())
            .collect();
        assert_eq!(remaining, [other_service_binding]);
    }

    #[tokio::test]
    async fn it_should_join_the_drain_controller_before_reporting_a_server_task_failure() {
        // Arrange
        let drain_controller_finished = Arc::new(AtomicBool::new(false));

        let failed_server_task = tokio::spawn(async { panic!("simulated health check API server task failure") });
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
        let failures = result.expect_err("a failed server task should be reported");
        assert!(
            failures.contains("health check API server task failed to join"),
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
        let failed_server_task = tokio::spawn(async { Err("simulated server error") });
        let timed_out_controller = tokio::spawn(async { GracefulShutdownOutcome::TimedOut });

        // Act
        let result = tokio::time::timeout(TEST_DEADLINE, join_owned_tasks(failed_server_task, timed_out_controller))
            .await
            .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        assert_eq!(
            result.expect_err("both failures should be reported"),
            "health check API server task failed: simulated server error; health check API drain controller timed out"
        );
    }

    #[tokio::test]
    async fn it_should_return_an_error_naming_each_failing_task_when_stopped_with_failed_tasks() {
        // Arrange
        let environment: Started = Environment {
            registar: Registar::default(),
            state: Running {
                server: CancellationRunning {
                    local_addr: "127.0.0.1:0".parse().unwrap(),
                    service_binding: ServiceBinding::new(Protocol::HTTP, "127.0.0.1:1212".parse().unwrap()).unwrap(),
                    task: tokio::spawn(async { panic!("simulated health check API server task failure") }),
                    shutdown_controller: tokio::spawn(async { GracefulShutdownOutcome::TimedOut }),
                },
                bind_to: "127.0.0.1:0".parse().unwrap(),
                cancel_server_on_drop: CancellationToken::new().drop_guard(),
            },
        };

        // Act
        let result = stop_within_deadline(environment).await;

        // Assert
        let Err(Error::Error(message)) = result else {
            panic!("stop() should return an error when a task failed");
        };
        assert!(
            message.contains("health check API server task failed to join") && message.contains("drain controller timed out"),
            "the error should name both failing tasks, but said: {message}"
        );
    }

    #[tokio::test]
    async fn it_should_remove_its_registration_even_when_stopped_with_a_failed_task() {
        // Arrange
        let service_binding = ServiceBinding::new(Protocol::HTTP, "127.0.0.1:1212".parse().unwrap()).unwrap();
        let registar = Registar::default();
        registar
            .give_form()
            .register(ServiceRegistration::new(
                service_binding.clone(),
                RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HealthCheckApi, 0)),
                None,
            ))
            .await
            .expect("the environment's service should register");
        let environment: Started = Environment {
            registar: registar.clone(),
            state: Running {
                server: CancellationRunning {
                    local_addr: "127.0.0.1:0".parse().unwrap(),
                    service_binding,
                    task: tokio::spawn(async { panic!("simulated health check API server task failure") }),
                    shutdown_controller: tokio::spawn(async { GracefulShutdownOutcome::Drained }),
                },
                bind_to: "127.0.0.1:0".parse().unwrap(),
                cancel_server_on_drop: CancellationToken::new().drop_guard(),
            },
        };

        // Act
        let result = stop_within_deadline(environment).await;

        // Assert
        assert!(result.is_err(), "stop() should return an error when a task failed");
        assert!(
            registar.services().await.is_empty(),
            "stop() should remove the registration although a task failed"
        );
    }

    #[tokio::test]
    async fn it_should_report_a_drain_timeout_even_when_both_tasks_join() {
        // Arrange
        let finished_server_task = tokio::spawn(async { Ok::<_, std::convert::Infallible>(()) });
        let timed_out_controller = tokio::spawn(async { GracefulShutdownOutcome::TimedOut });

        // Act
        let result = tokio::time::timeout(TEST_DEADLINE, join_owned_tasks(finished_server_task, timed_out_controller))
            .await
            .expect("joining the owned tasks should finish within the test deadline");

        // Assert
        assert_eq!(
            result.expect_err("a drain timeout should be reported"),
            "health check API drain controller timed out"
        );
    }
}
