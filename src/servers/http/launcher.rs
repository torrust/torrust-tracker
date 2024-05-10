use std::net::SocketAddr;
use std::sync::Arc;

use derive_more::{Constructor, Display};
use futures::{FutureExt as _, TryFutureExt as _};

use super::check::check_builder;
use super::handle::Handle;
use super::v1::routes::router;
use crate::core::Tracker;
use crate::servers::{registar, service};

#[derive(Constructor, Clone, Debug, Display)]
#[display(fmt = "intended_address: {addr}, with tracker, and  {}", "self.have_tls()")]
pub struct Launcher {
    pub tracker: Arc<Tracker>,
    pub addr: SocketAddr,
    pub tls: Option<axum_server::tls_rustls::RustlsConfig>,
}

impl Launcher {
    fn have_tls(&self) -> String {
        match self.tls {
            Some(_) => "some",
            None => "none",
        }
        .to_string()
    }
}

impl service::Launcher<Handle> for Launcher {
    fn start(
        self,
    ) -> Result<
        (
            service::TaskFuture<'static, (), service::Error>,
            Handle,
            registar::HealthCheckFactory,
        ),
        service::Error,
    > {
        let handle = Handle::default();

        let running: service::TaskFuture<'_, (), service::Error> = {
            let listener = std::net::TcpListener::bind(self.addr).map_err(|e| service::Error::UnableToBindToSocket {
                addr: self.addr,
                err: e.into(),
            })?;

            let addr = listener
                .local_addr()
                .map_err(|e| service::Error::UnableToGetLocalAddress { err: e.into() })?;

            let make_service = router(self.tracker, &addr).into_make_service_with_connect_info::<std::net::SocketAddr>();

            match self.tls.clone() {
                Some(tls) => axum_server::from_tcp_rustls(listener, tls)
                    .handle(handle.axum_handle.clone())
                    .serve(make_service)
                    .map_err(|e| service::Error::UnableToServe { err: e.into() })
                    .boxed(),

                None => axum_server::from_tcp(listener)
                    .handle(handle.axum_handle.clone())
                    .serve(make_service)
                    .map_err(|e| service::Error::UnableToServe { err: e.into() })
                    .boxed(),
            }
        };

        let check_factory = registar::HealthCheckFactory::new(check_builder);

        Ok((running, handle, check_factory))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::tracker;
    use crate::bootstrap::jobs::make_rust_tls;
    use crate::servers::http::launcher::Launcher;
    use crate::servers::{registar, service};

    #[tokio::test]
    async fn it_should_be_able_to_start_and_stop() {
        let cfg = Arc::new(ephemeral_mode_public());
        let tracker = tracker(&cfg);
        let config = &cfg.http_trackers[0];

        let bind_to = config.bind_address;

        let tls = make_rust_tls(
            config.ssl_enabled,
            &config.tsl_config.ssl_cert_path,
            &config.tsl_config.ssl_key_path,
        )
        .await
        .map(|tls| tls.expect("tls config failed"));

        let form = &registar::Registar::default();

        let stopped = service::Service::new(Launcher::new(tracker, bind_to, tls));

        let started = stopped.start().expect("it should start the server");
        let () = started.reg_form(form.form()).await.expect("it should register");

        let stopped = started.stop().await.expect("it should stop the server");

        drop(stopped);
    }
}
