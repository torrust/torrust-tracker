//! Scenario: a seeder and a leecher transfer a file via the tracker.
//!
//! This scenario verifies the most common `BitTorrent` tracker use-case:
//! a seeder publishes a torrent and a leecher downloads the complete file
//! through the tracker, which matches them as peers.

use anyhow::Context;

use super::super::qbittorrent::QbittorrentClient;
use super::super::scenario_steps::{
    add_torrent_file_to_client, ensure_torrent_is_absent, login_client, verify_payload_integrity, verify_tracker_swarm,
    wait_until_download_completes, wait_until_torrent_appears_in_client,
};
use super::super::tracker::TrackerApiClient;
use super::super::workspace::WorkspaceResources;

/// Runs the seeder-to-leecher transfer scenario.
///
/// # Errors
///
/// Returns an error if any step of the scenario fails.
pub(crate) async fn run(
    seeder: &QbittorrentClient,
    leecher: &QbittorrentClient,
    tracker: &TrackerApiClient,
    workspace: &WorkspaceResources,
) -> anyhow::Result<()> {
    let info_hash = workspace.shared.torrent.info_hash.clone();

    tracing::info!(torrent = %info_hash, "scenario start: seeder-to-leecher transfer");

    // ARRANGE: seeder seeds a new torrent

    login_client(
        seeder,
        &workspace.seeder.credentials,
        workspace.timing.polling_deadline,
        workspace.timing.login_poll_interval,
    )
    .await
    .context("seeder qBittorrent API did not become ready for authentication")?;

    // Guarantee a clean starting state — delete the torrent if a previous run left it behind.
    ensure_torrent_is_absent(
        seeder,
        &info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    add_torrent_file_to_client(
        seeder,
        &workspace.shared.torrent.torrent_file_name,
        &workspace.shared.torrent.torrent_bytes,
        &workspace.seeder.container_downloads_path,
    )
    .await?;

    // qBittorrent processes `add_torrent` asynchronously, so an immediate `list_torrents`
    // after upload can race and return 0.
    wait_until_torrent_appears_in_client(
        seeder,
        &info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    tracing::info!(torrent = %info_hash, "seeder is ready");

    // ACT: leecher downloads the torrent from the seeder via the tracker

    login_client(
        leecher,
        &workspace.leecher.credentials,
        workspace.timing.polling_deadline,
        workspace.timing.login_poll_interval,
    )
    .await
    .context("leecher qBittorrent API did not become ready for authentication")?;

    // Guarantee a clean starting state for the leecher.
    ensure_torrent_is_absent(
        leecher,
        &info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    add_torrent_file_to_client(
        leecher,
        &workspace.shared.torrent.torrent_file_name,
        &workspace.shared.torrent.torrent_bytes,
        &workspace.leecher.container_downloads_path,
    )
    .await?;

    tracing::info!(torrent = %info_hash, "download started: leecher is fetching from seeder");

    wait_until_torrent_appears_in_client(
        leecher,
        &info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;
    wait_until_download_completes(
        leecher,
        &info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    tracing::info!(torrent = %info_hash, "download finished");

    // ASSERT: downloaded file matches the original payload.

    verify_payload_integrity(
        &workspace
            .leecher
            .downloads_path
            .join(&workspace.shared.torrent.payload_file_name),
        &workspace.shared.path.join(&workspace.shared.torrent.payload_file_name),
    )
    .context("downloaded payload does not match the original")?;

    // ASSERT: tracker registered both peers (seeder announced; leecher completed).

    verify_tracker_swarm(tracker, &info_hash)
        .await
        .context("tracker swarm verification failed")?;

    tracing::info!(torrent = %info_hash, "scenario passed: seeder-to-leecher transfer");

    Ok(())
}
