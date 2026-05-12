use std::net::SocketAddr;
use std::time::{Duration, Instant};

use bittorrent_primitives::info_hash::InfoHash as TorrustInfoHash;
use bittorrent_tracker_client::udp;
use bittorrent_udp_tracker_protocol::TransactionId;
use reqwest::Url;
use serde::Serialize;

use crate::console::clients::udp::checker::{AnnounceParams, Client};
use crate::console::clients::udp::Error as UdpError;

pub const DEFAULT_INFO_HASH: &str = "9c38422213e30bff212b30c360d26f9a02136422"; // DevSkim: ignore DS173237

#[derive(Debug, Clone)]
pub struct MonitorUdpConfig {
    pub url: Url,
    pub interval: Duration,
    pub timeout: Duration,
    pub duration: Duration,
    pub info_hash: TorrustInfoHash,
}

#[derive(Debug, Clone, Default)]
struct Stats {
    total: u64,
    timeouts: u64,
    successes: u64,
    min_ms: Option<u64>,
    max_ms: Option<u64>,
    sum_ms: u64,
    last_ms: Option<u64>,
}

impl Stats {
    fn record_success(&mut self, elapsed_ms: u64) {
        self.total += 1;
        self.successes += 1;
        self.sum_ms += elapsed_ms;
        self.min_ms = Some(self.min_ms.map_or(elapsed_ms, |current| current.min(elapsed_ms)));
        self.max_ms = Some(self.max_ms.map_or(elapsed_ms, |current| current.max(elapsed_ms)));
        self.last_ms = Some(elapsed_ms);
    }

    fn record_timeout(&mut self) {
        self.total += 1;
        self.timeouts += 1;
        self.last_ms = None;
    }

    fn record_error(&mut self) {
        self.total += 1;
        self.last_ms = None;
    }

    fn average_ms(&self) -> Option<u64> {
        self.sum_ms.checked_div(self.successes)
    }

    /// Returns the percentage of probes that timed out, rounded down to the nearest integer.
    ///
    /// The denominator is `total = successes + timeouts + errors`. Error probes (those that
    /// fail for reasons other than a network timeout) count toward `total` without being
    /// counted as timeouts, so they reduce `timeout_percent` without being successes. For
    /// example, three probes where one succeeds, one times out, and one errors gives
    /// `timeout_percent = 1 × 100 / 3 = 33`, not `50`.
    fn timeout_percent(&self) -> u64 {
        self.timeouts.saturating_mul(100).checked_div(self.total).unwrap_or(0)
    }
}

#[derive(Serialize)]
struct ProbeEvent {
    event: &'static str,
    sequence: u64,
    url: String,
    status: &'static str,
    elapsed_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Serialize)]
struct MonitorResult {
    udp_trackers: Vec<UdpTrackerResult>,
}

#[derive(Serialize)]
struct UdpTrackerResult {
    url: String,
    status: MonitorStatus,
}

#[derive(Serialize)]
struct MonitorStatus {
    code: &'static str,
    message: String,
    stats: MonitorStats,
}

#[derive(Serialize)]
struct MonitorStats {
    total: u64,
    timeouts: u64,
    timeout_percent: u64,
    min_ms: Option<u64>,
    max_ms: Option<u64>,
    average_ms: Option<u64>,
    last_ms: Option<u64>,
}

enum ProbeOutcome {
    Ok,
    Timeout,
    Error { message: String },
}

/// # Errors
///
/// Returns an error if URL resolution or JSON serialization fails.
pub async fn run_monitor(config: MonitorUdpConfig) -> Result<(), String> {
    let started_at = Instant::now();
    let url = config.url.to_string();
    let mut interrupted = false;
    let mut stats = Stats::default();
    let mut sequence: u64 = 0;

    loop {
        // Exit before starting a new probe if the time budget is already exhausted.
        if started_at.elapsed() >= config.duration {
            break;
        }

        sequence += 1;

        let probe_started = Instant::now();
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                interrupted = true;
                break;
            }
            probe_result = run_probe(&config) => {
                // `as_millis()` returns u128; overflow into u64 would require a single probe
                // to run for over 584 million years, which cannot happen in practice.
                // `u64::MAX` is therefore an unreachable sentinel.
                let elapsed_ms = u64::try_from(probe_started.elapsed().as_millis()).unwrap_or(u64::MAX);

                match probe_result {
                    ProbeOutcome::Ok => {
                        stats.record_success(elapsed_ms);
                        emit_probe_event(&ProbeEvent {
                            event: "probe",
                            sequence,
                            url: url.clone(),
                            status: "ok",
                            elapsed_ms: Some(elapsed_ms),
                            message: None,
                        })?;
                    }
                    ProbeOutcome::Timeout => {
                        stats.record_timeout();
                        emit_probe_event(&ProbeEvent {
                            event: "probe",
                            sequence,
                            url: url.clone(),
                            status: "timeout",
                            elapsed_ms: None,
                            message: None,
                        })?;
                    }
                    ProbeOutcome::Error { message } => {
                        stats.record_error();
                        emit_probe_event(&ProbeEvent {
                            event: "probe",
                            sequence,
                            url: url.clone(),
                            status: "error",
                            elapsed_ms: None,
                            message: Some(message),
                        })?;
                    }
                }
            }
        }

        // Exit before sleeping if the duration elapsed during the probe itself,
        // so we never sleep after the last probe.
        if started_at.elapsed() >= config.duration {
            break;
        }

        let remaining = config.duration.saturating_sub(started_at.elapsed());
        let sleep_duration = config.interval.min(remaining);

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                interrupted = true;
                break;
            }
            () = tokio::time::sleep(sleep_duration) => {}
        }
    }

    let message = if interrupted {
        "monitor interrupted"
    } else {
        "monitor completed"
    };

    let output = MonitorResult {
        udp_trackers: vec![UdpTrackerResult {
            url,
            status: MonitorStatus {
                code: "ok",
                message: message.to_string(),
                stats: MonitorStats {
                    total: stats.total,
                    timeouts: stats.timeouts,
                    timeout_percent: stats.timeout_percent(),
                    min_ms: stats.min_ms,
                    max_ms: stats.max_ms,
                    average_ms: stats.average_ms(),
                    last_ms: stats.last_ms,
                },
            },
        }],
    };

    let final_json = serde_json::to_string(&output).map_err(|e| format!("final JSON serialization failed: {e}"))?;
    println!("{final_json}");

    Ok(())
}

fn emit_probe_event(event: &ProbeEvent) -> Result<(), String> {
    let json = serde_json::to_string(event).map_err(|e| format!("probe JSON serialization failed: {e}"))?;
    eprintln!("{json}");
    Ok(())
}

async fn run_probe(config: &MonitorUdpConfig) -> ProbeOutcome {
    let remote_addr = match resolve_socket_addr(&config.url) {
        Ok(remote_addr) => remote_addr,
        Err(message) => return ProbeOutcome::Error { message },
    };

    let client = match Client::new(remote_addr, config.timeout).await {
        Ok(client) => client,
        Err(err) => {
            if is_timeout_error(&err) {
                return ProbeOutcome::Timeout;
            }
            return ProbeOutcome::Error {
                message: err.to_string(),
            };
        }
    };

    let transaction_id = TransactionId::new(1);

    let connection_id = match client.send_connection_request(transaction_id).await {
        Ok(connection_id) => connection_id,
        Err(err) => {
            if is_timeout_error(&err) {
                return ProbeOutcome::Timeout;
            }
            return ProbeOutcome::Error {
                message: err.to_string(),
            };
        }
    };

    match client
        .send_announce_request(transaction_id, connection_id, config.info_hash, &AnnounceParams::default())
        .await
    {
        Ok(_response) => ProbeOutcome::Ok,
        Err(err) => {
            if is_timeout_error(&err) {
                ProbeOutcome::Timeout
            } else {
                ProbeOutcome::Error {
                    message: err.to_string(),
                }
            }
        }
    }
}

fn resolve_socket_addr(url: &Url) -> Result<SocketAddr, String> {
    let socket_addrs = url
        .socket_addrs(|| None)
        .map_err(|e| format!("failed to resolve tracker URL `{url}`: {e}"))?;

    socket_addrs
        .first()
        .copied()
        .ok_or_else(|| format!("no socket addresses resolved for tracker URL `{url}`"))
}

fn is_timeout_udp_client_error(err: &udp::Error) -> bool {
    matches!(
        err,
        udp::Error::TimeoutWhileBindingToSocket { .. }
            | udp::Error::TimeoutWhileConnectingToRemote { .. }
            | udp::Error::TimeoutWaitForWriteableSocket
            | udp::Error::TimeoutWhileSendingData { .. }
            | udp::Error::TimeoutWaitForReadableSocket
            | udp::Error::TimeoutWhileReceivingData
    )
}

fn is_timeout_error(err: &UdpError) -> bool {
    match err {
        UdpError::UnableToBindAndConnect { err, .. }
        | UdpError::UnableToSendConnectionRequest { err }
        | UdpError::UnableToReceiveConnectResponse { err }
        | UdpError::UnableToSendAnnounceRequest { err }
        | UdpError::UnableToReceiveAnnounceResponse { err }
        | UdpError::UnableToSendScrapeRequest { err }
        | UdpError::UnableToReceiveScrapeResponse { err }
        | UdpError::UnableToReceiveResponse { err }
        | UdpError::UnableToGetLocalAddr { err } => is_timeout_udp_client_error(err),
        UdpError::UnexpectedConnectionResponse { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::Stats;

    #[test]
    fn it_should_return_none_average_when_there_are_no_successful_probes() {
        let mut stats = Stats::default();
        stats.record_timeout();

        assert_eq!(stats.average_ms(), None);
    }

    #[test]
    fn it_should_compute_integer_average_for_successful_probes() {
        let mut stats = Stats::default();
        stats.record_success(100);
        stats.record_success(101);

        assert_eq!(stats.average_ms(), Some(100));
    }

    #[test]
    fn it_should_compute_timeout_percent_as_integer() {
        let mut stats = Stats::default();
        stats.record_success(100);
        stats.record_timeout();
        stats.record_timeout();

        assert_eq!(stats.timeout_percent(), 66);
    }

    #[test]
    fn it_should_return_all_null_latency_fields_when_every_probe_times_out() {
        let mut stats = Stats::default();
        stats.record_timeout();
        stats.record_timeout();
        stats.record_timeout();

        assert_eq!(stats.min_ms, None);
        assert_eq!(stats.max_ms, None);
        assert_eq!(stats.average_ms(), None);
        assert_eq!(stats.last_ms, None);
    }
}
