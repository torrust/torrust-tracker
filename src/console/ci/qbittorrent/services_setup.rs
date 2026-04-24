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
use super::qbittorrent_client::QbittorrentClient;
use super::types::{ComposeProjectName, QbittorrentImage, TrackerImage};
use super::workspace::WorkspaceResources;
use crate::console::ci::compose::{DockerCompose, RunningCompose};

const QBITTORRENT_WEBUI_PORT: u16 = 8080;
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
) -> anyhow::Result<(RunningCompose, QbittorrentClient, QbittorrentClient)> {
    let compose = configure_compose(compose_file, project_name, tracker_image, qbittorrent_image, resources)?;
    compose.build().context("failed to build local tracker image")?;
    let running_compose = compose.up().context("failed to start qBittorrent compose stack")?;
    let (seeder, leecher) = build_clients(&compose, resources.timing.polling_deadline.as_duration()).await?;
    Ok((running_compose, seeder, leecher))
}

async fn build_clients(compose: &DockerCompose, timeout: Duration) -> anyhow::Result<(QbittorrentClient, QbittorrentClient)> {
    let seeder = build_seeder_client(compose, timeout).await?;
    let leecher = build_leecher_client(compose, timeout).await?;
    Ok((seeder, leecher))
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
    QbittorrentClient::new(role.client_label(), &format!("http://127.0.0.1:{host_port}"), timeout)
        .with_context(|| format!("failed to create qBittorrent client for service '{service_name}'"))
}

fn configure_compose(
    compose_file: &Path,
    project_name: &ComposeProjectName,
    tracker_image: &TrackerImage,
    qbittorrent_image: &QbittorrentImage,
    workspace: &WorkspaceResources,
) -> anyhow::Result<DockerCompose> {
    Ok(DockerCompose::new(compose_file, project_name.as_str())
        .with_env("QBT_E2E_TRACKER_IMAGE", tracker_image.as_str())
        .with_env("QBT_E2E_QBITTORRENT_IMAGE", qbittorrent_image.as_str())
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

    Ok(absolute_path.to_string_lossy().to_string())
}
