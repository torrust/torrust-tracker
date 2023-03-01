use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use futures::Future;
use log::info;
use tokio::task::JoinHandle;
use warp::hyper;

use super::routes::router;
use crate::signals::shutdown_signal_with_message;
use crate::tracker::Tracker;

#[derive(Debug)]
pub enum Error {
    Error(String),
}

#[allow(clippy::module_name_repetitions)]
pub type StoppedApiServer = ApiServer<Stopped>;
#[allow(clippy::module_name_repetitions)]
pub type RunningApiServer = ApiServer<Running>;

#[allow(clippy::module_name_repetitions)]
pub struct ApiServer<S> {
    pub cfg: torrust_tracker_configuration::HttpApi,
    pub tracker: Arc<Tracker>,
    pub state: S,
}

pub struct Stopped;

pub struct Running {
    pub bind_address: SocketAddr,
    stop_job_sender: tokio::sync::oneshot::Sender<u8>,
    job: JoinHandle<()>,
}

impl ApiServer<Stopped> {
    pub fn new(cfg: torrust_tracker_configuration::HttpApi, tracker: Arc<Tracker>) -> Self {
        Self {
            cfg,
            tracker,
            state: Stopped {},
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `TcpListener` can not bind to `bind_address`.
    pub fn start(self) -> Result<ApiServer<Running>, Error> {
        let listener = TcpListener::bind(&self.cfg.bind_address).map_err(|e| Error::Error(e.to_string()))?;

        let bind_address = listener.local_addr().map_err(|e| Error::Error(e.to_string()))?;

        let cfg = self.cfg.clone();
        let tracker = self.tracker.clone();

        let (sender, receiver) = tokio::sync::oneshot::channel::<u8>();

        let job = tokio::spawn(async move {
            if let (true, Some(ssl_cert_path), Some(ssl_key_path)) = (cfg.ssl_enabled, cfg.ssl_cert_path, cfg.ssl_key_path) {
                let tls_config = RustlsConfig::from_pem_file(ssl_cert_path, ssl_key_path)
                    .await
                    .expect("Could not read ssl cert and/or key.");

                start_tls_from_tcp_listener_with_graceful_shutdown(listener, tls_config, &tracker, receiver)
                    .await
                    .expect("Could not start from tcp listener with tls.");
            } else {
                start_from_tcp_listener_with_graceful_shutdown(listener, &tracker, receiver)
                    .await
                    .expect("Could not start from tcp listener.");
            }
        });

        let running_api_server: ApiServer<Running> = ApiServer {
            cfg: self.cfg,
            tracker: self.tracker,
            state: Running {
                bind_address,
                stop_job_sender: sender,
                job,
            },
        };

        Ok(running_api_server)
    }
}

impl ApiServer<Running> {
    /// # Errors
    ///
    /// Will return `Err` if the oneshot channel to send the stop signal
    /// has already been called once.
    pub async fn stop(self) -> Result<ApiServer<Stopped>, Error> {
        self.state.stop_job_sender.send(1).map_err(|e| Error::Error(e.to_string()))?;

        let _ = self.state.job.await;

        let stopped_api_server: ApiServer<Stopped> = ApiServer {
            cfg: self.cfg,
            tracker: self.tracker,
            state: Stopped {},
        };

        Ok(stopped_api_server)
    }
}

pub fn start_from_tcp_listener_with_graceful_shutdown(
    tcp_listener: TcpListener,
    tracker: &Arc<Tracker>,
    shutdown_signal: tokio::sync::oneshot::Receiver<u8>,
) -> impl Future<Output = hyper::Result<()>> {
    let app = router(tracker);

    let context = tcp_listener.local_addr().expect("Could not get context.");

    axum::Server::from_tcp(tcp_listener)
        .expect("Could not bind to tcp listener.")
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal_with_message(
            shutdown_signal,
            format!("Shutting down {context}.."),
        ))
}

pub fn start_tls_from_tcp_listener_with_graceful_shutdown(
    tcp_listener: TcpListener,
    tls_config: RustlsConfig,
    tracker: &Arc<Tracker>,
    shutdown_signal: tokio::sync::oneshot::Receiver<u8>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let app = router(tracker);

    let context = tcp_listener.local_addr().expect("Could not get context.");

    let handle = Handle::new();

    let cloned_handle = handle.clone();

    tokio::spawn(async move {
        shutdown_signal_with_message(shutdown_signal, format!("Shutting down {context}..")).await;
        cloned_handle.shutdown();
    });

    axum_server::from_tcp_rustls(tcp_listener, tls_config)
        .handle(handle)
        .serve(app.into_make_service())
}

pub fn start(socket_addr: SocketAddr, tracker: &Arc<Tracker>) -> impl Future<Output = hyper::Result<()>> {
    let app = router(tracker);

    let server = axum::Server::bind(&socket_addr).serve(app.into_make_service());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust APIs server on http://{} ...", socket_addr);
    })
}

pub fn start_tls(
    socket_addr: SocketAddr,
    ssl_config: RustlsConfig,
    tracker: &Arc<Tracker>,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let app = router(tracker);

    let handle = Handle::new();
    let shutdown_handle = handle.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to listen to shutdown signal.");
        info!("Stopping Torrust APIs server on https://{} ...", socket_addr);
        shutdown_handle.shutdown();
    });

    axum_server::bind_rustls(socket_addr, ssl_config)
        .handle(handle)
        .serve(app.into_make_service())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_configuration::Configuration;

    use crate::apis::server::ApiServer;
    use crate::tracker;
    use crate::tracker::statistics;

    fn tracker_configuration() -> Arc<Configuration> {
        Arc::new(torrust_tracker_test_helpers::configuration::ephemeral())
    }

    #[tokio::test]
    async fn it_should_be_able_to_start_from_stopped_state_and_then_stop_again() {
        let cfg = tracker_configuration();

        let tracker = Arc::new(tracker::Tracker::new(&cfg, None, statistics::Repo::new()).unwrap());

        let stopped_api_server = ApiServer::new(cfg.http_api.clone(), tracker);

        let running_api_server_result = stopped_api_server.start();

        assert!(running_api_server_result.is_ok());

        let running_api_server = running_api_server_result.unwrap();

        assert!(running_api_server.stop().await.is_ok());
    }
}
