use clap::{Parser, Subcommand, ValueEnum};
use tracing::level_filters::LevelFilter;

use super::{check, http, output, udp};
use crate::console::clients::checker::error::AppError;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Json,
    Text,
}

impl OutputFormat {
    #[must_use]
    pub const fn is_pretty(self) -> bool {
        matches!(self, Self::Text)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Stdout is a terminal: the contract refuses to run (exit `2`).
    TtyRefusal,
    Check(AppError),
    Other(anyhow::Error),
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Other(value)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// HTTP tracker commands.
    Http {
        #[command(subcommand)]
        command: http::Command,
    },
    /// UDP tracker commands.
    Udp {
        #[command(subcommand)]
        command: udp::Command,
    },
    /// Tracker checker commands and configuration.
    Check {
        /// Output format for check results.
        #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
        format: OutputFormat,
        /// Arguments passed to the checker implementation.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

/// # Errors
///
/// Returns an error if command execution fails.
pub async fn run() -> Result<(), Error> {
    if output::stdout_is_terminal() {
        return Err(Error::TtyRefusal);
    }

    init_tracing_stderr(LevelFilter::INFO);

    let args = Args::try_parse().map_err(|error| Error::Check(AppError::from_clap(&error)))?;

    match args.command {
        Command::Http { command } => http::run(command).await.map_err(Error::Other)?,
        Command::Udp { command } => udp::run(command).await.map_err(Error::Other)?,
        Command::Check {
            format,
            args: checker_args,
        } => check::run(checker_args, format).await.map_err(Error::Check)?,
    }

    Ok(())
}

/// Tracing records are JSON lines on stderr; stdout is reserved for the result.
fn init_tracing_stderr(filter: LevelFilter) {
    if tracing_subscriber::fmt()
        .json()
        .with_writer(std::io::stderr)
        .with_max_level(filter)
        .try_init()
        .is_ok()
    {
        tracing::debug!("Logging initialized");
    }
}
