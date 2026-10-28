//! Failures of the poller, each with a stable `kind` for the NDJSON diagnostic on stderr.

use thiserror::Error;

/// A failure that stops the poller before it can report a result.
#[derive(Debug, Error)]
pub enum Error {
    /// The GitHub CLI executable could not be started.
    #[error("the GitHub CLI `{program}` cannot be started: {reason}")]
    GhUnavailable {
        /// The program that was started.
        program: String,
        /// The operating-system reason.
        reason: String,
    },
    /// The GitHub CLI is installed but holds no valid login.
    #[error("the GitHub CLI is not authenticated; run `gh auth login`: {0}")]
    GhNotAuthenticated(String),
    /// A GitHub API request made through the CLI failed.
    #[error("`gh api {endpoint}` failed: {message}")]
    GhApi {
        /// The requested endpoint.
        endpoint: String,
        /// The error text printed by `gh`.
        message: String,
    },
    /// A GitHub API response did not have the expected JSON shape.
    #[error("the response of `gh api {endpoint}` is not the expected JSON: {reason}")]
    MalformedResponse {
        /// The requested endpoint.
        endpoint: String,
        /// Why the response was rejected.
        reason: String,
    },
    /// The commit named by `--since-head-sha` has no usable committer date.
    #[error("commit {sha} has no usable committer date")]
    CommitDate {
        /// The commit that was looked up.
        sha: String,
    },
    /// The wait budget ran out while a `gh` call was still running, so the call was killed.
    #[error("the wait budget ended while a `gh` call was still running")]
    DeadlineExceeded,
    /// A `SIGINT` or `SIGTERM` stopped the poller.
    #[error("interrupted by a signal")]
    Interrupted,
}

impl Error {
    /// The `kind` field of the diagnostic written to stderr.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::GhUnavailable { .. } => "gh_unavailable",
            Self::GhNotAuthenticated(_) => "gh_not_authenticated",
            Self::GhApi { .. } => "gh_api_error",
            Self::MalformedResponse { .. } => "malformed_response",
            Self::CommitDate { .. } => "commit_date_error",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::Interrupted => "interrupted",
        }
    }
}
