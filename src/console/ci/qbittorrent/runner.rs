//! Program to run qBittorrent E2E checks.
//!
//! Example:
//!
//! ```text
//! cargo run --bin qbittorrent_e2e_runner -- --compose-file ./compose.qbittorrent-e2e.yaml --timeout-seconds 180
//! ```
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use anyhow::Context;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use clap::Parser;
use pbkdf2::pbkdf2_hmac;
use rand::distr::Alphanumeric;
use rand::RngExt;
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::Sha512;
use tracing::level_filters::LevelFilter;

use super::client_role::ClientRole;
use super::qbittorrent_client::QbittorrentClient;
use super::scenario_steps::{
    add_torrent_file_to_client, build_payload_fixture, build_torrent_fixture, login_client, wait_until_client_has_any_torrent,
    wait_until_download_completes,
};
use super::workspace::{EphemeralWorkspace, PermanentWorkspace, PreparedWorkspace, WorkspaceResources};
use crate::console::ci::compose::DockerCompose;

const TRACKER_IMAGE: &str = "torrust-tracker:qbt-e2e-local";
const QBITTORRENT_IMAGE: &str = "lscr.io/linuxserver/qbittorrent:5.1.4";
const QBITTORRENT_USERNAME: &str = "admin";
const QBITTORRENT_PASSWORD: &str = "torrust-e2e-pass";
const QBITTORRENT_WEBUI_PORT: u16 = 8080;
const QBITTORRENT_CONFIG_RELATIVE_PATH: &str = "qBittorrent/qBittorrent.conf";
const QBITTORRENT_DOWNLOADS_PATH: &str = "/downloads";
const QBITTORRENT_DOWNLOADS_TEMP_PATH: &str = "/downloads/temp";
const PAYLOAD_FILE_NAME: &str = "payload.bin";
const TORRENT_FILE_NAME: &str = "payload.torrent";
const PAYLOAD_SIZE_BYTES: usize = 1024 * 1024;
const TORRENT_PIECE_LENGTH: usize = 16 * 1024;
const TORRENT_POLL_INTERVAL: Duration = Duration::from_millis(500);
const LOGIN_POLL_INTERVAL: Duration = Duration::from_secs(1);
const COMPOSE_PORT_POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, Debug)]
struct TorrentUpload<'a> {
    file_name: &'a str,
    bytes: &'a [u8],
}

impl<'a> TorrentUpload<'a> {
    const fn new(file_name: &'a str, bytes: &'a [u8]) -> Self {
        Self { file_name, bytes }
    }
}

type ClientPair = (QbittorrentClient, QbittorrentClient);
type ClientPairRef<'a> = (&'a QbittorrentClient, &'a QbittorrentClient);

struct GeneratedPayloadAndTorrent {
    payload_bytes: Vec<u8>,
    torrent_bytes: Vec<u8>,
}

struct ScenarioRunner<'a> {
    compose: &'a DockerCompose,
    workspace: &'a WorkspaceResources,
    timeout: Duration,
}

impl<'a> ScenarioRunner<'a> {
    const fn new(compose: &'a DockerCompose, workspace: &'a WorkspaceResources, timeout: Duration) -> Self {
        Self {
            compose,
            workspace,
            timeout,
        }
    }

    async fn run(&self) -> anyhow::Result<()> {
        // ARRANGE: wait for all clients to be reachable and authenticated.
        let (seeder, leecher) = self.initialize_clients().await?;

        // ACT: simulate the seeder-first transfer story.
        let torrent_upload = TorrentUpload::new(TORRENT_FILE_NAME, &self.workspace.torrent_bytes);

        self.upload_torrent_to_clients((&seeder, &leecher), torrent_upload).await?;
        self.wait_for_torrent_counts((&seeder, &leecher)).await?;
        wait_until_download_completes(&leecher, self.timeout, TORRENT_POLL_INTERVAL).await?;
        self.verify_payload_integrity()
            .context("downloaded payload does not match the original")?;

        Ok(())
    }

    async fn initialize_clients(&self) -> anyhow::Result<ClientPair> {
        let seeder = self.initialize_client(ClientRole::Seeder).await?;
        let leecher = self.initialize_client(ClientRole::Leecher).await?;

        tracing::info!("qBittorrent WebUI login succeeded for both clients");

        Ok((seeder, leecher))
    }

    async fn initialize_client(&self, role: ClientRole) -> anyhow::Result<QbittorrentClient> {
        let service_name = role.service_name();
        let host_port = self
            .compose
            .wait_for_port_mapping(
                service_name,
                QBITTORRENT_WEBUI_PORT,
                self.timeout,
                COMPOSE_PORT_POLL_INTERVAL,
                &["tracker"],
            )
            .await
            .with_context(|| format!("failed to resolve {service_name} WebUI host port"))?;

        tracing::info!("{} WebUI host port: {host_port}", role.client_label());

        let client = QbittorrentClient::new(role.client_label(), &format!("http://127.0.0.1:{host_port}"), self.timeout)
            .with_context(|| format!("failed to create qBittorrent client for service '{service_name}'"))?;

        login_client(
            &client,
            QBITTORRENT_USERNAME,
            QBITTORRENT_PASSWORD,
            self.timeout,
            LOGIN_POLL_INTERVAL,
        )
        .await
        .with_context(|| format!("{service_name} qBittorrent API did not become ready for authentication"))?;

        Ok(client)
    }

    async fn upload_torrent_to_clients(
        &self,
        clients: ClientPairRef<'_>,
        torrent_upload: TorrentUpload<'_>,
    ) -> anyhow::Result<()> {
        let (seeder, leecher) = clients;

        add_torrent_file_to_client(
            seeder,
            torrent_upload.file_name,
            torrent_upload.bytes,
            QBITTORRENT_DOWNLOADS_PATH,
        )
        .await?;

        add_torrent_file_to_client(
            leecher,
            torrent_upload.file_name,
            torrent_upload.bytes,
            QBITTORRENT_DOWNLOADS_PATH,
        )
        .await?;

        tracing::info!("Torrent file uploaded to both qBittorrent clients");

        Ok(())
    }

    /// Polls both clients until each has at least one torrent, then logs the final counts.
    ///
    /// qBittorrent processes `add_torrent` asynchronously, so an immediate `list_torrents`
    /// after upload can race and return 0.
    async fn wait_for_torrent_counts(&self, clients: ClientPairRef<'_>) -> anyhow::Result<()> {
        let (seeder, leecher) = clients;

        wait_until_client_has_any_torrent(seeder, self.timeout, TORRENT_POLL_INTERVAL, "Seeder").await?;

        wait_until_client_has_any_torrent(leecher, self.timeout, TORRENT_POLL_INTERVAL, "Leecher").await
    }

    fn verify_payload_integrity(&self) -> anyhow::Result<()> {
        verify_payload_integrity(&self.workspace.leecher_downloads_path, &self.workspace.payload_bytes)
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Compose file used for the qBittorrent scenario.
    #[clap(long, default_value = "compose.qbittorrent-e2e.yaml")]
    compose_file: PathBuf,

    /// Tracker config template copied into the temporary E2E workspace.
    #[clap(long, default_value = "share/default/config/tracker.e2e.container.sqlite3.toml")]
    tracker_config_template: PathBuf,

    /// Timeout in seconds for API operations.
    #[clap(long, default_value_t = 180)]
    timeout_seconds: u64,

    /// Local docker image tag used for the tracker service.
    #[clap(long, default_value = TRACKER_IMAGE)]
    tracker_image: String,

    /// qBittorrent image used for both seeder and leecher containers.
    #[clap(long, default_value = QBITTORRENT_IMAGE)]
    qbittorrent_image: String,

    /// Prefix for the random docker compose project name.
    #[clap(long, default_value = "qbt-e2e")]
    project_prefix: String,

    /// Leave containers running after the test finishes instead of tearing them
    /// down.  Useful for post-run debugging (e.g. `docker logs <container>`).
    #[clap(long, default_value_t = false)]
    keep_containers: bool,
}

/// Runs the qBittorrent E2E smoke orchestration.
///
/// # Errors
///
/// Returns an error when compose orchestration fails.
pub async fn run() -> anyhow::Result<()> {
    tracing_stdout_init(LevelFilter::INFO);

    let args = Args::parse();
    let project_name = build_project_name(&args.project_prefix);
    tracing::info!("Using compose project name: {project_name}");

    // ARRANGE: build workspace artifacts, tracker image, and start all containers.
    let workspace = prepare_workspace(&args, &project_name)?;
    let resources = workspace.resources();

    build_tracker_image(&args.tracker_image).context("failed to build local tracker image")?;

    let compose = build_compose(&args, &project_name, resources)?;
    let mut running_compose = compose.up().context("failed to start qBittorrent compose stack")?;

    // ACT: run the transfer scenario and verify the result.
    let timeout = Duration::from_secs(args.timeout_seconds);
    let scenario_runner = ScenarioRunner::new(&compose, resources, timeout);
    scenario_runner.run().await?;

    // POST-SCENARIO: optionally keep containers for debugging.
    if args.keep_containers {
        tracing::info!(
            "Keeping containers alive for debugging. Project name: '{}'. \
             Workspace: '{}'. \
             Use `docker compose -p {} logs` to inspect them, \
             then `docker compose -p {} down --volumes` to clean up.",
            running_compose.project(),
            workspace.root_path().display(),
            running_compose.project(),
            running_compose.project(),
        );
        running_compose.keep();
    }

    Ok(())
}

fn prepare_workspace(args: &Args, project_name: &str) -> anyhow::Result<PreparedWorkspace> {
    if args.keep_containers {
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
        let resources = prepare_workspace_resources(persistent_root, args)?;

        Ok(PreparedWorkspace::Permanent(PermanentWorkspace { resources }))
    } else {
        let temp_dir = tempfile::tempdir().context("failed to create temporary workspace")?;
        let root_path = temp_dir.path().to_path_buf();
        let resources = prepare_workspace_resources(root_path, args)?;

        Ok(PreparedWorkspace::Ephemeral(EphemeralWorkspace {
            _temp_dir: temp_dir,
            resources,
        }))
    }
}

fn prepare_workspace_resources(root_path: PathBuf, args: &Args) -> anyhow::Result<WorkspaceResources> {
    let tracker_storage_path = root_path.join("tracker-storage");
    let shared_path = root_path.join("shared");
    let seeder_config_path = root_path.join("seeder-config");
    let leecher_config_path = root_path.join("leecher-config");
    let seeder_downloads_path = root_path.join("seeder-downloads");
    let leecher_downloads_path = root_path.join("leecher-downloads");

    fs::create_dir_all(&tracker_storage_path).context("failed to create tracker storage directory")?;
    fs::create_dir_all(&shared_path).context("failed to create shared artifacts directory")?;
    fs::create_dir_all(&seeder_downloads_path).context("failed to create seeder downloads directory")?;
    fs::create_dir_all(&leecher_downloads_path).context("failed to create leecher downloads directory")?;

    write_qbittorrent_config(&seeder_config_path, QBITTORRENT_USERNAME, QBITTORRENT_PASSWORD)
        .context("failed to generate seeder qBittorrent config")?;
    write_qbittorrent_config(&leecher_config_path, QBITTORRENT_USERNAME, QBITTORRENT_PASSWORD)
        .context("failed to generate leecher qBittorrent config")?;

    let tracker_config_path = write_tracker_config(&root_path, &args.tracker_config_template)?;
    let generated_payload_and_torrent = write_payload_and_torrent(&shared_path, &seeder_downloads_path)?;

    Ok(WorkspaceResources {
        root_path,
        tracker_config_path,
        tracker_storage_path,
        shared_path,
        seeder_config_path,
        leecher_config_path,
        seeder_downloads_path,
        leecher_downloads_path,
        payload_bytes: generated_payload_and_torrent.payload_bytes,
        torrent_bytes: generated_payload_and_torrent.torrent_bytes,
    })
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
        payload_bytes: payload_fixture.bytes,
        torrent_bytes: torrent_fixture.bytes,
    })
}

fn build_compose(args: &Args, project_name: &str, workspace: &WorkspaceResources) -> anyhow::Result<DockerCompose> {
    Ok(DockerCompose::new(&args.compose_file, project_name)
        .with_env("QBT_E2E_TRACKER_IMAGE", &args.tracker_image)
        .with_env("QBT_E2E_QBITTORRENT_IMAGE", &args.qbittorrent_image)
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

/// Verifies that the leecher's downloaded file matches the original payload byte-for-byte.
///
/// Reads the downloaded file from `leecher_downloads_path/payload.bin` and compares it to
/// `original_payload`. Logs the `SHA1` hash of the verified payload on success.
fn verify_payload_integrity(leecher_downloads_path: &Path, original_payload: &[u8]) -> anyhow::Result<()> {
    let downloaded_path = leecher_downloads_path.join(PAYLOAD_FILE_NAME);
    let downloaded_bytes = fs::read(&downloaded_path)
        .with_context(|| format!("failed to read downloaded payload from '{}'", downloaded_path.display()))?;

    if downloaded_bytes.len() != original_payload.len() {
        anyhow::bail!(
            "payload size mismatch: original {} bytes, downloaded {} bytes",
            original_payload.len(),
            downloaded_bytes.len()
        );
    }

    if downloaded_bytes != original_payload {
        let original_hash = sha1_hex(original_payload);
        let downloaded_hash = sha1_hex(&downloaded_bytes);
        anyhow::bail!("payload content mismatch: original SHA1 {original_hash}, downloaded SHA1 {downloaded_hash}");
    }

    let hash = sha1_hex(original_payload);
    tracing::info!(
        "Payload integrity verified: SHA1 {} ({} bytes match)",
        hash,
        original_payload.len()
    );

    Ok(())
}

fn sha1_hex(bytes: &[u8]) -> String {
    Sha1::digest(bytes).iter().fold(String::new(), |mut output, byte| {
        let _ = write!(output, "{byte:02x}");
        output
    })
}

fn tracing_stdout_init(filter: LevelFilter) {
    tracing_subscriber::fmt().with_max_level(filter).init();
    tracing::info!("Logging initialized");
}

fn build_project_name(prefix: &str) -> String {
    let suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .map(|character| character.to_ascii_lowercase())
        .collect();
    format!("{prefix}-{suffix}")
}

fn normalize_path_for_compose(path: &Path) -> anyhow::Result<String> {
    let absolute_path = fs::canonicalize(path).with_context(|| format!("failed to canonicalize path '{}'", path.display()))?;

    Ok(absolute_path.to_string_lossy().to_string())
}

fn build_tracker_image(image: &str) -> anyhow::Result<()> {
    let status = Command::new("docker")
        .args(["build", "-f", "Containerfile", "-t", image, "--target", "release", "."])
        .status()
        .context("failed to invoke docker build for tracker image")?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("docker build failed for tracker image '{image}'"))
    }
}

fn write_qbittorrent_config(config_root: &Path, username: &str, password: &str) -> anyhow::Result<()> {
    let config_path = config_root.join(QBITTORRENT_CONFIG_RELATIVE_PATH);
    let config_dir = config_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("qBittorrent config path has no parent directory"))?;
    let resume_dir = config_root.join("qBittorrent/BT_backup");
    let cache_dir = config_root.join(".cache/qBittorrent");

    fs::create_dir_all(config_dir)
        .with_context(|| format!("failed to create qBittorrent config directory '{}'", config_dir.display()))?;
    fs::create_dir_all(&resume_dir)
        .with_context(|| format!("failed to create qBittorrent resume directory '{}'", resume_dir.display()))?;
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("failed to create qBittorrent cache directory '{}'", cache_dir.display()))?;

    let password_hash = build_qbittorrent_password_hash(password);
    let config = format!(
        "[BitTorrent]\nSession\\AddTorrentStopped=false\nSession\\DefaultSavePath={QBITTORRENT_DOWNLOADS_PATH}\nSession\\TempPath={QBITTORRENT_DOWNLOADS_TEMP_PATH}\n[Preferences]\nWebUI\\LocalHostAuth=false\nWebUI\\Port={QBITTORRENT_WEBUI_PORT}\nWebUI\\Password_PBKDF2=\"{password_hash}\"\nWebUI\\Username={username}\n"
    );

    fs::write(&config_path, config).with_context(|| format!("failed to write qBittorrent config '{}'", config_path.display()))?;

    Ok(())
}

fn build_qbittorrent_password_hash(password: &str) -> String {
    let salt: [u8; 16] = rand::random();
    let mut digest = [0_u8; 64];
    pbkdf2_hmac::<Sha512>(password.as_bytes(), &salt, 100_000, &mut digest);

    format!(
        "@ByteArray({}:{})",
        BASE64_STANDARD.encode(salt),
        BASE64_STANDARD.encode(digest)
    )
}
