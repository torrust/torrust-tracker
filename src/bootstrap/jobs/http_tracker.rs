//! HTTP tracker job starter.
//!
//! The function [`http_tracker::start_job`](crate::bootstrap::jobs::http_tracker::start_job) starts a new HTTP tracker server.
//!
//! > **NOTICE**: the application can launch more than one HTTP tracker on different ports.
//! Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
//!
//! The [`http_tracker::start_job`](crate::bootstrap::jobs::http_tracker::start_job) function spawns a new asynchronous task,
//! that tasks is the "**launcher**". The "**launcher**" starts the actual server and sends a message back to the main application.
//! The main application waits until receives the message [`ServerJobStarted`] from the "**launcher**".
//!
//! The "**launcher**" is an intermediary thread that decouples the HTTP servers from the process that handles it. The HTTP could be used independently in the future.
//! In that case it would not need to notify a parent process.
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use log::info;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::HttpTracker;

use crate::core;
use crate::servers::http::v1::launcher;
use crate::servers::http::Version;

/// This is the message that the "**launcher**" spawned task sends to the main application process to notify that the HTTP server was successfully started.
///
/// > **NOTICE**: it does not mean the HTTP server is ready to receive requests. It only means the new server started. It might take some time to the server to be ready to accept request.
#[derive(Debug)]
pub struct ServerJobStarted();

/// It starts a new HTTP server with the provided configuration and version.
///
/// Right now there is only one version but in the future we could support more than one HTTP tracker version at the same time.
/// This feature allows supporting breaking changes on `BitTorrent` BEPs.
pub async fn start_job(config: &HttpTracker, tracker: Arc<core::Tracker>, version: Version) -> JoinHandle<()> {
    match version {
        Version::V1 => start_v1(config, tracker.clone()).await,
    }
}

/// # Panics
///
/// It would panic if the `config::HttpTracker` struct would contain inappropriate values.
async fn start_v1(config: &HttpTracker, tracker: Arc<core::Tracker>) -> JoinHandle<()> {
    let bind_addr = config
        .bind_address
        .parse::<std::net::SocketAddr>()
        .expect("Tracker API bind_address invalid.");
    let ssl_enabled = config.ssl_enabled;
    let ssl_cert_path = config.ssl_cert_path.clone();
    let ssl_key_path = config.ssl_key_path.clone();

    let (tx, rx) = oneshot::channel::<ServerJobStarted>();

    // Run the API server
    let join_handle = tokio::spawn(async move {
        if !ssl_enabled {
            info!("Starting Torrust HTTP tracker server on: http://{}", bind_addr);

            let handle = launcher::start(bind_addr, tracker);

            tx.send(ServerJobStarted())
                .expect("the HTTP tracker server should not be dropped");

            if let Ok(()) = handle.await {
                info!("Torrust HTTP tracker server on http://{} stopped", bind_addr);
            }
        } else if ssl_enabled && ssl_cert_path.is_some() && ssl_key_path.is_some() {
            info!("Starting Torrust HTTP tracker server on: https://{}", bind_addr);

            let ssl_config = RustlsConfig::from_pem_file(ssl_cert_path.unwrap(), ssl_key_path.unwrap())
                .await
                .unwrap();

            let handle = launcher::start_tls(bind_addr, ssl_config, tracker);

            tx.send(ServerJobStarted())
                .expect("the HTTP tracker server should not be dropped");

            if let Ok(()) = handle.await {
                info!("Torrust HTTP tracker server on https://{} stopped", bind_addr);
            }
        }
    });

    // Wait until the HTTP tracker server job is running
    match rx.await {
        Ok(_msg) => info!("Torrust HTTP tracker server started"),
        Err(e) => panic!("the HTTP tracker server was dropped: {e}"),
    }

    join_handle
}
