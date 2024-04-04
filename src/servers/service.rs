use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::BoxFuture;
use thiserror::Error;
use tokio::task::{JoinError, JoinHandle};
use tracing::{debug, instrument, trace};

use super::registar::{FnSpawnServiceHeathCheck, ServiceRegistration, ServiceRegistrationForm};
use super::signals::Halted;
use super::udp::server::UdpError;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Failed to bind to socket: {addr}, with error: {err}")]
    UnableToBindToSocket { addr: SocketAddr, err: Arc<std::io::Error> },
    #[error("Failed to get Local Address from Socket: {err:?}")]
    UnableToGetLocalAddress { err: Arc<std::io::Error> },
    #[error("Failed to get listening address.")]
    UnableToGetListeningAddress {},
    #[error("Failed to serve service: {err:?}")]
    UnableToServe { err: Arc<std::io::Error> },
    #[error("Failed to receive started message: {err:?}")]
    UnableToReceiveStartedMessage {
        err: Arc<tokio::sync::oneshot::error::RecvError>,
    },
    #[error("Failed to send ServiceRegistration: {err:?}")]
    UnableToSendRegistrationMessage { err: ServiceRegistration },
    #[error("Failed to send Halted: {err:?}")]
    UnableToSendHaltingMessage { err: Halted },
    #[error("Failed to join task when stopping: {err:?}")]
    UnableToJoinStoppingService { err: Arc<tokio::task::JoinError> },
    #[error("Failed to start udp service: {err:?}")]
    UnableToStartUdpService { err: UdpError },

    #[error("Failed to join the tokio task: {err:?}")]
    UnableJoinTokioTask { err: Arc<JoinError> },
}

pub type AddrFuture<'a> = BoxFuture<'a, Option<SocketAddr>>;

pub type TaskFuture<'a, T, E> = BoxFuture<'a, Result<T, E>>;
pub type TaskHandle<T, E> = JoinHandle<Result<T, E>>;

pub trait Handle: Debug + Default + Send + 'static {
    /// Stops the Service
    ///
    /// # Errors
    ///
    /// This function will return an error if the service fails to stop cleanly.
    ///
    fn stop(self) -> Result<(), Error>;

    fn listening(&self) -> AddrFuture<'_>;
}

pub trait Launcher<H>: Clone + Debug + Display + Send + 'static
where
    H: Handle,
{
    /// Starts the service
    ///
    /// # Errors
    ///
    /// This function will return an error if the launching fails,
    /// or when the future returns with an error.
    ///
    fn start(self) -> Result<(TaskFuture<'static, (), Error>, H, FnSpawnServiceHeathCheck), Error>;
}

/// A service instance controller.
///
/// It's responsible for:
///
/// - Starting and stopping a service.
/// - Keeping the state of the server: `running` or `stopped`.
///
#[allow(clippy::module_name_repetitions)]
pub struct Service<S, L, H>
where
    L: Launcher<H> + Send + 'static,
    H: Handle,
    S: Debug,
{
    phantom: PhantomData<H>,
    /// The state of the server: `running` or `stopped`.
    pub state: S,
    launcher: L,
}

impl<S: Debug, L: Debug, H: Debug> Debug for Service<S, L, H>
where
    L: Launcher<H> + Send + 'static,
    H: Handle,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Service").field("state", &self.state).finish_non_exhaustive()
    }
}

/// A stopped service state.
#[derive(Debug)]
pub struct Stopped {}

/// A running service state.
pub struct Started<H>
where
    H: Handle,
{
    task: TaskHandle<(), Error>,
    pub handle: H,
    check_fn: FnSpawnServiceHeathCheck,
}

impl<H: Handle> Started<H> {
    #[instrument(skip(task))]
    pub fn new(task: TaskFuture<'static, (), Error>, handle: H, check_fn: FnSpawnServiceHeathCheck) -> Self {
        debug!("spawning the task in tokio");
        let task: TaskHandle<(), Error> = tokio::task::spawn(task);

        Self { task, handle, check_fn }
    }
}

impl<H: Debug> Debug for Started<H>
where
    H: Handle,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Started")
            .field("handle", &self.handle)
            .finish_non_exhaustive()
    }
}

impl<H: Handle, L: Launcher<H> + Send + 'static> Service<Stopped, L, H> {
    #[must_use]
    #[instrument(ret, fields(launcher = %launcher))]
    pub fn new(launcher: L) -> Self {
        Self {
            phantom: PhantomData,
            state: Stopped {},
            launcher,
        }
    }

    /// It starts the server and returns a  controller in `running` state.
    ///
    /// # Errors
    ///
    /// It would return an error if the underling launcher returns an error.
    ///
    #[instrument(err, ret)]
    pub fn start(self) -> Result<Service<Started<H>, L, H>, Error> {
        trace!("starting the task");
        let (task, handle, check_fn) = self.launcher.clone().start()?;

        trace!("building the service in the started state");
        Ok(Service {
            phantom: PhantomData,
            state: Started::new(task, handle, check_fn),
            launcher: self.launcher,
        })
    }
}

impl<'a, H: Handle, L: Launcher<H> + Send + 'a> Service<Started<H>, L, H> {
    /// Returns the listening Address of this Service.
    ///
    /// # Errors
    ///
    /// This function will return an error if unable to get address.
    ///
    #[instrument(err, ret)]
    pub async fn listening(&self) -> Result<SocketAddr, Error> {
        trace!("awaiting the service to inform it's ready and listening");
        self.state
            .handle
            .listening()
            .await
            .ok_or_else(|| Error::UnableToGetListeningAddress {})
    }

    /// It registers the service on a form.
    ///
    /// # Errors
    ///
    /// It would return an error unable to get the current address,
    /// or unable to complete the registration.
    ///
    #[instrument(err, ret, skip(form))]
    pub async fn reg_form(&self, form: ServiceRegistrationForm) -> Result<(), Error> {
        trace!("awaiting for the service to be ready and return it's local address");
        let addr = self.listening().await?;

        trace!(
            "sends the service registration on the supplied form,
        with the local address and self-check closure"
        );
        form.send(ServiceRegistration::new(addr, self.state.check_fn))
            .map_err(|err| Error::UnableToSendRegistrationMessage { err })
    }

    /// It returns the active task with it's handler.
    ///
    /// When the task completes, it will return a controller in the stopped state.
    ///
    #[instrument()]
    pub fn run(self) -> (TaskHandle<Service<Stopped, L, H>, Error>, H) {
        trace!(
            "gets the service task future and changes it's future
        return type to include the service in the stopped state"
        );

        let task = tokio::task::spawn(async move {
            let () = self
                .state
                .task
                .await
                .map_err(|e| Error::UnableJoinTokioTask { err: e.into() })??;

            Ok(Service {
                phantom: PhantomData,
                state: Stopped {},
                launcher: self.launcher,
            })
        });

        (task, self.state.handle)
    }

    /// It stops the server and returns a controller in `stopped`
    /// state.
    ///
    /// # Errors
    ///
    /// It would return an error if unable to stop, of the task finished with an error.
    #[instrument(err, ret)]
    pub async fn stop(self) -> Result<Service<Stopped, L, H>, Error> {
        let () = self.state.handle.stop()?;
        let () = self
            .state
            .task
            .await
            .map_err(|e| Error::UnableJoinTokioTask { err: e.into() })??;

        Ok(Service {
            phantom: PhantomData,
            state: Stopped {},
            launcher: self.launcher,
        })
    }
}
