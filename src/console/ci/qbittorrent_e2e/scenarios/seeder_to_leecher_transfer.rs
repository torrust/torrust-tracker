//! Scenario: a seeder and a leecher transfer a file via the tracker.
//!
//! This scenario verifies the most common `BitTorrent` tracker use-case:
//! a seeder publishes a torrent and a leecher downloads the complete file
//! through the tracker, which matches them as peers.
//!
//! The scenario is run twice — once with an HTTP announce URL and once with a
//! UDP announce URL — to exercise both tracker protocol implementations.

use std::fs;

use anyhow::Context;
use reqwest::Url;

use super::super::qbittorrent::QbittorrentClient;
use super::super::scenario_steps::{
    add_torrent_file_to_client, build_payload_fixture, build_torrent_fixture, ensure_torrent_is_absent, login_client,
    verify_payload_integrity, verify_tracker_swarm, wait_until_download_completes, wait_until_torrent_appears_in_client,
};
use super::super::tracker::TrackerApiClient;
use super::super::types::{FileName, InfoHash, PayloadSize, PieceLength};
use super::super::workspace::WorkspaceResources;

const PAYLOAD_SIZE_BYTES: PayloadSize = PayloadSize::new(1024 * 1024);
const TORRENT_PIECE_LENGTH: PieceLength = PieceLength::new(16 * 1024);

#[derive(Clone, Copy)]
enum Protocol {
    Http,
    Udp,
}

impl Protocol {
    fn label(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Udp => "udp",
        }
    }
}

/// Per-case data built fresh for each protocol run.
struct ScenarioCase {
    /// Protocol label used to disambiguate tracing events for repeated runs.
    protocol: Protocol,
    /// File name of the payload binary (e.g. `"payload-http.bin"`).
    payload_file_name: FileName,
    /// File name of the `.torrent` metainfo (e.g. `"payload-http.torrent"`).
    torrent_file_name: FileName,
    /// Raw bytes of the `.torrent` metainfo file passed to the qBittorrent API.
    torrent_bytes: Vec<u8>,
    /// v1 info hash of the torrent (lowercase hex, 40 chars).
    info_hash: InfoHash,
}

/// Runs the seeder-to-leecher transfer scenario for both the HTTP and UDP trackers.
///
/// # Errors
///
/// Returns an error if any step of either scenario case fails.
pub(crate) async fn run(
    seeder: &QbittorrentClient,
    leecher: &QbittorrentClient,
    tracker: &TrackerApiClient,
    workspace: &WorkspaceResources,
) -> anyhow::Result<()> {
    let http_case = prepare_case(workspace, Protocol::Http, &workspace.tracker_endpoints.http_announce_url)
        .context("failed to prepare HTTP scenario case")?;
    run_case(seeder, leecher, tracker, workspace, &http_case)
        .await
        .context("HTTP tracker scenario failed")?;

    let udp_case = prepare_case(workspace, Protocol::Udp, &workspace.tracker_endpoints.udp_announce_url)
        .context("failed to prepare UDP scenario case")?;
    run_case(seeder, leecher, tracker, workspace, &udp_case)
        .await
        .context("UDP tracker scenario failed")?;

    Ok(())
}

/// Prepares the shared and seeder-downloads files for one protocol run.
///
/// Writes `payload-{protocol}.bin` to both the shared directory and the seeder
/// downloads directory, then writes `payload-{protocol}.torrent` (pointing at
/// `announce_url`) to the shared directory.
///
/// # Errors
///
/// Returns an error when any file operation or torrent encoding fails.
fn prepare_case(workspace: &WorkspaceResources, protocol: Protocol, announce_url: &Url) -> anyhow::Result<ScenarioCase> {
    let payload_file_name = format!("payload-{}.bin", protocol.label());
    let torrent_file_name = format!("payload-{}.torrent", protocol.label());

    let payload_fixture = build_payload_fixture(PAYLOAD_SIZE_BYTES);

    let payload_path = workspace.shared.path.join(&payload_file_name);
    fs::write(&payload_path, &payload_fixture.bytes)
        .with_context(|| format!("failed to write payload file '{}'", payload_path.display()))?;

    let seeder_payload_path = workspace.seeder.downloads_path.join(&payload_file_name);
    fs::copy(&payload_path, &seeder_payload_path).with_context(|| {
        format!(
            "failed to prime seeder downloads with payload '{}'",
            seeder_payload_path.display()
        )
    })?;

    let torrent_fixture = build_torrent_fixture(
        &payload_fixture,
        &payload_file_name,
        announce_url.as_ref(),
        TORRENT_PIECE_LENGTH,
    )
    .context("failed to build torrent fixture")?;

    let torrent_path = workspace.shared.path.join(&torrent_file_name);
    fs::write(&torrent_path, &torrent_fixture.bytes)
        .with_context(|| format!("failed to write torrent file '{}'", torrent_path.display()))?;

    Ok(ScenarioCase {
        protocol,
        payload_file_name: FileName::new(&payload_file_name),
        torrent_file_name: FileName::new(&torrent_file_name),
        torrent_bytes: torrent_fixture.bytes,
        info_hash: torrent_fixture.info_hash,
    })
}

async fn run_case(
    seeder: &QbittorrentClient,
    leecher: &QbittorrentClient,
    tracker: &TrackerApiClient,
    workspace: &WorkspaceResources,
    case: &ScenarioCase,
) -> anyhow::Result<()> {
    let info_hash = &case.info_hash;
    let scenario_case = case.protocol.label();

    tracing::info!(case = scenario_case, torrent = %info_hash, "scenario start: seeder-to-leecher transfer");

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
        info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    add_torrent_file_to_client(
        seeder,
        &case.torrent_file_name,
        &case.torrent_bytes,
        &workspace.seeder.container_downloads_path,
    )
    .await?;

    // qBittorrent processes `add_torrent` asynchronously, so an immediate `list_torrents`
    // after upload can race and return 0.
    wait_until_torrent_appears_in_client(
        seeder,
        info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    tracing::info!(case = scenario_case, torrent = %info_hash, "seeder is ready");

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
        info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    add_torrent_file_to_client(
        leecher,
        &case.torrent_file_name,
        &case.torrent_bytes,
        &workspace.leecher.container_downloads_path,
    )
    .await?;

    tracing::info!(case = scenario_case, torrent = %info_hash, "download started: leecher is fetching from seeder");

    wait_until_torrent_appears_in_client(
        leecher,
        info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;
    wait_until_download_completes(
        leecher,
        info_hash,
        workspace.timing.polling_deadline,
        workspace.timing.torrent_poll_interval,
    )
    .await?;

    tracing::info!(case = scenario_case, torrent = %info_hash, "download finished");

    // ASSERT: downloaded file matches the original payload.

    verify_payload_integrity(
        &workspace.leecher.downloads_path.join(&case.payload_file_name),
        &workspace.shared.path.join(&case.payload_file_name),
    )
    .context("downloaded payload does not match the original")?;

    // ASSERT: tracker registered both peers (seeder announced; leecher completed).

    verify_tracker_swarm(tracker, info_hash)
        .await
        .context("tracker swarm verification failed")?;

    tracing::info!(case = scenario_case, torrent = %info_hash, "scenario passed: seeder-to-leecher transfer");

    Ok(())
}
