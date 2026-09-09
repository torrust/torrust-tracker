use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use torrust_tracker_lib::app;

/// Command-line arguments accepted by the tracker executable.
#[derive(Debug, Parser)]
#[command(name = "torrust-tracker")]
struct Cli {
    /// Path to the TOML configuration file to load.
    // issue: #2151
    #[arg(short = 'c', long, value_parser = parse_non_empty_path)]
    config_toml_path: Option<PathBuf>,
}

fn parse_non_empty_path(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() {
        return Err("configuration TOML path must not be empty".to_owned());
    }

    Ok(PathBuf::from(value))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match app::start_with_explicit_config_toml_path(cli.config_toml_path).await {
        Ok((_app_container, jobs)) => {
            let shutdown_signal = wait_for_shutdown_signal().await;

            tracing::info!("Torrust tracker shutting down ({shutdown_signal}) ...");

            jobs.cancel();

            jobs.wait_for_all(Duration::from_secs(10)).await;

            tracing::info!("Torrust tracker successfully shutdown.");
        }
        Err(error) => {
            tracing::error!(%error, "Tracker startup failed");
            report_startup_failure(&error);
            std::process::exit(1);
        }
    }
}

/// Waits until the process receives a supported shutdown signal.
///
/// Tokio registers the Ctrl-C listener when its future is first polled. The
/// outer, biased `select!` polls Ctrl-C after the SIGTERM stream is created
/// and before its immediately-ready branch logs the observable readiness
/// marker. The native executable-boundary tests wait for that marker, so they
/// can signal the child without racing listener registration.
///
/// Pin `ctrl_c` because it is polled in the outer `select!` and then awaited
/// again in the inner signal wait.
#[cfg(unix)]
async fn wait_for_shutdown_signal() -> &'static str {
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);
    let mut sigterm =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).expect("failed to install SIGTERM handler");

    tokio::select! {
        biased;
        result = &mut ctrl_c => {
            result.expect("failed to install Ctrl-C handler");
            "SIGINT"
        },
        result = sigterm.recv() => {
            result.expect("SIGTERM handler stream closed unexpectedly");
            "SIGTERM"
        },
        () = std::future::ready(()) => {
            tracing::info!("Tracker shutdown signal handlers installed.");

            tokio::select! {
                result = &mut ctrl_c => {
                    result.expect("failed to install Ctrl-C handler");
                    "SIGINT"
                },
                result = sigterm.recv() => {
                    result.expect("SIGTERM handler stream closed unexpectedly");
                    "SIGTERM"
                },
            }
        },
    }
}

/// Waits until the process receives Ctrl-C on a non-Unix platform.
#[cfg(not(unix))]
async fn wait_for_shutdown_signal() -> &'static str {
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    tokio::select! {
        biased;
        result = &mut ctrl_c => {
            result.expect("failed to install Ctrl-C handler");
            "SIGINT"
        },
        _ = std::future::ready(()) => {
            tracing::info!("Tracker shutdown signal handlers installed.");
            ctrl_c.await.expect("failed to install Ctrl-C handler");
            "SIGINT"
        },
    }
}

#[allow(clippy::print_stderr)]
fn report_startup_failure(error: &app::Error) {
    eprintln!("Tracker startup failed: {error}");
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use clap::error::ErrorKind;
    use clap::{CommandFactory, Parser};

    use super::Cli;

    #[test]
    fn it_should_parse_a_short_config_toml_path_argument() {
        // Arrange
        let arguments = ["torrust-tracker", "-c", "tracker.toml"];

        // Act
        let cli = Cli::try_parse_from(arguments).expect("short option should parse");

        // Assert
        assert_eq!(cli.config_toml_path, Some(PathBuf::from("tracker.toml")));
    }

    #[test]
    fn it_should_parse_a_long_config_toml_path_argument() {
        // Arrange
        let arguments = ["torrust-tracker", "--config-toml-path", "/etc/torrust/tracker.toml"];

        // Act
        let cli = Cli::try_parse_from(arguments).expect("long option should parse");

        // Assert
        assert_eq!(cli.config_toml_path, Some(PathBuf::from("/etc/torrust/tracker.toml")));
    }

    #[test]
    fn it_should_return_a_usage_error_when_the_config_toml_path_value_is_missing() {
        // Arrange
        let arguments = ["torrust-tracker", "--config-toml-path"];

        // Act
        let error = Cli::try_parse_from(arguments).expect_err("missing option value should fail");

        // Assert
        assert_eq!(error.kind(), ErrorKind::InvalidValue);
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn it_should_return_a_usage_error_when_the_config_toml_path_value_is_empty() {
        // Arrange
        let arguments = ["torrust-tracker", "--config-toml-path", ""];

        // Act
        let error = Cli::try_parse_from(arguments).expect_err("empty option value should fail");

        // Assert
        assert_eq!(error.kind(), ErrorKind::ValueValidation);
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn it_should_return_a_usage_error_when_an_argument_is_unknown() {
        // Arrange
        let arguments = ["torrust-tracker", "--unknown"];

        // Act
        let error = Cli::try_parse_from(arguments).expect_err("unknown option should fail");

        // Assert
        assert_eq!(error.kind(), ErrorKind::UnknownArgument);
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn it_should_render_help() {
        // Arrange
        let arguments = ["torrust-tracker", "--help"];

        // Act
        let error = Cli::try_parse_from(arguments).expect_err("help should stop parsing");

        // Assert
        assert_eq!(error.kind(), ErrorKind::DisplayHelp);
        assert_eq!(error.exit_code(), 0);
        assert!(error.to_string().contains("--config-toml-path <CONFIG_TOML_PATH>"));
    }

    #[test]
    fn it_should_not_bind_the_config_toml_path_argument_from_the_environment() {
        // Arrange

        // Act
        let command = Cli::command();
        let argument = command
            .get_arguments()
            .find(|argument| argument.get_id() == "config_toml_path")
            .expect("config TOML path argument should exist");

        // Assert
        assert_eq!(argument.get_env(), None);
    }
}
