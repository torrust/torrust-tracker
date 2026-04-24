//! Docker Compose stack provisioning for the `qBittorrent` E2E tests.
//!
//! This module starts the full infrastructure stack: builds the tracker image,
//! brings up the Docker Compose services, and constructs the `qBittorrent` API
//! clients for the seeder and leecher containers.
use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::Context;

use super::client_role::ClientRole;
use super::qbittorrent_client::QbittorrentClient;
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
    project_name: &str,
    tracker_image: &str,
    qbittorrent_image: &str,
    resources: &WorkspaceResources,
) -> anyhow::Result<(RunningCompose, QbittorrentClient, QbittorrentClient)> {
    let compose = build_compose(compose_file, project_name, tracker_image, qbittorrent_image, resources)?;
    compose.build().context("failed to build local tracker image")?;
    let running_compose = compose.up().context("failed to start qBittorrent compose stack")?;
    let (seeder, leecher) = build_api_clients(&compose, resources.timeout).await?;
    Ok((running_compose, seeder, leecher))
}

async fn build_api_clients(compose: &DockerCompose, timeout: Duration) -> anyhow::Result<(QbittorrentClient, QbittorrentClient)> {
    let seeder_port = wait_for_client_port(compose, ClientRole::Seeder, timeout).await?;
    let leecher_port = wait_for_client_port(compose, ClientRole::Leecher, timeout).await?;
    let seeder = build_client(ClientRole::Seeder, seeder_port, timeout)?;
    let leecher = build_client(ClientRole::Leecher, leecher_port, timeout)?;
    Ok((seeder, leecher))
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

fn build_compose(
    compose_file: &Path,
    project_name: &str,
    tracker_image: &str,
    qbittorrent_image: &str,
    workspace: &WorkspaceResources,
) -> anyhow::Result<DockerCompose> {
    Ok(DockerCompose::new(compose_file, project_name)
        .with_env("QBT_E2E_TRACKER_IMAGE", tracker_image)
        .with_env("QBT_E2E_QBITTORRENT_IMAGE", qbittorrent_image)
        .with_env(
            "QBT_E2E_TRACKER_CONFIG_PATH",
            normalize_path_for_compose(&workspace.tracker_config_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_TRACKER_STORAGE_PATH",
            normalize_path_for_compose(&workspace.tracker_storage_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_SHARED_PATH",
            normalize_path_for_compose(&workspace.shared_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_SEEDER_CONFIG_PATH",
            normalize_path_for_compose(&workspace.seeder_config_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_LEECHER_CONFIG_PATH",
            normalize_path_for_compose(&workspace.leecher_config_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_SEEDER_DOWNLOADS_PATH",
            normalize_path_for_compose(&workspace.seeder_downloads_path)?.as_str(),
        )
        .with_env(
            "QBT_E2E_LEECHER_DOWNLOADS_PATH",
            normalize_path_for_compose(&workspace.leecher_downloads_path)?.as_str(),
        ))
}

fn normalize_path_for_compose(path: &Path) -> anyhow::Result<String> {
    let absolute_path = fs::canonicalize(path).with_context(|| format!("failed to canonicalize path '{}'", path.display()))?;

    Ok(absolute_path.to_string_lossy().to_string())
}
