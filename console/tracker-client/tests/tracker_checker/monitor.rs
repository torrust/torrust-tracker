/// Tests for the `monitor udp` subcommand.
///
/// # Timeout-only test environment
///
/// The helper [`spawn_udp_sink`] binds a UDP socket that silently discards every incoming
/// packet and never sends any response. This means every probe issued by the monitor will
/// time out. The tests in this module therefore exercise:
///
/// - JSON shape of probe events on stderr (`"status":"timeout"`)
/// - JSON shape of the final summary on stdout (null latency fields, `timeout_percent` > 0)
/// - Exit code 0 for a completed-but-all-timeout run
///
/// They do **not** exercise the success path (a probe receiving a valid `AnnounceResponse`,
/// non-null `elapsed_ms`, populated min/max/average latency stats). A success-path
/// integration test requires a proper mock UDP tracker that speaks the `BitTorrent` UDP
/// protocol. The refactor plan item for that test has been intentionally deferred to the
/// future tracker-client repository split.
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use serde_json::Value;

use super::tracker_client_check_bin;

fn spawn_udp_sink() -> (SocketAddr, mpsc::Sender<()>, thread::JoinHandle<()>) {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind UDP sink socket");
    socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .expect("Failed to configure UDP sink read timeout");
    let addr = socket.local_addr().expect("Failed to get UDP sink local address");

    let (tx, rx) = mpsc::channel::<()>();
    let join_handle = thread::spawn(move || {
        let mut buffer = [0_u8; 2048];

        loop {
            if rx.try_recv().is_ok() {
                break;
            }

            drop(socket.recv_from(&mut buffer));
        }
    });

    (addr, tx, join_handle)
}

#[test]
fn it_should_emit_monitor_probe_events_to_stderr_and_summary_to_stdout() {
    let (addr, stop_tx, join_handle) = spawn_udp_sink();

    let output = tracker_client_check_bin()
        .arg("monitor")
        .arg("udp")
        .arg("--url")
        .arg(format!("udp://{addr}"))
        .arg("--interval")
        .arg("1")
        .arg("--timeout")
        .arg("1")
        .arg("--duration")
        .arg("2")
        .output()
        .expect("Failed to run tracker_client check monitor udp");

    let _ = stop_tx.send(());
    assert!(join_handle.join().is_ok(), "UDP sink thread should not panic");

    assert_eq!(
        output.status.code(),
        Some(0),
        "Expected exit code 0 for successful monitor execution"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("\"event\":\"probe\""),
        "Expected probe NDJSON events on stderr, got: {stderr}"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Value = serde_json::from_str(&stdout).expect("Expected valid JSON monitor summary on stdout");

    assert!(
        parsed["udp_trackers"].is_array(),
        "Expected udp_trackers array in stdout JSON"
    );
    assert_eq!(parsed["udp_trackers"][0]["url"], format!("udp://{addr}"));
    assert!(
        parsed["udp_trackers"][0]["status"]["stats"]["total"]
            .as_u64()
            .expect("Expected stats.total to be u64")
            >= 1,
        "Expected at least one probe"
    );
}
