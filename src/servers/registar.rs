//! Registar. Registers Services for Health Check.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use derive_more::Constructor;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::debug;

/// A [`ServiceHeathCheckResult`] is returned by a completed health check.
pub type ServiceHeathCheckResult = Result<String, String>;

/// The [`ServiceHealthCheckJob`] has a health check job with it's metadata
///
/// The `job` awaits a [`ServiceHeathCheckResult`].
#[derive(Debug, Constructor)]
pub struct ServiceHealthCheckJob {
    pub binding: SocketAddr,
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
    binding: SocketAddr,
    check_fn: FnSpawnServiceHeathCheck,
}

impl ServiceRegistration {
    #[must_use]
    pub fn spawn_check(&self) -> ServiceHealthCheckJob {
        (self.check_fn)(&self.binding)
    }
}

/// A [`ServiceRegistrationForm`] will return a completed [`ServiceRegistration`] to the [`Registar`].
pub type ServiceRegistrationForm = tokio::sync::oneshot::Sender<ServiceRegistration>;

/// The [`ServiceRegistry`] contains each unique [`ServiceRegistration`] by it's [`SocketAddr`].
pub type ServiceRegistry = Arc<Mutex<HashMap<SocketAddr, ServiceRegistration>>>;

/// The [`Registar`] manages the [`ServiceRegistry`].
#[derive(Clone, Debug)]
pub struct Registar {
    registry: ServiceRegistry,
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

        let mut mutex = self.registry.lock().await;

        mutex.insert(service_registration.binding, service_registration);
    }

    /// Returns the [`ServiceRegistry`] of services
    #[must_use]
    pub fn entries(&self) -> ServiceRegistry {
        self.registry.clone()
    }
}
