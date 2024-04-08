//! Registar. Registers Services for Health Check.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

use derive_more::{AsRef, Constructor, DebugCustom, Display, From};
use futures::future::BoxFuture;
use thiserror::Error;
use torrust_tracker_located_error::DynError;
use tracing::debug;

/// A [`HeathCheckFuture`] preforms a health check when spawned.
pub type HeathCheckFuture<'a> = BoxFuture<'a, HeathCheckResult>;

pub type HealthCheckBuilder = fn(SocketAddr) -> HeathCheckFuture<'static>;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Failed Check: for {addr}, {msg}")]
    UnableToPreformSuccessfulHealthCheck { addr: SocketAddr, msg: String },

    #[error("Failed To Connect to {addr}, {msg}, err: {err}")]
    UnableToConnectToRemote { addr: SocketAddr, msg: String, err: DynError },

    #[error("Failed To Preform Check {addr}, {msg}, err: {err}")]
    UnableToPreformCheck { addr: SocketAddr, msg: String, err: DynError },

    #[error("Failed To Get A Successful Response {addr}, {msg}, err: {err}")]
    UnableToObtainGoodResponse { addr: SocketAddr, msg: String, err: DynError },

    #[error("Failed To Get Any Response {addr}, {msg}, err: {err}")]
    UnableToGetAnyResponse { addr: SocketAddr, msg: String, err: DynError },
}

#[derive(Error, Debug, Display, Clone)]
pub enum Success {
    #[display(fmt = "Success: {addr}, {msg}")]
    AllGood { addr: SocketAddr, msg: String },
}

#[derive(Clone, Debug, AsRef, Display, From)]
#[display(fmt = "Result: {}", "self.get_result_display()")]
pub struct HeathCheckResult(Result<Success, Error>);

impl HeathCheckResult {
    fn get_result_display(&self) -> String {
        let result = self.as_ref().clone();

        result.map_or_else(|e| e.to_string(), |f| format!("Ok: {f}"))
    }
}

/// The [`ServiceHealthCheck`] provides a builder that generates check futures.
///
#[derive(AsRef, Constructor, Clone, DebugCustom)]
#[debug(fmt = "...")]
pub struct HealthCheckFactory {
    pub builder: HealthCheckBuilder,
}

impl HealthCheckFactory {
    fn make<'a>(&self, addr: SocketAddr) -> HeathCheckFuture<'a> {
        (self.builder)(addr)
    }
}

/// The [`Registration`] [`Form`] is provided to the [`Registar`] for registration.
///
pub type Form = tokio::sync::oneshot::Sender<Registration>;

/// A [`Registration`] is provided to the [`Registar`] for registration.
///
#[derive(Debug, Constructor)]
pub struct Registration {
    addr: SocketAddr,
    check_factory: HealthCheckFactory,
}

impl Registration {
    /// Creates the Check Task Future
    ///
    /// Note: This future  is not spawned yet.
    #[must_use]
    pub fn check_task<'a>(&self) -> HeathCheckFuture<'a> {
        self.check_factory.make(self.addr)
    }
}

type Db = HashMap<SocketAddr, Registration>;

/// The [`Registry`] contains each unique [`ServiceRegistration`] by it's [`SocketAddr`].
///
#[derive(Default, Display)]
#[display(fmt = "targets: {:?}", "self.targets()")]
pub struct Registry {
    db: Mutex<Db>,
}

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.targets()).finish()
    }
}

impl Registry {
    /// Returns the locked db of this [`Registry`].
    ///
    /// # Errors
    ///
    /// This function will error if the lock is not functioning.
    pub fn lock(&self) -> LockResult<MutexGuard<'_, Db>> {
        self.db.lock()
    }

    fn targets(&self) -> Vec<SocketAddr> {
        self.lock().expect("it should get a lock").keys().copied().collect()
    }
}

/// The [`Registar`] manages the [`Registry`].
///
#[derive(Clone, AsRef, Debug, Display, Default)]
pub struct Registar {
    registry: Arc<Registry>,
}

impl Registar {
    #[must_use]
    pub fn new(register: Registry) -> Self {
        Self {
            registry: register.into(),
        }
    }

    /// Gets the registration form and preforms the asynchronous registration.
    ///
    /// # Panics
    ///
    /// Inside the dropped future it can panic if the receiving channel is broken,
    /// or if unable to get a lock for the registry.
    #[must_use]
    pub fn form(&self) -> Form {
        let (tx, rx) = tokio::sync::oneshot::channel::<Registration>();

        let registry = self.registry.clone();

        drop(tokio::spawn(async move {
            let form = rx.await.expect("it should make a form");

            let mut db = registry.db.lock().expect("it should get a lock");

            db.insert(form.addr, form)
        }));

        tx
    }
}
