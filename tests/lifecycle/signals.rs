//! Unix executable-boundary signal tests for the tracker binary.

#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

#[cfg(unix)]
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
