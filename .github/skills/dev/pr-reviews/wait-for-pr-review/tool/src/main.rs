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

/// Largest accepted `--timeout-seconds`: one day. Far above any realistic wait, and a deadline one day ahead
/// is far inside the `Instant` range of every platform this tool supports, so the deadline sum does not
/// overflow in practice. Rust does not guarantee a minimum `Instant` range.
const MAX_TIMEOUT_SECONDS: u64 = 86_400;

/// The command run when `--gh-command` is not given.
const DEFAULT_GH_COMMAND: &str = "gh";

/// Longest the best-effort identity lookup may take. It also never takes more than a quarter of
/// `--timeout-seconds`, so a stalled lookup leaves the rest of the budget to the wait.
const MAX_LOOKUP_SECONDS: u64 = 10;

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
    /// Longest wait before a `timeout` result, in seconds. At most 86400 (one day).
    #[arg(long, default_value_t = 900, value_parser = value_parser!(u64).range(1..=MAX_TIMEOUT_SECONDS))]
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
    /// The GitHub CLI command to run for every call to GitHub: one program name or a path, never split on
    /// spaces and never run through a shell. It must accept the `gh` command line (`api ...`). The value
    /// `gh`, the default, is checked with `gh auth status` first; any other value is not. The `wait_started` and
    /// `gh_identity` records on stderr report the command and the login it runs as.
    #[arg(long, default_value = DEFAULT_GH_COMMAND, value_parser = gh_command)]
    gh_command: String,
}

fn gh_command(text: &str) -> Result<String, String> {
    if text.is_empty() {
        Err("expected a program name or path".to_owned())
    } else {
        Ok(text.to_owned())
    }
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
    gh_command: &'a str,
    pr_number: u64,
    reviewers: &'a [String],
    timeout_seconds: u64,
    poll_interval_seconds: u64,
}

#[derive(Serialize)]
struct GhIdentity<'a> {
    kind: &'static str,
    gh_command: &'a str,
    gh_login: Option<&'a str>,
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
        gh_command: &cli.gh_command,
        pr_number: cli.pr_number,
        reviewers: &cli.reviewers,
        timeout_seconds: cli.timeout_seconds,
        poll_interval_seconds: cli.poll_interval_seconds,
    }));

    let started = Instant::now();
    let timeout = Duration::from_secs(cli.timeout_seconds);
    if let Err(error) = report_identity(&cli.gh_command, &interrupted, started, timeout) {
        return emit_diagnostic(error.kind(), &error.to_string(), 1);
    }

    let pull_request = PullRequest {
        owner: cli.owner,
        repository: cli.repository,
        number: cli.pr_number,
    };
    let config = Config {
        reviewers: cli.reviewers,
        timeout,
        poll_interval: Duration::from_secs(cli.poll_interval_seconds),
        since_head_sha: cli.since_head_sha,
        since_moment: cli.since,
    };
    let gh = GhCli::new(&cli.gh_command, Arc::clone(&interrupted), started + config.timeout);
    let gh = if cli.gh_command == DEFAULT_GH_COMMAND {
        gh
    } else {
        gh.without_auth_check()
    };
    let clock = SystemClock::starting_at(started, interrupted);

    match wait(&gh, &clock, &pull_request, &config) {
        Ok(event) => emit_json(&event),
        Err(error) => emit_diagnostic(error.kind(), &error.to_string(), 1),
    }
}

/// The deadline of the identity lookup: `min(10 s, timeout / 4)` after the start, so a stalled lookup leaves
/// at least three quarters of the wait budget.
fn lookup_bound(timeout: Duration) -> Duration {
    Duration::from_secs(MAX_LOOKUP_SECONDS).min(timeout / 4)
}

/// Writes the `gh_identity` record: the command and the login it runs as. The lookup is best effort and
/// has its own bound, [`lookup_bound`], so a stalled or failing lookup never changes the result: its login is
/// then `null`.
///
/// # Errors
///
/// Returns [`wait_for_pr_review::Error::Interrupted`] when a signal ended the lookup. The caller reports the
/// interruption and does not start the wait, so no other command is run after the signal.
fn report_identity(
    command: &str,
    interrupted: &Arc<AtomicBool>,
    started: Instant,
    timeout: Duration,
) -> Result<(), wait_for_pr_review::Error> {
    let lookup = GhCli::new(command, Arc::clone(interrupted), started + lookup_bound(timeout));

    let login = match lookup.login() {
        Ok(login) => Some(login),
        Err(error @ wait_for_pr_review::Error::Interrupted) => return Err(error),
        Err(_) => None,
    };

    drop(write_diagnostic(&GhIdentity {
        kind: "gh_identity",
        gh_command: command,
        gh_login: login.as_deref(),
    }));

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    // `yes` never exits on its own, whatever its arguments, so the lookup is still running when the flag is
    // seen: the interruption is then deterministic and needs no fake script.
    #[cfg(unix)]
    #[test]
    fn it_should_return_the_interruption_of_the_identity_lookup_to_the_caller() {
        let interrupted = Arc::new(AtomicBool::new(true));

        let result = report_identity("yes", &interrupted, Instant::now(), Duration::from_secs(900));

        assert!(matches!(result, Err(wait_for_pr_review::Error::Interrupted)));
    }

    #[test]
    fn it_should_bound_the_lookup_at_a_quarter_of_a_short_timeout() {
        assert_eq!(lookup_bound(Duration::from_secs(8)), Duration::from_secs(2));
        assert_eq!(lookup_bound(Duration::from_secs(10)), Duration::from_millis(2500));
    }

    #[test]
    fn it_should_cap_the_lookup_bound_at_ten_seconds_for_a_long_timeout() {
        assert_eq!(lookup_bound(Duration::from_secs(40)), Duration::from_secs(10));
        assert_eq!(lookup_bound(Duration::from_secs(900)), Duration::from_secs(10));
        assert_eq!(
            lookup_bound(Duration::from_secs(MAX_TIMEOUT_SECONDS)),
            Duration::from_secs(10)
        );
    }
}
