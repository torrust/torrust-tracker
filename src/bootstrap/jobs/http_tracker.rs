//! HTTP tracker job starter.
//!
//! The function [`http_tracker::start_job`](crate::bootstrap::jobs::http_tracker::start_job) starts a new HTTP tracker server.
//!
//! > **NOTICE**: the application can launch more than one HTTP tracker on different ports.
//! > Refer to the [configuration documentation](https://docs.rs/torrust-tracker-configuration) for the configuration options.
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
use torrust_server_lib::registar::ServiceRegistrationForm;
use torrust_tracker_axum_http_server::Version;
use torrust_tracker_axum_http_server::server::{HttpServer, Launcher};
use torrust_tracker_axum_server::tls::make_rust_tls;
use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use tracing::instrument;

/// It starts a new HTTP server with the provided configuration and version.
///
/// Right now there is only one version but in the future we could support more than one HTTP tracker version at the same time.
/// This feature allows supporting breaking changes on `BitTorrent` BEPs.
///
/// # Panics
///
/// It would panic if the `config::HttpTracker` struct would contain inappropriate values.
#[instrument(
    skip(http_tracker_container, form, metadata),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
pub async fn start_job(
    http_tracker_container: Arc<HttpTrackerCoreContainer>,
    form: ServiceRegistrationForm<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
    version: Version,
) -> Option<JoinHandle<()>> {
    let socket = http_tracker_container.http_tracker_config.bind_address;

    tracing::info!(
        bind_address = %socket,
        tracker_usage_statistics = http_tracker_container.http_tracker_config.tracker_usage_statistics,
        "Starting HTTP tracker instance"
    );

    let tls = if let Some(tls_config) = &http_tracker_container.http_tracker_config.tls_config {
        Some(
            make_rust_tls(tls_config)
                .await
                .expect("it should have a valid http tracker tls configuration"),
        )
    } else {
        None
    };

    match version {
        Version::V1 => Some(start_v1(socket, tls, http_tracker_container, form, metadata).await),
    }
}

#[allow(clippy::async_yields_async)]
#[instrument(
    skip(socket, tls, http_tracker_container, form, metadata),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
async fn start_v1(
    socket: SocketAddr,
    tls: Option<RustlsConfig>,
    http_tracker_container: Arc<HttpTrackerCoreContainer>,
    form: ServiceRegistrationForm<RuntimeServiceMetadata>,
    metadata: RuntimeServiceMetadata,
) -> JoinHandle<()> {
    let server = HttpServer::new(Launcher::new(
        socket,
        tls,
        http_tracker_container.http_tracker_config.network.ipv6_v6only,
    ))
    .start(http_tracker_container, form, metadata)
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

    use torrust_server_lib::registar::Registar;
    use torrust_tracker_axum_http_server::Version;
    use torrust_tracker_http_core::container::HttpTrackerCoreContainer;
    use torrust_tracker_primitives::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};
    use torrust_tracker_test_helpers::configuration::ephemeral_public;

    use crate::bootstrap::app::initialize_global_services;
    use crate::bootstrap::jobs::http_tracker::start_job;

    #[tokio::test]
    async fn it_should_start_http_tracker() {
        let cfg = Arc::new(ephemeral_public());
        let core_config = Arc::new(cfg.core.clone());
        let http_tracker = cfg.http_trackers.clone().expect("missing HTTP tracker configuration");
        let http_tracker_config = Arc::new(http_tracker[0].clone());
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);

        initialize_global_services(&cfg);

        let http_tracker_container =
            HttpTrackerCoreContainer::initialize(&core_config, &http_tracker_config, configuration_instance_id).await;

        let version = Version::V1;

        start_job(
            http_tracker_container,
            Registar::default().give_form(),
            RuntimeServiceMetadata::new(configuration_instance_id),
            version,
        )
        .await
        .expect("it should be able to join to the http tracker start-job");
    }
}
