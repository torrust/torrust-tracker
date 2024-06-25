//! Module to handle the HTTP server instances.

use futures::FutureExt;

use crate::servers::service::{self};
use crate::servers::signals::Halted;
use crate::servers::tcp::graceful_axum_shutdown;

#[derive(Debug)]
pub struct Handle {
    pub axum_handle: axum_server::Handle,
    tx_shutdown: Option<tokio::sync::oneshot::Sender<Halted>>,
}

impl Handle {
    fn shutdown(&mut self) -> Result<(), service::Error> {
        let () = if let Some(tx) = self.tx_shutdown.take() {
            tx.send(Halted::Normal)
                .map_err(|err| service::Error::UnableToSendHaltingMessage { err })?;
        } else {
            panic!("it has already taken the channel?");
        };
        Ok(())
    }
}

impl Default for Handle {
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

impl service::Handle for Handle {
    fn stop(mut self) -> Result<(), service::Error> {
        self.shutdown()
    }

    fn listening(&self) -> service::AddrFuture<'_> {
        self.axum_handle.listening().boxed()
    }

    fn into_graceful_shutdown_future<'a>(self) -> futures::prelude::future::BoxFuture<'a, Result<(), service::Error>> {
        todo!()
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.shutdown().expect("it should shutdown when dropped");
    }
}
