use futures::FutureExt as _;
use tracing::{error, info, instrument, trace, warn};

use crate::servers::service;
use crate::servers::signals::Halted;
use crate::servers::tcp::graceful_axum_shutdown;

pub struct Handle {
    pub axum_handle: axum_server::Handle,
    tx_shutdown: Option<tokio::sync::oneshot::Sender<Halted>>,
}

impl std::fmt::Debug for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("axum_handle_conn:", &self.axum_handle.connection_count())
            .finish_non_exhaustive()
    }
}

impl Handle {
    #[instrument]
    fn shutdown(&mut self) -> Result<(), service::Error> {
        trace!("the internal shut down was called");
        if let Some(tx) = self.tx_shutdown.take() {
            trace!("sending a normal halt on the shutdown channel");
            tx.send(Halted::Normal)
                .map_err(|err| service::Error::UnableToSendHaltingMessage { err })?;
        } else {
            error!("shutdown was called, but the channel was missing!");
            panic!();
        };
        Ok(())
    }
}

impl Default for Handle {
    #[instrument(ret)]
    fn default() -> Self {
        trace!("setup the shutdown channel");
        let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel::<Halted>();

        trace!("setup the axum handle");
        let axum_handle = axum_server::Handle::default();

        trace!("setup the graceful axum meta-handler");
        let () = graceful_axum_shutdown(axum_handle.clone(), rx_shutdown, "Health Check Server".to_string());

        trace!("returning the new default handler");
        Self {
            axum_handle: axum_server::Handle::new(),
            tx_shutdown: Some(tx_shutdown),
        }
    }
}

impl service::Handle for Handle {
    #[instrument(ret)]
    fn stop(mut self) -> Result<(), service::Error> {
        info!("shutdown function was called");
        self.shutdown()
    }

    #[instrument]
    fn listening(&self) -> service::AddrFuture<'_> {
        info!("return the listening future form the axum handler");
        self.axum_handle.listening().boxed()
    }

    fn into_graceful_shutdown_future<'a>(self) -> futures::prelude::future::BoxFuture<'a, Result<(), service::Error>> {
        todo!()
    }
}

impl Drop for Handle {
    #[instrument]
    fn drop(&mut self) {
        match self.tx_shutdown {
            Some(_) => {
                warn!("shutting down via drop");
                self.shutdown().expect("it should shutdown when dropped");
            }
            None => {
                trace!("shutdown has already been called, dropping");
            }
        }
    }
}
