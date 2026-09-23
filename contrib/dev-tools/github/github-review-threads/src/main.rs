//! Command-line interface for fetching pull-request review threads.

use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::{fs, process};

use clap::{Parser, Subcommand};
use github_review_threads::{GhCli, PullRequest, ReplyStatus, fetch, list_unresolved, reply_status, show_unresolved};
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
    /// Emit unresolved threads as one JSON object with a `threads` array.
    List {
        /// Raw GraphQL response written by `fetch`.
        #[arg(long)]
        threads_file: PathBuf,
    },
    /// Emit unresolved threads with their review comments as one JSON object with a `threads` array.
    Show {
        /// Raw GraphQL response written by `fetch`.
        #[arg(long)]
        threads_file: PathBuf,
    },
    /// Verify that each unresolved thread includes a reply from the requested login.
    ReplyStatus {
        /// Raw GraphQL response written by `fetch`.
        #[arg(long)]
        threads_file: PathBuf,
        /// GitHub login expected to reply to each unresolved thread.
        #[arg(long)]
        login: String,
    },
}

#[derive(Serialize)]
struct Diagnostic<'a> {
    kind: &'a str,
    message: &'a str,
}

/// The `missing_reply` diagnostic keeps every per-thread row so the operator knows which threads to answer.
#[derive(Serialize)]
struct MissingReplyDiagnostic<'a> {
    kind: &'a str,
    message: String,
    #[serde(flatten)]
    status: &'a ReplyStatus,
}

fn main() -> ExitCode {
    if io::stdout().is_terminal() {
        return emit_diagnostic("tty_refusal", "stdout is a TTY; pipe the output to consume result data", 2);
    }

    let cli = Cli::try_parse().unwrap_or_else(|error| {
        let message = error.to_string();
        let _ = emit_diagnostic("usage_error", &message, 2);
        process::exit(2);
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
        Commands::List { threads_file } => read_response(&threads_file)
            .and_then(|response| list_unresolved(&response).map_err(|error| error.to_string()))
            .map_or_else(
                |error| emit_diagnostic("list_error", &error, 1),
                |threads| emit_json(&threads),
            ),
        Commands::Show { threads_file } => read_response(&threads_file)
            .and_then(|response| show_unresolved(&response).map_err(|error| error.to_string()))
            .map_or_else(
                |error| emit_diagnostic("show_error", &error, 1),
                |threads| emit_json(&threads),
            ),
        Commands::ReplyStatus { threads_file, login } => read_response(&threads_file)
            .and_then(|response| reply_status(&response, &login).map_err(|error| error.to_string()))
            .map_or_else(
                |error| emit_diagnostic("reply_status_error", &error, 1),
                |status| {
                    if status.has_missing_replies() {
                        let diagnostic = MissingReplyDiagnostic {
                            kind: "missing_reply",
                            message: format!("{} unresolved thread(s) have no reply from {login}", status.without_reply()),
                            status: &status,
                        };
                        write_diagnostic(&diagnostic, 1)
                    } else {
                        emit_json(&status)
                    }
                },
            ),
    }
}

fn read_response(path: &PathBuf) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("cannot read review-thread response: {error}"))
}

fn emit_json(value: &impl Serialize) -> ExitCode {
    let mut buffer = match serde_json::to_vec(value) {
        Ok(buffer) => buffer,
        Err(error) => return emit_diagnostic("output_error", &error.to_string(), 1),
    };
    buffer.push(b'\n');

    match io::stdout().lock().write_all(&buffer) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => emit_diagnostic("output_error", &error.to_string(), 1),
    }
}

fn emit_diagnostic(kind: &str, message: &str, exit_code: u8) -> ExitCode {
    write_diagnostic(&Diagnostic { kind, message }, exit_code)
}

fn write_diagnostic(diagnostic: &impl Serialize, exit_code: u8) -> ExitCode {
    drop(serde_json::to_writer(io::stderr().lock(), diagnostic));
    drop(writeln!(io::stderr()));
    ExitCode::from(exit_code)
}
