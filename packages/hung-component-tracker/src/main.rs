//! Test-only tracker executable: the real tracker plus one component that never stops.
//!
//! It starts the tracker exactly as the `torrust-tracker` executable does, then reserves and spawns
//! one extra component, `hung_component`, whose future never completes and never observes its
//! cancellation token. It then runs the same shutdown sequence as the shipped executable
//! ([`app::run_until_shutdown_signal`]), so on `SIGINT` or `SIGTERM` the supervisor must abort the
//! hung component at the process shutdown deadline and exit with the incomplete-shutdown code.
//!
//! The integration tests of this crate run it as a child process. It is never shipped: the crate is
//! not published and is not a default workspace member.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use torrust_tracker_lib::app;
use torrust_tracker_lib::bootstrap::jobs::manager::ComponentResult;
use torrust_tracker_lib::bootstrap::logging;

/// The name the supervisor logs for the component that never stops.
const HUNG_COMPONENT: &str = "hung_component";

/// Command-line arguments: the shipped executable's `--config-toml-path` flag, without its check that
/// rejects an empty path (the tests always pass a path).
#[derive(Debug, Parser)]
#[command(name = "hung-component-tracker")]
struct Cli {
    /// Path to the TOML configuration file to load.
    #[arg(short = 'c', long)]
    config_toml_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> ExitCode {
    logging::install();

    let cli = Cli::parse();

    match app::start_with_explicit_config_toml_path(cli.config_toml_path).await {
        Ok((app_container, mut jobs)) => {
            // The reservation's cancellation token is dropped unused: the component never stops.
            let reservation = jobs.reserve(HUNG_COMPONENT);
            jobs.spawn(reservation, never_stops())
                .expect("the hung component is reserved from the same manager that spawns it");

            app::run_until_shutdown_signal(app_container, jobs).await
        }
        Err(error) => {
            report_startup_failure(&error);
            ExitCode::FAILURE
        }
    }
}

/// A component runner that never completes, fails or observes cancellation.
async fn never_stops() -> ComponentResult {
    std::future::pending().await
}

#[allow(
    clippy::print_stderr,
    reason = "process entry point reports user-facing startup failures on standard error"
)]
fn report_startup_failure(error: &app::Error) {
    eprintln!("Tracker startup failed: {error}");
}
