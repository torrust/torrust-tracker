use std::net::SocketAddr;
use std::sync::Arc;

use log::{info, warn};
use tokio::task::JoinHandle;

use crate::tracker::TorrentTracker;
use crate::{HttpServer, HttpTrackerConfig};

pub fn start_job(config: &HttpTrackerConfig, tracker: Arc<TorrentTracker>) -> JoinHandle<()> {
    let bind_addr = config.bind_address.parse::<SocketAddr>().unwrap();
    let ssl_enabled = config.ssl_enabled;
    let ssl_cert_path = config.ssl_cert_path.clone();
    let ssl_key_path = config.ssl_key_path.clone();

    tokio::spawn(async move {
        let http_tracker = HttpServer::new(tracker);

        if !ssl_enabled {
            info!("Starting HTTP server on: {}", bind_addr);
            http_tracker.start(bind_addr).await;
        } else if ssl_enabled && ssl_cert_path.is_some() && ssl_key_path.is_some() {
            info!("Starting HTTPS server on: {} (TLS)", bind_addr);
            http_tracker
                .start_tls(bind_addr, ssl_cert_path.unwrap(), ssl_key_path.unwrap())
                .await;
        } else {
            warn!("Could not start HTTP tracker on: {}, missing SSL Cert or Key!", bind_addr);
        }
    })
}
