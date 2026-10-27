//! Unix executable-boundary tests for the shutdown policy of the tracker binary.
//!
//! They start the real binary with configured shutdown budgets, signal its exact PID, and check
//! the exit status and the shutdown output.

#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

#[cfg(unix)]
#[path = "../common/native_tracker/mod.rs"]
mod native_tracker;

#[cfg(unix)]
use std::io::Write as _;
#[cfg(unix)]
use std::net::TcpStream;

#[cfg(unix)]
use native_tracker::configuration::{NativeTrackerConfigurationSources, NativeTrackerShutdownBudgets};
#[cfg(unix)]
use native_tracker::{NativeTracker, NativeTrackerExit};
#[cfg(unix)]
use nix::sys::signal::{Signal, kill};
#[cfg(unix)]
use nix::unistd::Pid;

#[cfg(unix)]
const SUCCESS_MESSAGE: &str = "Torrust tracker successfully shutdown.";
#[cfg(unix)]
const INCOMPLETE_MESSAGE: &str = "Torrust tracker shutdown was incomplete";

/// The budgets of the "drain budget reached" scenario. The longest drain, 2 s, ends well before the
/// 5 s header-read timeout of a held connection, which in turn ends before the 10 s deadline, so a
/// tracker that ignored the configured drain would stop cleanly and a tracker that waited for the
/// deadline would still be running. `max(2, 1) + 1 = 3` is below the deadline.
#[cfg(unix)]
const BUDGETS_THAT_END_A_HELD_CONNECTION: NativeTrackerShutdownBudgets = NativeTrackerShutdownBudgets {
    deadline_in_secs: 10,
    http_drain_in_secs: 2,
    udp_request_drain_in_secs: 1,
    event_listener_drain_in_secs: 1,
};

#[cfg(unix)]
async fn ready_tracker(budgets: Option<NativeTrackerShutdownBudgets>) -> NativeTracker {
    let sources = NativeTrackerConfigurationSources::with_cli_health_check_port(0);
    let sources = budgets.map_or(sources, |budgets| sources.with_shutdown_budgets(budgets));
    let mut tracker = NativeTracker::start_with_configuration_sources(sources);
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should report health Status::Ok before the signal");
    tracker
}

#[cfg(unix)]
fn send_signal(tracker: &NativeTracker, signal: Signal) {
    let pid = tracker.pid().expect("running tracker child should have a PID");
    kill(Pid::from_raw(i32::try_from(pid).expect("child PID should fit i32")), signal)
        .expect("deliver the signal to the exact tracker child PID");
}

/// Opens a connection to the HTTP tracker and sends only part of a request header, so the server keeps
/// waiting for the rest. A second request that completes proves the server accepted the first connection.
#[cfg(unix)]
async fn hold_partial_request_open(tracker: &NativeTracker) -> TcpStream {
    let address = tracker
        .http_tracker_address()
        .await
        .expect("ready tracker should have logged its HTTP address");
    let mut held_connection = TcpStream::connect(address).expect("open the connection to hold");
    held_connection
        .write_all(b"GET /health_check HTTP/1.1\r\nHost: tracker\r\n")
        .expect("send a partial request header");
    reqwest::get(format!("http://{address}/health_check")) // DevSkim: ignore DS137138
        .await
        .expect("the HTTP tracker should answer once it has accepted the held connection");
    held_connection
}

#[cfg(unix)]
fn assert_exit_code(exit: &NativeTrackerExit, expected: i32) {
    assert_eq!(
        exit.status.code(),
        Some(expected),
        "expected exit code {expected}, got {}; tracker output:\n{}",
        exit.status,
        exit.output
    );
}

#[cfg(unix)]
async fn assert_clean_stop_with_default_deadline(signal: Signal) {
    // Arrange
    // A tracker with no shutdown settings has the defaults: a 25 s process deadline.
    let tracker = ready_tracker(None).await;

    // Act
    send_signal(&tracker, signal);
    let exit = tracker.wait_for_exit().await.expect("tracker should exit after the signal");

    // Assert
    assert_exit_code(&exit, 0);
    let output = exit.plain_output();
    assert!(output.contains(SUCCESS_MESSAGE), "tracker output:\n{output}");
    assert!(
        output.contains("timeout_seconds=25"),
        "the supervisor should wait under the default 25 s deadline; tracker output:\n{output}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_exit_with_code_zero_when_sigterm_stops_a_tracker_with_no_open_connections() {
    assert_clean_stop_with_default_deadline(Signal::SIGTERM).await;
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_exit_with_code_zero_when_sigint_stops_a_tracker_with_no_open_connections() {
    assert_clean_stop_with_default_deadline(Signal::SIGINT).await;
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_apply_the_configured_process_deadline_when_the_tracker_stops() {
    // Arrange
    // The configured deadline is 7 s; the default is 25 s. `max(2, 1) + 1 = 3` is below it.
    let tracker = ready_tracker(Some(NativeTrackerShutdownBudgets {
        deadline_in_secs: 7,
        ..BUDGETS_THAT_END_A_HELD_CONNECTION
    }))
    .await;

    // Act
    send_signal(&tracker, Signal::SIGTERM);
    let exit = tracker.wait_for_exit().await.expect("tracker should exit after SIGTERM");

    // Assert
    assert_exit_code(&exit, 0);
    let output = exit.plain_output();
    assert!(
        output.contains("timeout_seconds=7"),
        "the supervisor should wait under the configured 7 s deadline; tracker output:\n{output}"
    );
}

#[cfg(unix)]
async fn assert_failed_exit_when_a_held_connection_outlasts_the_http_drain(signal: Signal) {
    // Arrange
    // The HTTP drain budget is 2 s and the header-read timeout of the held connection is 5 s, so
    // without the configured budget the connection would end first and the stop would be clean.
    let tracker = ready_tracker(Some(BUDGETS_THAT_END_A_HELD_CONNECTION)).await;
    let held_connection = hold_partial_request_open(&tracker).await;

    // Act
    send_signal(&tracker, signal);
    let exit = tracker.wait_for_exit().await.expect("tracker should exit after the signal");
    drop(held_connection);

    // Assert
    assert_exit_code(&exit, 1);
    let output = exit.plain_output();
    assert!(
        output.contains(INCOMPLETE_MESSAGE),
        "the incomplete shutdown should be reported; tracker output:\n{output}"
    );
    assert!(
        !output.contains(SUCCESS_MESSAGE),
        "a failed shutdown must not be reported as successful; tracker output:\n{output}"
    );
    assert!(
        output.contains("timeout_seconds=10"),
        "the supervisor should wait under the configured 10 s deadline; tracker output:\n{output}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_exit_with_code_one_when_a_connection_held_open_outlasts_the_http_drain_after_sigterm() {
    assert_failed_exit_when_a_held_connection_outlasts_the_http_drain(Signal::SIGTERM).await;
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_exit_with_code_one_when_a_connection_held_open_outlasts_the_http_drain_after_sigint() {
    assert_failed_exit_when_a_held_connection_outlasts_the_http_drain(Signal::SIGINT).await;
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_exit_with_code_one_and_apply_the_configured_deadline_when_a_service_fails_to_start() {
    // Arrange
    // The health-check port is already taken, so that service fails to start after the HTTP tracker has
    // started, and the application stops the services it already started under the configured deadline.
    let occupied_port = std::net::TcpListener::bind("127.0.0.1:0").expect("reserve a port the tracker cannot bind");
    let port = occupied_port.local_addr().expect("read the reserved port").port();
    let tracker = NativeTracker::start_with_configuration_sources(
        NativeTrackerConfigurationSources::with_cli_health_check_port(port).with_shutdown_budgets(NativeTrackerShutdownBudgets {
            deadline_in_secs: 7,
            ..BUDGETS_THAT_END_A_HELD_CONNECTION
        }),
    );

    // Act
    let exit = tracker
        .wait_for_exit()
        .await
        .expect("a tracker whose health check cannot bind should exit by itself");
    drop(occupied_port);

    // Assert
    assert_exit_code(&exit, 1);
    let output = exit.plain_output();
    assert!(
        output.contains("timeout_seconds=7"),
        "the cleanup after the failed start should wait under the configured 7 s deadline; tracker output:\n{output}"
    );
}

#[cfg(unix)]
async fn assert_rejected_before_any_service_starts(budgets: NativeTrackerShutdownBudgets, expected_keys: &[&str]) {
    // Arrange
    let tracker = NativeTracker::start_with_configuration_sources(
        NativeTrackerConfigurationSources::with_cli_health_check_port(0).with_shutdown_budgets(budgets),
    );

    // Act
    let exit = tracker
        .wait_for_exit()
        .await
        .expect("a tracker with invalid shutdown budgets should exit by itself");

    // Assert
    assert_exit_code(&exit, 1);
    // An override from the environment names its key in upper case.
    let output = exit.plain_output();
    let lowercase_output = output.to_lowercase();
    for key in expected_keys {
        assert!(
            lowercase_output.contains(key),
            "the error should name `{key}`; tracker output:\n{output}"
        );
    }
    assert!(
        !output.contains("Started on:"),
        "no service should start with invalid shutdown budgets; tracker output:\n{output}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_reject_a_zero_process_deadline_before_any_service_starts() {
    assert_rejected_before_any_service_starts(
        NativeTrackerShutdownBudgets {
            deadline_in_secs: 0,
            http_drain_in_secs: 15,
            udp_request_drain_in_secs: 5,
            event_listener_drain_in_secs: 5,
        },
        &["deadline_in_secs"],
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_reject_budgets_whose_drains_do_not_fit_in_the_process_deadline_before_any_service_starts() {
    // 20 + 5 = 25 is not below the 25 s deadline.
    assert_rejected_before_any_service_starts(
        NativeTrackerShutdownBudgets {
            deadline_in_secs: 25,
            http_drain_in_secs: 20,
            udp_request_drain_in_secs: 5,
            event_listener_drain_in_secs: 5,
        },
        &["http_drain_in_secs", "event_listener_drain_in_secs", "deadline_in_secs"],
    )
    .await;
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_accept_budgets_whose_drains_leave_one_second_of_slack_in_the_process_deadline() {
    // Arrange
    // 19 + 5 = 24 is the largest HTTP drain below the 25 s deadline with a 5 s listener drain.
    let tracker = ready_tracker(Some(NativeTrackerShutdownBudgets {
        deadline_in_secs: 25,
        http_drain_in_secs: 19,
        udp_request_drain_in_secs: 5,
        event_listener_drain_in_secs: 5,
    }))
    .await;

    // Act
    send_signal(&tracker, Signal::SIGTERM);
    let exit = tracker.wait_for_exit().await.expect("tracker should exit after SIGTERM");

    // Assert
    assert_exit_code(&exit, 0);
    let output = exit.plain_output();
    assert!(output.contains(SUCCESS_MESSAGE), "tracker output:\n{output}");
}

#[cfg(not(unix))]
#[test]
fn it_should_skip_posix_shutdown_policy_scenarios_on_non_unix_platforms() {
    // The target deliberately compiles as a zero-test-placeholder equivalent on non-Unix platforms.
}
