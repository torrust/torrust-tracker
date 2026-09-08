//! Unix executable-boundary signal tests for the tracker binary.

#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

#[cfg(unix)]
#[path = "../common/native_tracker.rs"]
mod native_tracker;

#[cfg(unix)]
use std::time::Duration;

#[cfg(unix)]
use nix::sys::signal::{Signal, kill};
#[cfg(unix)]
use nix::unistd::Pid;

#[cfg(unix)]
#[tokio::test]
async fn it_should_gracefully_shutdown_the_tracker_binary_when_sigterm_is_delivered_to_its_exact_pid() {
    // Arrange
    let mut tracker = native_tracker::NativeTracker::start();
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should report health Status::Ok before SIGTERM");
    let pid = tracker.pid().expect("running tracker child should have a PID");

    // Act
    kill(
        Pid::from_raw(i32::try_from(pid).expect("child PID should fit i32")),
        Signal::SIGTERM,
    )
    .expect("deliver SIGTERM to the exact tracker child PID");
    let output = tracker
        .shutdown()
        .await
        .expect("tracker should exit gracefully after SIGTERM");

    // Assert
    assert!(
        output.contains("Torrust tracker shutting down (SIGTERM) ..."),
        "tracker output:\n{output}"
    );
    assert!(output.contains("Waiting for job to finish"), "tracker output:\n{output}");
    assert!(
        output.contains("Torrust tracker successfully shutdown."),
        "tracker output:\n{output}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_distinguish_sigint_from_sigterm_when_shutting_down_the_tracker_binary() {
    // Arrange
    let mut tracker = native_tracker::NativeTracker::start();
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should report health Status::Ok before SIGINT");
    let pid = tracker.pid().expect("running tracker child should have a PID");

    // Act
    kill(
        Pid::from_raw(i32::try_from(pid).expect("child PID should fit i32")),
        Signal::SIGINT,
    )
    .expect("deliver SIGINT to the exact tracker child PID");
    let output = tracker.shutdown().await.expect("tracker should exit gracefully after SIGINT");

    // Assert
    assert!(
        output.contains("Torrust tracker shutting down (SIGINT) ..."),
        "tracker output:\n{output}"
    );
    assert!(
        !output.contains("Torrust tracker shutting down (SIGTERM) ..."),
        "tracker output:\n{output}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_run_two_tracker_binaries_with_independent_cli_configurations() {
    // Arrange
    let mut first_tracker = native_tracker::NativeTracker::start();
    let mut second_tracker = native_tracker::NativeTracker::start();

    // Act
    let (first_ready, second_ready) = tokio::join!(first_tracker.wait_until_ready(), second_tracker.wait_until_ready());
    first_ready.expect("first tracker should become ready before its fixture deadline");
    second_ready.expect("second tracker should become ready before its fixture deadline");
    let first_address = first_tracker
        .health_check_address()
        .expect("ready first tracker should expose its health-check address");
    let second_address = second_tracker
        .health_check_address()
        .expect("ready second tracker should expose its health-check address");
    let first_configuration_path = first_tracker
        .configuration_path()
        .expect("first tracker should retain its CLI configuration path");
    let second_configuration_path = second_tracker
        .configuration_path()
        .expect("second tracker should retain its CLI configuration path");
    let first_storage_path = first_tracker
        .storage_path()
        .expect("first tracker should retain its isolated storage path");
    let second_storage_path = second_tracker
        .storage_path()
        .expect("second tracker should retain its isolated storage path");
    let first_pid = first_tracker.pid().expect("ready first tracker should have a PID");
    let second_pid = second_tracker.pid().expect("ready second tracker should have a PID");

    kill(
        Pid::from_raw(i32::try_from(first_pid).expect("first child PID should fit i32")),
        Signal::SIGTERM,
    )
    .expect("deliver SIGTERM to the first tracker child");
    kill(
        Pid::from_raw(i32::try_from(second_pid).expect("second child PID should fit i32")),
        Signal::SIGTERM,
    )
    .expect("deliver SIGTERM to the second tracker child");
    let (first_shutdown, second_shutdown) = tokio::join!(first_tracker.shutdown(), second_tracker.shutdown());

    // Assert
    assert_ne!(
        first_address, second_address,
        "port-zero health-check bindings should be independent"
    );
    assert_ne!(
        first_configuration_path, second_configuration_path,
        "fixtures should own distinct CLI configuration paths"
    );
    assert_ne!(
        first_storage_path, second_storage_path,
        "fixtures should own distinct workspace-local storage paths"
    );
    assert_ne!(first_pid, second_pid, "fixtures should own distinct tracker children");
    assert!(
        first_shutdown
            .expect("first tracker should gracefully exit after SIGTERM")
            .contains("Torrust tracker successfully shutdown."),
        "first tracker should report graceful shutdown"
    );
    assert!(
        second_shutdown
            .expect("second tracker should gracefully exit after SIGTERM")
            .contains("Torrust tracker successfully shutdown."),
        "second tracker should report graceful shutdown"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn it_should_force_kill_and_reap_the_tracker_binary_when_the_fixture_is_dropped() {
    // Arrange
    let mut tracker = native_tracker::NativeTracker::start();
    let cleanup_complete = tracker.take_drop_cleanup_observer();

    // Act
    drop(tracker);

    // Assert
    tokio::time::timeout(Duration::from_secs(5), cleanup_complete)
        .await
        .expect("fixture drop cleanup should complete within the deadline")
        .expect("fixture drop cleanup observer should be notified")
        .and_then(|signal| {
            (signal == Signal::SIGKILL as i32)
                .then_some(())
                .ok_or_else(|| format!("fixture drop cleanup should reap a SIGKILL-terminated child, got signal {signal}"))
        })
        .expect("fixture drop cleanup should force-kill and reap the tracker child");
}

#[cfg(not(unix))]
#[test]
fn it_should_skip_posix_signal_lifecycle_scenarios_on_non_unix_platforms() {
    // The target deliberately compiles as a zero-test-placeholder equivalent on non-Unix platforms.
}
