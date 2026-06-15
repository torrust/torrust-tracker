use clap::{Parser, Subcommand, ValueEnum};
use tracing::level_filters::LevelFilter;

use super::{check, http, udp};
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
pub enum Error {
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
    init_tracing_stdout(LevelFilter::INFO);

    let args = Args::parse();

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

fn init_tracing_stdout(filter: LevelFilter) {
    if tracing_subscriber::fmt().with_max_level(filter).try_init().is_ok() {
        tracing::debug!("Logging initialized");
    }
}
