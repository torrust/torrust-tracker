//! Integration tests for the `tracker_checker` binary.
//!
//! These tests verify the CLI I/O contract:
//! - stderr receives a JSON error envelope on configuration errors
//! - exit code 2 is returned for configuration errors
//! - exit code 0 is returned when the binary runs successfully (even if tracker checks fail)
//!
//! Reference: [Tracker CLI I/O Contract](../docs/contracts/tracker-cli-io-contract.md)

use std::process::Command;

fn tracker_checker_bin() -> Command {
    Command::new(resolve_tracker_checker_binary())
}

fn resolve_tracker_checker_binary() -> std::path::PathBuf {
    if let Some(path) = std::env::var_os("NEXTEST_BIN_EXE_tracker_checker") {
        return path.into();
    }

    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_tracker_checker") {
        return path.into();
    }

    let compile_time_path = std::path::PathBuf::from(env!("CARGO_BIN_EXE_tracker_checker"));
    if compile_time_path.exists() {
        return compile_time_path;
    }

    let current_exe = std::env::current_exe().expect("Failed to determine current test executable path");
    let profile_dir = current_exe
        .parent()
        .and_then(std::path::Path::parent)
        .expect("Failed to determine Cargo profile directory from test executable path");

    let mut candidate = profile_dir.join("tracker_checker");
    if cfg!(windows) {
        candidate.set_extension("exe");
    }

    if candidate.exists() {
        return candidate;
    }

    panic!(
        "Unable to locate tracker_checker binary. Tried NEXTEST_BIN_EXE_tracker_checker, CARGO_BIN_EXE_tracker_checker, compile-time CARGO_BIN_EXE_tracker_checker, and sibling binary near test executable"
    );
}

#[path = "tracker_checker/configuration.rs"]
mod configuration;

#[path = "tracker_checker/monitor.rs"]
mod monitor;
