use std::net::SocketAddr;
use std::sync::Arc;

use derive_more::{Constructor, Display};
use futures::{FutureExt as _, TryFutureExt as _};
use tracing::{info, instrument, trace};

use super::handle::Handle;
use crate::servers::health_check_api::check::check_builder;
use crate::servers::health_check_api::v0::routes::router;
use crate::servers::registar::{HealthCheckFactory, Registry};
use crate::servers::service;

#[derive(Clone, Constructor, Debug, Display)]
#[display(fmt = "intended_address: {addr}, registry: {registry}")]
pub struct Launcher {
    pub addr: SocketAddr,
    pub registry: Arc<Registry>,
}

impl service::Launcher<Handle> for Launcher {
    #[instrument(err, fields(self = %self))]
    fn start(self) -> Result<(service::TaskFuture<'static, (), service::Error>, Handle, HealthCheckFactory), service::Error> {
        trace!("setup the health check handler");
        let handle = Handle::default();

        trace!("make service task");
        let task: service::TaskFuture<'_, (), service::Error> = {
            trace!(address = ?self.addr, "try to bind on socket");
            let listener = std::net::TcpListener::bind(self.addr).map_err(|e| service::Error::UnableToBindToSocket {
                addr: self.addr,
                err: e.into(),
            })?;

            trace!("try to get local address");
            let addr = listener
                .local_addr()
                .map_err(|e| service::Error::UnableToGetLocalAddress { err: e.into() })?;
            info!(address = ?addr, "health tracker bound to tcp socket: {addr}");

            trace!("setup router");

            let router = router(self.registry, addr);

            trace!("make router into service");
            let make_service = router.into_make_service_with_connect_info::<SocketAddr>();

            info!("start and return axum service");
            axum_server::from_tcp(listener)
                .handle(handle.axum_handle.clone())
                .serve(make_service)
                .map_err(|e| service::Error::UnableToServe { err: e.into() })
                .boxed()
        };

        let check_factory = HealthCheckFactory::new(check_builder);

        trace!("returning the axum task, handle, and check function closure");
        Ok((task, handle, check_factory))
    }
}
