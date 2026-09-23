//! Pull-request review-thread retrieval through the GitHub CLI.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

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

/// Lists unresolved threads with the first comment URL used by the shell script.
///
/// # Errors
///
/// Returns an error when `response` does not match the expected GraphQL shape.
pub fn list_unresolved(response: &[u8]) -> Result<ListedThreads, Error> {
    let threads = parse_response(response)?
        .threads()
        .filter(|thread| !thread.is_resolved)
        .map(|thread| ListedThread {
            id: thread.id,
            is_outdated: thread.is_outdated,
            path: thread.path,
            url: thread.comments.nodes.first().map(|comment| comment.url.clone()),
        })
        .collect();

    Ok(ListedThreads { threads })
}

/// Lists each unresolved thread's comments and review metadata.
///
/// # Errors
///
/// Returns an error when `response` does not match the expected GraphQL shape.
pub fn show_unresolved(response: &[u8]) -> Result<ShownThreads, Error> {
    let threads = parse_response(response)?
        .threads()
        .filter(|thread| !thread.is_resolved)
        .map(|thread| ShownThread {
            id: thread.id,
            is_outdated: thread.is_outdated,
            path: thread.path,
            comments: thread
                .comments
                .nodes
                .into_iter()
                .map(|comment| ShownComment {
                    url: comment.url,
                    author: comment.author.map(|author| author.login),
                    body: comment.body,
                })
                .collect(),
        })
        .collect();

    Ok(ShownThreads { threads })
}

/// Reports whether each unresolved thread includes a comment by `login`.
///
/// # Errors
///
/// Returns an error when `response` does not match the expected GraphQL shape.
pub fn reply_status(response: &[u8], login: &str) -> Result<ReplyStatus, Error> {
    let threads = parse_response(response)?
        .threads()
        .filter(|thread| !thread.is_resolved)
        .map(|thread| ReplyStatusThread {
            thread_id: thread.id,
            path: thread.path,
            url: thread.comments.nodes.first().map(|comment| comment.url.clone()),
            has_reply: thread
                .comments
                .nodes
                .iter()
                .any(|comment| comment.author.as_ref().is_some_and(|author| author.login == login)),
        })
        .collect::<Vec<_>>();
    let with_reply = threads.iter().filter(|thread| thread.has_reply).count();
    let total = threads.len();

    Ok(ReplyStatus {
        threads,
        summary: ReplyStatusSummary {
            total,
            with_reply,
            without_reply: total - with_reply,
        },
    })
}

/// The single JSON result object emitted by `list`.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ListedThreads {
    threads: Vec<ListedThread>,
}

/// A compact unresolved-thread result.
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListedThread {
    id: String,
    is_outdated: bool,
    path: String,
    url: Option<String>,
}

/// The single JSON result object emitted by `show`.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ShownThreads {
    threads: Vec<ShownThread>,
}

/// An unresolved thread with each review comment.
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShownThread {
    id: String,
    is_outdated: bool,
    path: String,
    comments: Vec<ShownComment>,
}

/// A review comment shown in an unresolved thread.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ShownComment {
    url: String,
    author: Option<String>,
    body: String,
}

/// Reply status for all unresolved threads.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ReplyStatus {
    threads: Vec<ReplyStatusThread>,
    summary: ReplyStatusSummary,
}

impl ReplyStatus {
    /// Returns whether at least one unresolved thread lacks the requested reply.
    #[must_use]
    pub const fn has_missing_replies(&self) -> bool {
        self.without_reply() > 0
    }

    /// Returns how many unresolved threads lack the requested reply.
    #[must_use]
    pub const fn without_reply(&self) -> usize {
        self.summary.without_reply
    }
}

/// Reply state for one unresolved thread.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ReplyStatusThread {
    thread_id: String,
    path: String,
    url: Option<String>,
    has_reply: bool,
}

/// Aggregate reply state for unresolved threads.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ReplyStatusSummary {
    total: usize,
    with_reply: usize,
    without_reply: usize,
}

fn parse_response(response: &[u8]) -> Result<GraphQlResponse, Error> {
    serde_json::from_slice(response).map_err(|error| Error::MalformedResponse(error.to_string()))
}

#[derive(Deserialize)]
struct GraphQlResponse {
    data: GraphQlData,
}

impl GraphQlResponse {
    fn threads(self) -> impl Iterator<Item = ReviewThread> {
        self.data.repository.pull_request.review_threads.nodes.into_iter()
    }
}

#[derive(Deserialize)]
struct GraphQlData {
    repository: Repository,
}

#[derive(Deserialize)]
struct Repository {
    #[serde(rename = "pullRequest")]
    pull_request: PullRequestData,
}

#[derive(Deserialize)]
struct PullRequestData {
    #[serde(rename = "reviewThreads")]
    review_threads: ReviewThreads,
}

#[derive(Deserialize)]
struct ReviewThreads {
    nodes: Vec<ReviewThread>,
}

#[derive(Deserialize)]
struct ReviewThread {
    id: String,
    #[serde(rename = "isResolved")]
    is_resolved: bool,
    #[serde(rename = "isOutdated")]
    is_outdated: bool,
    path: String,
    comments: ReviewComments,
}

#[derive(Deserialize)]
struct ReviewComments {
    nodes: Vec<ReviewComment>,
}

#[derive(Deserialize)]
struct ReviewComment {
    url: String,
    body: String,
    author: Option<ReviewAuthor>,
}

#[derive(Deserialize)]
struct ReviewAuthor {
    login: String,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::{PullRequest, ThreadSource, fetch, list_unresolved, reply_status, show_unresolved};

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

    #[test]
    fn it_should_list_only_unresolved_threads() {
        // Arrange: the fixture has one resolved and two unresolved threads.
        let response = include_bytes!("../tests/fixtures/review-threads.json");

        // Act: list the unresolved threads.
        let threads = list_unresolved(response).unwrap();

        // Assert: the compact projection matches the existing script's selected fields.
        assert_eq!(
            serde_json::to_value(threads).unwrap(),
            json!({"threads":[
                {"id":"THREAD_UNRESOLVED_CURRENT","isOutdated":false,"path":"src/example.rs","url":"https://example.test/pull/42#discussion-3"},
                {"id":"THREAD_UNRESOLVED_OUTDATED","isOutdated":true,"path":"src/legacy.rs","url":"https://example.test/pull/42#discussion-4"}
            ]}),
        );
    }

    #[test]
    fn it_should_show_all_comments_for_each_unresolved_thread() {
        // Arrange: one unresolved fixture thread contains both reviewer and author comments.
        let response = include_bytes!("../tests/fixtures/review-threads.json");

        // Act: project the unresolved threads for detailed display.
        let threads = show_unresolved(response).unwrap();

        // Assert: the detailed projection preserves both comments from the multi-comment thread.
        assert_eq!(threads.threads.len(), 2);
        assert_eq!(
            serde_json::to_value(&threads.threads[1]).unwrap()["comments"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn it_should_report_missing_and_present_author_replies() {
        // Arrange: only the outdated unresolved thread has a comment from the author.
        let response = include_bytes!("../tests/fixtures/review-threads.json");

        // Act: check unresolved threads for the author reply.
        let status = reply_status(response, "author").unwrap();

        // Assert: the reply summary retains the shell script's pass and failure cases.
        assert_eq!(
            serde_json::to_value(status).unwrap(),
            json!({
                "threads":[
                    {"thread_id":"THREAD_UNRESOLVED_CURRENT","path":"src/example.rs","url":"https://example.test/pull/42#discussion-3","has_reply":false},
                    {"thread_id":"THREAD_UNRESOLVED_OUTDATED","path":"src/legacy.rs","url":"https://example.test/pull/42#discussion-4","has_reply":true}
                ],
                "summary":{"total":2,"with_reply":1,"without_reply":1}
            }),
        );
    }
}
