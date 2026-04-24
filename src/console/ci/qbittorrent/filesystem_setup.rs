//! Filesystem setup for the `qBittorrent` E2E tests.
//!
//! This module creates the directory tree, service configuration files, and
//! shared test fixtures that the `Docker` Compose stack needs before it starts.
//!
//! # Workspace Layout
//!
//! After [`prepare`] returns, the workspace root contains:
//!
//! ```text
//! <workspace-root>/
//! ├── leecher-config/
//! │   └── qBittorrent/
//! │       └── qBittorrent.conf
//! ├── leecher-downloads/
//! ├── seeder-config/
//! │   └── qBittorrent/
//! │       └── qBittorrent.conf
//! ├── seeder-downloads/
//! │   └── payload.bin          ← pre-seeded payload copy
//! ├── shared/
//! │   ├── payload.bin          ← source payload file
//! │   └── payload.torrent
//! ├── tracker-config.toml
//! └── tracker-storage/
//!     └── database/
//!         └── sqlite3.db       ← created at runtime by the tracker
//! ```
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;

use super::qbittorrent_config::QbittorrentConfigBuilder;
use super::scenario_steps::{build_payload_fixture, build_torrent_fixture};
use super::workspace::{
    EphemeralWorkspace, PermanentWorkspace, PreparedWorkspace, SharedFixtures, TimingConfig, TrackerFilesystem,
    WorkspaceResources,
};

const QBITTORRENT_USERNAME: &str = "admin";
const QBITTORRENT_PASSWORD: &str = "torrust-e2e-pass";
const PAYLOAD_FILE_NAME: &str = "payload.bin";
const TORRENT_FILE_NAME: &str = "payload.torrent";
const PAYLOAD_SIZE_BYTES: usize = 1024 * 1024;
const TORRENT_PIECE_LENGTH: usize = 16 * 1024;
const QBITTORRENT_DOWNLOADS_PATH: &str = "/downloads";
const TORRENT_POLL_INTERVAL: Duration = Duration::from_millis(500);
const LOGIN_POLL_INTERVAL: Duration = Duration::from_secs(1);

struct GeneratedPayloadAndTorrent {
    torrent_bytes: Vec<u8>,
}

/// Creates and populates the workspace for a single E2E test run.
///
/// Returns an ephemeral workspace (temporary directory, auto-cleaned on drop)
/// when `keep_containers` is `false`, or a permanent workspace under
/// `storage/qbt-e2e/<project_name>` when it is `true`.
///
/// # Errors
///
/// Returns an error when any directory or file operation fails.
pub(crate) fn prepare(
    tracker_config_template: &Path,
    project_name: &str,
    keep_containers: bool,
    timeout: Duration,
) -> anyhow::Result<PreparedWorkspace> {
    if keep_containers {
        let persistent_root = std::env::current_dir()
            .context("failed to resolve current working directory")?
            .join("storage")
            .join("qbt-e2e")
            .join(project_name);
        fs::create_dir_all(&persistent_root).with_context(|| {
            format!(
                "failed to create persistent qBittorrent workspace '{}'",
                persistent_root.display()
            )
        })?;
        let resources = prepare_resources(persistent_root, tracker_config_template, timeout)?;

        Ok(PreparedWorkspace::Permanent(PermanentWorkspace { resources }))
    } else {
        let temp_dir = tempfile::tempdir().context("failed to create temporary workspace")?;
        let root_path = temp_dir.path().to_path_buf();
        let resources = prepare_resources(root_path, tracker_config_template, timeout)?;

        Ok(PreparedWorkspace::Ephemeral(EphemeralWorkspace {
            _temp_dir: temp_dir,
            resources,
        }))
    }
}

fn prepare_resources(
    root_path: PathBuf,
    tracker_config_template: &Path,
    timeout: Duration,
) -> anyhow::Result<WorkspaceResources> {
    let (tracker_config_path, tracker_storage_path) = setup_tracker_workspace(&root_path, tracker_config_template)?;
    let (seeder_config_path, seeder_downloads_path) = setup_qbittorrent_workspace(&root_path, "seeder")?;
    let (leecher_config_path, leecher_downloads_path) = setup_qbittorrent_workspace(&root_path, "leecher")?;
    let (shared_path, generated) = setup_shared_fixtures(&root_path, &seeder_downloads_path)?;

    Ok(WorkspaceResources {
        root_path,
        tracker: TrackerFilesystem {
            config_path: tracker_config_path,
            storage_path: tracker_storage_path,
        },
        seeder_config_path,
        leecher_config_path,
        seeder_downloads_path,
        leecher_downloads_path,
        shared: SharedFixtures {
            path: shared_path,
            payload_file_name: PAYLOAD_FILE_NAME.to_string(),
            torrent_file_name: TORRENT_FILE_NAME.to_string(),
            torrent_bytes: generated.torrent_bytes,
        },
        timing: TimingConfig {
            polling_deadline: timeout,
            login_poll_interval: LOGIN_POLL_INTERVAL,
            torrent_poll_interval: TORRENT_POLL_INTERVAL,
        },
        username: QBITTORRENT_USERNAME.to_string(),
        password: QBITTORRENT_PASSWORD.to_string(),
        downloads_path: QBITTORRENT_DOWNLOADS_PATH.to_string(),
    })
}

fn setup_tracker_workspace(root: &Path, config_template: &Path) -> anyhow::Result<(PathBuf, PathBuf)> {
    let tracker_storage_path = root.join("tracker-storage");
    fs::create_dir_all(&tracker_storage_path).context("failed to create tracker storage directory")?;
    let tracker_config_path = write_tracker_config(root, config_template)?;
    Ok((tracker_config_path, tracker_storage_path))
}

fn setup_qbittorrent_workspace(root: &Path, role: &str) -> anyhow::Result<(PathBuf, PathBuf)> {
    let config_path = root.join(format!("{role}-config"));
    let downloads_path = root.join(format!("{role}-downloads"));
    fs::create_dir_all(&downloads_path).with_context(|| format!("failed to create {role} downloads directory"))?;
    QbittorrentConfigBuilder::new(QBITTORRENT_USERNAME, QBITTORRENT_PASSWORD)
        .write_to(&config_path)
        .with_context(|| format!("failed to generate {role} qBittorrent config"))?;
    Ok((config_path, downloads_path))
}

fn setup_shared_fixtures(root: &Path, seeder_downloads: &Path) -> anyhow::Result<(PathBuf, GeneratedPayloadAndTorrent)> {
    let shared_path = root.join("shared");
    fs::create_dir_all(&shared_path).context("failed to create shared artifacts directory")?;
    let generated = write_payload_and_torrent(&shared_path, seeder_downloads)?;
    Ok((shared_path, generated))
}

fn write_tracker_config(workspace_root: &Path, tracker_config_template: &Path) -> anyhow::Result<PathBuf> {
    let tracker_config_path = workspace_root.join("tracker-config.toml");
    let tracker_config = fs::read_to_string(tracker_config_template).with_context(|| {
        format!(
            "failed to read tracker config template '{}'",
            tracker_config_template.display()
        )
    })?;

    fs::write(&tracker_config_path, tracker_config)
        .with_context(|| format!("failed to write generated tracker config '{}'", tracker_config_path.display()))?;

    Ok(tracker_config_path)
}

fn write_payload_and_torrent(shared_path: &Path, seeder_downloads_path: &Path) -> anyhow::Result<GeneratedPayloadAndTorrent> {
    let payload_path = shared_path.join(PAYLOAD_FILE_NAME);
    let torrent_path = shared_path.join(TORRENT_FILE_NAME);
    let payload_fixture = build_payload_fixture(PAYLOAD_SIZE_BYTES);

    fs::write(&payload_path, &payload_fixture.bytes)
        .with_context(|| format!("failed to write payload file '{}'", payload_path.display()))?;
    fs::copy(&payload_path, seeder_downloads_path.join(PAYLOAD_FILE_NAME)).with_context(|| {
        format!(
            "failed to prime seeder downloads with payload '{}'",
            seeder_downloads_path.join(PAYLOAD_FILE_NAME).display()
        )
    })?;

    let torrent_fixture = build_torrent_fixture(
        &payload_fixture,
        PAYLOAD_FILE_NAME,
        "http://tracker:7070/announce",
        TORRENT_PIECE_LENGTH,
    )?;
    fs::write(&torrent_path, &torrent_fixture.bytes)
        .with_context(|| format!("failed to write torrent file '{}'", torrent_path.display()))?;

    Ok(GeneratedPayloadAndTorrent {
        torrent_bytes: torrent_fixture.bytes,
    })
}
