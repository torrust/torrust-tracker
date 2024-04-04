use std::net::SocketAddr;
use std::sync::Arc;

use torrust_tracker::servers::health_check_api::server::{HealthCheckHandle, HealthCheckLauncher};
use torrust_tracker::servers::registar::{Registar, ServiceRegistrationForm, ServiceRegistry};
use torrust_tracker::servers::service;
use torrust_tracker_configuration::HealthCheckApi;

type Started = service::Service<service::Started<HealthCheckHandle>, HealthCheckLauncher, HealthCheckHandle>;

pub struct Environment {
    service: Started,
    pub addr: SocketAddr,
}

impl Environment {
    /// Starts the health check environment.
    ///
    /// # Panics
    ///
    /// Panics if something goes wrong...
    pub async fn new(config: &Arc<HealthCheckApi>, registar: &Registar) -> Self {
        let registry = registar.clone().get_registry();

        let addr = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("it should have a valid http tracker bind address");

        let form = registar.give_form();

        let (started, listening) = Self::start_v0(addr, &registry, form).await;

        Self {
            service: started,
            addr: listening,
        }
    }

    /// Starts the first (un-versioned) tracker service health check.
    ///
    /// # Panics
    ///
    /// Panics if something goes wrong...
    async fn start_v0(addr: SocketAddr, registry: &ServiceRegistry, form: ServiceRegistrationForm) -> (Started, SocketAddr) {
        let service = service::Service::new(HealthCheckLauncher::new(addr, registry.clone()));

        let started: Started = service.start().expect("it should start");

        let () = started.reg_form(form).await.expect("it should register");

        let listening = started.listening().await.expect("it should start listening");

        (started, listening)
    }

    /// Starts the health check environment.
    ///
    /// # Panics
    ///
    /// Panics if something goes wrong...
    pub async fn stop(self) -> Result<(), service::Error> {
        let stopped = self.service.stop().await?;

        drop(stopped);

        Ok(())
    }
}
