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
use torrust_tracker_configuration::HealthCheckApi;

use super::Started;
use crate::servers::health_check_api::{server, HEALTH_CHECK_API_LOG_TARGET};
use crate::servers::logging::STARTED_ON;
use crate::servers::registar::ServiceRegistry;
use crate::servers::signals::Halted;

/// This function starts a new Health Check API server with the provided
/// configuration.
///
/// The functions starts a new concurrent task that will run the API server.
/// This task will send a message to the main application process to notify
/// that the API server was successfully started.
///
/// # Panics
///
/// It would panic if unable to send the  `ApiServerJobStarted` notice.
pub async fn start_job(config: &HealthCheckApi, register: ServiceRegistry) -> JoinHandle<()> {
    let bind_addr = config.bind_address;

    let (tx_start, rx_start) = oneshot::channel::<Started>();
    let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();

    let protocol = "http";

    // Run the API server
    let join_handle = tokio::spawn(async move {
        tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "Starting on: {protocol}://{}", bind_addr);

        let handle = server::start(bind_addr, tx_start, rx_halt, register);

        if let Ok(()) = handle.await {
            tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "Stopped server running on: {protocol}://{}", bind_addr);
        }
    });

    // Wait until the server sends the started message
    match rx_start.await {
        Ok(msg) => tracing::info!(target: HEALTH_CHECK_API_LOG_TARGET, "{STARTED_ON}: {protocol}://{}", msg.address),
        Err(e) => panic!("the Health Check API server was dropped: {e}"),
    }

    // Wait until the server finishes
    tokio::spawn(async move {
        assert!(!tx_halt.is_closed(), "Halt channel for Health Check API should be open");

        join_handle
            .await
            .expect("it should be able to join to the Health Check API server task");
    })
}
