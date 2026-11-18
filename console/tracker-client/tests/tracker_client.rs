//! Integration tests for the unified `tracker_client` binary.

mod common;

use std::process::Command;

fn tracker_client_bin() -> Command {
    Command::new(common::resolve_tracker_client_binary())
}

#[path = "tracker_client/scrape.rs"]
mod scrape;

#[test]
fn it_should_show_unified_subcommands_in_help() {
    let output = tracker_client_bin()
        .arg("--help")
        .output()
        .expect("Failed to run tracker_client --help");

    assert_eq!(output.status.code(), Some(0));

    // The help text is a JSON record on stderr: stdout carries result data only.
    assert!(output.stdout.is_empty(), "Expected empty stdout");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let record: serde_json::Value = serde_json::from_str(stderr.trim_end()).expect("help should be one JSON record");
    let help = record["message"].as_str().expect("help record should have a message");
    assert!(help.contains("http"), "Expected http subcommand in help output: {help}");
    assert!(help.contains("udp"), "Expected udp subcommand in help output: {help}");
    assert!(help.contains("check"), "Expected check subcommand in help output: {help}");
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
