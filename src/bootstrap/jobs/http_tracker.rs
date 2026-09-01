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
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::ServiceRegistrationForm;
use torrust_tracker_axum_http_server::Version;
use torrust_tracker_axum_http_server::server::{HttpServer, Launcher};
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use tracing::instrument;

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
) -> Result<Option<JoinHandle<()>>, Error> {
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

#[allow(clippy::async_yields_async)]
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
) -> Result<JoinHandle<()>, Error> {
    let server = HttpServer::new(Launcher::new(
        socket,
        tls,
        http_tracker_container.http_tracker_config.network.ipv6_v6only,
    ))
    .start(http_tracker_container, form, metadata)
    .await
    .map_err(|source| Error::Listener { source })?;

    Ok(tokio::spawn(async move {
        assert!(
            !server.state.halt_task.is_closed(),
            "Halt channel for HTTP tracker should be open"
        );
        let torrust_tracker_axum_http_server::server::Running { halt_task, mut task, .. } = server.state;

        tokio::select! {
            () = cancellation_token.cancelled() => {
                if halt_task.send(torrust_server_lib::signals::Halted::Normal).is_err() {
                    tracing::warn!("Could not signal HTTP tracker to stop after cancellation");
                }
                if let Err(error) = (&mut task).await {
                    tracing::warn!(%error, "Could not join HTTP tracker after cancellation");
                }
            }
            result = &mut task => {
                if let Err(error) = result {
                    tracing::warn!(%error, "HTTP tracker task failed");
                }
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
    use std::sync::Arc;

    use tempfile::TempDir;
    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::registar::Registar;
    use torrust_tracker_axum_http_server::Version;
    use torrust_tracker_configuration::v3_0_0::database::Database;
    use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::configuration::{ephemeral_public, ephemeral_with_no_services};

    use crate::bootstrap::app::initialize_global_services;
    use crate::bootstrap::jobs::http_tracker::{Error, start_job};
    use crate::container::AppContainer;

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

        // Act / Assert
        start_job(
            http_tracker_container,
            Registar::default().give_form(),
            RuntimeServiceMetadata::new(configuration_instance_id),
            version,
            CancellationToken::new(),
        )
        .await
        .expect("it should be able to start the HTTP tracker");
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
