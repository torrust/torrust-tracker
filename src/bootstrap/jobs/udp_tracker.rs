//! UDP tracker job starter.
//!
//! The [`udp_tracker::start_job`](crate::bootstrap::jobs::udp_tracker::start_job)
//! function starts a new UDP tracker server.
//!
//! > **NOTICE**: that the application can launch more than one UDP tracker
//! > on different ports. Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! > for the configuration options.
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use torrust_server_lib::registar::ServiceRegistrationForm;
use torrust_server_lib::signals::Halted;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use torrust_tracker_udp_core::container::UdpTrackerCoreContainer;
use torrust_tracker_udp_core::{ConnectionIdValidationPolicy, UDP_TRACKER_LOG_TARGET};
use torrust_tracker_udp_server::container::UdpTrackerServerContainer;
use torrust_tracker_udp_server::server::Server;
use torrust_tracker_udp_server::server::spawner::Spawner;
use tracing::instrument;

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
) -> Result<JoinHandle<()>, Error> {
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

    Ok(tokio::spawn(async move {
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, "Wait for launcher (UDP service) to finish ...");
        tracing::debug!(target: UDP_TRACKER_LOG_TARGET, "Is halt channel closed before waiting?: {}", server.state.halt_task.is_closed());

        assert!(
            !server.state.halt_task.is_closed(),
            "Halt channel for UDP tracker should be open"
        );

        let torrust_tracker_udp_server::server::states::Running { halt_task, mut task, .. } = server.state;
        tokio::select! {
            () = cancellation_token.cancelled() => {
                if halt_task.send(Halted::Normal).is_err() {
                    tracing::warn!(target: UDP_TRACKER_LOG_TARGET, "Could not signal UDP tracker to stop after cancellation");
                }
                if let Err(error) = (&mut task).await {
                    tracing::warn!(target: UDP_TRACKER_LOG_TARGET, %error, "Could not join UDP tracker after cancellation");
                }
            }
            result = &mut task => {
                if let Err(error) = result {
                    tracing::warn!(target: UDP_TRACKER_LOG_TARGET, %error, "UDP tracker task failed");
                }
            }
        }
    }))
}
