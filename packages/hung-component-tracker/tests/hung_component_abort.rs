//! Executable-boundary tests for the shutdown deadline abort of a component that never stops.
//!
//! The `hung-component-tracker` binary is the real tracker, started and shut down by the same library
//! functions as the shipped executable, plus one component, `hung_component`, that never completes and
//! ignores its cancellation token. These tests signal its exact PID and check that the supervisor
//! aborts what is still running at the process deadline and exits with the incomplete-shutdown code.

#![cfg(unix)]

mod common;

use std::time::Duration;

use common::{ChildExit, HungComponentTracker, ShutdownBudgets};
use nix::sys::signal::Signal;

/// A short process deadline `P`, with drain budgets that pass validation: `max(1, 1) + 1 = 2 < 4`.
const BUDGETS: ShutdownBudgets = ShutdownBudgets {
    deadline_in_secs: 4,
    http_drain_in_secs: 1,
    udp_request_drain_in_secs: 1,
    event_listener_drain_in_secs: 1,
};

/// The process deadline `P` of [`BUDGETS`].
const DEADLINE: Duration = Duration::from_secs(4);

/// How long after `P` the process may take to abort, join and exit. Aborting and joining take
/// milliseconds; the margin absorbs a loaded CI runner. A health check API that waited for its own
/// one-second drain after `P` would still fit, so the margin is not what proves it was aborted: the
/// abort log line is.
const EXIT_MARGIN: Duration = Duration::from_secs(3);

/// How long the fixture waits for the exit before it kills the child and fails the test.
const EXIT_TIMEOUT: Duration = Duration::from_secs(30);

const ABORTED_AFTER_DEADLINE: &str = "Job was aborted after the shutdown deadline";
const INCOMPLETE_SHUTDOWN: &str = "Torrust tracker shutdown was incomplete";

fn assert_the_supervisor_aborts_the_hung_component_at_the_deadline(signal: Signal) {
    // Arrange
    // The binary differs from the shipped tracker only by `hung_component`, which never stops.
    let mut tracker = HungComponentTracker::start(BUDGETS);
    tracker.wait_until_signal_handlers_installed();

    // Act
    let exit = tracker.signal_and_wait_for_exit(signal, EXIT_TIMEOUT);

    // Assert
    assert!(
        exit.output.contains(&format!("Torrust tracker shutting down ({signal})")),
        "the tracker should handle {signal}; output:\n{}",
        exit.output
    );
    assert_eq!(
        exit.status.code(),
        Some(1),
        "the incomplete shutdown should exit with code 1, got {}; output:\n{}",
        exit.status,
        exit.output
    );
    assert!(
        exit.elapsed_since_signal >= DEADLINE,
        "the tracker should wait for the {DEADLINE:?} deadline before aborting, exited after {:?}; output:\n{}",
        exit.elapsed_since_signal,
        exit.output
    );
    assert!(
        exit.elapsed_since_signal <= DEADLINE + EXIT_MARGIN,
        "the tracker should exit within {EXIT_MARGIN:?} of the {DEADLINE:?} deadline, exited after {:?}; output:\n{}",
        exit.elapsed_since_signal,
        exit.output
    );
    assert_aborted_after_the_deadline(&exit, "hung_component");
    // It stops last, so it waits for the hung component and is never asked to stop.
    assert_aborted_after_the_deadline(&exit, "health_check_api");
    assert_eq!(
        incomplete_components(&exit),
        vec!["health_check_api", "hung_component"],
        "the final error line should name exactly the aborted components; output:\n{}",
        exit.output
    );
}

fn assert_aborted_after_the_deadline(exit: &ChildExit, component: &str) {
    let job_field = format!("job={component}");
    assert!(
        exit.output
            .lines()
            .any(|line| line.contains(ABORTED_AFTER_DEADLINE) && line.split_whitespace().any(|word| word == job_field)),
        "the supervisor should log `{component}` as aborted after the deadline; output:\n{}",
        exit.output
    );
}

/// The sorted names of the `incomplete_components` field of the final error line.
fn incomplete_components(exit: &ChildExit) -> Vec<&str> {
    let line = exit
        .output
        .lines()
        .find(|line| line.contains(INCOMPLETE_SHUTDOWN))
        .unwrap_or_else(|| panic!("the tracker should log the incomplete shutdown; output:\n{}", exit.output));
    let (_, names) = line
        .split_once("incomplete_components=")
        .unwrap_or_else(|| panic!("the incomplete shutdown line should name the components: {line}"));
    let mut names: Vec<&str> = names.split(", ").map(str::trim).collect();
    names.sort_unstable();
    names
}

#[test]
fn it_should_abort_a_hung_component_at_the_shutdown_deadline_and_exit_with_code_one_after_sigterm() {
    assert_the_supervisor_aborts_the_hung_component_at_the_deadline(Signal::SIGTERM);
}

#[test]
fn it_should_abort_a_hung_component_at_the_shutdown_deadline_and_exit_with_code_one_after_sigint() {
    assert_the_supervisor_aborts_the_hung_component_at_the_deadline(Signal::SIGINT);
}
