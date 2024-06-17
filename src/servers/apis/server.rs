//! Logic to run the HTTP API server.
//!
//! It contains two main structs: `ApiServer` and `Launcher`,
//! and two main functions: `start` and `start_tls`.
//!
//! The `ApiServer` struct is responsible for:
//! - Starting and stopping the server.
//! - Storing the configuration.
//!
//! `ApiServer` relies on a launcher to start the actual server.
///
/// 1. `ApiServer::start` -> spawns new asynchronous task.
/// 2. `Launcher::start` -> starts the server on the spawned task.
///
/// The `Launcher` struct is responsible for:
///
/// - Knowing how to start the server with graceful shutdown.
///
/// For the time being the `ApiServer` and `Launcher` are only used in tests
/// where we need to start and stop the server multiple times. In production
/// code and the main application uses the `start` and `start_tls` functions
/// to start the servers directly since we do not need to control the server
/// when it's running. In the future we might need to control the server,
/// for example, to restart it to apply new configuration changes, to remotely
/// shutdown the server, etc.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use derive_more::{Constructor, Display};
use futures::{FutureExt as _, TryFutureExt as _};
use torrust_tracker_configuration::AccessTokens;
use torrust_tracker_located_error::DynError;
use tracing::info;

use super::routes::router;
use crate::core::Tracker;
use crate::servers::custom_axum_server::{self, TimeoutAcceptor};
use crate::servers::registar::{self, HealthCheckFactory, HeathCheckFuture, HeathCheckResult};
use crate::servers::service::{self, AddrFuture, Error, TaskFuture};
use crate::servers::signals::Halted;
use crate::servers::tcp::graceful_axum_shutdown;

/// Build a health check future task for checking
/// the http api health check endpoint.
///
#[must_use]
fn check_builder(addr: SocketAddr) -> HeathCheckFuture<'static> {
    let url = format!("http://{addr}/api/health_check");

    info!("checking api health check at: {url}");

    let response = reqwest::get(url)
        .map_err(move |e| registar::Error::UnableToGetAnyResponse {
            addr,
            msg: "Udp Client".to_string(),
            err: DynError::into(Arc::new(e)),
        })
        .boxed();

    let check = response
        .and_then(move |r| async move {
            r.error_for_status()
                .map_err(move |e| registar::Error::UnableToObtainGoodResponse {
                    addr,
                    msg: "Udp Client".to_string(),
                    err: DynError::into(Arc::new(e)),
                })
        })
        .boxed();

    check
        .map_ok(move |r| registar::Success::AllGood {
            addr,
            msg: r.status().to_string(),
        })
        .map(HeathCheckResult::from)
        .boxed()
}

#[derive(Debug)]
pub struct ApiHandle {
    pub axum_handle: axum_server::Handle,
    tx_shutdown: Option<tokio::sync::oneshot::Sender<Halted>>,
}

impl ApiHandle {
    fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(tx) = self.tx_shutdown.take() {
            tx.send(Halted::Normal)
                .map_err(|err| Error::UnableToSendHaltingMessage { err })?;
        } else {
            panic!("it has already taken the channel?");
        };

        Ok(())
    }
}

impl Default for ApiHandle {
    fn default() -> Self {
        let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel::<Halted>();

        let axum_handle = axum_server::Handle::default();

        let () = graceful_axum_shutdown(axum_handle.clone(), rx_shutdown, "Api service".to_owned());

        Self {
            axum_handle: axum_server::Handle::new(),
            tx_shutdown: Some(tx_shutdown),
        }
    }
}

impl service::Handle for ApiHandle {
    fn stop(mut self) -> Result<(), Error> {
        self.shutdown()
    }

    fn listening(&self) -> AddrFuture<'_> {
        self.axum_handle.listening().boxed()
    }

    fn into_graceful_shutdown_future<'a>(self) -> futures::prelude::future::BoxFuture<'a, Result<(), Error>> {
        todo!()
    }
}

impl Drop for ApiHandle {
    fn drop(&mut self) {
        self.shutdown().expect("it should shutdown when dropped");
    }
}

#[derive(Constructor, Clone, Debug, Display)]
#[display(fmt = "intended_address: {addr}, with tracker, tokens, and  {}", "self.have_tls()")]
pub struct ApiLauncher {
    pub tracker: Arc<Tracker>,
    pub access_tokens: Arc<AccessTokens>,
    pub addr: SocketAddr,
    pub tls: Option<RustlsConfig>,
}

impl ApiLauncher {
    fn have_tls(&self) -> String {
        match self.tls {
            Some(_) => "some",
            None => "none",
        }
        .to_string()
    }
}

impl service::Launcher<ApiHandle> for ApiLauncher {
    fn start(self) -> Result<(TaskFuture<'static, (), Error>, ApiHandle, HealthCheckFactory), Error> {
        let handle = ApiHandle::default();

        let running: TaskFuture<'_, (), Error> = {
            let listener = std::net::TcpListener::bind(self.addr).map_err(|e| Error::UnableToBindToSocket {
                addr: self.addr,
                err: e.into(),
            })?;

            let addr = listener
                .local_addr()
                .map_err(|e| Error::UnableToGetLocalAddress { err: e.into() })?;

            let make_service =
                router(self.tracker, self.access_tokens, &addr).into_make_service_with_connect_info::<std::net::SocketAddr>();

            match self.tls.clone() {
                Some(tls) => custom_axum_server::from_tcp_rustls_with_timeouts(listener, tls)
                    .handle(handle.axum_handle.clone())
                    .acceptor(TimeoutAcceptor)
                    .serve(make_service)
                    .map_err(|e| Error::UnableToServe { err: e.into() })
                    .boxed(),

                None => custom_axum_server::from_tcp_with_timeouts(listener)
                    .handle(handle.axum_handle.clone())
                    .acceptor(TimeoutAcceptor)
                    .serve(make_service)
                    .map_err(|e| Error::UnableToServe { err: e.into() })
                    .boxed(),
            }
        };

        let check_factory = HealthCheckFactory::new(check_builder);

        Ok((running, handle, check_factory))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::tracker;
    use crate::bootstrap::jobs::make_rust_tls;
    use crate::servers::apis::server::ApiLauncher;
    use crate::servers::registar::Registar;
    use crate::servers::service::Service;

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_mode_public());
        let config = &cfg.http_api;

        let tracker = tracker(&cfg);

        let addr = config.bind_address;

        let tls = match &config.tsl_config {
            Some(tls_config) => Some(
                make_rust_tls(tls_config)
                    .await
                    .expect("it should have a valid tracker api tls configuration"),
            ),
            None => None,
        };

        let access_tokens = Arc::new(config.access_tokens.clone());

        let register = &Registar::default();

        let stopped = Service::new(ApiLauncher::new(tracker, access_tokens, addr, tls));

        let started = stopped.start().expect("it should start the server");
        let () = started.reg_form(register.form()).await.expect("it should register");

        let stopped = started.stop().await.expect("it should stop the server");

        drop(stopped);
    }
}
