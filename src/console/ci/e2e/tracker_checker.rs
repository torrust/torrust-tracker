use std::io;
use std::process::Command;

use tracing::info;

/// Runs the Tracker Checker.
///
/// # Errors
///
/// Will return an error if the Tracker Checker fails.
pub fn run(config_content: &str) -> io::Result<()> {
    info!("Running Tracker Checker: TORRUST_CHECKER_CONFIG=[config] cargo run --bin tracker_checker");
    info!("Tracker Checker config:\n{config_content}");

    let status = Command::new("cargo")
        .env("TORRUST_CHECKER_CONFIG", config_content)
        .args(["run", "--bin", "tracker_checker"])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to run Tracker Checker"))
    }
}
