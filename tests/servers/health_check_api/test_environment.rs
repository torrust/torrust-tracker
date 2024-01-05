use std::net::SocketAddr;

use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use torrust_tracker::bootstrap::jobs::Started;
use torrust_tracker::servers::health_check_api::server;
use torrust_tracker::servers::registar::ServiceRegistry;
use torrust_tracker_configuration::HealthCheckApi;

/// Start the test environment for the Health Check API.
/// It runs the API server.
pub async fn start(config: &HealthCheckApi, register: ServiceRegistry) -> (SocketAddr, JoinHandle<()>) {
    let bind_addr = config
        .bind_address
        .parse::<std::net::SocketAddr>()
        .expect("Health Check API bind_address invalid.");

    let (tx, rx) = oneshot::channel::<Started>();

    let join_handle = tokio::spawn(async move {
        let handle = server::start(bind_addr, tx, register);
        if let Ok(()) = handle.await {
            panic!("Health Check API server on http://{bind_addr} stopped");
        }
    });

    let bound_addr = match rx.await {
        Ok(msg) => msg.address,
        Err(e) => panic!("the Health Check API server was dropped: {e}"),
    };

    (bound_addr, join_handle)
}
