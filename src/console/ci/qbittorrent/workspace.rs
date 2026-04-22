use std::path::{Path, PathBuf};

pub(crate) struct WorkspaceResources {
    pub(crate) root_path: PathBuf,
    pub(crate) tracker_config_path: PathBuf,
    pub(crate) tracker_storage_path: PathBuf,
    pub(crate) shared_path: PathBuf,
    pub(crate) seeder_config_path: PathBuf,
    pub(crate) leecher_config_path: PathBuf,
    pub(crate) seeder_downloads_path: PathBuf,
    pub(crate) leecher_downloads_path: PathBuf,
    pub(crate) payload_bytes: Vec<u8>,
    pub(crate) torrent_bytes: Vec<u8>,
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
