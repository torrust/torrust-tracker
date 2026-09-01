//! Health Check API job starter.
//!
//! The [`health_check_api::start_job`](crate::bootstrap::jobs::health_check_api::start_job)
//! function starts the Health Check REST API.
//!
//! The [`health_check_api::start_job`](crate::bootstrap::jobs::health_check_api::start_job)
//! function spawns a new asynchronous task, that tasks is the "**launcher**".
//! The "**launcher**" starts the actual server and sends a message back
//! to the main application.
//!
//! The "**launcher**" is an intermediary thread that decouples the Health Check
//! API server from the process that handles it.
//!
//! Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! for the API configuration options.

use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::logging::STARTED_ON;
use torrust_server_lib::registar::{Registar, ServiceRegistration};
use torrust_server_lib::signals::{Halted, Started};
use torrust_tracker_axum_health_check_api_server::{HEALTH_CHECK_API_LOG_TARGET, server};
use torrust_tracker_configuration::v3_0_0::health_check_api::HealthCheckApi;
use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
use tracing::instrument;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not start the health check API listener. Check that its bind address is available: {source}")]
    Listener { source: std::io::Error },

    #[error("Health check API startup notification was not received: {source}")]
    StartupNotification { source: oneshot::error::RecvError },

    #[error("Could not register the health check API service: {source}")]
    Registration {
        source: torrust_server_lib::registar::RegistrationError,
    },
}

/// This function starts a new Health Check API server with the provided
/// configuration.
///
/// The functions starts a new concurrent task that will run the API server.
/// This task will send a message to the main application process to notify
/// that the API server was successfully started.
///
/// # Errors
///
/// Returns listener, startup-notification, or service-registration errors.
///
/// # Panics
///
/// Panics if its internally created halt channel is unexpectedly closed before
/// the starter returns.
#[allow(clippy::async_yields_async)]
#[instrument(skip(config, registar))]
pub async fn start_job(
    config: &HealthCheckApi,
    registar: Registar<RuntimeServiceMetadata>,
    cancellation_token: CancellationToken,
) -> Result<JoinHandle<()>, Error> {
    let bind_addr = config.bind_address;

    let (tx_start, rx_start) = oneshot::channel::<Started>();
    let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

    let protocol = "http";

    tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "Starting on: {protocol}://{}", bind_addr);
    let running = server::start(bind_addr, tx_start, rx_halt, registar.clone()).map_err(|source| Error::Listener { source })?;

    // Wait until the server sends the started message
    match rx_start.await {
        Ok(msg) => {
            tracing::info!(
                target: HEALTH_CHECK_API_LOG_TARGET,
                service_role = ServiceRole::HealthCheckApi.as_str(),
                instance_index = 0,
                service_binding = %msg.service_binding,
                "Started health check API"
            );

            if let Err(source) = registar
                .give_form()
                .register(ServiceRegistration::new(
                    msg.service_binding,
                    RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HealthCheckApi, 0)),
                    None,
                ))
                .await
            {
                let _ = tx_halt.send(Halted::Normal);
                drop(running.await);
                return Err(Error::Registration { source });
            }

            tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "{STARTED_ON}: {protocol}://{}", msg.address);
        }
        Err(source) => return Err(Error::StartupNotification { source }),
    }

    Ok(tokio::spawn(async move {
        assert!(!tx_halt.is_closed(), "Halt channel for Health Check API should be open");
        tokio::pin!(running);
        tokio::select! {
            () = cancellation_token.cancelled() => {
                let _ = tx_halt.send(Halted::Normal);
                if let Err(error) = (&mut running).await {
                    tracing::warn!(%error, "Health check API stopped with an error after cancellation");
                }
            }
            result = &mut running => if let Err(error) = result {
                tracing::warn!(%error, "Health check API runtime task failed");
            },
        }
        tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "Stopped server running on: {protocol}://{}", bind_addr);
    }))
}
