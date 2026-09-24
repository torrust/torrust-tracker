//! Health Check API job starter.
//!
//! The [`health_check_api::start_job`](crate::bootstrap::jobs::health_check_api::start_job)
//! function starts the Health Check API as a token-aware component. The
//! component owns the server task and its drain controller and joins both
//! before reporting its outcome to the `JobManager`.
//!
//! Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! for the API configuration options.

use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::Registar;
use torrust_tracker_axum_health_check_api_server::{HEALTH_CHECK_API_LOG_TARGET, server};
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
use torrust_tracker_configuration::v3_0_0::health_check_api::HealthCheckApi;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use tracing::instrument;

use crate::bootstrap::jobs::manager::{ComponentCompletion, ComponentError, ComponentResult, TokenAwareServerTask};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not start the health check API: {source}")]
    Start { source: server::Error },
}

/// This function starts a new Health Check API server with the provided
/// configuration.
///
/// The server drains when `cancellation_token` is cancelled. The returned
/// component future joins the server and its drain controller before
/// reporting its outcome.
///
/// # Errors
///
/// Returns listener or service-registration errors.
#[allow(
    clippy::async_yields_async,
    reason = "startup awaits registration errors before returning the cancellation-aware runtime future"
)]
#[instrument(skip(config, registar))]
pub async fn start_job(
    config: &HealthCheckApi,
    registar: Registar<RuntimeServiceMetadata>,
    cancellation_token: CancellationToken,
) -> Result<impl Future<Output = ComponentResult> + Send + 'static, Error> {
    let bind_addr = config.bind_address;

    let server = server::start_with_cancellation(
        bind_addr,
        registar,
        RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HealthCheckApi, 0)),
        cancellation_token.clone(),
    )
    .await
    .map_err(|source| Error::Start { source })?;
    let server_task = TokenAwareServerTask::new(server.task, server.shutdown_controller);

    Ok(async move {
        let completion = supervise_token_aware_server(server_task, cancellation_token).await;
        tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "Stopped server running on: http://{}", bind_addr);
        completion
    })
}

async fn supervise_token_aware_server<T, E>(
    mut server_task: TokenAwareServerTask<Result<T, E>, GracefulShutdownOutcome>,
    cancellation_token: CancellationToken,
) -> ComponentResult
where
    E: std::fmt::Display,
{
    tokio::select! {
        biased;
        () = cancellation_token.cancelled() => {
            let server_result = server_task.join().await;
            let drain_outcome = server_task
                .join_shutdown_controller()
                .await
                .map_err(|error| ComponentError::new(format!("health check API drain controller failed: {error}")))?;
            let server_result = server_result
                .map_err(|error| ComponentError::new(format!("health check API server task join failed while stopping: {error}")))?;
            server_result.map_err(|error| ComponentError::new(format!("health check API server runtime failed while stopping: {error}")))?;
            match drain_outcome {
                GracefulShutdownOutcome::Drained => Ok(ComponentCompletion::Cancelled),
                GracefulShutdownOutcome::TimedOut => Err(ComponentError::new("health check API graceful drain timed out")),
            }
        }
        result = server_task.join() => {
            cancellation_token.cancel();
            let drain_outcome = server_task
                .join_shutdown_controller()
                .await
                .map_err(|error| ComponentError::new(format!("health check API drain controller failed: {error}")))?;
            let server_result =
                result.map_err(|error| ComponentError::new(format!("health check API server task join failed: {error}")))?;
            server_result.map_err(|error| ComponentError::new(format!("health check API server runtime returned an error: {error}")))?;
            match drain_outcome {
                GracefulShutdownOutcome::Drained => Ok(ComponentCompletion::Completed),
                GracefulShutdownOutcome::TimedOut => Err(ComponentError::new("health check API graceful drain timed out")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, TcpListener};
    use std::time::Duration;

    use tokio::sync::oneshot;
    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::registar::Registar;
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_configuration::v3_0_0::health_check_api::HealthCheckApi;

    use crate::bootstrap::jobs::health_check_api::{start_job, supervise_token_aware_server};
    use crate::bootstrap::jobs::manager::{ComponentCompletion, TokenAwareServerTask};

    const TEST_COMPLETION_TIMEOUT: Duration = Duration::from_secs(5);

    fn available_health_check_address() -> SocketAddr {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("select available health-check address");
        listener.local_addr().expect("read available health-check address")
    }

    async fn wait_until_bindable(address: SocketAddr) -> bool {
        tokio::time::timeout(TEST_COMPLETION_TIMEOUT, async {
            while TcpListener::bind(address).is_err() {
                tokio::task::yield_now().await;
            }
        })
        .await
        .is_ok()
    }

    async fn started_drain_controller(
        cancellation_token: CancellationToken,
    ) -> (
        tokio::task::JoinHandle<GracefulShutdownOutcome>,
        oneshot::Receiver<()>,
        oneshot::Sender<()>,
    ) {
        let (controller_started_sender, controller_started) = oneshot::channel();
        let (cancellation_observed_sender, cancellation_observed) = oneshot::channel();
        let (release_sender, release) = oneshot::channel();
        let shutdown_controller = tokio::spawn(async move {
            controller_started_sender
                .send(())
                .expect("test should wait for the drain controller to start");
            cancellation_token.cancelled().await;
            cancellation_observed_sender
                .send(())
                .expect("test should wait for the drain controller to observe cancellation");
            release
                .await
                .expect("test should release the drain controller after proving the supervisor waits");
            GracefulShutdownOutcome::Drained
        });
        tokio::time::timeout(TEST_COMPLETION_TIMEOUT, controller_started)
            .await
            .expect("the drain controller should start within the test deadline")
            .expect("the drain controller should start before the component runs");

        (shutdown_controller, cancellation_observed, release_sender)
    }

    async fn panicking_server_task() -> Result<(), &'static str> {
        tokio::task::yield_now().await;
        panic!("health check API server task failure");
    }

    #[tokio::test]
    async fn it_should_release_the_listener_when_the_component_is_dropped_before_it_runs() {
        // Arrange
        let config = HealthCheckApi {
            bind_address: available_health_check_address(),
        };
        let component = start_job(&config, Registar::default(), CancellationToken::new())
            .await
            .expect("the health check API component should start");

        // Act
        drop(component);

        // Assert
        assert!(
            wait_until_bindable(config.bind_address).await,
            "dropping the unpolled health check API component must release its listener at {} within {TEST_COMPLETION_TIMEOUT:?}",
            config.bind_address
        );
    }

    #[tokio::test]
    async fn it_should_complete_after_draining_when_the_health_check_api_server_stops_independently() {
        // Arrange
        let server_task = tokio::spawn(async { Ok::<(), &str>(()) });
        let cancellation_token = CancellationToken::new();
        let (shutdown_controller, cancellation_observed, release_controller) =
            started_drain_controller(cancellation_token.clone()).await;

        // Act
        let supervisor = tokio::spawn(supervise_token_aware_server(
            TokenAwareServerTask::new(server_task, shutdown_controller),
            cancellation_token,
        ));
        tokio::time::timeout(TEST_COMPLETION_TIMEOUT, cancellation_observed)
            .await
            .expect("the drain controller should observe cancellation within the test deadline")
            .expect("the component should cancel its drain controller when its health check API server stops independently");

        // Assert
        assert!(
            !supervisor.is_finished(),
            "the component must wait for its drain controller before reporting independent health check API server completion"
        );
        release_controller
            .send(())
            .expect("the drain controller should still be waiting for test release");
        let completion = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, supervisor)
            .await
            .expect("the component supervisor should complete within the test deadline")
            .expect("the component supervisor should not panic");
        assert_eq!(completion, Ok(ComponentCompletion::Completed));
    }

    #[tokio::test]
    async fn it_should_fail_after_draining_when_the_health_check_api_server_task_panics() {
        // Arrange
        let server_task = tokio::spawn(panicking_server_task());
        let cancellation_token = CancellationToken::new();
        let (shutdown_controller, cancellation_observed, release_controller) =
            started_drain_controller(cancellation_token.clone()).await;

        // Act
        let supervisor = tokio::spawn(supervise_token_aware_server(
            TokenAwareServerTask::new(server_task, shutdown_controller),
            cancellation_token,
        ));
        tokio::time::timeout(TEST_COMPLETION_TIMEOUT, cancellation_observed)
            .await
            .expect("the drain controller should observe cancellation within the test deadline")
            .expect("the component should cancel its drain controller when its health check API server task fails");

        // Assert
        assert!(
            !supervisor.is_finished(),
            "the component must wait for its drain controller before reporting a health check API server task failure"
        );
        release_controller
            .send(())
            .expect("the drain controller should still be waiting for test release");
        let result = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, supervisor)
            .await
            .expect("the component supervisor should complete within the test deadline")
            .expect("the component supervisor should not panic");
        let error = result.expect_err("a panicking health check API server task should fail the component");
        assert!(error.to_string().contains("health check API server task join failed"));
    }

    #[tokio::test]
    async fn it_should_fail_after_draining_when_the_health_check_api_server_returns_an_error() {
        // Arrange
        let server_task = tokio::spawn(async { Err::<(), _>("health check API serving failure") });
        let cancellation_token = CancellationToken::new();
        let (shutdown_controller, cancellation_observed, release_controller) =
            started_drain_controller(cancellation_token.clone()).await;

        // Act
        let supervisor = tokio::spawn(supervise_token_aware_server(
            TokenAwareServerTask::new(server_task, shutdown_controller),
            cancellation_token,
        ));
        tokio::time::timeout(TEST_COMPLETION_TIMEOUT, cancellation_observed)
            .await
            .expect("the drain controller should observe cancellation within the test deadline")
            .expect("the component should cancel its drain controller when its health check API server returns an error");

        // Assert
        assert!(
            !supervisor.is_finished(),
            "the component must wait for its drain controller before reporting a health check API server error"
        );
        release_controller
            .send(())
            .expect("the drain controller should still be waiting for test release");
        let result = tokio::time::timeout(TEST_COMPLETION_TIMEOUT, supervisor)
            .await
            .expect("the component supervisor should complete within the test deadline")
            .expect("the component supervisor should not panic");
        let error = result.expect_err("a health check API server error should fail the component");
        assert!(
            error
                .to_string()
                .contains("health check API server runtime returned an error: health check API serving failure")
        );
    }
}
