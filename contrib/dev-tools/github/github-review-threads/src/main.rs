//! Command-line interface for fetching pull-request review threads.

use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use github_review_threads::{GhCli, PullRequest, fetch};
use serde::Serialize;

/// Fetch pull-request review threads through the GitHub CLI.
#[derive(Debug, Parser)]
#[command(name = "github-review-threads")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Write the raw GraphQL response to a file.
    Fetch {
        /// Pull-request number.
        #[arg(long)]
        pr_number: u64,
        /// Destination for the raw GraphQL response.
        #[arg(long)]
        output_file: Option<PathBuf>,
        /// Repository owner.
        #[arg(long, default_value = "torrust")]
        owner: String,
        /// Repository name.
        #[arg(long, default_value = "torrust-tracker")]
        repository: String,
    },
}

#[derive(Serialize)]
struct Diagnostic<'a> {
    kind: &'a str,
    message: &'a str,
}

fn main() -> ExitCode {
    if io::stdout().is_terminal() {
        return emit_diagnostic("tty_refusal", "stdout is a TTY; pipe the output to consume result data", 2);
    }

    let cli = Cli::try_parse().unwrap_or_else(|error| {
        let message = error.to_string();
        let _ = emit_diagnostic("usage_error", &message, 2);
        std::process::exit(2);
    });

    match cli.command {
        Commands::Fetch {
            pr_number,
            output_file,
            owner,
            repository,
        } => {
            let output_file = output_file.unwrap_or_else(|| PathBuf::from(format!("/tmp/pr_threads_{pr_number}.json")));
            let pull_request = PullRequest {
                owner,
                repository,
                number: pr_number,
            };

            match fetch(&GhCli, &pull_request, &output_file) {
                Ok(result) => emit_json(&result),
                Err(error) => emit_diagnostic("fetch_error", &error.to_string(), 1),
            }
        }
    }
}

fn emit_json(value: &impl Serialize) -> ExitCode {
    match serde_json::to_writer(io::stdout().lock(), value) {
        Ok(()) => {
            drop(writeln!(io::stdout()));
            ExitCode::SUCCESS
        }
        Err(_) => ExitCode::from(1),
    }
}

fn emit_diagnostic(kind: &str, message: &str, exit_code: u8) -> ExitCode {
    let diagnostic = Diagnostic { kind, message };
    drop(serde_json::to_writer(io::stderr().lock(), &diagnostic));
    drop(writeln!(io::stderr()));
    ExitCode::from(exit_code)
}
