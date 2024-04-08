use std::net::SocketAddr;
use std::sync::Arc;

use derive_more::{Constructor, Display};
use futures::{FutureExt as _, TryFutureExt as _};
use tracing::instrument;

use super::check::build;
use super::handle::Handle;
use super::server::Udp;
use crate::core::Tracker;
use crate::servers::{registar, service};

#[derive(Constructor, Clone, Debug, Display)]
#[display(fmt = "intended_address: {addr}, with tracker")]
pub struct Launcher {
    pub tracker: Arc<Tracker>,
    pub addr: SocketAddr,
}

impl service::Launcher<Handle> for Launcher {
    #[instrument(err)]
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
        let std_socket = std::net::UdpSocket::bind(self.addr).map_err(|e| service::Error::UnableToBindToSocket {
            addr: self.addr,
            err: e.into(),
        })?;

        let socket = tokio::net::UdpSocket::from_std(std_socket).map_err(|e| service::Error::UnableToBindToSocket {
            addr: self.addr,
            err: e.into(),
        })?;

        let (task, handle) =
            Udp::make_task(self.tracker, socket.into()).map_err(|err| service::Error::UnableToStartUdpService { err })?;

        let task = task.map_err(|err| service::Error::UnableToStartUdpService { err }).boxed();

        let check_factory = registar::HealthCheckFactory::new(build);

        Ok((task, handle, check_factory))
    }
}
