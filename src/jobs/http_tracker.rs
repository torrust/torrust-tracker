use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use log::info;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::HttpTracker;

use crate::http::axum_implementation::launcher;
use crate::http::Version;
use crate::tracker;

#[derive(Debug)]
pub struct ServerJobStarted();

pub async fn start_job(config: &HttpTracker, tracker: Arc<tracker::Tracker>, version: Version) -> JoinHandle<()> {
    match version {
        Version::Axum => start_axum(config, tracker.clone()).await,
    }
}

/// # Panics
///
/// It would panic if the `config::HttpTracker` struct would contain inappropriate values.
async fn start_axum(config: &HttpTracker, tracker: Arc<tracker::Tracker>) -> JoinHandle<()> {
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
