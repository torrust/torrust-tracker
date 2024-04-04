//! Module to handle the HTTP server instances.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use derive_more::{Constructor, Display};
use futures::{FutureExt, TryFutureExt as _};

use super::v1::routes::router;
use crate::core::Tracker;
use crate::servers::registar::{FnSpawnServiceHeathCheck, ServiceHealthCheckJob};
use crate::servers::service::{AddrFuture, Error, Handle, Launcher, TaskFuture};
use crate::servers::signals::Halted;
use crate::servers::tcp::graceful_axum_shutdown;

/// Checks the Health by connecting to the HTTP tracker endpoint.
///
/// # Errors
///
/// This function will return an error if unable to connect.
/// Or if the request returns an error.
#[must_use]
fn check_fn(binding: &SocketAddr) -> ServiceHealthCheckJob {
    let url = format!("http://{binding}/health_check");

    let info = format!("checking http tracker health check at: {url}");

    let job = tokio::spawn(async move {
        match reqwest::get(url).await {
            Ok(response) => Ok(response.status().to_string()),
            Err(err) => Err(err.to_string()),
        }
    });

    ServiceHealthCheckJob::new(*binding, info, job)
}

#[derive(Debug)]
pub struct HttpHandle {
    pub axum_handle: axum_server::Handle,
    tx_shutdown: Option<tokio::sync::oneshot::Sender<Halted>>,
}

impl HttpHandle {
    fn shutdown(&mut self) -> Result<(), Error> {
        let () = if let Some(tx) = self.tx_shutdown.take() {
            tx.send(Halted::Normal)
                .map_err(|err| Error::UnableToSendHaltingMessage { err })?;
        } else {
            panic!("it has already taken the channel?");
        };
        Ok(())
    }
}

impl Default for HttpHandle {
    fn default() -> Self {
        let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel::<Halted>();

        let axum_handle = axum_server::Handle::default();

        let () = graceful_axum_shutdown(axum_handle.clone(), rx_shutdown, "HTTP service".to_string());

        Self {
            axum_handle: axum_server::Handle::new(),
            tx_shutdown: Some(tx_shutdown),
        }
    }
}

impl Handle for HttpHandle {
    fn stop(mut self) -> Result<(), Error> {
        self.shutdown()
    }

    fn listening(&self) -> AddrFuture<'_> {
        self.axum_handle.listening().boxed()
    }
}

impl Drop for HttpHandle {
    fn drop(&mut self) {
        self.shutdown().expect("it should shutdown when dropped");
    }
}

#[derive(Constructor, Clone, Debug, Display)]
#[display(fmt = "intended_address: {addr}, with tracker, and  {}", "self.have_tls()")]
pub struct HttpLauncher {
    pub tracker: Arc<Tracker>,
    pub addr: SocketAddr,
    pub tls: Option<RustlsConfig>,
}

impl HttpLauncher {
    fn have_tls(&self) -> String {
        match self.tls {
            Some(_) => "some",
            None => "none",
        }
        .to_string()
    }
}

impl Launcher<HttpHandle> for HttpLauncher {
    fn start(self) -> Result<(TaskFuture<'static, (), Error>, HttpHandle, FnSpawnServiceHeathCheck), Error> {
        let handle = HttpHandle::default();

        let running: TaskFuture<'_, (), Error> = {
            let listener = std::net::TcpListener::bind(self.addr).map_err(|e| Error::UnableToBindToSocket {
                addr: self.addr,
                err: e.into(),
            })?;

            let addr = listener
                .local_addr()
                .map_err(|e| Error::UnableToGetLocalAddress { err: e.into() })?;

            let make_service = router(self.tracker, &addr).into_make_service_with_connect_info::<std::net::SocketAddr>();

            match self.tls.clone() {
                Some(tls) => axum_server::from_tcp_rustls(listener, tls)
                    .handle(handle.axum_handle.clone())
                    .serve(make_service)
                    .map_err(|e| Error::UnableToServe { err: e.into() })
                    .boxed(),

                None => axum_server::from_tcp(listener)
                    .handle(handle.axum_handle.clone())
                    .serve(make_service)
                    .map_err(|e| Error::UnableToServe { err: e.into() })
                    .boxed(),
            }
        };

        Ok((running, handle, check_fn))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::tracker;
    use crate::bootstrap::jobs::make_rust_tls;
    use crate::servers::http::service::HttpLauncher;
    use crate::servers::registar::Registar;
    use crate::servers::service::Service;

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_mode_public());
        let tracker = tracker(&cfg);
        let config = &cfg.http_trackers[0];

        let bind_to = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("Tracker API bind_address invalid.");

        let tls = make_rust_tls(config.ssl_enabled, &config.ssl_cert_path, &config.ssl_key_path)
            .await
            .map(|tls| tls.expect("tls config failed"));

        let form = &Registar::default();

        let stopped = Service::new(HttpLauncher::new(tracker, bind_to, tls));

        let started = stopped.start().expect("it should start the server");
        let () = started.reg_form(form.give_form()).await.expect("it should register");

        let stopped = started.stop().await.expect("it should stop the server");

        drop(stopped);
    }
}
