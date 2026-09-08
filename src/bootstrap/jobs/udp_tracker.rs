//! UDP tracker job starter.
//!
//! The [`udp_tracker::start_job`](crate::bootstrap::jobs::udp_tracker::start_job)
//! function starts a new UDP tracker server.
//!
//! > **NOTICE**: that the application can launch more than one UDP tracker
//! > on different ports. Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! > for the configuration options.
use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::ServiceRegistrationForm;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::{ConnectionIdValidationPolicy, UDP_TRACKER_LOG_TARGET};
use torrust_tracker_udp_server::container::UdpTrackerServerContainer;
use torrust_tracker_udp_server::server::Server;
use torrust_tracker_udp_server::server::spawner::Spawner;
use tracing::instrument;

use crate::bootstrap::jobs::manager::{ComponentCompletion, ComponentError, ComponentResult, NestedServerTask};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not start the UDP tracker listener. Check that its bind address is available: {source}")]
    Listener {
        source: torrust_tracker_udp_server::server::UdpError,
    },
}

/// It starts a new UDP server with the provided configuration.
///
/// It spawns a new asynchronous task for the new UDP server.
///
/// # Errors
///
/// Returns a typed listener-start error.
///
/// # Panics
///
/// Panics if its internally created halt channel is unexpectedly closed before
/// the starter task begins waiting for cancellation or completion.
///
#[allow(clippy::async_yields_async)]
#[instrument(
    skip(udp_tracker_core_container, udp_tracker_server_container, form, metadata),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
pub async fn start_job(
    udp_tracker_core_container: Arc<UdpTrackerCoreContainer>,
    udp_tracker_server_container: Arc<UdpTrackerServerContainer>,
    form: ServiceRegistrationForm<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
    connection_id_validation: ConnectionIdValidationPolicy,
    cancellation_token: CancellationToken,
) -> Result<impl Future<Output = ComponentResult> + Send + 'static, Error> {
    let bind_to = udp_tracker_core_container.udp_tracker_config.bind_address;
    let cookie_lifetime = udp_tracker_core_container.udp_tracker_config.cookie_lifetime;

    tracing::info!(
        bind_address = %bind_to,
        tracker_usage_statistics = udp_tracker_core_container.udp_tracker_config.tracker_usage_statistics,
        "Starting UDP tracker instance"
    );

    let server = Server::new(Spawner::new(bind_to))
        .start(
            udp_tracker_core_container,
            udp_tracker_server_container,
            form,
            metadata,
            cookie_lifetime,
            connection_id_validation,
        )
        .await
        .map_err(|source| Error::Listener { source })?;

    Ok(async move {
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, "Wait for launcher (UDP service) to finish ...");
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, "Is halt channel closed before waiting?: {}", server.state.halt_task.is_closed());

        assert!(
            !server.state.halt_task.is_closed(),
            "Halt channel for UDP tracker should be open"
        );

        let torrust_tracker_udp_server::server::states::Running { halt_task, task, .. } = server.state;
        let mut server_task = NestedServerTask::new(halt_task, task);
        tokio::select! {
            () = cancellation_token.cancelled() => {
                let _ = server_task.signal_shutdown();
                let result = server_task
                    .join()
                    .await
                    .map_err(|error| ComponentError::new(format!("UDP tracker failed while stopping: {error}")))?;
                result.map_err(|error| ComponentError::new(format!("UDP tracker failed while stopping: {error}")))?;
                Ok(ComponentCompletion::Cancelled)
            }
            result = server_task.join() => {
                let result = result
                    .map_err(|error| ComponentError::new(format!("UDP tracker runtime task failed: {error}")))?;
                result.map_err(|error| ComponentError::new(format!("UDP tracker runtime task failed: {error}")))?;
                Ok(ComponentCompletion::Completed)
            }
        }
    })
}
