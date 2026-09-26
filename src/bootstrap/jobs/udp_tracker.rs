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

use crate::bootstrap::jobs::manager::{ComponentCompletion, ComponentError, ComponentResult, OwnedTask};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not start the UDP tracker listener. Check that its bind address is available: {source}")]
    Listener {
        source: torrust_tracker_udp_server::server::UdpError,
    },
}

/// It starts a new UDP server with the provided configuration.
///
/// The receive loop stops when `cancellation_token` is cancelled. The returned
/// component future owns and joins that loop before reporting its outcome.
///
/// # Errors
///
/// Returns a typed listener-start error.
///
#[allow(
    clippy::async_yields_async,
    reason = "startup awaits listener errors before returning the cancellation-aware UDP runtime future"
)]
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
        .start_with_cancellation(
            udp_tracker_core_container,
            udp_tracker_server_container,
            form,
            metadata,
            cookie_lifetime,
            connection_id_validation,
            cancellation_token,
        )
        .await
        .map_err(|source| Error::Listener { source })?;

    Ok(supervise_receive_loop(OwnedTask::new(server.task)))
}

/// Joins the receive loop and turns its result into the component outcome.
///
/// The loop returns `Ok(())` only after cancellation; any other exit is an error.
async fn supervise_receive_loop<E>(mut receive_loop: OwnedTask<Result<(), E>>) -> ComponentResult
where
    E: std::fmt::Display,
{
    tracing::debug!(target: UDP_TRACKER_LOG_TARGET, "Wait for the UDP receive loop to finish ...");

    let loop_result = receive_loop
        .join()
        .await
        .map_err(|error| ComponentError::new(format!("UDP tracker receive loop task failed: {error}")))?;

    loop_result.map_err(|error| ComponentError::new(format!("UDP tracker receive loop stopped with an error: {error}")))?;

    Ok(ComponentCompletion::Cancelled)
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, UdpSocket};
    use std::sync::Arc;
    use std::time::Duration;

    use tokio_util::sync::CancellationToken;
    use torrust_tracker_primitives::RuntimeServiceMetadata;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;
    use torrust_tracker_udp_core::ConnectionIdValidationPolicy;

    use crate::bootstrap::app::initialize_global_services;
    use crate::bootstrap::jobs::manager::{ComponentCompletion, OwnedTask};
    use crate::bootstrap::jobs::udp_tracker::{start_job, supervise_receive_loop};
    use crate::container::AppContainer;

    const TEST_COMPLETION_TIMEOUT: Duration = Duration::from_secs(5);
    const BIND_RETRY_INTERVAL: Duration = Duration::from_millis(10);

    fn available_udp_address() -> SocketAddr {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("select available UDP address");
        socket.local_addr().expect("read available UDP address")
    }

    async fn wait_until_bindable(address: SocketAddr) -> bool {
        tokio::time::timeout(TEST_COMPLETION_TIMEOUT, async {
            while UdpSocket::bind(address).is_err() {
                tokio::time::sleep(BIND_RETRY_INTERVAL).await;
            }
        })
        .await
        .is_ok()
    }

    async fn panicking_receive_loop() -> Result<(), &'static str> {
        tokio::task::yield_now().await;
        panic!("UDP receive loop failure");
    }

    #[tokio::test]
    async fn it_should_report_cancelled_when_the_receive_loop_stops_after_cancellation() {
        let receive_loop = tokio::spawn(async { Ok::<(), &str>(()) });

        let completion = supervise_receive_loop(OwnedTask::new(receive_loop)).await;

        assert_eq!(completion, Ok(ComponentCompletion::Cancelled));
    }

    #[tokio::test]
    async fn it_should_fail_when_the_receive_loop_stops_with_an_error() {
        let receive_loop = tokio::spawn(async { Err::<(), _>("socket closed") });

        let completion = supervise_receive_loop(OwnedTask::new(receive_loop)).await;

        let error = completion.expect_err("a receive-loop error should fail the component");
        assert!(
            error
                .to_string()
                .contains("UDP tracker receive loop stopped with an error: socket closed")
        );
    }

    #[tokio::test]
    async fn it_should_fail_when_the_receive_loop_task_panics() {
        let receive_loop = tokio::spawn(panicking_receive_loop());

        let completion = supervise_receive_loop(OwnedTask::new(receive_loop)).await;

        let error = completion.expect_err("a panicking receive loop should fail the component");
        assert!(error.to_string().contains("UDP tracker receive loop task failed"));
    }

    #[tokio::test]
    async fn it_should_release_the_socket_when_the_component_is_dropped_before_it_runs() {
        // Arrange
        let udp_address = available_udp_address();
        let mut configuration = ephemeral_public();
        configuration.udp_trackers.as_mut().expect("test configuration enables UDP")[0].bind_address = udp_address;
        initialize_global_services(&configuration);
        let app_container = Arc::new(
            AppContainer::initialize(&configuration)
                .await
                .expect("composition should succeed"),
        );
        let (configuration_instance_id, udp_tracker_container) =
            app_container.udp_tracker_container(0).expect("UDP tracker container exists");
        let component = start_job(
            udp_tracker_container,
            app_container.udp_tracker_server_container(),
            app_container.registar.give_form(),
            RuntimeServiceMetadata::new(configuration_instance_id),
            ConnectionIdValidationPolicy::Strict,
            CancellationToken::new(),
        )
        .await
        .expect("the UDP tracker component should start");

        // Act
        drop(component);

        // Assert
        assert!(
            wait_until_bindable(udp_address).await,
            "dropping the unpolled UDP tracker component must release its socket at {udp_address} within {TEST_COMPLETION_TIMEOUT:?}"
        );
    }
}
