//! Health Check API job starter.
//!
//! The [`health_check_api::start_job`](crate::bootstrap::jobs::health_check_api::start_job)
//! function starts the Health Check REST API.
//!
//! The [`health_check_api::start_job`](crate::bootstrap::jobs::health_check_api::start_job)  
//! function spawns a new asynchronous task, that tasks is the "**launcher**".
//! The "**launcher**" starts the actual server and sends a message back
//! to the main application.
//!
//! The "**launcher**" is an intermediary thread that decouples the Health Check
//! API server from the process that handles it.
//!
//! Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! for the API configuration options.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use torrust_tracker_configuration::HealthCheckApi;
use tracing::instrument;

use super::Error;
use crate::servers::health_check_api::launcher::Launcher;
use crate::servers::health_check_api::Version;
use crate::servers::registar::{Form, Registar, Registry};
use crate::servers::service::Service;

/// This function starts a new Health Check API server with the provided
/// configuration.
///
/// The functions starts a new concurrent task that will run the API server.
/// This task will send a message to the main application process to notify
/// that the API server was successfully started.
///
/// # Panics
///
/// It would panic if unable to send the  `ApiServerJobStarted` notice.
///
#[allow(clippy::async_yields_async)]
#[instrument(ret, fields(registar = %registar))]
pub async fn start_job(config: &HealthCheckApi, registar: &Registar, version: Version) -> JoinHandle<()> {
    let addr = config.bind_address.parse().expect("it should parse the binding address");

    let form = registar.form();
    let registry = registar.as_ref().clone();

    match version {
        Version::V0 => start_v0(addr, registry, form).await.expect("it should start the service"),
    }
}

/// Starts the first (un-versioned) tracker service health check.
/// # Panics
///
/// Panics if something goes wrong...
///
#[allow(clippy::async_yields_async)]
#[instrument(err, ret, skip(form), fields(registry = %registry))]
async fn start_v0(addr: SocketAddr, registry: Arc<Registry>, form: Form) -> Result<JoinHandle<()>, Error> {
    let service = Service::new(Launcher::new(addr, registry));

    let started = service.start().map_err(Error::from)?;

    let () = tokio::time::timeout(Duration::from_secs(5), started.reg_form(form))
        .await
        .map_err(Error::from)?
        .map_err(Error::from)?;

    let (task, _) = started.run();

    Ok(tokio::spawn(async move {
        let server = task.await.expect("it should shutdown");
        drop(server);
    }))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    //use tracing_test::traced_test;
    use crate::bootstrap::jobs::health_check_api::{start_job, start_v0};
    use crate::servers::health_check_api::Version;
    use crate::servers::registar::Registar;

    #[tokio::test]
    //#[traced_test]
    async fn it_should_start_and_stop_the_job_for_the_health_check_service() {
        let cfg = Arc::new(ephemeral_mode_public());
        let config = &cfg.health_check_api;
        let version = Version::V0;

        let registar = Registar::default();

        let job = start_job(config, &registar, version).await;

        job.abort();

        drop(job);
    }

    #[tokio::test]
    //#[traced_test]
    async fn it_should_start_and_stop_the_v0_of_the_health_check_service() {
        let addr = ephemeral_mode_public()
            .health_check_api
            .bind_address
            .parse()
            .expect("it should parse the binding address");

        let registar = Registar::default();
        let form = registar.form();
        let registry = registar.as_ref().clone();

        let job = start_v0(addr, registry, form).await.expect("it should start");

        job.abort();

        drop(job);
    }
}
