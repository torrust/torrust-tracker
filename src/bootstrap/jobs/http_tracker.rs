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
use tracing::info;

use super::make_rust_tls;
use crate::core;
use crate::servers::http::server::{HttpServer, Launcher};
use crate::servers::http::Version;
use crate::servers::registar::ServiceRegistrationForm;

/// It starts a new HTTP server with the provided configuration and version.
///
/// Right now there is only one version but in the future we could support more than one HTTP tracker version at the same time.
/// This feature allows supporting breaking changes on `BitTorrent` BEPs.
///
/// # Panics
///
/// It would panic if the `config::HttpTracker` struct would contain inappropriate values.
///
pub async fn start_job(
    config: &HttpTracker,
    tracker: Arc<core::Tracker>,
    form: ServiceRegistrationForm,
    version: Version,
) -> Option<JoinHandle<()>> {
    if config.enabled {
        let socket = config
            .bind_address
            .parse::<std::net::SocketAddr>()
            .expect("it should have a valid http tracker bind address");

        let tls = make_rust_tls(config.ssl_enabled, &config.ssl_cert_path, &config.ssl_key_path)
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

async fn start_v1(
    socket: SocketAddr,
    tls: Option<RustlsConfig>,
    tracker: Arc<core::Tracker>,
    form: ServiceRegistrationForm,
) -> JoinHandle<()> {
    let server = HttpServer::new(Launcher::new(socket, tls))
        .start(tracker, form)
        .await
        .expect("it should be able to start to the http tracker");

    tokio::spawn(async move {
        assert!(
            !server.state.halt_task.is_closed(),
            "Halt channel for HTTP tracker should be open"
        );
        server
            .state
            .task
            .await
            .expect("it should be able to join to the http tracker task");
    })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use torrust_tracker_test_helpers::configuration::ephemeral_mode_public;

    use crate::bootstrap::app::initialize_with_configuration;
    use crate::bootstrap::jobs::http_tracker::start_job;
    use crate::servers::http::Version;
    use crate::servers::registar::Registar;

    #[tokio::test]
    async fn it_should_start_http_tracker() {
        let cfg = Arc::new(ephemeral_mode_public());
        let config = &cfg.http_trackers[0];
        let tracker = initialize_with_configuration(&cfg);
        let version = Version::V1;

        start_job(config, tracker, Registar::default().give_form(), version)
            .await
            .expect("it should be able to join to the http tracker start-job");
    }
}
