//! Tracker API job starter.
//!
//! The [`tracker_apis::start_job`](crate::bootstrap::jobs::tracker_apis::start_job)
//! function starts a the HTTP tracker REST API.
//!
//! > **NOTICE**: that even thought there is only one job the API has different
//! > versions. API consumers can choose which version to use. The API version is
//! > part of the URL, for example: `http://localhost:1212/api/v1/stats`.
//!
//! The [`tracker_apis::start_job`](crate::bootstrap::jobs::tracker_apis::start_job)
//! function spawns a new asynchronous task, that tasks is the "**launcher**".
//! The "**launcher**" starts the actual server and sends a message back
//! to the main application. The main application waits until receives
//! the message [`ApiServerJobStarted`]
//! from the "**launcher**".
//!
//! The "**launcher**" is an intermediary thread that decouples the API server
//! from the process that handles it. The API could be used independently
//! in the future. In that case it would not need to notify a parent process.
//!
//! Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! for the API configuration options.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::ServiceRegistrationForm;
use torrust_tracker_axum_rest_api_server::Version;
use torrust_tracker_axum_rest_api_server::server::{ApiServer, Launcher};
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_configuration::v3_0_0::tracker_api::AccessTokens;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
use tracing::instrument;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not load TLS material for the tracker API. Verify the configured certificate and key paths: {source}")]
    Tls {
        source: torrust_tracker_axum_server::tls::Error,
    },

    #[error("Could not start the tracker API listener. Check that its bind address is available: {source}")]
    Listener {
        source: torrust_tracker_axum_rest_api_server::server::Error,
    },
}

/// This is the message that the "launcher" spawned task sends to the main
/// application process to notify the API server was successfully started.
///
/// > **NOTICE**: it does not mean the API server is ready to receive requests.
/// > It only means the new server started. It might take some time to the server
/// > to be ready to accept request.
#[derive(Debug)]
pub struct ApiServerJobStarted();

/// This function starts a new API server with the provided configuration.
///
/// The functions starts a new concurrent task that will run the API server.
/// This task will send a message to the main application process to notify
/// that the API server was successfully started.
///
/// # Errors
///
/// Returns TLS-material or listener-start errors without losing their sources.
///
#[instrument(
    skip(http_api_container, form, metadata),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
pub async fn start_job(
    http_api_container: Arc<TrackerHttpApiCoreContainer>,
    form: ServiceRegistrationForm<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
    version: Version,
    cancellation_token: CancellationToken,
) -> Result<Option<JoinHandle<()>>, Error> {
    let bind_to = http_api_container.http_api_config.bind_address;

    let tls = if let Some(tls_config) = &http_api_container.http_api_config.tls_config {
        Some(make_rust_tls(tls_config).await.map_err(|source| Error::Tls { source })?)
    } else {
        None
    };

    let access_tokens = Arc::new(http_api_container.http_api_config.access_tokens.clone());

    match version {
        Version::V1 => Ok(Some(
            start_v1(
                bind_to,
                tls,
                http_api_container,
                form,
                metadata,
                access_tokens,
                cancellation_token,
            )
            .await?,
        )),
    }
}

#[allow(clippy::async_yields_async)]
#[instrument(
    skip(socket, tls, http_api_container, form, metadata, access_tokens),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
async fn start_v1(
    socket: SocketAddr,
    tls: Option<RustlsConfig>,
    http_api_container: Arc<TrackerHttpApiCoreContainer>,
    form: ServiceRegistrationForm<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
    access_tokens: Arc<AccessTokens>,
    cancellation_token: CancellationToken,
) -> Result<JoinHandle<()>, Error> {
    let server = ApiServer::new(Launcher::new(socket, tls))
        .start(http_api_container, form, metadata, access_tokens)
        .await
        .map_err(|source| Error::Listener { source })?;

    Ok(tokio::spawn(async move {
        assert!(!server.state.halt_task.is_closed(), "Halt channel should be open");
        let torrust_tracker_axum_rest_api_server::server::Running { halt_task, mut task, .. } = server.state;
        tokio::select! {
            () = cancellation_token.cancelled() => {
                if halt_task.send(torrust_server_lib::signals::Halted::Normal).is_err() {
                    tracing::warn!("Could not signal tracker API to stop after cancellation");
                }
                if let Err(error) = (&mut task).await {
                    tracing::warn!(%error, "Could not join tracker API after cancellation");
                }
            }
            result = &mut task => if let Err(error) = result {
                tracing::warn!(%error, "Tracker API task failed");
            },
        }
    }))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::registar::Registar;
    use torrust_tracker_axum_rest_api_server::Version;
    use torrust_tracker_primitives::{ConfigurationInstanceId, ServiceRole};
    use torrust_tracker_rest_api_runtime_adapter::v1::container::TrackerHttpApiCoreContainer;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use crate::bootstrap::app::initialize_global_services;
    use crate::bootstrap::jobs::tracker_apis::start_job;

    #[tokio::test]
    async fn it_should_start_http_tracker() {
        let cfg = Arc::new(ephemeral_public());

        let core_config = Arc::new(cfg.core.clone());

        let http_tracker_config = cfg.http_trackers.clone().expect("missing HTTP tracker configuration");
        let http_tracker_config = Arc::new(http_tracker_config[0].clone());
        let http_tracker_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);

        let udp_tracker_configurations = cfg.udp_trackers.clone().expect("missing UDP tracker configuration");
        let udp_tracker_config = Arc::new(udp_tracker_configurations[0].clone());
        let udp_tracker_server_config = cfg.udp_tracker_server.clone();
        let udp_tracker_configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);

        let http_api_config = Arc::new(cfg.http_api.clone().expect("missing HTTP API configuration"));

        initialize_global_services(&cfg);

        let http_api_container = TrackerHttpApiCoreContainer::initialize(
            &core_config,
            &http_tracker_config,
            http_tracker_configuration_instance_id,
            &udp_tracker_config,
            &udp_tracker_server_config,
            udp_tracker_configuration_instance_id,
            &http_api_config,
        )
        .await;

        let version = Version::V1;

        start_job(
            http_api_container,
            Registar::default().give_form(),
            torrust_tracker_primitives::RuntimeServiceMetadata::new(torrust_tracker_primitives::ConfigurationInstanceId::new(
                torrust_tracker_primitives::ServiceRole::RestApi,
                0,
            )),
            version,
            CancellationToken::new(),
        )
        .await
        .expect("it should be able to start the tracker API");
    }
}
