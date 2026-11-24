//! Executable-boundary test: trackers whose usage statistics are disabled log no `WARN` and no
//! `ERROR` for the requests they serve, and the start lines of the statistics listeners say so.
//!
//! It starts the real binary with one UDP tracker and one HTTP tracker, both with
//! `tracker_usage_statistics = false`, sends one announce to each, stops the tracker with `SIGTERM`
//! and reads the log the child wrote.

#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

#[cfg(unix)]
#[path = "../common/http_requests.rs"]
mod http_requests;
#[cfg(unix)]
#[path = "../common/native_tracker/mod.rs"]
mod native_tracker;
#[cfg(unix)]
#[path = "../common/udp_requests.rs"]
mod udp_requests;

#[cfg(unix)]
use native_tracker::NativeTracker;
#[cfg(unix)]
use native_tracker::configuration::NativeTrackerConfigurationSources;
#[cfg(unix)]
use nix::sys::signal::{Signal, kill};
#[cfg(unix)]
use nix::unistd::Pid;
#[cfg(unix)]
use url::Url;

#[cfg(unix)]
#[tokio::test]
async fn it_should_not_log_a_warning_or_an_error_for_the_requests_of_trackers_with_statistics_disabled() {
    // Arrange
    let sources = NativeTrackerConfigurationSources::with_cli_health_check_port(0).with_statistics_disabled_udp_tracker();
    let mut tracker = NativeTracker::start_with_configuration_sources(sources);
    tracker
        .wait_until_ready()
        .await
        .expect("tracker should report health Status::Ok");
    let udp_address = tracker.udp_tracker_address().await.expect("the UDP tracker logs its address");
    let http_address = tracker
        .http_tracker_address()
        .await
        .expect("the HTTP tracker logs its address");
    let http_url = Url::parse(&format!("http://{http_address}")).expect("the HTTP tracker address is a URL"); // DevSkim: ignore DS137138

    // Act
    udp_requests::udp_announce(udp_address, &[1; 20], &[2; 20], 6881).await;
    http_requests::http_announce(&http_url, &[1; 20], &[3; 20], 6882).await;
    let pid = tracker.pid().expect("running tracker child should have a PID");
    kill(
        Pid::from_raw(i32::try_from(pid).expect("child PID should fit i32")),
        Signal::SIGTERM,
    )
    .expect("deliver SIGTERM to the exact tracker child PID");
    let exit = tracker.wait_for_exit().await.expect("the tracker exits after SIGTERM");

    // Assert
    let output = exit.plain_output();
    let warnings_and_errors: Vec<&str> = output
        .lines()
        .filter(|line| line.contains(" WARN ") || line.contains(" ERROR "))
        .collect();
    assert!(
        warnings_and_errors.is_empty(),
        "a tracker with statistics disabled logs no WARN and no ERROR for its requests, found:\n{}",
        warnings_and_errors.join("\n")
    );
    let start_lines_with_the_counts = output
        .lines()
        .filter(|line| line.contains("Starting") && line.contains("enabled_instances=0 disabled_instances=1"))
        .count();
    assert_eq!(
        start_lines_with_the_counts, 3,
        "the UDP core, UDP server and HTTP core statistics listeners report one disabled instance, in:\n{output}"
    );
}
