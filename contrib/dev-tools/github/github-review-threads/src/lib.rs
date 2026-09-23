//! Pull-request review-thread retrieval through the GitHub CLI.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

const REVIEW_THREADS_QUERY: &str = r"query($owner: String!, $repo: String!, $pullNumber: Int!) {
    repository(owner: $owner, name: $repo) {
        pullRequest(number: $pullNumber) {
            reviewThreads(first: 100) {
                nodes {
                    id
                    isResolved
                    isOutdated
                    path
                    isCollapsed
                    comments(first: 20) {
                        nodes {
                            url
                            body
                            createdAt
                            author {
                                login
                            }
                        }
                    }
                }
            }
        }
    }
}";

/// The pull request whose review threads are requested.
#[derive(Debug, Eq, PartialEq)]
pub struct PullRequest {
    /// GitHub repository owner.
    pub owner: String,
    /// GitHub repository name.
    pub repository: String,
    /// Pull-request number.
    pub number: u64,
}

/// Source of a raw GitHub GraphQL review-thread response.
pub trait ThreadSource {
    /// Fetches the raw response for `pull_request`.
    ///
    /// # Errors
    ///
    /// Returns an error when the source cannot obtain a response.
    fn fetch(&self, pull_request: &PullRequest) -> Result<Vec<u8>, Error>;
}

/// A review-thread source implemented with the installed GitHub CLI.
#[derive(Debug, Default)]
pub struct GhCli;

impl ThreadSource for GhCli {
    fn fetch(&self, pull_request: &PullRequest) -> Result<Vec<u8>, Error> {
        let output = Command::new("gh")
            .args([
                "api",
                "graphql",
                "-F",
                &format!("owner={}", pull_request.owner),
                "-F",
                &format!("repo={}", pull_request.repository),
                "-F",
                &format!("pullNumber={}", pull_request.number),
                "-f",
                &format!("query={REVIEW_THREADS_QUERY}"),
            ])
            .output()
            .map_err(|error| Error::Source(error.to_string()))?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(Error::Source(String::from_utf8_lossy(&output.stderr).trim().to_owned()))
        }
    }
}

/// Fetches a raw response and writes it to `output_file` without altering its shape.
///
/// # Errors
///
/// Returns an error when the source fails or the output file cannot be written.
pub fn fetch(source: &impl ThreadSource, pull_request: &PullRequest, output_file: &Path) -> Result<FetchResult, Error> {
    let response = source.fetch(pull_request)?;
    serde_json::from_slice::<serde_json::Value>(&response).map_err(|error| Error::MalformedResponse(error.to_string()))?;
    fs::write(output_file, response).map_err(|error| Error::OutputFile(error.to_string()))?;

    Ok(FetchResult {
        status: "ok",
        pr_number: pull_request.number,
        output_file: output_file.to_path_buf(),
    })
}

/// The JSON result emitted after a successful fetch.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct FetchResult {
    status: &'static str,
    pr_number: u64,
    output_file: PathBuf,
}

/// A fetch operation failure safe to render as a JSON diagnostic.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The configured source could not return the GraphQL response.
    Source(String),
    /// The source returned bytes that are not valid JSON.
    MalformedResponse(String),
    /// The response could not be written to the requested path.
    OutputFile(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source(message) => write!(formatter, "review-thread source failed: {message}"),
            Self::MalformedResponse(message) => write!(formatter, "review-thread response is not valid JSON: {message}"),
            Self::OutputFile(message) => write!(formatter, "cannot write review-thread response: {message}"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{PullRequest, ThreadSource, fetch};

    struct FixtureSource;

    struct MalformedSource;

    impl ThreadSource for FixtureSource {
        fn fetch(&self, _: &PullRequest) -> Result<Vec<u8>, super::Error> {
            Ok(include_bytes!("../tests/fixtures/review-threads.json").to_vec())
        }
    }

    impl ThreadSource for MalformedSource {
        fn fetch(&self, _: &PullRequest) -> Result<Vec<u8>, super::Error> {
            Ok(Vec::from("not-json"))
        }
    }

    #[test]
    fn it_should_write_the_fixture_response_without_changing_its_shape() {
        // Arrange: the source returns the captured GraphQL response for a pull request.
        let source = FixtureSource;
        let pull_request = PullRequest {
            owner: String::from("torrust"),
            repository: String::from("torrust-tracker"),
            number: 42,
        };
        let temporary_directory = tempfile::tempdir().unwrap();
        let output_file = temporary_directory.path().join("review-threads.json");

        // Act: fetch the response into the requested output file.
        let result = fetch(&source, &pull_request, &output_file).unwrap();

        // Assert: the bytes and JSON result preserve the shell-script data contract.
        assert_eq!(
            fs::read(&output_file).unwrap(),
            include_bytes!("../tests/fixtures/review-threads.json"),
        );
        assert_eq!(result.pr_number, 42);
        assert_eq!(result.status, "ok");
        assert_eq!(result.output_file, output_file);
    }

    #[test]
    fn it_should_not_write_a_malformed_response() {
        // Arrange: the source returns bytes that are not a GraphQL JSON response.
        let source = MalformedSource;
        let pull_request = PullRequest {
            owner: String::from("torrust"),
            repository: String::from("torrust-tracker"),
            number: 42,
        };
        let temporary_directory = tempfile::tempdir().unwrap();
        let output_file = temporary_directory.path().join("review-threads.json");

        // Act: fetch attempts to validate and write the response.
        let error = fetch(&source, &pull_request, &output_file).unwrap_err();

        // Assert: invalid source data produces an error before a file is created.
        assert!(matches!(error, super::Error::MalformedResponse(_)));
        assert!(!output_file.exists());
    }
}
