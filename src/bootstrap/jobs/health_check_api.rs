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

use log::info;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::HealthCheckApi;

use super::Started;
use crate::servers::health_check_api::server;
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
    let bind_addr = config
        .bind_address
        .parse::<std::net::SocketAddr>()
        .expect("it should have a valid health check bind address");

    let (tx_start, rx_start) = oneshot::channel::<Started>();
    let (tx_halt, rx_halt) = tokio::sync::oneshot::channel::<Halted>();
    drop(tx_halt);

    // Run the API server
    let join_handle = tokio::spawn(async move {
        info!(target: "Health Check API", "Starting on: http://{}", bind_addr);

        let handle = server::start(bind_addr, tx_start, rx_halt, register);

        if let Ok(()) = handle.await {
            info!(target: "Health Check API", "Stopped server running on: http://{}", bind_addr);
        }
    });

    // Wait until the API server job is running
    match rx_start.await {
        Ok(msg) => info!(target: "Health Check API", "Started on: http://{}", msg.address),
        Err(e) => panic!("the Health Check API server was dropped: {e}"),
    }

    join_handle
}
