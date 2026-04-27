/// Credentials for authenticating with the `qBittorrent` web UI.
#[derive(Debug, Clone)]
pub(crate) struct QbittorrentCredentials {
    /// Web-UI username.
    pub(crate) username: String,
    /// Web-UI password.
    pub(crate) password: String,
}
