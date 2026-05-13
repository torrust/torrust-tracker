//! Shared test utilities for tracker-client integration tests.

use std::path::PathBuf;

/// Resolves the path to the `tracker_client` binary for integration tests.
///
/// Resolution order:
/// 1. `NEXTEST_BIN_EXE_tracker_client` env var (set by cargo-nextest)
/// 2. `CARGO_BIN_EXE_tracker_client` env var (set by cargo test)
/// 3. Compile-time `CARGO_BIN_EXE_tracker_client` macro
/// 4. Sibling binary next to the test executable (fallback for non-standard runners)
#[must_use]
pub fn resolve_tracker_client_binary() -> PathBuf {
    if let Some(path) = std::env::var_os("NEXTEST_BIN_EXE_tracker_client") {
        return path.into();
    }

    if let Some(path) = std::env::var_os("CARGO_BIN_EXE_tracker_client") {
        return path.into();
    }

    let compile_time_path = PathBuf::from(env!("CARGO_BIN_EXE_tracker_client"));
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
