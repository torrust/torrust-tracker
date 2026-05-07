//! Container services setup for the `qBittorrent` E2E tests.
//!
//! This module starts the full infrastructure stack: builds the tracker image,
//! brings up the `Docker` Compose services, and constructs the `qBittorrent` API
//! clients for the seeder and leecher containers.
use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::Context;

use super::client_role::ClientRole;
use super::qbittorrent::{QbittorrentClient, QBITTORRENT_WEBUI_PORT};
use super::tracker::{TrackerApiClient, TrackerConfig};
use super::types::{ComposeProjectName, QbittorrentImage, TrackerImage};
use super::workspace::WorkspaceResources;
use crate::console::ci::compose::{DockerCompose, RunningCompose};
const COMPOSE_PORT_POLL_INTERVAL: Duration = Duration::from_secs(1);

/// Builds the tracker image, starts all Docker Compose services, and returns
/// the running stack guard together with the seeder and leecher API clients.
///
/// # Errors
///
/// Returns an error when image building, service start-up, or client
/// construction fails.
pub(crate) async fn start(
    compose_file: &Path,
    project_name: &ComposeProjectName,
    tracker_image: &TrackerImage,
    qbittorrent_image: &QbittorrentImage,
    resources: &WorkspaceResources,
    tracker_config: &TrackerConfig,
    skip_build: bool,
) -> anyhow::Result<(RunningCompose, QbittorrentClient, QbittorrentClient, TrackerApiClient)> {
    let compose = configure_compose(
        compose_file,
        project_name,
        tracker_image,
        qbittorrent_image,
        resources,
        tracker_config,
    )?;
    if !skip_build {
        compose.build().context("failed to build local tracker image")?;
    }
    let running_compose = compose.up().context("failed to start qBittorrent compose stack")?;
    let timeout = resources.timing.polling_deadline.as_duration();
    let (seeder, leecher) = build_clients(&compose, timeout).await?;
    let tracker = build_tracker_api_client(&compose, tracker_config, timeout).await?;
    Ok((running_compose, seeder, leecher, tracker))
}

async fn build_clients(compose: &DockerCompose, timeout: Duration) -> anyhow::Result<(QbittorrentClient, QbittorrentClient)> {
    let seeder = build_seeder_client(compose, timeout).await?;
    let leecher = build_leecher_client(compose, timeout).await?;
    Ok((seeder, leecher))
}

async fn build_tracker_api_client(
    compose: &DockerCompose,
    tracker_config: &TrackerConfig,
    timeout: Duration,
) -> anyhow::Result<TrackerApiClient> {
    let container_port = tracker_config.http_api_bind_address().port();
    let host_port = compose
        .wait_for_port_mapping("tracker", container_port, timeout, COMPOSE_PORT_POLL_INTERVAL, &[])
        .await
        .context("failed to resolve tracker REST API host port")?;

    tracing::info!("Tracker REST API host port: {host_port}");

    TrackerApiClient::new(host_port, tracker_config).context("failed to build tracker REST API client")
}

async fn build_seeder_client(compose: &DockerCompose, timeout: Duration) -> anyhow::Result<QbittorrentClient> {
    let port = wait_for_client_port(compose, ClientRole::Seeder, timeout).await?;
    build_client(ClientRole::Seeder, port, timeout)
}

async fn build_leecher_client(compose: &DockerCompose, timeout: Duration) -> anyhow::Result<QbittorrentClient> {
    let port = wait_for_client_port(compose, ClientRole::Leecher, timeout).await?;
    build_client(ClientRole::Leecher, port, timeout)
}

async fn wait_for_client_port(compose: &DockerCompose, role: ClientRole, timeout: Duration) -> anyhow::Result<u16> {
    let service_name = role.service_name();
    let host_port = compose
        .wait_for_port_mapping(
            service_name,
            QBITTORRENT_WEBUI_PORT,
            timeout,
            COMPOSE_PORT_POLL_INTERVAL,
            &["tracker"],
        )
        .await
        .with_context(|| format!("failed to resolve {service_name} WebUI host port"))?;

    tracing::info!("{} WebUI host port: {host_port}", role.client_label());

    Ok(host_port)
}

fn build_client(role: ClientRole, host_port: u16, timeout: Duration) -> anyhow::Result<QbittorrentClient> {
    let service_name = role.service_name();
    QbittorrentClient::new(role.client_label(), &format!("http://localhost:{host_port}"), timeout)
        .with_context(|| format!("failed to create qBittorrent client for service '{service_name}'"))
}

fn configure_compose(
    compose_file: &Path,
    project_name: &ComposeProjectName,
    tracker_image: &TrackerImage,
    qbittorrent_image: &QbittorrentImage,
    workspace: &WorkspaceResources,
    tracker_config: &TrackerConfig,
) -> anyhow::Result<DockerCompose> {
    let tracker_http_tracker_port = tracker_config.http_tracker_bind_address().port().to_string();
    let tracker_udp_port = tracker_config.udp_bind_address().port().to_string();
    let tracker_http_api_port = tracker_config.http_api_bind_address().port().to_string();
    let tracker_health_check_api_port = tracker_config.health_check_api_bind_address().port().to_string();

    Ok(DockerCompose::new(compose_file, project_name.as_str())
        .with_env("QBT_E2E_TRACKER_IMAGE", tracker_image.as_str())
        .with_env("QBT_E2E_QBITTORRENT_IMAGE", qbittorrent_image.as_str())
        .with_env("QBT_E2E_TRACKER_HTTP_TRACKER_PORT", tracker_http_tracker_port.as_str())
        .with_env("QBT_E2E_TRACKER_UDP_PORT", tracker_udp_port.as_str())
        .with_env("QBT_E2E_TRACKER_HTTP_API_PORT", tracker_http_api_port.as_str())
        .with_env(
            "QBT_E2E_TRACKER_HEALTH_CHECK_API_PORT",
            tracker_health_check_api_port.as_str(),
        )
        .with_env(
            "QBT_E2E_TRACKER_CONFIG_PATH",
            normalize_path_for_compose(&workspace.tracker.config_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_TRACKER_STORAGE_PATH",
            normalize_path_for_compose(&workspace.tracker.storage_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_SHARED_PATH",
            normalize_path_for_compose(&workspace.shared.path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_SEEDER_CONFIG_PATH",
            normalize_path_for_compose(&workspace.seeder.config_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_LEECHER_CONFIG_PATH",
            normalize_path_for_compose(&workspace.leecher.config_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_SEEDER_DOWNLOADS_PATH",
            normalize_path_for_compose(&workspace.seeder.downloads_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_LEECHER_DOWNLOADS_PATH",
            normalize_path_for_compose(&workspace.leecher.downloads_path)?.as_str(),
        ))
}

fn normalize_path_for_compose(path: &Path) -> anyhow::Result<String> {
    let absolute_path = fs::canonicalize(path).with_context(|| format!("failed to canonicalize path '{}'", path.display()))?;

    Ok(absolute_path.to_string_lossy().into_owned())
}
