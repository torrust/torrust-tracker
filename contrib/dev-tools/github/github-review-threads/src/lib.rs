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
                    line
                    resolvedBy {
                        login
                    }
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

/// Which fetched threads a projection returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadSelection {
    /// Every thread, including resolved and outdated ones: the evidence view.
    All,
    /// Only unresolved threads: the view for reply and resolution actions.
    UnresolvedOnly,
}

impl ThreadSelection {
    const fn includes(self, thread: &ReviewThread) -> bool {
        matches!(self, Self::All) || !thread.is_resolved
    }
}

/// Lists the selected threads with their resolution evidence and first comment URL.
///
/// # Errors
///
/// Returns an error when `response` does not match the expected GraphQL shape.
pub fn list(response: &[u8], selection: ThreadSelection) -> Result<ListedThreads, Error> {
    let threads = parse_response(response)?
        .threads()
        .filter(|thread| selection.includes(thread))
        .map(|thread| ListedThread {
            url: thread.comments.nodes.first().map(|comment| comment.url.clone()),
            id: thread.id,
            is_resolved: thread.is_resolved,
            is_outdated: thread.is_outdated,
            resolved_by: thread.resolved_by.map(|resolver| resolver.login),
            path: thread.path,
            line: thread.line,
        })
        .collect();

    Ok(ListedThreads { threads })
}

/// Lists the selected threads with their resolution evidence and review comments.
///
/// # Errors
///
/// Returns an error when `response` does not match the expected GraphQL shape.
pub fn show(response: &[u8], selection: ThreadSelection) -> Result<ShownThreads, Error> {
    let threads = parse_response(response)?
        .threads()
        .filter(|thread| selection.includes(thread))
        .map(|thread| ShownThread {
            id: thread.id,
            is_resolved: thread.is_resolved,
            is_outdated: thread.is_outdated,
            resolved_by: thread.resolved_by.map(|resolver| resolver.login),
            path: thread.path,
            line: thread.line,
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

/// A compact thread result with its resolution evidence.
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListedThread {
    id: String,
    is_resolved: bool,
    is_outdated: bool,
    resolved_by: Option<String>,
    path: String,
    line: Option<u64>,
    url: Option<String>,
}

/// The single JSON result object emitted by `show`.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ShownThreads {
    threads: Vec<ShownThread>,
}

/// A thread with its resolution evidence and each review comment.
#[derive(Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShownThread {
    id: String,
    is_resolved: bool,
    is_outdated: bool,
    resolved_by: Option<String>,
    path: String,
    line: Option<u64>,
    comments: Vec<ShownComment>,
}

/// A review comment shown in a thread.
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
    // Absent in files fetched before these fields were requested, and null for outdated lines or deleted resolvers.
    line: Option<u64>,
    #[serde(rename = "resolvedBy")]
    resolved_by: Option<ReviewAuthor>,
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

    use serde_json::{Value, json};

    use super::{
        ListedThread, PullRequest, REVIEW_THREADS_QUERY, ShownThread, ThreadSelection, ThreadSource, fetch, list, reply_status,
        show,
    };

    const THREAD_STATES: &[u8] = include_bytes!("../tests/fixtures/thread-states.json");

    const FETCHED_BEFORE_RESOLVER_AND_LINE: &[u8] = include_bytes!("../tests/fixtures/review-threads.json");

    const FETCH_REVIEW_THREADS_SKILL: &str =
        include_str!("../../../../../.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md");

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

    fn listed_ids(threads: &[ListedThread]) -> Vec<&str> {
        threads.iter().map(|thread| thread.id.as_str()).collect()
    }

    fn shown<'a>(threads: &'a [ShownThread], id: &str) -> &'a ShownThread {
        threads
            .iter()
            .find(|thread| thread.id == id)
            .expect("the fixture should hold the thread")
    }

    /// Splits a GraphQL selection into field names and braces, so layout differences do not matter.
    fn graphql_tokens(query: &str) -> Vec<String> {
        query
            .replace('{', " { ")
            .replace('}', " } ")
            .split_whitespace()
            .map(String::from)
            .collect()
    }

    /// Reads the skill's only `-f query='...'` block; a missing or second block fails here, a changed one fails parity.
    fn skill_fallback_query(skill: &str) -> &str {
        assert_eq!(
            skill.matches("-f query='").count(),
            1,
            "the skill should hold exactly one fallback query"
        );
        let start = skill.find("-f query='").expect("the skill should hold a fallback query") + "-f query='".len();
        let length = skill[start..].find('\'').expect("the fallback query should be closed");
        &skill[start..start + length]
    }

    #[test]
    fn it_should_request_the_line_and_resolver_of_each_thread() {
        // Arrange: evidence fields adjacent to thread-only `path`; adjacency pins them at thread level, not comments.
        let evidence_fields = ["path", "line", "resolvedBy", "{", "login", "}"];

        // Act: read the query sent to GitHub.
        let tokens = graphql_tokens(REVIEW_THREADS_QUERY);

        // Assert: a dropped field would otherwise read back as null, indistinguishable from unknown.
        assert!(tokens.windows(evidence_fields.len()).any(|window| window == evidence_fields));
    }

    #[test]
    fn it_should_document_the_same_fallback_query_the_tool_sends() {
        // Arrange: the `gh api graphql` fallback documented in the `fetch-review-threads` skill.
        let fallback_query = skill_fallback_query(FETCH_REVIEW_THREADS_SKILL);

        // Act: compare its selection with the tool's query, ignoring layout.
        let fallback_tokens = graphql_tokens(fallback_query);

        // Assert: both evidence paths request the same fields in the same shape.
        assert_eq!(fallback_tokens, graphql_tokens(REVIEW_THREADS_QUERY));
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
    fn it_should_list_resolved_unresolved_and_outdated_threads_by_default() {
        // Arrange: the fixture has three resolved threads, one of them outdated, and two unresolved threads.
        let selection = ThreadSelection::All;

        // Act: list every fetched thread.
        let threads = list(THREAD_STATES, selection).unwrap();

        // Assert: no thread is filtered out, and the fetch order is kept.
        assert_eq!(
            listed_ids(&threads.threads),
            [
                "THREAD_RESOLVED_BY_MAINTAINER",
                "THREAD_RESOLVED_OUTDATED",
                "THREAD_RESOLVED_WITHOUT_RESOLVER",
                "THREAD_UNRESOLVED_CURRENT",
                "THREAD_UNRESOLVED_OUTDATED",
            ],
        );
    }

    #[test]
    fn it_should_list_only_unresolved_threads_when_unresolved_only_is_selected() {
        // Arrange: the same fixture, selected for reply or resolution triage.
        let selection = ThreadSelection::UnresolvedOnly;

        // Act: list the threads still awaiting action.
        let threads = list(THREAD_STATES, selection).unwrap();

        // Assert: every resolved thread, current or outdated, is excluded.
        assert_eq!(
            listed_ids(&threads.threads),
            ["THREAD_UNRESOLVED_CURRENT", "THREAD_UNRESOLVED_OUTDATED"],
        );
    }

    #[test]
    fn it_should_list_the_resolution_evidence_of_a_resolved_thread() {
        // Arrange: the fixture thread resolved by `maintainer` on line 12.
        let resolved_thread_id = "THREAD_RESOLVED_BY_MAINTAINER";

        // Act: list every fetched thread.
        let threads = list(THREAD_STATES, ThreadSelection::All).unwrap();

        // Assert: the compact row names the resolver and the line alongside the existing fields.
        let row = threads.threads.iter().find(|thread| thread.id == resolved_thread_id).unwrap();
        assert_eq!(
            serde_json::to_value(row).unwrap(),
            json!({
                "id": resolved_thread_id,
                "isResolved": true,
                "isOutdated": false,
                "resolvedBy": "maintainer",
                "path": "src/resolved.rs",
                "line": 12,
                "url": "https://example.test/pull/43#discussion-1",
            }),
        );
    }

    #[test]
    fn it_should_show_who_resolved_a_thread_and_on_which_line() {
        // Arrange: the fixture thread resolved by `maintainer` on line 12.
        let resolved_thread_id = "THREAD_RESOLVED_BY_MAINTAINER";

        // Act: show every fetched thread.
        let threads = show(THREAD_STATES, ThreadSelection::All).unwrap();

        // Assert: the resolution evidence is exposed rather than filtered away.
        let thread = shown(&threads.threads, resolved_thread_id);
        assert!(thread.is_resolved);
        assert_eq!(thread.resolved_by.as_deref(), Some("maintainer"));
        assert_eq!(thread.line, Some(12));
    }

    #[test]
    fn it_should_keep_a_missing_resolver_as_an_explicit_null() {
        // Arrange: a resolved fixture thread whose `resolvedBy` is null, as for a deleted account.
        let thread_without_resolver = "THREAD_RESOLVED_WITHOUT_RESOLVER";

        // Act: show every fetched thread.
        let threads = show(THREAD_STATES, ThreadSelection::All).unwrap();

        // Assert: the resolver is null in the JSON result and no login is synthesized.
        let thread = serde_json::to_value(shown(&threads.threads, thread_without_resolver)).unwrap();
        assert_eq!(thread["isResolved"], true);
        assert_eq!(thread.get("resolvedBy"), Some(&Value::Null));
    }

    #[test]
    fn it_should_keep_the_missing_line_of_an_outdated_thread_as_an_explicit_null() {
        // Arrange: an outdated fixture thread whose `line` is null because its line was rewritten.
        let outdated_thread = "THREAD_UNRESOLVED_OUTDATED";

        // Act: show every fetched thread.
        let threads = show(THREAD_STATES, ThreadSelection::All).unwrap();

        // Assert: the line is null in the JSON result and the path still locates the thread.
        let thread = serde_json::to_value(shown(&threads.threads, outdated_thread)).unwrap();
        assert_eq!(thread["isOutdated"], true);
        assert_eq!(thread.get("line"), Some(&Value::Null));
        assert_eq!(thread["path"], "src/legacy.rs");
    }

    #[test]
    fn it_should_read_a_file_fetched_before_the_resolver_and_line_were_requested() {
        // Arrange: a response captured by the retired scripts, whose threads have no `resolvedBy` or `line`.
        let response = FETCHED_BEFORE_RESOLVER_AND_LINE;

        // Act: show every thread in the older file.
        let threads = show(response, ThreadSelection::All).unwrap();

        // Assert: every thread is read, with the absent fields reported as unknown.
        assert_eq!(threads.threads.len(), 3);
        assert!(
            threads
                .threads
                .iter()
                .all(|thread| thread.resolved_by.is_none() && thread.line.is_none())
        );
    }

    #[test]
    fn it_should_list_only_unresolved_threads() {
        // Arrange: the fixture has one resolved and two unresolved threads.
        let response = FETCHED_BEFORE_RESOLVER_AND_LINE;

        // Act: list the unresolved threads.
        let threads = list(response, ThreadSelection::UnresolvedOnly).unwrap();

        // Assert: the compact projection keeps the retired script's rows and adds the new evidence fields.
        assert_eq!(
            serde_json::to_value(threads).unwrap(),
            json!({"threads":[
                {"id":"THREAD_UNRESOLVED_CURRENT","isResolved":false,"isOutdated":false,"resolvedBy":null,"path":"src/example.rs","line":null,"url":"https://example.test/pull/42#discussion-3"},
                {"id":"THREAD_UNRESOLVED_OUTDATED","isResolved":false,"isOutdated":true,"resolvedBy":null,"path":"src/legacy.rs","line":null,"url":"https://example.test/pull/42#discussion-4"}
            ]}),
        );
    }

    #[test]
    fn it_should_show_all_comments_for_each_unresolved_thread() {
        // Arrange: one unresolved fixture thread contains both reviewer and author comments.
        let response = FETCHED_BEFORE_RESOLVER_AND_LINE;

        // Act: project the unresolved threads for detailed display.
        let threads = show(response, ThreadSelection::UnresolvedOnly).unwrap();

        // Assert: the detailed projection preserves both comments from the multi-comment thread.
        assert_eq!(threads.threads.len(), 2);
        assert_eq!(threads.threads[1].comments.len(), 2);
    }

    #[test]
    fn it_should_check_replies_on_unresolved_threads_only() {
        // Arrange: the fixture's resolved threads have no `author` comment, while both unresolved threads do.
        let login = "author";

        // Act: check the fixture for replies from that login.
        let status = reply_status(THREAD_STATES, login).unwrap();

        // Assert: the resolution guard ignores resolved threads, so no reply is reported missing.
        assert_eq!(
            status
                .threads
                .iter()
                .map(|thread| thread.thread_id.as_str())
                .collect::<Vec<_>>(),
            ["THREAD_UNRESOLVED_CURRENT", "THREAD_UNRESOLVED_OUTDATED"],
        );
        assert!(!status.has_missing_replies());
    }

    #[test]
    fn it_should_report_missing_and_present_author_replies() {
        // Arrange: only the outdated unresolved thread has a comment from the author.
        let response = FETCHED_BEFORE_RESOLVER_AND_LINE;

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
