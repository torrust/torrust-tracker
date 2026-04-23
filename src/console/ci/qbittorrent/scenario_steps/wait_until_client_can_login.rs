use std::time::{Duration, Instant};

use super::super::poller::Poller;
use super::super::qbittorrent_client::QbittorrentClient;
use crate::console::ci::compose::DockerCompose;

/// Authentication and polling settings for client login readiness.
pub(in super::super) struct LoginReadinessSettings<'a> {
    pub(in super::super) username: &'a str,
    pub(in super::super) preferred_password: &'a str,
    pub(in super::super) fallback_password: &'a str,
    pub(in super::super) timeout: Duration,
    pub(in super::super) login_poll_interval: Duration,
    pub(in super::super) log_poll_interval: Duration,
}

struct LoginCandidates {
    passwords: Vec<String>,
    last_log_check: Option<Instant>,
    log_poll_interval: Duration,
}

impl LoginCandidates {
    fn new(passwords: Vec<String>, log_poll_interval: Duration) -> Self {
        Self {
            passwords,
            last_log_check: None,
            log_poll_interval,
        }
    }

    fn should_refresh_logs(&self) -> bool {
        self.passwords.len() <= 2
            && self
                .last_log_check
                .map_or(true, |last_check| last_check.elapsed() >= self.log_poll_interval)
    }

    fn mark_logs_checked(&mut self) {
        self.last_log_check = Some(Instant::now());
    }

    fn add_if_new(&mut self, password: String) {
        if self.passwords.iter().all(|candidate| candidate != &password) {
            self.passwords.push(password);
        }
    }

    fn iter(&self) -> impl Iterator<Item = &str> {
        self.passwords.iter().map(String::as_str)
    }
}

/// Waits until a qBittorrent client accepts login credentials.
///
/// This step polls authentication with known password candidates and augments them with temporary
/// credentials discovered in container logs.
///
/// # Errors
///
/// Returns an error when authentication never succeeds before timeout.
pub(in super::super) async fn wait_until_client_can_login(
    client: &QbittorrentClient,
    compose: &DockerCompose,
    service_name: &str,
    settings: &LoginReadinessSettings<'_>,
) -> anyhow::Result<String> {
    let poller = Poller::new(settings.timeout, settings.login_poll_interval);
    let mut candidates = LoginCandidates::new(
        vec![
            settings.preferred_password.to_string(),
            settings.fallback_password.to_string(),
        ],
        settings.log_poll_interval,
    );
    let mut last_error = String::from("qBittorrent WebUI did not accept known credentials yet");

    loop {
        if candidates.should_refresh_logs() {
            candidates.mark_logs_checked();

            if let Ok(logs) = compose.logs(&[service_name]) {
                if let Some(password) = extract_temporary_webui_password(&logs) {
                    candidates.add_if_new(password);
                }
            }
        }

        for candidate_password in candidates.iter() {
            match client.login(settings.username, candidate_password).await {
                Ok(()) => return Ok(candidate_password.to_string()),
                Err(error) => {
                    last_error = error.to_string();
                }
            }
        }

        tracing::info!("Waiting for qBittorrent WebUI authentication: {last_error}");

        poller
            .retry_or_timeout(|| {
                format!("timed out waiting for qBittorrent WebUI authentication readiness. Last error: {last_error}")
            })
            .await?;
    }
}

fn extract_temporary_webui_password(logs: &str) -> Option<String> {
    const PREFIX: &str = "A temporary password is provided for this session:";

    logs.lines()
        .rev()
        .find_map(|line| line.split_once(PREFIX).map(|(_, password)| password.trim().to_string()))
        .filter(|password| !password.is_empty())
}
