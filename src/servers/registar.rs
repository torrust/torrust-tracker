//! Registar. Registers Services for Health Check.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use derive_more::{AsRef, Constructor, Display};
use tokio::task::JoinHandle;
use tracing::debug;

/// A [`ServiceHeathCheckResult`] is returned by a completed health check.
pub type ServiceHeathCheckResult = Result<String, String>;

/// The [`ServiceHealthCheckJob`] has a health check job with it's metadata
///
/// The `job` awaits a [`ServiceHeathCheckResult`].
#[derive(Debug, Constructor)]
pub struct ServiceHealthCheckJob {
    pub addr: SocketAddr,
    pub info: String,
    pub job: JoinHandle<ServiceHeathCheckResult>,
}

/// The function specification [`FnSpawnServiceHeathCheck`].
///
/// A function fulfilling this specification will spawn a new [`ServiceHealthCheckJob`].
pub type FnSpawnServiceHeathCheck = fn(&SocketAddr) -> ServiceHealthCheckJob;

/// A [`ServiceRegistration`] is provided to the [`Registar`] for registration.
///
/// Each registration includes a function that fulfils the [`FnSpawnServiceHeathCheck`] specification.
#[derive(Clone, Debug, Constructor)]
pub struct ServiceRegistration {
    addr: SocketAddr,
    check_fn: FnSpawnServiceHeathCheck,
}

impl ServiceRegistration {
    #[must_use]
    pub fn spawn_check(&self) -> ServiceHealthCheckJob {
        (self.check_fn)(&self.addr)
    }
}

/// A [`ServiceRegistrationForm`] will return a completed [`ServiceRegistration`] to the [`Registar`].
pub type ServiceRegistrationForm = tokio::sync::oneshot::Sender<ServiceRegistration>;

/// The [`ServiceRegistry`] contains each unique [`ServiceRegistration`] by it's [`SocketAddr`].
#[derive(AsRef, Clone, Debug, Default, Display)]
#[display(fmt = "targets: {:?}", "self.targets()")]
pub struct ServiceRegistry {
    registry: Arc<Mutex<HashMap<SocketAddr, ServiceRegistration>>>,
}

impl ServiceRegistry {
    fn targets(&self) -> Vec<SocketAddr> {
        self.registry.lock().expect("it should get a lock").keys().copied().collect()
    }
}

/// The [`Registar`] manages the [`ServiceRegistry`].
#[derive(Clone, Debug)]
pub struct Registar {
    registry: ServiceRegistry,
}

impl Registar {
    /// Please try to only use for testing...
    #[must_use]
    pub fn get_registry(&self) -> ServiceRegistry {
        self.registry.clone()
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Registar {
    fn default() -> Self {
        Self {
            registry: ServiceRegistry::default(),
        }
    }
}

impl Registar {
    #[must_use]
    pub fn new(register: ServiceRegistry) -> Self {
        Self { registry: register }
    }

    /// Registers a Service
    #[must_use]
    pub fn give_form(&self) -> ServiceRegistrationForm {
        let (tx, rx) = tokio::sync::oneshot::channel::<ServiceRegistration>();
        let register = self.clone();
        tokio::spawn(async move {
            register.insert(rx).await;
        });
        tx
    }

    /// Inserts a listing into the registry.
    async fn insert(&self, rx: tokio::sync::oneshot::Receiver<ServiceRegistration>) {
        debug!("Waiting for the started service to send registration data ...");

        let service_registration = rx
            .await
            .expect("it should receive the service registration from the started service");

        let mut mutex = self.registry.as_ref().lock().expect("it should get a lock");

        mutex.insert(service_registration.addr, service_registration);
    }

    /// Returns the [`ServiceRegistry`] of services
    #[must_use]
    pub fn entries(&self) -> ServiceRegistry {
        self.registry.clone()
    }
}
