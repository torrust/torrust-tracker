use std::path::{Path, PathBuf};

use super::qbittorrent::QbittorrentCredentials;
use super::types::{ContainerPath, Deadline, FileName, InfoHash, PollInterval};

pub(crate) struct PeerConfig {
    /// Path to `{role}-config/` on the host.
    pub(crate) config_path: PathBuf,
    /// Path to `{role}-downloads/` on the host.
    pub(crate) downloads_path: PathBuf,
    /// Credentials for the `qBittorrent` web UI.
    pub(crate) credentials: QbittorrentCredentials,
    /// Download path inside the container (e.g. `"/downloads"`).
    pub(crate) container_downloads_path: ContainerPath,
}

pub(crate) struct TrackerFilesystem {
    /// Path to `tracker-config.toml` on the host.
    pub(crate) config_path: PathBuf,
    /// Path to the `tracker-storage/` directory on the host.
    pub(crate) storage_path: PathBuf,
}

pub(crate) struct TorrentFixture {
    /// File name of the payload (e.g. `"payload.bin"`).
    pub(crate) payload_file_name: FileName,
    /// File name of the torrent file (e.g. `"payload.torrent"`).
    pub(crate) torrent_file_name: FileName,
    /// Raw bytes of the torrent file, held in memory.
    pub(crate) torrent_bytes: Vec<u8>,
    /// v1 [`InfoHash`]: SHA-1 of the bencoded `info` dict, lowercase hex (40 chars).
    /// Matches the hash format returned by the qBittorrent Web API.
    pub(crate) info_hash: InfoHash,
}

pub(crate) struct SharedFixtures {
    /// Path to the `shared/` directory on the host.
    pub(crate) path: PathBuf,
    /// The torrent fixture used by the current scenario.
    pub(crate) torrent: TorrentFixture,
}

pub(crate) struct TimingConfig {
    /// Maximum time any single polling loop will wait before giving up.
    /// Passed directly to `Poller::new` as the loop deadline.
    pub(crate) polling_deadline: Deadline,
    /// Sleep duration between login-readiness retries.
    pub(crate) login_poll_interval: PollInterval,
    /// Sleep duration between torrent-state retries.
    pub(crate) torrent_poll_interval: PollInterval,
}

pub(crate) struct WorkspaceResources {
    pub(crate) root_path: PathBuf,
    pub(crate) tracker: TrackerFilesystem,
    pub(crate) seeder: PeerConfig,
    pub(crate) leecher: PeerConfig,
    pub(crate) shared: SharedFixtures,
    pub(crate) timing: TimingConfig,
}

pub(crate) struct EphemeralWorkspace {
    pub(crate) _temp_dir: tempfile::TempDir,
    pub(crate) resources: WorkspaceResources,
}

pub(crate) struct PermanentWorkspace {
    pub(crate) resources: WorkspaceResources,
}

pub(crate) enum PreparedWorkspace {
    Ephemeral(EphemeralWorkspace),
    Permanent(PermanentWorkspace),
}

impl PreparedWorkspace {
    pub(crate) fn resources(&self) -> &WorkspaceResources {
        match self {
            Self::Ephemeral(workspace) => &workspace.resources,
            Self::Permanent(workspace) => &workspace.resources,
        }
    }

    pub(crate) fn root_path(&self) -> &Path {
        &self.resources().root_path
    }
}
