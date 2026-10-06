//! Tests for the `monitor udp` subcommand against fake UDP trackers.
//!
//! An answering tracker exercises the success path (`ok` probes with latency
//! stats); a silent one exercises the timeout path. Both check the output
//! channels: one NDJSON probe event per probe on stderr, one JSON summary on
//! stdout, and exit code 0.
use serde_json::Value;

use super::tracker_client_check_bin;
use crate::common::fake_trackers::FakeUdpTracker;

/// What one `monitor udp` run produced, split by output channel.
struct MonitorRun {
    exit_code: Option<i32>,
    probes: Vec<Value>,
    summary: Value,
}

impl MonitorRun {
    /// Monitors `tracker` for two seconds, probing every second.
    fn of(tracker: &FakeUdpTracker) -> Self {
        let output = tracker_client_check_bin()
            .args(["monitor", "udp", "--url", &format!("udp://{}", tracker.address())])
            .args(["--interval", "1", "--timeout", "1", "--duration", "2"])
            .output()
            .expect("tracker_client check monitor udp should run");

        let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
        let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
        let result: Value =
            serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("stdout should be one JSON summary: {stdout}"));

        Self {
            exit_code: output.status.code(),
            probes: stderr
                .lines()
                .map(|line| serde_json::from_str(line).unwrap_or_else(|_| panic!("stderr should be NDJSON: {line}")))
                .collect(),
            summary: result["udp_trackers"][0].clone(),
        }
    }

    fn stats(&self) -> &Value {
        &self.summary["status"]["stats"]
    }

    fn probe_statuses(&self) -> Vec<&str> {
        self.probes
            .iter()
            .map(|probe| {
                assert_eq!(probe["event"], "probe", "unexpected stderr record: {probe}");
                probe["status"].as_str().expect("probe status")
            })
            .collect()
    }
}

#[test]
fn it_should_report_ok_probes_with_latency_stats_when_the_tracker_answers() {
    // Arrange
    let tracker = FakeUdpTracker::start();

    // Act
    let run = MonitorRun::of(&tracker);

    // Assert
    assert_eq!(run.exit_code, Some(0));

    let statuses = run.probe_statuses();
    assert!(!statuses.is_empty(), "expected at least one probe");
    assert!(statuses.iter().all(|status| *status == "ok"), "probe statuses: {statuses:?}");
    assert!(
        run.probes.iter().all(|probe| probe["elapsed_ms"].is_u64()),
        "probes: {:?}",
        run.probes
    );

    assert_eq!(run.stats()["total"], statuses.len());
    assert_eq!(run.stats()["timeouts"], 0);
    for latency in ["min_ms", "max_ms", "average_ms", "last_ms"] {
        assert!(
            run.stats()[latency].is_u64(),
            "{latency} should be populated: {}",
            run.stats()
        );
    }
}

#[test]
fn it_should_report_timeout_probes_without_latency_stats_when_the_tracker_never_answers() {
    // Arrange
    let tracker = FakeUdpTracker::start_silent();

    // Act
    let run = MonitorRun::of(&tracker);

    // Assert
    assert_eq!(run.exit_code, Some(0));
    assert_eq!(run.summary["url"], format!("udp://{}", tracker.address()));

    let statuses = run.probe_statuses();
    assert!(!statuses.is_empty(), "expected at least one probe");
    assert!(
        statuses.iter().all(|status| *status == "timeout"),
        "probe statuses: {statuses:?}"
    );

    assert_eq!(run.stats()["total"], statuses.len());
    assert_eq!(run.stats()["timeout_percent"], 100);
    for latency in ["min_ms", "max_ms", "average_ms", "last_ms"] {
        assert!(run.stats()[latency].is_null(), "{latency} should be null: {}", run.stats());
    }
}
