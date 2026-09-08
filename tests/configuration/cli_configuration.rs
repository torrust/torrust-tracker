//! Executable-boundary configuration contracts for the tracker CLI.

#![cfg_attr(not(unix), allow(dead_code, unused_imports))]

#[cfg(unix)]
#[path = "../common/native_tracker.rs"]
#[allow(dead_code)]
mod native_tracker;

#[cfg(unix)]
#[path = "cli_configuration/base_source_precedence.rs"]
mod base_source_precedence;

#[cfg(unix)]
#[path = "cli_configuration/per_value_overrides.rs"]
mod per_value_overrides;

#[cfg(not(unix))]
#[test]
fn it_should_skip_native_cli_configuration_scenarios_on_non_unix_platforms() {
    // The shared native fixture currently uses Unix process exit-status extensions.
}
