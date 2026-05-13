//! Integration tests for the `tracker_client check` command.
//!
//! These tests verify the CLI I/O contract:
//! - stderr receives a JSON error envelope on configuration errors
//! - exit code 2 is returned for configuration errors
//! - exit code 0 is returned when the binary runs successfully (even if tracker checks fail)
//!
//! Reference: [Tracker CLI I/O Contract](../docs/contracts/tracker-cli-io-contract.md)

mod common;

use std::process::Command;

fn tracker_client_check_bin() -> Command {
    let mut command = Command::new(common::resolve_tracker_client_binary());
    command.arg("check");
    command.arg("--");
    command
}

#[path = "tracker_checker/configuration.rs"]
mod configuration;

#[path = "tracker_checker/monitor.rs"]
mod monitor;
