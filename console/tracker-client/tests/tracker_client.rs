//! Integration tests for the unified `tracker_client` binary.

use std::process::Command;

fn tracker_client_bin() -> Command {
    Command::new(resolve_tracker_client_binary())
}

fn resolve_tracker_client_binary() -> std::path::PathBuf {
    if let Some(path) = std::env::var_os("NEXTEST_BIN_EXE_tracker_client") {
        return path.into();
    }

    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_tracker_client") {
        return path.into();
    }

    let compile_time_path = std::path::PathBuf::from(env!("CARGO_BIN_EXE_tracker_client"));
    if compile_time_path.exists() {
        return compile_time_path;
    }

    let current_exe = std::env::current_exe().expect("Failed to determine current test executable path");
    let profile_dir = current_exe
        .parent()
        .and_then(std::path::Path::parent)
        .expect("Failed to determine Cargo profile directory from test executable path");

    let mut candidate = profile_dir.join("tracker_client");
    if cfg!(windows) {
        candidate.set_extension("exe");
    }

    if candidate.exists() {
        return candidate;
    }

    panic!(
        "Unable to locate tracker_client binary. Tried NEXTEST_BIN_EXE_tracker_client, CARGO_BIN_EXE_tracker_client, compile-time CARGO_BIN_EXE_tracker_client, and sibling binary near test executable"
    );
}

#[test]
fn it_should_show_unified_subcommands_in_help() {
    let output = tracker_client_bin()
        .arg("--help")
        .output()
        .expect("Failed to run tracker_client --help");

    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("http"), "Expected http subcommand in help output: {stdout}");
    assert!(stdout.contains("udp"), "Expected udp subcommand in help output: {stdout}");
    assert!(stdout.contains("check"), "Expected check subcommand in help output: {stdout}");
}

#[test]
fn it_should_fail_http_announce_for_invalid_infohash() {
    let output = tracker_client_bin()
        .arg("http")
        .arg("announce")
        .arg("http://127.0.0.1:7070")
        .arg("invalid_info_hash")
        .output()
        .expect("Failed to run tracker_client http announce");

    assert_eq!(output.status.code(), Some(1));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid infohash"),
        "Expected invalid infohash message, got: {stderr}"
    );
}

#[test]
fn it_should_fail_udp_scrape_for_invalid_infohash() {
    let output = tracker_client_bin()
        .arg("udp")
        .arg("scrape")
        .arg("udp://127.0.0.1:6969")
        .arg("invalid_info_hash")
        .output()
        .expect("Failed to run tracker_client udp scrape");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to parse info-hash"),
        "Expected clap validation error with info-hash parse failure, got: {stderr}"
    );
}
