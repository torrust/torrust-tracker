use std::net::SocketAddr;
use std::sync::Arc;

use log::{info, warn};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::config::HttpTracker;
use crate::http::server::Http;
use crate::tracker;

#[derive(Debug)]
pub struct ServerJobStarted();

/// # Panics
///
/// It would panic if the `config::HttpTracker` struct would contain an inappropriate values.
pub async fn start_job(config: &HttpTracker, tracker: Arc<tracker::Tracker>) -> JoinHandle<()> {
    let bind_addr = config
        .bind_address
        .parse::<SocketAddr>()
        .expect("HTTP tracker server bind_address invalid.");
    let ssl_enabled = config.ssl_enabled;
    let ssl_cert_path = config.ssl_cert_path.clone();
    let ssl_key_path = config.ssl_key_path.clone();

    let (tx, rx) = oneshot::channel::<ServerJobStarted>();

    // Run the HTTP tracker server
    let join_handle = tokio::spawn(async move {
        let http_tracker = Http::new(tracker);

        if !ssl_enabled {
            info!("Starting HTTP tracker server on: http://{}", bind_addr);

            let handle = http_tracker.start(bind_addr);

            tx.send(ServerJobStarted())
                .expect("HTTP tracker server should not be dropped");

            handle.await;

            info!("HTTP tracker server on http://{} stopped", bind_addr);
        } else if ssl_enabled && ssl_cert_path.is_some() && ssl_key_path.is_some() {
            info!("Starting HTTPS server on: https://{}", bind_addr);

            let handle = http_tracker.start_tls(bind_addr, ssl_cert_path.unwrap(), ssl_key_path.unwrap());

            tx.send(ServerJobStarted())
                .expect("HTTP tracker server should not be dropped");

            handle.await;

            info!("HTTP tracker server on https://{} stopped", bind_addr);
        } else {
            warn!(
                "Could not start HTTPS tracker server on: {}, missing SSL Cert or Key!",
                bind_addr
            );
        }
    });

    // Wait until the HTTPS tracker server job is running
    match rx.await {
        Ok(_msg) => info!("HTTP tracker server started"),
        Err(e) => panic!("HTTP tracker server was dropped: {e}"),
    }

    join_handle
}
