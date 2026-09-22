//! HTTP tracker job starter.
//!
//! The function [`http_tracker::start_job`](crate::bootstrap::jobs::http_tracker::start_job) starts a new HTTP tracker server.
//!
//! > **NOTICE**: the application can launch more than one HTTP tracker on different ports.
//! > Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
//!
//! The [`http_tracker::start_job`](crate::bootstrap::jobs::http_tracker::start_job) function spawns a new asynchronous task,
//! that tasks is the "**launcher**". The "**launcher**" starts the actual server and sends a message back to the main application.
//!
//! The "**launcher**" is an intermediary thread that decouples the HTTP servers from the process that handles it. The HTTP could be used independently in the future.
//! In that case it would not need to notify a parent process.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::ServiceRegistrationForm;
use torrust_tracker_axum_http_server::Version;
use torrust_tracker_axum_http_server::server::{HttpServer, Launcher};
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use tracing::instrument;

use crate::bootstrap::jobs::manager::{ComponentCompletion, ComponentError, ComponentResult, TokenAwareServerTask};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not load TLS material for the HTTP tracker. Verify the configured certificate and key paths: {source}")]
    Tls {
        source: torrust_tracker_axum_server::tls::Error,
    },

    #[error("Could not start the HTTP tracker listener. Check that its bind address is available: {source}")]
    Listener {
        source: torrust_tracker_axum_http_server::server::Error,
    },
}

/// It starts a new HTTP server with the provided configuration and version.
///
/// Right now there is only one version but in the future we could support more than one HTTP tracker version at the same time.
/// This feature allows supporting breaking changes on `BitTorrent` BEPs.
///
/// # Errors
///
/// Returns TLS-material or listener-start errors without losing their sources.
///
#[instrument(
    skip(http_tracker_container, form, metadata),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
pub async fn start_job(
    http_tracker_container: Arc<HttpTrackerCoreContainer>,
    form: ServiceRegistrationForm<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
    version: Version,
    cancellation_token: CancellationToken,
) -> Result<Option<impl Future<Output = ComponentResult> + Send + 'static>, Error> {
    let socket = http_tracker_container.http_tracker_config.bind_address;

    tracing::info!(
        bind_address = %socket,
        tracker_usage_statistics = http_tracker_container.http_tracker_config.tracker_usage_statistics,
        "Starting HTTP tracker instance"
    );

    let tls = if let Some(tls_config) = &http_tracker_container.http_tracker_config.tls_config {
        Some(make_rust_tls(tls_config).await.map_err(|source| Error::Tls { source })?)
    } else {
        None
    };

    match version {
        Version::V1 => Ok(Some(
            start_v1(socket, tls, http_tracker_container, form, metadata, cancellation_token).await?,
        )),
    }
}

#[allow(
    clippy::async_yields_async,
    reason = "startup awaits listener errors before returning the cancellation-aware HTTP runtime future"
)]
#[instrument(
    skip(socket, tls, http_tracker_container, form, metadata),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
async fn start_v1(
    socket: SocketAddr,
    tls: Option<RustlsConfig>,
    http_tracker_container: Arc<HttpTrackerCoreContainer>,
    form: ServiceRegistrationForm<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
    cancellation_token: CancellationToken,
) -> Result<impl Future<Output = ComponentResult> + Send + 'static, Error> {
    let server = HttpServer::new(Launcher::new(
        socket,
        tls,
        http_tracker_container.http_tracker_config.network.ipv6_v6only,
    ))
    .start_with_cancellation(http_tracker_container, form, metadata, cancellation_token.clone())
    .await
    .map_err(|source| Error::Listener { source })?;

    Ok(supervise_token_aware_server(
        TokenAwareServerTask::new(server.task, server.shutdown_controller),
        cancellation_token,
    ))
}

async fn supervise_token_aware_server<T>(
    mut server_task: TokenAwareServerTask<T, GracefulShutdownOutcome>,
    cancellation_token: CancellationToken,
) -> ComponentResult {
    tokio::select! {
        biased;
        () = cancellation_token.cancelled() => {
            server_task
                .join()
                .await
                .map_err(|error| ComponentError::new(format!("HTTP tracker failed while stopping: {error}")))?;
            match server_task
                .join_shutdown_controller()
                .await
                .map_err(|error| ComponentError::new(format!("HTTP tracker drain controller failed: {error}")))?
            {
                GracefulShutdownOutcome::Drained => Ok(ComponentCompletion::Cancelled),
                GracefulShutdownOutcome::TimedOut => Err(ComponentError::new("HTTP tracker graceful drain timed out")),
            }
        }
        result = server_task.join() => {
            cancellation_token.cancel();
            let drain_outcome = server_task
                .join_shutdown_controller()
                .await
                .map_err(|error| ComponentError::new(format!("HTTP tracker drain controller failed: {error}")))?;
            result.map_err(|error| ComponentError::new(format!("HTTP tracker runtime task failed: {error}")))?;
            match drain_outcome {
                GracefulShutdownOutcome::Drained => Ok(ComponentCompletion::Completed),
                GracefulShutdownOutcome::TimedOut => Err(ComponentError::new("HTTP tracker graceful drain timed out")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
    use std::sync::Arc;

    use tempfile::TempDir;
    use tokio::sync::oneshot;
    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::registar::Registar;
    use torrust_tracker_axum_http_server::Version;
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;
    use torrust_tracker_configuration::v3_0_0::database::Database;
    use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::configuration::{ephemeral_public, ephemeral_with_no_services};

    use crate::bootstrap::app::initialize_global_services;
    use crate::bootstrap::jobs::http_tracker::{Error, start_job, supervise_token_aware_server};
    use crate::bootstrap::jobs::manager::{ComponentCompletion, TokenAwareServerTask};
    use crate::container::AppContainer;

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
        controller_started
            .await
            .expect("the drain controller should start before the component runs");

        (shutdown_controller, cancellation_observed, release_sender)
    }

    #[tokio::test]
    async fn it_should_start_http_tracker() {
        // Arrange
        // Keep the database parent directory alive for the whole test. Use the
        // test's current working directory rather than the process temp path:
        // nextest changes its temporary paths after archive extraction in the
        // container image.
        let database_workspace = TempDir::new_in(std::env::current_dir().expect("read test working directory"))
            .expect("create test database workspace");
        let database_path = database_workspace.path().join("tracker.sqlite3.db");
        let mut cfg = ephemeral_public();
        cfg.core.database = Some(Database::Sqlite3 {
            path: database_path.to_string_lossy().into_owned(),
        });
        let cfg = Arc::new(cfg);
        let core_config = Arc::new(cfg.core.clone());
        let http_tracker = cfg.http_trackers.clone().expect("missing HTTP tracker configuration");
        let http_tracker_config = Arc::new(http_tracker[0].clone());
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);

        initialize_global_services(&cfg);

        let http_tracker_container =
            HttpTrackerCoreContainer::initialize(&core_config, &http_tracker_config, configuration_instance_id).await;

        let version = Version::V1;

        // Act
        let cancellation_token = CancellationToken::new();
        let job = start_job(
            http_tracker_container,
            Registar::default().give_form(),
            RuntimeServiceMetadata::new(configuration_instance_id),
            version,
            cancellation_token.clone(),
        )
        .await
        .expect("it should be able to start the HTTP tracker")
        .expect("V1 should return an HTTP tracker runner");

        let job = tokio::spawn(job);
        cancellation_token.cancel();

        // Assert
        let completion = job
            .await
            .expect("HTTP tracker should stop without panicking")
            .expect("HTTP tracker runner should report a successful cooperative cancellation");

        // Assert
        assert_eq!(completion, ComponentCompletion::Cancelled);
    }

    #[tokio::test]
    async fn it_should_complete_after_draining_when_the_server_stops_independently() {
        // Arrange
        let server_task = tokio::spawn(async {});
        let cancellation_token = CancellationToken::new();
        let (shutdown_controller, cancellation_observed, release_controller) =
            started_drain_controller(cancellation_token.clone()).await;

        // Act
        let supervisor = tokio::spawn(supervise_token_aware_server(
            TokenAwareServerTask::new(server_task, shutdown_controller),
            cancellation_token,
        ));
        cancellation_observed
            .await
            .expect("the component should cancel its drain controller when its server stops independently");

        // Assert
        assert!(
            !supervisor.is_finished(),
            "the component must wait for its drain controller before reporting independent server completion"
        );
        release_controller
            .send(())
            .expect("the drain controller should still be waiting for test release");
        let completion = supervisor.await.expect("the component supervisor should not panic");
        assert_eq!(completion, Ok(ComponentCompletion::Completed));
    }

    #[tokio::test]
    async fn it_should_fail_after_draining_when_the_server_task_panics() {
        // Arrange
        let server_task = tokio::spawn(async {
            panic!("server task failure");
        });
        let cancellation_token = CancellationToken::new();
        let (shutdown_controller, cancellation_observed, release_controller) =
            started_drain_controller(cancellation_token.clone()).await;

        // Act
        let supervisor = tokio::spawn(supervise_token_aware_server(
            TokenAwareServerTask::new(server_task, shutdown_controller),
            cancellation_token,
        ));
        cancellation_observed
            .await
            .expect("the component should cancel its drain controller when its server task fails");

        // Assert
        assert!(
            !supervisor.is_finished(),
            "the component must wait for its drain controller before reporting a server task failure"
        );
        release_controller
            .send(())
            .expect("the drain controller should still be waiting for test release");
        let result = supervisor.await.expect("the component supervisor should not panic");
        let error = result.expect_err("a panicking server task should fail the HTTP component");
        assert!(error.to_string().contains("HTTP tracker runtime task failed"));
    }

    #[tokio::test]
    async fn it_should_return_a_tls_error_before_starting_the_http_listener() {
        // Arrange
        let mut configuration = ephemeral_with_no_services();
        let http_tracker_config = torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker {
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            tls_config: Some(torrust_tracker_configuration::v3_0_0::tls::TlsConfig::default()),
            ..Default::default()
        };
        configuration.http_trackers = Some(vec![http_tracker_config]);
        let app_container = AppContainer::initialize(&configuration)
            .await
            .expect("compose HTTP tracker container");
        let (instance_id, http_tracker_container) = app_container.http_tracker_container(0).expect("get HTTP tracker container");

        // Act
        let result = start_job(
            http_tracker_container,
            app_container.registar.give_form(),
            RuntimeServiceMetadata::new(instance_id),
            Version::V1,
            CancellationToken::new(),
        )
        .await;

        // Assert
        assert!(matches!(result, Err(Error::Tls { .. })));
    }

    #[tokio::test]
    async fn it_should_return_a_listener_error_through_the_public_http_starter() {
        // Arrange
        let listener = TcpListener::bind("127.0.0.1:0").expect("reserve HTTP listener address");
        let mut configuration = ephemeral_with_no_services();
        configuration.http_trackers = Some(vec![torrust_tracker_configuration::v3_0_0::http_tracker::HttpTracker {
            bind_address: listener.local_addr().expect("read HTTP listener address"),
            ..Default::default()
        }]);
        let app_container = AppContainer::initialize(&configuration)
            .await
            .expect("compose HTTP tracker container");
        let (instance_id, http_tracker_container) = app_container.http_tracker_container(0).expect("get HTTP tracker container");

        // Act
        let result = start_job(
            http_tracker_container,
            app_container.registar.give_form(),
            RuntimeServiceMetadata::new(instance_id),
            Version::V1,
            CancellationToken::new(),
        )
        .await;

        // Assert
        assert!(matches!(result, Err(Error::Listener { .. })));
    }
}
