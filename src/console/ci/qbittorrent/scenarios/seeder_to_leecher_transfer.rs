//! Scenario: a seeder and a leecher transfer a file via the tracker.
//!
//! This scenario verifies the most common `BitTorrent` tracker use-case:
//! a seeder publishes a torrent and a leecher downloads the complete file
//! through the tracker, which matches them as peers.

use anyhow::Context;

use super::super::qbittorrent_client::QbittorrentClient;
use super::super::scenario_steps::{
    add_torrent_file_to_client, login_client, verify_payload_integrity, wait_until_client_has_any_torrent,
    wait_until_download_completes,
};
use super::super::workspace::WorkspaceResources;

/// Runs the seeder-to-leecher transfer scenario.
///
/// # Errors
///
/// Returns an error if any step of the scenario fails.
pub(crate) async fn run(
    seeder: &QbittorrentClient,
    leecher: &QbittorrentClient,
    workspace: &WorkspaceResources,
) -> anyhow::Result<()> {
    // ARRANGE: seeder seeds a new torrent

    login_client(
        seeder,
        &workspace.seeder.credentials.username,
        &workspace.seeder.credentials.password,
        workspace.timing.polling_deadline,
        workspace.timing.login_poll_interval,
    )
    .await
    .context("seeder qBittorrent API did not become ready for authentication")?;

    add_torrent_file_to_client(
        seeder,
        &workspace.shared.torrent.torrent_file_name,
        &workspace.shared.torrent.torrent_bytes,
        &workspace.seeder.container_downloads_path,
    )
    .await?;

    // qBittorrent processes `add_torrent` asynchronously, so an immediate `list_torrents`
    // after upload can race and return 0.
    wait_until_client_has_any_torrent(
        seeder,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
        "Seeder",
    )
    .await?;

    // ACT: leecher downloads the torrent from the seeder via the tracker

    login_client(
        leecher,
        &workspace.leecher.credentials.username,
        &workspace.leecher.credentials.password,
        workspace.timing.polling_deadline,
        workspace.timing.login_poll_interval,
    )
    .await
    .context("leecher qBittorrent API did not become ready for authentication")?;
    tracing::info!("qBittorrent WebUI login succeeded for both clients");

    add_torrent_file_to_client(
        leecher,
        &workspace.shared.torrent.torrent_file_name,
        &workspace.shared.torrent.torrent_bytes,
        &workspace.leecher.container_downloads_path,
    )
    .await?;
    tracing::info!("Torrent file uploaded to both qBittorrent clients");

    wait_until_client_has_any_torrent(
        leecher,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
        "Leecher",
    )
    .await?;
    wait_until_download_completes(
        leecher,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    // ASSERT: downloaded file matches the original payload.

    verify_payload_integrity(
        &workspace
            .leecher
            .downloads_path
            .join(&workspace.shared.torrent.payload_file_name),
        &workspace.shared.path.join(&workspace.shared.torrent.payload_file_name),
    )
    .context("downloaded payload does not match the original")?;

    Ok(())
}
