use std::path::PathBuf;
use std::process::ExitCode;

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
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match app::start_with_explicit_config_toml_path(cli.config_toml_path).await {
        Ok((app_container, jobs)) => app::run_until_shutdown_signal(app_container, jobs).await,
        Err(error) => {
            tracing::error!(%error, "Tracker startup failed");
            report_startup_failure(&error);
            ExitCode::FAILURE
        }
    }
}

#[allow(
    clippy::print_stderr,
    reason = "process entry point reports user-facing startup failures on standard error"
)]
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
