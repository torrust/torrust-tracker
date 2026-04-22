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
use tokio::time::sleep;
use tracing::level_filters::LevelFilter;

use super::bencode::BencodeValue;
use super::qbittorrent_client::QbittorrentClient;
use crate::console::ci::compose::DockerCompose;

const TRACKER_IMAGE: &str = "torrust-tracker:qbt-e2e-local";
const QBITTORRENT_IMAGE: &str = "lscr.io/linuxserver/qbittorrent:5.1.4";
const QBITTORRENT_USERNAME: &str = "admin";
const QBITTORRENT_PASSWORD: &str = "torrust-e2e-pass";
const QBITTORRENT_FALLBACK_PASSWORD: &str = "adminadmin";
const QBITTORRENT_WEBUI_PORT: u16 = 8080;
const QBITTORRENT_CONFIG_RELATIVE_PATH: &str = "qBittorrent/qBittorrent.conf";
const PAYLOAD_FILE_NAME: &str = "payload.bin";
const TORRENT_FILE_NAME: &str = "payload.torrent";
const PAYLOAD_SIZE_BYTES: usize = 1024 * 1024;
const TORRENT_PIECE_LENGTH: usize = 16 * 1024;

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

struct WorkspaceResources {
    root_path: PathBuf,
    tracker_config_path: PathBuf,
    tracker_storage_path: PathBuf,
    shared_path: PathBuf,
    seeder_config_path: PathBuf,
    leecher_config_path: PathBuf,
    seeder_downloads_path: PathBuf,
    leecher_downloads_path: PathBuf,
    payload_bytes: Vec<u8>,
    torrent_bytes: Vec<u8>,
}

struct EphemeralWorkspace {
    _temp_dir: tempfile::TempDir,
    resources: WorkspaceResources,
}

struct PermanentWorkspace {
    resources: WorkspaceResources,
}

enum PreparedWorkspace {
    Ephemeral(EphemeralWorkspace),
    Permanent(PermanentWorkspace),
}

impl PreparedWorkspace {
    fn resources(&self) -> &WorkspaceResources {
        match self {
            Self::Ephemeral(workspace) => &workspace.resources,
            Self::Permanent(workspace) => &workspace.resources,
        }
    }

    fn root_path(&self) -> &Path {
        &self.resources().root_path
    }
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

    let workspace = prepare_workspace(&args, &project_name)?;
    let resources = workspace.resources();

    build_tracker_image(&args.tracker_image).context("failed to build local tracker image")?;

    let compose = build_compose(&args, &project_name, resources)?;
    let mut running_compose = compose.up().context("failed to start qBittorrent compose stack")?;

    let timeout = Duration::from_secs(args.timeout_seconds);
    let (seeder, leecher) = initialize_clients(&compose, timeout).await?;
    upload_torrent_to_clients(&seeder, &leecher, &resources.torrent_bytes).await?;
    wait_for_torrent_counts(&seeder, &leecher, timeout).await?;
    wait_for_leecher_completion(&leecher, timeout).await?;
    verify_payload_integrity(&resources.leecher_downloads_path, &resources.payload_bytes)
        .context("downloaded payload does not match the original")?;

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
    let (payload_bytes, torrent_bytes) = write_payload_and_torrent(&shared_path, &seeder_downloads_path)?;

    Ok(WorkspaceResources {
        root_path,
        tracker_config_path,
        tracker_storage_path,
        shared_path,
        seeder_config_path,
        leecher_config_path,
        seeder_downloads_path,
        leecher_downloads_path,
        payload_bytes,
        torrent_bytes,
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

fn write_payload_and_torrent(shared_path: &Path, seeder_downloads_path: &Path) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let payload_path = shared_path.join(PAYLOAD_FILE_NAME);
    let torrent_path = shared_path.join(TORRENT_FILE_NAME);
    let payload_bytes = build_payload_bytes(PAYLOAD_SIZE_BYTES);

    fs::write(&payload_path, &payload_bytes)
        .with_context(|| format!("failed to write payload file '{}'", payload_path.display()))?;
    fs::copy(&payload_path, seeder_downloads_path.join(PAYLOAD_FILE_NAME)).with_context(|| {
        format!(
            "failed to prime seeder downloads with payload '{}'",
            seeder_downloads_path.join(PAYLOAD_FILE_NAME).display()
        )
    })?;

    let torrent_bytes = build_torrent_bytes(&payload_bytes, PAYLOAD_FILE_NAME, "http://tracker:7070/announce")?;
    fs::write(&torrent_path, &torrent_bytes)
        .with_context(|| format!("failed to write torrent file '{}'", torrent_path.display()))?;

    Ok((payload_bytes, torrent_bytes))
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

async fn initialize_clients(
    compose: &DockerCompose,
    timeout: Duration,
) -> anyhow::Result<(QbittorrentClient, QbittorrentClient)> {
    let seeder = initialize_client(compose, "qbittorrent-seeder", "Seeder", timeout).await?;
    let leecher = initialize_client(compose, "qbittorrent-leecher", "Leecher", timeout).await?;

    tracing::info!("qBittorrent WebUI login succeeded for both clients");

    Ok((seeder, leecher))
}

async fn initialize_client(
    compose: &DockerCompose,
    service: &str,
    client_label: &str,
    timeout: Duration,
) -> anyhow::Result<QbittorrentClient> {
    let host_port = resolve_service_host_port(compose, service, QBITTORRENT_WEBUI_PORT, timeout)
        .await
        .with_context(|| format!("failed to resolve {service} WebUI host port"))?;

    tracing::info!("{client_label} WebUI host port: {host_port}");

    let client = QbittorrentClient::new(&format!("http://127.0.0.1:{host_port}"), timeout)
        .with_context(|| format!("failed to create qBittorrent client for service '{service}'"))?;

    let _password = wait_for_qbittorrent_login(&client, compose, service, timeout)
        .await
        .with_context(|| format!("{service} qBittorrent API did not become ready for authentication"))?;

    Ok(client)
}

async fn upload_torrent_to_clients(
    seeder: &QbittorrentClient,
    leecher: &QbittorrentClient,
    torrent_bytes: &[u8],
) -> anyhow::Result<()> {
    upload_torrent_to_client(seeder, torrent_bytes, "seeder").await?;
    upload_torrent_to_client(leecher, torrent_bytes, "leecher").await?;

    tracing::info!("Torrent file uploaded to both qBittorrent clients");

    Ok(())
}

async fn upload_torrent_to_client(client: &QbittorrentClient, torrent_bytes: &[u8], client_label: &str) -> anyhow::Result<()> {
    client
        .add_torrent(TORRENT_FILE_NAME, torrent_bytes.to_vec(), "/downloads")
        .await
        .with_context(|| format!("failed to upload torrent to {client_label} qBittorrent instance"))?;

    Ok(())
}

/// Polls both clients until each has at least one torrent, then logs the final counts.
///
/// qBittorrent processes `add_torrent` asynchronously, so an immediate `list_torrents`
/// after upload would race and return 0. This function retries every 500 ms until both
/// clients report ≥ 1 torrent or the timeout expires.
async fn wait_for_torrent_counts(
    seeder: &QbittorrentClient,
    leecher: &QbittorrentClient,
    timeout: Duration,
) -> anyhow::Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    let poll_interval = Duration::from_millis(500);

    loop {
        let seeder_count = wait_for_torrent_count(seeder, "seeder").await?;
        let leecher_count = wait_for_torrent_count(leecher, "leecher").await?;

        tracing::info!("Seeder has {seeder_count} torrent(s), leecher has {leecher_count} torrent(s)");

        if seeder_count >= 1 && leecher_count >= 1 {
            tracing::info!("Both clients have at least one torrent — upload confirmed");
            return Ok(());
        }

        if std::time::Instant::now() >= deadline {
            anyhow::bail!("timed out waiting for torrents: seeder has {seeder_count}, leecher has {leecher_count}");
        }

        sleep(poll_interval).await;
    }
}

async fn wait_for_torrent_count(client: &QbittorrentClient, client_label: &str) -> anyhow::Result<usize> {
    Ok(client
        .list_torrents()
        .await
        .with_context(|| format!("failed to list {client_label} torrents"))?
        .len())
}

/// Polls the leecher until its torrent reaches 100% progress.
///
/// qBittorrent downloads asynchronously. This function retries every 500 ms until the
/// first torrent on the leecher reports `progress >= 1.0`, indicating a full download.
async fn wait_for_leecher_completion(leecher: &QbittorrentClient, timeout: Duration) -> anyhow::Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    let poll_interval = Duration::from_millis(500);

    loop {
        let torrents = leecher
            .list_torrents()
            .await
            .context("failed to list leecher torrents while polling for completion")?;

        if let Some(torrent) = torrents.first() {
            tracing::info!(
                "Leecher torrent progress: {:.1}% (state: {})",
                torrent.progress * 100.0,
                torrent.state
            );

            if torrent.progress >= 1.0 {
                tracing::info!("Leecher torrent download complete (100%)");
                return Ok(());
            }
        }

        if std::time::Instant::now() >= deadline {
            anyhow::bail!("timed out waiting for leecher to complete download");
        }

        sleep(poll_interval).await;
    }
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
        let original_hash: String = Sha1::digest(original_payload).iter().fold(String::new(), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        });
        let downloaded_hash: String = Sha1::digest(&downloaded_bytes).iter().fold(String::new(), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        });
        anyhow::bail!("payload content mismatch: original SHA1 {original_hash}, downloaded SHA1 {downloaded_hash}");
    }

    let hash: String = Sha1::digest(original_payload).iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    });
    tracing::info!(
        "Payload integrity verified: SHA1 {} ({} bytes match)",
        hash,
        original_payload.len()
    );

    Ok(())
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
        "[BitTorrent]\nSession\\AddTorrentStopped=false\nSession\\DefaultSavePath=/downloads\nSession\\TempPath=/downloads/temp\n[Preferences]\nWebUI\\LocalHostAuth=false\nWebUI\\Port={QBITTORRENT_WEBUI_PORT}\nWebUI\\Password_PBKDF2=\"{password_hash}\"\nWebUI\\Username={username}\n"
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

async fn wait_for_qbittorrent_login(
    client: &QbittorrentClient,
    compose: &DockerCompose,
    service: &str,
    timeout: Duration,
) -> anyhow::Result<String> {
    let start = std::time::Instant::now();
    let poll_interval = Duration::from_secs(1);
    let log_poll_interval = Duration::from_secs(5);
    let mut last_log_check: Option<std::time::Instant> = None;
    let mut last_error = String::from("qBittorrent WebUI did not accept known credentials yet");
    let mut candidate_passwords = vec![QBITTORRENT_PASSWORD.to_string(), QBITTORRENT_FALLBACK_PASSWORD.to_string()];

    while start.elapsed() < timeout {
        let should_refresh_logs =
            candidate_passwords.len() <= 2 && last_log_check.map_or(true, |last_check| last_check.elapsed() >= log_poll_interval);
        if should_refresh_logs {
            last_log_check = Some(std::time::Instant::now());

            if let Ok(logs) = compose.logs(&[service]) {
                if let Some(password) = extract_temporary_webui_password(&logs) {
                    let is_known_password = candidate_passwords.iter().any(|candidate| candidate == &password);
                    if !is_known_password {
                        candidate_passwords.push(password);
                    }
                }
            }
        }

        for candidate_password in &candidate_passwords {
            match client.login(QBITTORRENT_USERNAME, candidate_password).await {
                Ok(()) => return Ok(candidate_password.clone()),
                Err(error) => {
                    last_error = error.to_string();
                }
            }
        }

        tracing::info!("Waiting for qBittorrent WebUI authentication: {last_error}");

        sleep(poll_interval).await;
    }

    Err(anyhow::anyhow!(
        "timed out waiting for qBittorrent WebUI authentication readiness. Last error: {last_error}"
    ))
}

fn extract_temporary_webui_password(logs: &str) -> Option<String> {
    const PREFIX: &str = "A temporary password is provided for this session:";

    logs.lines()
        .rev()
        .find_map(|line| line.split_once(PREFIX).map(|(_, password)| password.trim().to_string()))
        .filter(|password| !password.is_empty())
}

async fn resolve_service_host_port(
    compose: &DockerCompose,
    service: &str,
    container_port: u16,
    timeout: Duration,
) -> anyhow::Result<u16> {
    let start = std::time::Instant::now();
    let poll_interval = Duration::from_secs(1);
    let mut last_error: Option<std::io::Error> = None;

    while start.elapsed() < timeout {
        if let Ok(ps_output) = compose.ps() {
            if compose_service_has_exited(&ps_output, service) {
                let logs_output = compose
                    .logs(&[service])
                    .unwrap_or_else(|error| format!("failed to collect compose logs output: {error}"));

                return Err(anyhow::anyhow!(
                    "compose service '{service}' exited while waiting for port mapping '{container_port}'.\nCompose ps:\n{ps_output}\nCompose logs:\n{logs_output}"
                ));
            }
        }

        match compose.port(service, container_port) {
            Ok(host_port) => return Ok(host_port),
            Err(error) => {
                last_error = Some(error);
                tracing::info!("Waiting for compose port mapping for service '{service}'");
                sleep(poll_interval).await;
            }
        }
    }

    let ps_output = compose
        .ps()
        .unwrap_or_else(|error| format!("failed to collect compose ps output: {error}"));
    let logs_output = compose
        .logs(&[service, "tracker"])
        .unwrap_or_else(|error| format!("failed to collect compose logs output: {error}"));

    Err(anyhow::anyhow!(
        "timed out waiting for compose port mapping for service '{}' and port '{}'. Last error: {}\nCompose ps:\n{}\nCompose logs:\n{}",
        service,
        container_port,
        last_error.as_ref().map_or_else(
            || "no port error captured".to_string(),
            std::string::ToString::to_string,
        ),
        ps_output,
        logs_output
    ))
}

fn compose_service_has_exited(ps_output: &str, service: &str) -> bool {
    ps_output.lines().any(|line| {
        line.contains(service)
            && (line.contains("exited") || line.contains("dead") || line.contains("created") || line.contains("removing"))
    })
}

fn build_payload_bytes(length: usize) -> Vec<u8> {
    let pattern = (0_u8..=250_u8).collect::<Vec<_>>();

    (0..length).map(|index| pattern[index % pattern.len()]).collect()
}

fn build_torrent_bytes(payload_bytes: &[u8], payload_name: &str, announce_url: &str) -> anyhow::Result<Vec<u8>> {
    let pieces = payload_bytes
        .chunks(TORRENT_PIECE_LENGTH)
        .map(|piece| Sha1::digest(piece).to_vec())
        .collect::<Vec<_>>()
        .concat();

    let info = BencodeValue::Dictionary(vec![
        (b"length".to_vec(), BencodeValue::Integer(i64::try_from(payload_bytes.len())?)),
        (b"name".to_vec(), BencodeValue::Bytes(payload_name.as_bytes().to_vec())),
        (
            b"piece length".to_vec(),
            BencodeValue::Integer(i64::try_from(TORRENT_PIECE_LENGTH)?),
        ),
        (b"pieces".to_vec(), BencodeValue::Bytes(pieces)),
    ]);

    let info_bytes = info.encode();
    let torrent = BencodeValue::Dictionary(vec![
        (b"announce".to_vec(), BencodeValue::Bytes(announce_url.as_bytes().to_vec())),
        (b"created by".to_vec(), BencodeValue::Bytes(b"torrust-qb-e2e".to_vec())),
        (b"creation date".to_vec(), BencodeValue::Integer(0)),
        (b"info".to_vec(), BencodeValue::Raw(info_bytes)),
    ]);

    Ok(torrent.encode())
}
