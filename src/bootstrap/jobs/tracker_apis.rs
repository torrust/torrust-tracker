//! Tracker API job starter.
//!
//! The [`tracker_apis::start_job`](crate::bootstrap::jobs::tracker_apis::start_job)
//! function starts a the HTTP tracker REST API.
//!
//! > **NOTICE**: that even thought there is only one job the API has different
//! > versions. API consumers can choose which version to use. The API version is
//! > part of the URL, for example: `http://localhost:1212/api/v1/stats`.
//!
//! The [`tracker_apis::start_job`](crate::bootstrap::jobs::tracker_apis::start_job)  
//! function spawns a new asynchronous task, that tasks is the "**launcher**".
//! The "**launcher**" starts the actual server and sends a message back
//! to the main application. The main application waits until receives
//! the message [`ApiServerJobStarted`]
//! from the "**launcher**".
//!
//! The "**launcher**" is an intermediary thread that decouples the API server
//! from the process that handles it. The API could be used independently
//! in the future. In that case it would not need to notify a parent process.
//!
//! Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration)
//! for the API configuration options.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::{AccessTokens, HttpApi};
use tracing::instrument;

use super::make_rust_tls;
use crate::core;
use crate::servers::apis::server::ApiLauncher;
use crate::servers::apis::Version;
use crate::servers::registar::Form;
use crate::servers::service::Service;

/// This is the message that the "launcher" spawned task sends to the main
/// application process to notify the API server was successfully started.
///
/// > **NOTICE**: it does not mean the API server is ready to receive requests.
/// > It only means the new server started. It might take some time to the server
/// > to be ready to accept request.
#[derive(Debug)]
pub struct ApiServerJobStarted();

/// This function starts a new API server with the provided configuration.
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
#[instrument(ret)]
pub async fn start_job(config: &HttpApi, tracker: Arc<core::Tracker>, form: Form, version: Version) -> Option<JoinHandle<()>> {
    let bind_to = config.bind_address;

    let tls = match &config.tsl_config {
        Some(tls_config) => Some(
            make_rust_tls(tls_config)
                .await
                .expect("it should have a valid tracker api tls configuration"),
        ),
        None => None,
    };

    let access_tokens = Arc::new(config.access_tokens.clone());

    match version {
        Version::V1 => Some(start_v1(bind_to, tls, tracker.clone(), form, access_tokens).await),
    }
}

#[allow(clippy::async_yields_async)]
#[instrument(ret)]
async fn start_v1(
    socket: SocketAddr,
    tls: Option<RustlsConfig>,
    tracker: Arc<core::Tracker>,
    form: Form,
    access_tokens: Arc<AccessTokens>,
) -> JoinHandle<()> {
    let service = Service::new(ApiLauncher::new(tracker, access_tokens, socket, tls));

    let started = service.start().expect("it should start");

    let () = started.reg_form(form).await.expect("it should register");

    let (task, _) = started.run();

    tokio::spawn(async move {
        let server = task.await.expect("it should shutdown");
        drop(server);
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::tracker;
    use crate::bootstrap::jobs::tracker_apis::start_job;
    use crate::servers::apis::Version;
    use crate::servers::registar::Registar;

    #[tokio::test]
    async fn it_should_start_http_tracker() {
        let cfg = Arc::new(ephemeral_mode_public());
        let config = &cfg.http_api.as_ref().unwrap();
        let tracker = tracker(&cfg);
        let version = Version::V1;

        start_job(config, tracker, Registar::default().form(), version)
            .await
            .expect("it should be able to join to the tracker api start-job");
    }
}
