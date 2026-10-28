//! Command-line interface of the PR review wait poller.
//!
//! Output follows the global CLI output contract: the result is one JSON object on stdout, and every
//! stderr record is one JSON line with a `kind` field.
//!
//! skill-link: wait-for-pr-review

use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use clap::error::ErrorKind;
use clap::{Parser, value_parser};
use serde::Serialize;
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::flag;
use wait_for_pr_review::timestamp::parse_utc;
use wait_for_pr_review::{Config, GhCli, PullRequest, SystemClock, wait};

/// Block until a watched reviewer reviews or comments, the head commit changes, or the pull request
/// closes. Print the event as one JSON object. Stdout must not be a terminal; pipe it, for example `| jq .`.
#[derive(Debug, Parser)]
#[command(name = "wait-for-pr-review")]
struct Cli {
    /// Pull-request number.
    #[arg(long)]
    pr_number: u64,
    /// Repository owner.
    #[arg(long, default_value = "torrust")]
    owner: String,
    /// Repository name.
    #[arg(long, default_value = "torrust-tracker")]
    repository: String,
    /// Login whose reviews and comments count as events. Repeat to watch several. Replaces the defaults.
    #[arg(long = "reviewer", default_values = ["da2ce7", "copilot-pull-request-reviewer[bot]"])]
    reviewers: Vec<String>,
    /// Longest wait before a `timeout` result, in seconds.
    #[arg(long, default_value_t = 900, value_parser = value_parser!(u64).range(1..))]
    timeout_seconds: u64,
    /// Pause between two looks at the pull request, in seconds.
    #[arg(long, default_value_t = 120, value_parser = value_parser!(u64).range(1..))]
    poll_interval_seconds: u64,
    /// The commit you have checked out, usually the one you just pushed. A pull request head that
    /// differs from it is reported as `head_changed`. Unless `--since` is given, reviews and comments
    /// created after this commit was made count as new, so a reply that lands before the poller starts
    /// is not missed.
    #[arg(long, value_parser = commit_sha)]
    since_head_sha: Option<String>,
    /// Count reviews and comments created strictly after this UTC time, for example `2026-10-28T07:00:00Z`.
    /// Pass the `cursor` of the previous result to resume after an event that needed no push, so the
    /// event is not reported again and nothing after it is missed. Pass `--since-head-sha` as well, so a
    /// head that moved is not adopted silently.
    #[arg(long, value_parser = utc_timestamp)]
    since: Option<i64>,
}

fn utc_timestamp(text: &str) -> Result<i64, String> {
    parse_utc(text).ok_or_else(|| "expected a UTC timestamp such as 2026-10-28T07:00:00Z".to_owned())
}

fn commit_sha(text: &str) -> Result<String, String> {
    if (7..=40).contains(&text.len()) && text.chars().all(|character| character.is_ascii_hexdigit()) {
        Ok(text.to_owned())
    } else {
        Err("expected a hexadecimal commit SHA of 7 to 40 characters".to_owned())
    }
}

#[derive(Serialize)]
struct Diagnostic<'a> {
    kind: &'a str,
    message: &'a str,
}

#[derive(Serialize)]
struct WaitStarted<'a> {
    kind: &'static str,
    pr_number: u64,
    reviewers: &'a [String],
    timeout_seconds: u64,
    poll_interval_seconds: u64,
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let message = error.to_string();
            return match error.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => emit_diagnostic("help", &message, 0),
                _ => emit_diagnostic("usage_error", &message, 2),
            };
        }
    };

    if io::stdout().is_terminal() {
        return emit_diagnostic("tty_refusal", "stdout is a TTY; pipe the output to consume result data", 2);
    }

    let interrupted = Arc::new(AtomicBool::new(false));
    for signal in [SIGINT, SIGTERM] {
        if let Err(error) = flag::register(signal, Arc::clone(&interrupted)) {
            return emit_diagnostic("signal_handler_error", &error.to_string(), 1);
        }
    }

    // The progress record is best effort: the result on stdout, or the failure record and its exit code,
    // is what the caller relies on.
    drop(write_diagnostic(&WaitStarted {
        kind: "wait_started",
        pr_number: cli.pr_number,
        reviewers: &cli.reviewers,
        timeout_seconds: cli.timeout_seconds,
        poll_interval_seconds: cli.poll_interval_seconds,
    }));

    let pull_request = PullRequest {
        owner: cli.owner,
        repository: cli.repository,
        number: cli.pr_number,
    };
    let config = Config {
        reviewers: cli.reviewers,
        timeout: Duration::from_secs(cli.timeout_seconds),
        poll_interval: Duration::from_secs(cli.poll_interval_seconds),
        since_head_sha: cli.since_head_sha,
        since_moment: cli.since,
    };
    let started = Instant::now();
    let gh = GhCli::new("gh", Arc::clone(&interrupted), started + config.timeout);
    let clock = SystemClock::starting_at(started, interrupted);

    match wait(&gh, &clock, &pull_request, &config) {
        Ok(event) => emit_json(&event),
        Err(error) => emit_diagnostic(error.kind(), &error.to_string(), 1),
    }
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
    match write_diagnostic(&Diagnostic { kind, message }) {
        Ok(()) => ExitCode::from(exit_code),
        // Every failure code already tells the caller that something went wrong. The help text is the
        // one record whose whole purpose is to be read, so help that could not be written is a failure.
        Err(_) => ExitCode::from(exit_code.max(1)),
    }
}

/// Writes one complete NDJSON record to stderr. The line is built first and written with one call, so
/// a failure cannot leave half a record behind.
fn write_diagnostic(diagnostic: &impl Serialize) -> io::Result<()> {
    let mut line = serde_json::to_vec(diagnostic).map_err(io::Error::from)?;
    line.push(b'\n');

    io::stderr().lock().write_all(&line)
}
