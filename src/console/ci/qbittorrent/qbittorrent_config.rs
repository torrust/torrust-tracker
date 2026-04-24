//! Builder for the qBittorrent configuration file written into the E2E workspace.
use std::fs;
use std::path::Path;

use anyhow::Context;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha512;

const CONFIG_RELATIVE_PATH: &str = "qBittorrent/qBittorrent.conf";
const DEFAULT_WEBUI_PORT: u16 = 8080;
const DEFAULT_DOWNLOADS_PATH: &str = "/downloads";
const DEFAULT_DOWNLOADS_TEMP_PATH: &str = "/downloads/temp";

/// Builds and writes the qBittorrent configuration file for the E2E workspace.
///
/// Provides a fluent interface to configure credentials and paths. Call
/// [`write_to`](QbittorrentConfigBuilder::write_to) to create the required
/// directory layout and write `qBittorrent/qBittorrent.conf`.
pub(super) struct QbittorrentConfigBuilder<'a> {
    username: &'a str,
    password: &'a str,
    webui_port: u16,
    downloads_path: &'a str,
    downloads_temp_path: &'a str,
}

impl<'a> QbittorrentConfigBuilder<'a> {
    /// Creates a builder with default port (`8080`) and download paths (`/downloads`).
    pub(super) fn new(username: &'a str, password: &'a str) -> Self {
        Self {
            username,
            password,
            webui_port: DEFAULT_WEBUI_PORT,
            downloads_path: DEFAULT_DOWNLOADS_PATH,
            downloads_temp_path: DEFAULT_DOWNLOADS_TEMP_PATH,
        }
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(super) fn webui_port(mut self, port: u16) -> Self {
        self.webui_port = port;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(super) fn downloads_path(mut self, path: &'a str) -> Self {
        self.downloads_path = path;
        self
    }

    #[expect(dead_code, reason = "reserved for future scenario configuration")]
    pub(super) fn downloads_temp_path(mut self, path: &'a str) -> Self {
        self.downloads_temp_path = path;
        self
    }

    /// Writes the qBittorrent configuration to `config_root`.
    ///
    /// Creates the required directory layout under `config_root` and writes
    /// `qBittorrent/qBittorrent.conf` with the supplied credentials and paths.
    ///
    /// # Errors
    ///
    /// Returns an error when creating directories or writing the config file fails.
    pub(super) fn write_to(&self, config_root: &Path) -> anyhow::Result<()> {
        let config_path = config_root.join(CONFIG_RELATIVE_PATH);
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

        let password_hash = build_password_hash(self.password);
        let config = self.format_config(&password_hash);

        fs::write(&config_path, config)
            .with_context(|| format!("failed to write qBittorrent config '{}'", config_path.display()))?;

        Ok(())
    }

    fn format_config(&self, password_hash: &str) -> String {
        let username = self.username;
        let webui_port = self.webui_port;
        let downloads_path = self.downloads_path;
        let downloads_temp_path = self.downloads_temp_path;

        format!(
            "[BitTorrent]\n\
             Session\\AddTorrentStopped=false\n\
             Session\\DefaultSavePath={downloads_path}\n\
             Session\\TempPath={downloads_temp_path}\n\
             \n\
             [Preferences]\n\
             WebUI\\LocalHostAuth=false\n\
             WebUI\\Port={webui_port}\n\
             WebUI\\Password_PBKDF2=\"{password_hash}\"\n\
             WebUI\\Username={username}\n"
        )
    }
}

fn build_password_hash(password: &str) -> String {
    let salt: [u8; 16] = rand::random();
    let mut digest = [0_u8; 64];
    pbkdf2_hmac::<Sha512>(password.as_bytes(), &salt, 100_000, &mut digest);

    format!(
        "@ByteArray({}:{})",
        BASE64_STANDARD.encode(salt),
        BASE64_STANDARD.encode(digest)
    )
}
