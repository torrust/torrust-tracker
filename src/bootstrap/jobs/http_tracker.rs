//! HTTP tracker job starter.
//!
//! The function [`http_tracker::start_job`](crate::bootstrap::jobs::http_tracker::start_job) starts a new HTTP tracker server.
//!
//! > **NOTICE**: the application can launch more than one HTTP tracker on different ports.
//! Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
//!
//! The [`http_tracker::start_job`](crate::bootstrap::jobs::http_tracker::start_job) function spawns a new asynchronous task,
//! that tasks is the "**launcher**". The "**launcher**" starts the actual server and sends a message back to the main application.
//!
//! The "**launcher**" is an intermediary thread that decouples the HTTP servers from the process that handles it. The HTTP could be used independently in the future.
//! In that case it would not need to notify a parent process.
use std::net::SocketAddr;
use std::sync::Arc;

use axum_server::tls_rustls::RustlsConfig;
use tokio::task::JoinHandle;
use torrust_tracker_configuration::HttpTracker;
use tracing::{info, instrument};

use super::make_rust_tls_from_path_buf;
use crate::core;
use crate::servers::http::launcher::Launcher;
use crate::servers::http::Version;
use crate::servers::registar::Form;
use crate::servers::service::Service;

/// It starts a new HTTP server with the provided configuration and version.
///
/// Right now there is only one version but in the future we could support more than one HTTP tracker version at the same time.
/// This feature allows supporting breaking changes on `BitTorrent` BEPs.
///
/// # Panics
///
/// It would panic if the `config::HttpTracker` struct would contain inappropriate values.
///
#[allow(clippy::async_yields_async)]
#[instrument(ret)]
pub async fn start_job(
    config: &HttpTracker,
    tracker: Arc<core::Tracker>,
    form: Form,
    version: Version,
) -> Option<JoinHandle<()>> {
    if config.enabled {
        let socket = config.bind_address;

        let tls = make_rust_tls_from_path_buf(
            config.ssl_enabled,
            &config.tsl_config.ssl_cert_path,
            &config.tsl_config.ssl_key_path,
        )
        .await
        .map(|tls| tls.expect("it should have a valid http tracker tls configuration"));

        match version {
            Version::V1 => Some(start_v1(socket, tls, tracker.clone(), form).await),
        }
    } else {
        info!("Note: Not loading Http Tracker Service, Not Enabled in Configuration.");
        None
    }
}

#[allow(clippy::async_yields_async)]
#[instrument(ret)]
async fn start_v1(socket: SocketAddr, tls: Option<RustlsConfig>, tracker: Arc<core::Tracker>, form: Form) -> JoinHandle<()> {
    let service = Service::new(Launcher::new(tracker, socket, tls));

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
    use crate::bootstrap::jobs::http_tracker::start_job;
    use crate::servers::http::Version;
    use crate::servers::registar::Registar;

    #[tokio::test]
    async fn it_should_start_http_tracker() {
        let cfg = Arc::new(ephemeral_mode_public());
        let config = &cfg.http_trackers[0];
        let tracker = tracker(&cfg);
        let version = Version::V1;

        let job = start_job(config, tracker, Registar::default().form(), version)
            .await
            .expect("it should be able to join to the http tracker start-job");

        job.abort();

        drop(job);
    }
}
