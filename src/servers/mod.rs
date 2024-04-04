//! Servers. Services that can be started and stopped.
pub mod apis;
pub mod health_check_api;
pub mod http;
pub mod registar;
pub mod service;
pub mod signals;
pub mod udp;

pub mod tcp {
    use std::time::Duration;

    use tracing::info;

    use super::signals::Halted;
    use crate::servers::signals::shutdown_signal_with_message;

    pub fn graceful_axum_shutdown(
        handle: axum_server::Handle,
        rx_shutdown: tokio::sync::oneshot::Receiver<Halted>,
        message: String,
    ) {
        tokio::task::spawn(async move {
            match handle.listening().await {
                Some(addr) => {
                    shutdown_signal_with_message(rx_shutdown, format!("{message}, on socket address: {addr}")).await;

                    info!("Sending graceful shutdown signal");
                    handle.graceful_shutdown(Some(Duration::from_secs(90)));

                    println!("!! shuting down in 90 seconds !!");

                    loop {
                        tokio::time::sleep(Duration::from_secs(1)).await;

                        info!("remaining alive connections: {}", handle.connection_count());
                    }
                }
                None => handle.shutdown(),
            }
        });
    }
}
