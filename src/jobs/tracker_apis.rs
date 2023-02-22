use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use log::info;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::HttpApi;

use crate::apis::server;
use crate::tracker;

#[derive(Debug)]
pub struct ApiServerJobStarted();

/// # Panics
///
/// It would panic if unable to send the  `ApiServerJobStarted` notice.
pub async fn start_job(config: &HttpApi, tracker: Arc<tracker::Tracker>) -> JoinHandle<()> {
    let bind_addr = config
        .bind_address
        .parse::<std::net::SocketAddr>()
        .expect("Tracker API bind_address invalid.");
    let ssl_enabled = config.ssl_enabled;
    let ssl_cert_path = config.ssl_cert_path.clone();
    let ssl_key_path = config.ssl_key_path.clone();

    let (tx, rx) = oneshot::channel::<ApiServerJobStarted>();

    // Run the API server
    let join_handle = tokio::spawn(async move {
        if !ssl_enabled {
            info!("Starting Torrust APIs server on: http://{}", bind_addr);

            let handle = server::start(bind_addr, &tracker);

            tx.send(ApiServerJobStarted()).expect("the API server should not be dropped");

            if let Ok(()) = handle.await {
                info!("Torrust APIs server on http://{} stopped", bind_addr);
            }
        } else if ssl_enabled && ssl_cert_path.is_some() && ssl_key_path.is_some() {
            info!("Starting Torrust APIs server on: https://{}", bind_addr);

            let ssl_config = RustlsConfig::from_pem_file(ssl_cert_path.unwrap(), ssl_key_path.unwrap())
                .await
                .unwrap();

            let handle = server::start_tls(bind_addr, ssl_config, &tracker);

            tx.send(ApiServerJobStarted()).expect("the API server should not be dropped");

            if let Ok(()) = handle.await {
                info!("Torrust APIs server on https://{} stopped", bind_addr);
            }
        }
    });

    // Wait until the APIs server job is running
    match rx.await {
        Ok(_msg) => info!("Torrust APIs server started"),
        Err(e) => panic!("the API server was dropped: {e}"),
    }

    join_handle
}
