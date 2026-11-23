//! `tracker_client` never writes the key of a private HTTP tracker (`/announce/<key>`,
//! `/scrape/<key>`) to stdout or stderr, and never writes the `Debug` text of a `reqwest::Error`.
//! Where a tracker URL identifies a target, its key is written as `****` (section 7 of the global
//! CLI output contract, `docs/adrs/20260519000000_define_global_cli_output_contract.md`).
//!
//! Regression tests of issue #3004.

mod common;

use std::net::{SocketAddr, TcpListener};
use std::process::{Command, Output};

use common::fake_trackers::FakeHttpTracker;
use serde_json::Value;

const KEY: &str = "SecretTrackerKey123";

const INFO_HASH: &str = "9c38422213e30bff212b30c360d26f9a02136422"; // DevSkim: ignore DS173237

fn tracker_client(args: &[&str]) -> Command {
    let mut command = Command::new(common::resolve_tracker_client_binary());
    command
        .args(args)
        .env_remove("TORRUST_CHECKER_CONFIG")
        .env_remove("TORRUST_CHECKER_CONFIG_PATH");
    command
}

/// Runs `tracker_client check` with `config` as its configuration.
fn tracker_client_check(config: &str) -> Command {
    let mut command = tracker_client(&["check"]);
    command.env("TORRUST_CHECKER_CONFIG", config);
    command
}

fn run(mut command: Command) -> Output {
    command.output().expect("tracker_client should run")
}

/// An address where nothing listens: the port is taken from a listener that is then dropped.
fn closed_local_address() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("it should bind a local port");
    listener.local_addr().expect("the listener should have an address")
}

fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be UTF-8")
}

fn stderr_text(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be UTF-8")
}

/// The only stderr line, parsed as JSON.
fn only_stderr_record(output: &Output) -> Value {
    let stderr = stderr_text(output);
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 1, "stderr should be one JSON record: {stderr}");
    serde_json::from_str(lines[0]).unwrap_or_else(|_| panic!("stderr should be JSON: {stderr}"))
}

fn assert_key_not_written(output: &Output) {
    let stdout = stdout_text(output);
    let stderr = stderr_text(output);
    assert!(!stdout.contains(KEY), "stdout should not contain the tracker key: {stdout}");
    assert!(!stderr.contains(KEY), "stderr should not contain the tracker key: {stderr}");
}

fn assert_no_reqwest_debug_text(output: &Output) {
    let stdout = stdout_text(output);
    let stderr = stderr_text(output);
    assert!(
        !stdout.contains("reqwest::Error"),
        "stdout should not contain reqwest Debug text: {stdout}"
    );
    assert!(
        !stderr.contains("reqwest::Error"),
        "stderr should not contain reqwest Debug text: {stderr}"
    );
}

fn error_message(record: &Value) -> &str {
    record["error"]["message"]
        .as_str()
        .unwrap_or_else(|| panic!("the record should have an error message: {record}"))
}

#[test]
fn it_should_mask_the_tracker_key_when_an_http_announce_gets_no_response() {
    let keyed_url = format!("http://{}/announce/{KEY}", closed_local_address()); // DevSkim: ignore DS137138

    let output = run(tracker_client(&["http", "announce", &keyed_url, INFO_HASH]));

    assert_eq!(output.status.code(), Some(1), "a failed announce should exit with 1");
    assert_key_not_written(&output);
    let message = error_message(&only_stderr_record(&output)).to_owned();
    assert!(
        message.contains("/announce/****"),
        "the message should show the masked URL: {message}"
    );
    assert_no_reqwest_debug_text(&output);
}

#[test]
fn it_should_mask_the_tracker_key_when_an_http_scrape_gets_no_response() {
    let keyed_url = format!("http://{}/scrape/{KEY}", closed_local_address()); // DevSkim: ignore DS137138

    let output = run(tracker_client(&["http", "scrape", &keyed_url, INFO_HASH]));

    assert_eq!(output.status.code(), Some(1), "a failed scrape should exit with 1");
    assert_key_not_written(&output);
    let message = error_message(&only_stderr_record(&output)).to_owned();
    assert!(
        message.contains("/scrape/****"),
        "the message should show the masked URL: {message}"
    );
    assert_no_reqwest_debug_text(&output);
}

#[test]
fn it_should_mask_the_tracker_key_when_an_http_announce_response_body_is_cut_short() {
    let tracker = FakeHttpTracker::start_with_cut_short_body();
    let keyed_url = format!("{}/announce/{KEY}", tracker.url());

    let output = run(tracker_client(&["http", "announce", &keyed_url, INFO_HASH]));

    assert_eq!(output.status.code(), Some(1), "a failed body read should exit with 1");
    assert_key_not_written(&output);
    let message = error_message(&only_stderr_record(&output)).to_owned();
    assert!(
        message.contains("/announce/****"),
        "the message should show the masked URL: {message}"
    );
}

#[test]
fn it_should_mask_the_tracker_key_when_an_http_scrape_response_body_is_cut_short() {
    let tracker = FakeHttpTracker::start_with_cut_short_body();
    let keyed_url = format!("{}/scrape/{KEY}", tracker.url());

    let output = run(tracker_client(&["http", "scrape", &keyed_url, INFO_HASH]));

    assert_eq!(output.status.code(), Some(1), "a failed body read should exit with 1");
    assert_key_not_written(&output);
    let message = error_message(&only_stderr_record(&output)).to_owned();
    assert!(
        message.contains("/scrape/****"),
        "the message should show the masked URL: {message}"
    );
    assert_no_reqwest_debug_text(&output);
}

#[test]
fn it_should_not_write_reqwest_debug_text_when_a_check_response_body_is_cut_short() {
    let tracker = FakeHttpTracker::start_with_cut_short_body();
    let config = format!(
        r#"{{"udp_trackers":[],"http_trackers":["{}/announce/{KEY}"],"health_checks":[]}}"#,
        tracker.url()
    );

    let output = run(tracker_client_check(&config));

    assert_no_reqwest_debug_text(&output);
    assert_key_not_written(&output);
}

#[test]
fn it_should_mask_the_tracker_key_in_the_check_result_when_the_tracker_gets_no_response() {
    let keyed_url = format!("http://{}/announce/{KEY}", closed_local_address()); // DevSkim: ignore DS137138
    let config = format!(r#"{{"udp_trackers":[],"http_trackers":["{keyed_url}"],"health_checks":[]}}"#);

    let output = run(tracker_client_check(&config));

    assert_key_not_written(&output);
    let result: Value = serde_json::from_str(&stdout_text(&output)).expect("stdout should be one JSON result");
    let url = result["results"][0]["Http"]["Err"]["url"]
        .as_str()
        .unwrap_or_else(|| panic!("the result should carry the tracker URL: {result}"));
    assert_eq!(url, keyed_url.replace(KEY, "****"), "the result should show the masked URL");
    assert_no_reqwest_debug_text(&output);
}

/// Runs `check` with `url` as the value of `http_trackers`. The value must be a list, so a single
/// string makes the JSON parser echo it in the error.
fn check_with_wrong_typed_http_trackers(url: &str) -> Output {
    run(tracker_client_check(&format!(
        r#"{{"udp_trackers":[],"http_trackers":"{url}","health_checks":[]}}"#
    )))
}

/// Runs `http announce` with `url` as an extra argument. The command takes one tracker URL, so the
/// argument parser echoes the extra one in the usage error.
fn announce_with_misplaced_argument(url: &str) -> Output {
    run(tracker_client(&[
        "http",
        "announce",
        "http://127.0.0.1:7070/announce", // DevSkim: ignore DS137138
        INFO_HASH,
        url,
    ]))
}

fn assert_echo_is_masked(output: &Output, expected_exit_code: i32, expected_masked_url: &str) {
    assert_eq!(
        output.status.code(),
        Some(expected_exit_code),
        "an echo error should exit with {expected_exit_code}"
    );
    assert_key_not_written(output);
    let message = error_message(&only_stderr_record(output)).to_owned();
    assert!(
        message.contains(expected_masked_url),
        "the message should show the masked URL {expected_masked_url}: {message}"
    );
}

#[test]
fn it_should_mask_the_tracker_key_when_the_check_configuration_echoes_a_wrong_typed_url() {
    let output = check_with_wrong_typed_http_trackers(&format!("http://127.0.0.1:7070/announce/{KEY}")); // DevSkim: ignore DS137138

    assert_echo_is_masked(&output, 2, "http://127.0.0.1:7070/announce/****");
}

#[test]
fn it_should_mask_the_tracker_key_when_the_check_configuration_echoes_a_url_with_an_uppercase_scheme() {
    let output = check_with_wrong_typed_http_trackers(&format!("HTTP://127.0.0.1:7070/announce/{KEY}"));

    assert_echo_is_masked(&output, 2, "HTTP://127.0.0.1:7070/announce/****");
}

#[test]
fn it_should_mask_the_tracker_key_when_the_check_configuration_echoes_a_url_that_does_not_parse() {
    let output = check_with_wrong_typed_http_trackers(&format!("http://127.0.0.1:bad/announce/{KEY}")); // DevSkim: ignore DS137138

    assert_echo_is_masked(&output, 2, "http://127.0.0.1:bad/announce/****");
}

#[test]
fn it_should_mask_the_tracker_key_when_a_usage_error_echoes_a_misplaced_url() {
    let output = announce_with_misplaced_argument(&format!("http://127.0.0.1:7070/announce/{KEY}")); // DevSkim: ignore DS137138

    assert_echo_is_masked(&output, 2, "http://127.0.0.1:7070/announce/****");
}

#[test]
fn it_should_mask_the_tracker_key_when_a_usage_error_echoes_a_misplaced_url_with_an_uppercase_scheme() {
    let output = announce_with_misplaced_argument(&format!("HTTP://127.0.0.1:7070/announce/{KEY}"));

    assert_echo_is_masked(&output, 2, "HTTP://127.0.0.1:7070/announce/****");
}

#[test]
fn it_should_mask_the_tracker_key_when_a_usage_error_echoes_a_misplaced_url_that_does_not_parse() {
    let output = announce_with_misplaced_argument(&format!("http://127.0.0.1:bad/announce/{KEY}")); // DevSkim: ignore DS137138

    assert_echo_is_masked(&output, 2, "http://127.0.0.1:bad/announce/****");
}

#[test]
fn it_should_not_write_reqwest_debug_text_when_a_health_check_gets_no_response() {
    let config = format!(
        r#"{{"udp_trackers":[],"http_trackers":[],"health_checks":["http://{}/health_check"]}}"#, // DevSkim: ignore DS137138
        closed_local_address()
    );

    let output = run(tracker_client_check(&config));

    assert_no_reqwest_debug_text(&output);
}

#[test]
fn it_should_mask_the_tracker_key_when_a_usage_error_echoes_a_misplaced_url_with_a_backslash_separator() {
    let output = announce_with_misplaced_argument(&format!("http://127.0.0.1:7070/announce\\{KEY}")); // DevSkim: ignore DS137138

    assert_echo_is_masked(&output, 2, "http://127.0.0.1:7070/announce\\****");
}

#[test]
fn it_should_mask_the_tracker_key_when_the_check_configuration_echoes_a_url_with_a_backslash_separator() {
    // In JSON the backslash is written `\\`; the parser echoes it the same way.
    let output = check_with_wrong_typed_http_trackers(&format!("http://127.0.0.1:7070/announce\\\\{KEY}")); // DevSkim: ignore DS137138

    assert_echo_is_masked(&output, 2, "http://127.0.0.1:7070/announce\\\\****");
}
