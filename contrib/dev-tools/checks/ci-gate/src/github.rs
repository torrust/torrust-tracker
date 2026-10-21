//! The GitHub adapter of the merge verdict.
//!
//! The verdict ([`crate::verdict`]) is provider-neutral: it needs the classification, the result of
//! each group, and an optional override actor. This module produces the last two from GitHub, as
//! decided for issue #2525, task T5:
//!
//! - **Results.** The latest check run of each group's job for the head commit, by the Actions app,
//!   whose name is the name of the group. A skipped run is ignored, because a label event skips the
//!   group jobs and must not hide the earlier result, and a run that has not completed is missing.
//! - **Override.** The label [`OVERRIDE_LABEL`] counts only when it is on the pull request, when the
//!   actor of its latest `labeled` event has the role `admin` or `maintain`, and when it was applied
//!   after the head commit's first check suite was created, because a push revokes an override. An
//!   override that is ignored never fails the command: the verdict is computed without it and the
//!   reason is printed.
//! - **Failure.** Everything is read through the [`GithubApi`] trait, so the rules are tested with
//!   fixtures. An error reading the check runs is an error of the command, so the gate fails closed. An
//!   error reading anything the override needs means that there is no override.
//!
//! The workflow only runs the command; it does not look at any of this.

use std::collections::BTreeMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::change_detection::Classification;
use crate::job_group::JobGroup;
use crate::verdict::{GroupResult, Verdict, VerdictResult, decide};

/// The label that asks for a maintainer override of the verdict.
pub const OVERRIDE_LABEL: &str = "ci-override";

/// The application that creates the check runs of the group jobs.
const ACTIONS_APP: &str = "github-actions";

/// The roles of a collaborator whose override counts.
const OVERRIDE_ROLES: [&str; 2] = ["admin", "maintain"];

/// Read access to the GitHub REST API.
pub trait GithubApi {
    /// Returns every page of a GET request, in order. A request that is not paginated returns one page.
    ///
    /// # Errors
    ///
    /// Returns a message when the request fails or its response is not JSON.
    fn pages(&self, path: &str) -> Result<Vec<Value>, String>;
}

/// The pull request whose verdict is decided.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PullRequestRef {
    repository: String,
    number: u64,
    head_sha: String,
}

impl PullRequestRef {
    /// Checks the parts that become part of a request path, so that they cannot change the request.
    ///
    /// # Errors
    ///
    /// Returns a message when `repository` is not `owner/name`, or `head_sha` is not 40 hexadecimal
    /// digits.
    pub fn new(repository: &str, number: u64, head_sha: &str) -> Result<Self, String> {
        let valid_part = |part: &str| {
            !part.is_empty()
                && part != "."
                && part != ".."
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        };
        let is_repository = repository
            .split_once('/')
            .is_some_and(|(owner, name)| valid_part(owner) && valid_part(name));
        if !is_repository {
            return Err(format!("`{repository}` is not a repository of the form `owner/name`"));
        }
        if head_sha.len() != 40 || !head_sha.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("`{head_sha}` is not a commit id of 40 hexadecimal digits"));
        }

        Ok(Self {
            repository: repository.to_string(),
            number,
            head_sha: head_sha.to_string(),
        })
    }

    /// The repository, as `owner/name`.
    #[must_use]
    pub fn repository(&self) -> &str {
        &self.repository
    }
}

/// The verdict, and why an override that was asked for was not used.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GithubVerdict {
    #[serde(flatten)]
    pub verdict: VerdictResult,
    /// Why the override label did not apply, when it is on the pull request and the verdict failed
    /// without it.
    pub override_ignored: Option<String>,
}

#[derive(Deserialize)]
struct CheckRunsPage {
    check_runs: Vec<CheckRun>,
}

#[derive(Deserialize)]
struct CheckRun {
    id: u64,
    name: String,
    status: String,
    conclusion: Option<String>,
    app: Option<App>,
}

#[derive(Deserialize)]
struct App {
    slug: String,
}

#[derive(Deserialize)]
struct Label {
    name: String,
}

#[derive(Deserialize)]
struct Event {
    #[serde(rename = "event")]
    kind: String,
    label: Option<Label>,
    actor: Option<User>,
    created_at: String,
}

#[derive(Deserialize)]
struct User {
    login: String,
}

#[derive(Deserialize)]
struct CheckSuitesPage {
    check_suites: Vec<CheckSuite>,
}

#[derive(Deserialize)]
struct CheckSuite {
    created_at: String,
}

#[derive(Deserialize)]
struct Permission {
    permission: String,
}

/// Decides whether a pull request may merge, from what GitHub says about its head commit.
///
/// The override is looked up only when the verdict fails without it.
///
/// # Errors
///
/// Returns a message when the check runs cannot be read, or when `classification` does not list every
/// job group exactly once.
pub fn decide_for_pull_request(
    api: &dyn GithubApi,
    pull_request: &PullRequestRef,
    classification: &Classification,
) -> Result<GithubVerdict, String> {
    let check_runs: Vec<CheckRun> = fetch_wrapped(
        api,
        &format!(
            "repos/{}/commits/{}/check-runs?per_page=100",
            pull_request.repository, pull_request.head_sha
        ),
        |page: CheckRunsPage| page.check_runs,
    )
    .map_err(|error| format!("cannot read the check runs of the head commit: {error}"))?;
    let results = group_results(&check_runs);

    let without_override = decide(classification, &results, None)?;
    if without_override.verdict == Verdict::Pass {
        return Ok(GithubVerdict {
            verdict: without_override,
            override_ignored: None,
        });
    }

    match assess_override(api, pull_request) {
        OverrideAssessment::NotRequested => Ok(GithubVerdict {
            verdict: without_override,
            override_ignored: None,
        }),
        OverrideAssessment::Ignored(reason) => Ok(GithubVerdict {
            verdict: without_override,
            override_ignored: Some(reason),
        }),
        OverrideAssessment::Applies(actor) => Ok(GithubVerdict {
            verdict: decide(classification, &results, Some(&actor))?,
            override_ignored: None,
        }),
    }
}

/// The result of each group, from the check runs of the head commit.
fn group_results(check_runs: &[CheckRun]) -> BTreeMap<JobGroup, GroupResult> {
    let mut latest: BTreeMap<JobGroup, &CheckRun> = BTreeMap::new();

    for run in check_runs {
        if run.app.as_ref().map(|app| app.slug.as_str()) != Some(ACTIONS_APP) || run.conclusion.as_deref() == Some("skipped") {
            continue;
        }
        let Some(group) = JobGroup::ALL.into_iter().find(|group| group.name() == run.name) else {
            continue;
        };
        // A higher id is a later run, also when a job is rerun.
        if latest.get(&group).is_none_or(|current| run.id > current.id) {
            latest.insert(group, run);
        }
    }

    latest
        .into_iter()
        .filter_map(|(group, run)| result_of(run).map(|result| (group, result)))
        .collect()
}

/// The result of a run that has completed. Anything but a success or a cancellation is a failure.
fn result_of(run: &CheckRun) -> Option<GroupResult> {
    if run.status != "completed" {
        return None;
    }

    Some(match run.conclusion.as_deref() {
        Some("success") => GroupResult::Success,
        Some("cancelled") => GroupResult::Cancelled,
        _ => GroupResult::Failure,
    })
}

enum OverrideAssessment {
    NotRequested,
    Ignored(String),
    Applies(String),
}

/// Who applied the override label, and when.
struct OverrideRequest {
    actor: Option<String>,
    applied_at: String,
}

fn assess_override(api: &dyn GithubApi, pull_request: &PullRequestRef) -> OverrideAssessment {
    let base = format!("repos/{}", pull_request.repository);

    let labels: Vec<Label> = match fetch_list(api, &format!("{base}/issues/{}/labels?per_page=100", pull_request.number)) {
        Ok(labels) => labels,
        Err(error) => return OverrideAssessment::Ignored(format!("cannot read the labels of the pull request: {error}")),
    };
    let events: Vec<Event> = match fetch_list(api, &format!("{base}/issues/{}/events?per_page=100", pull_request.number)) {
        Ok(events) => events,
        Err(error) => return OverrideAssessment::Ignored(format!("cannot read the events of the pull request: {error}")),
    };
    let Some(request) = override_request(&labels, &events) else {
        return OverrideAssessment::NotRequested;
    };
    let Some(actor) = request.actor.as_deref() else {
        return OverrideAssessment::Ignored("the actor who applied the label is not known".to_string());
    };
    if !is_login(actor) {
        return OverrideAssessment::Ignored(format!("`{actor}` is not a valid login"));
    }

    let first_check_suite_at = fetch_wrapped(
        api,
        &format!("{base}/commits/{}/check-suites?per_page=100", pull_request.head_sha),
        |page: CheckSuitesPage| page.check_suites,
    )
    .map(|suites| suites.into_iter().map(|suite| suite.created_at).min());
    let first_check_suite_at = match first_check_suite_at {
        Ok(time) => time,
        Err(error) => return OverrideAssessment::Ignored(format!("cannot read the check suites of the head commit: {error}")),
    };

    let permission = api
        .pages(&format!("{base}/collaborators/{actor}/permission"))
        .and_then(|pages| {
            pages
                .into_iter()
                .next()
                .ok_or_else(|| "the response is empty".to_string())
                .and_then(|page| serde_json::from_value::<Permission>(page).map_err(|error| error.to_string()))
        })
        .map(|permission| permission.permission);

    match judge_override(
        &request.applied_at,
        first_check_suite_at.as_deref(),
        permission.as_deref().map_err(String::as_str),
    ) {
        Ok(()) => OverrideAssessment::Applies(actor.to_string()),
        Err(reason) => OverrideAssessment::Ignored(format!("the override by `{actor}` was ignored: {reason}")),
    }
}

/// The latest `labeled` event of the override label, when the label is still on the pull request.
fn override_request(labels: &[Label], events: &[Event]) -> Option<OverrideRequest> {
    if !labels.iter().any(|label| label.name == OVERRIDE_LABEL) {
        return None;
    }

    events
        .iter()
        .filter(|event| event.kind == "labeled" && event.label.as_ref().is_some_and(|label| label.name == OVERRIDE_LABEL))
        .max_by(|a, b| a.created_at.cmp(&b.created_at))
        .map(|event| OverrideRequest {
            actor: event.actor.as_ref().map(|user| user.login.clone()),
            applied_at: event.created_at.clone(),
        })
}

/// Whether an override applied at `applied_at` still counts, as `Ok`, or why it does not.
///
/// It must have been applied after the head commit's first check suite, which a push creates, and by
/// someone whose permission is `admin` or `maintain`. A permission that could not be read is an error.
fn judge_override(applied_at: &str, first_check_suite_at: Option<&str>, permission: Result<&str, &str>) -> Result<(), String> {
    let Some(first_check_suite_at) = first_check_suite_at else {
        return Err(
            "the head commit has no check suite, so it cannot be told whether the label is older than the push".to_string(),
        );
    };
    if !is_timestamp(applied_at) || !is_timestamp(first_check_suite_at) {
        return Err(format!(
            "the times `{applied_at}` and `{first_check_suite_at}` are not in the form of `2026-10-21T10:49:13Z`"
        ));
    }
    if applied_at <= first_check_suite_at {
        return Err(format!(
            "the label was applied at {applied_at}, not after the head commit's first check suite at {first_check_suite_at}, so a push revoked it; remove the label and apply it again"
        ));
    }

    match permission {
        Ok(permission) if OVERRIDE_ROLES.contains(&permission) => Ok(()),
        Ok(permission) => Err(format!(
            "the permission `{permission}` is not one of {}",
            OVERRIDE_ROLES.join(", ")
        )),
        Err(error) => Err(format!("cannot read the permission: {error}")),
    }
}

/// A time as GitHub writes it, which compares correctly as text.
fn is_timestamp(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 20
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            4 | 7 => *byte == b'-',
            10 => *byte == b'T',
            13 | 16 => *byte == b':',
            19 => *byte == b'Z',
            _ => byte.is_ascii_digit(),
        })
}

/// A login that cannot change a request path: letters, digits, hyphens, and the `[bot]` suffix.
fn is_login(login: &str) -> bool {
    let name = login.strip_suffix("[bot]").unwrap_or(login);
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

fn fetch_list<T: DeserializeOwned>(api: &dyn GithubApi, path: &str) -> Result<Vec<T>, String> {
    let mut items = Vec::new();
    for page in api.pages(path)? {
        let page_items: Vec<T> =
            serde_json::from_value(page).map_err(|error| format!("unexpected response of `{path}`: {error}"))?;
        items.extend(page_items);
    }
    Ok(items)
}

fn fetch_wrapped<P: DeserializeOwned, T>(api: &dyn GithubApi, path: &str, items: fn(P) -> Vec<T>) -> Result<Vec<T>, String> {
    let mut all = Vec::new();
    for page in api.pages(path)? {
        let page: P = serde_json::from_value(page).map_err(|error| format!("unexpected response of `{path}`: {error}"))?;
        all.extend(items(page));
    }
    Ok(all)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use rstest::rstest;
    use serde_json::json;

    use super::*;
    use crate::change_detection::classify;

    const REPOSITORY: &str = "owner/repo";
    const HEAD_SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const CHECK_RUNS_PATH: &str = "repos/owner/repo/commits/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/check-runs?per_page=100";
    const LABELS_PATH: &str = "repos/owner/repo/issues/7/labels?per_page=100";
    const EVENTS_PATH: &str = "repos/owner/repo/issues/7/events?per_page=100";
    const CHECK_SUITES_PATH: &str = "repos/owner/repo/commits/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/check-suites?per_page=100";

    /// Answers each request with a fixture, and remembers which requests were made.
    struct FakeApi {
        responses: std::collections::BTreeMap<String, Result<Vec<Value>, String>>,
        requests: RefCell<Vec<String>>,
    }

    impl FakeApi {
        fn new() -> Self {
            Self {
                responses: std::collections::BTreeMap::new(),
                requests: RefCell::new(Vec::new()),
            }
        }

        fn with(mut self, path: &str, pages: Vec<Value>) -> Self {
            self.responses.insert(path.to_string(), Ok(pages));
            self
        }

        fn failing(mut self, path: &str, error: &str) -> Self {
            self.responses.insert(path.to_string(), Err(error.to_string()));
            self
        }

        fn requests(&self) -> Vec<String> {
            self.requests.borrow().clone()
        }
    }

    impl GithubApi for FakeApi {
        fn pages(&self, path: &str) -> Result<Vec<Value>, String> {
            self.requests.borrow_mut().push(path.to_string());
            self.responses
                .get(path)
                .cloned()
                .unwrap_or_else(|| Err(format!("the test has no fixture for {path}")))
        }
    }

    fn pull_request() -> PullRequestRef {
        PullRequestRef::new(REPOSITORY, 7, HEAD_SHA).expect("the pull request reference is valid")
    }

    fn check_run(id: u64, name: &str, status: &str, conclusion: Option<&str>) -> Value {
        json!({ "id": id, "name": name, "status": status, "conclusion": conclusion, "app": { "slug": "github-actions" } })
    }

    fn check_runs_page(runs: Vec<Value>) -> Vec<Value> {
        vec![json!({ "total_count": runs.len(), "check_runs": Value::Array(runs) })]
    }

    fn labeled_event(actor: &str, at: &str) -> Value {
        json!({ "event": "labeled", "label": { "name": "ci-override" }, "actor": { "login": actor }, "created_at": at })
    }

    fn permission_path(login: &str) -> String {
        format!("repos/owner/repo/collaborators/{login}/permission")
    }

    // The change needs `docs-lint` and `db-benchmarking`, as the sandbox pull request did.
    fn classification_needing_db_benchmarking() -> Classification {
        classify(&["packages/persistence-benchmark/note.md"], "develop")
    }

    /// The check runs of a head commit whose `db-benchmarking` failed, which is the situation an override is for.
    fn failing_head() -> FakeApi {
        FakeApi::new().with(
            CHECK_RUNS_PATH,
            check_runs_page(vec![
                check_run(1, "docs-lint", "completed", Some("success")),
                check_run(2, "db-benchmarking", "completed", Some("failure")),
            ]),
        )
    }

    fn api_with_override(api: FakeApi, actor: &str, applied_at: &str, first_check_suite_at: &str, permission: &str) -> FakeApi {
        api.with(LABELS_PATH, vec![json!([{ "name": "ci-override" }])])
            .with(EVENTS_PATH, vec![json!([labeled_event(actor, applied_at)])])
            .with(
                CHECK_SUITES_PATH,
                vec![json!({ "check_suites": [{ "created_at": first_check_suite_at }] })],
            )
            .with(&permission_path(actor), vec![json!({ "permission": permission })])
    }

    #[rstest]
    #[case::success("completed", Some("success"), Some(GroupResult::Success))]
    #[case::failure("completed", Some("failure"), Some(GroupResult::Failure))]
    #[case::cancelled("completed", Some("cancelled"), Some(GroupResult::Cancelled))]
    #[case::timed_out_is_a_failure("completed", Some("timed_out"), Some(GroupResult::Failure))]
    #[case::an_unknown_conclusion_is_a_failure("completed", Some("something_new"), Some(GroupResult::Failure))]
    #[case::skipped_is_ignored("completed", Some("skipped"), None)]
    #[case::in_progress_is_missing("in_progress", None, None)]
    #[case::queued_is_missing("queued", None, None)]
    fn it_should_map_the_check_run_of_a_group_to_its_result(
        #[case] status: &str,
        #[case] conclusion: Option<&str>,
        #[case] expected: Option<GroupResult>,
    ) {
        // Arrange
        let page = check_runs_page(vec![check_run(10, "testing", status, conclusion)]);
        let check_runs =
            fetch_wrapped(&FakeApi::new().with("p", page), "p", |page: CheckRunsPage| page.check_runs).expect("valid fixture");

        // Act
        let results = group_results(&check_runs);

        // Assert
        assert_eq!(
            results.get(&JobGroup::Testing).copied(),
            expected,
            "a check run `{status}`/`{conclusion:?}` of `testing`"
        );
    }

    #[rstest]
    #[case::another_application(json!({ "id": 10, "name": "testing", "status": "completed", "conclusion": "success", "app": { "slug": "copilot" } }))]
    #[case::no_application(json!({ "id": 10, "name": "testing", "status": "completed", "conclusion": "success", "app": null }))]
    #[case::a_name_that_is_not_a_group(json!({ "id": 10, "name": "Testing", "status": "completed", "conclusion": "success", "app": { "slug": "github-actions" } }))]
    #[case::the_name_of_a_skipped_matrix_job(json!({ "id": 10, "name": "matrix.group", "status": "completed", "conclusion": "skipped", "app": { "slug": "github-actions" } }))]
    fn it_should_ignore_a_check_run_that_is_not_a_group_job_of_the_actions_app(#[case] run: Value) {
        // Arrange
        let page = check_runs_page(vec![run.clone()]);
        let check_runs =
            fetch_wrapped(&FakeApi::new().with("p", page), "p", |page: CheckRunsPage| page.check_runs).expect("valid fixture");

        // Act
        let results = group_results(&check_runs);

        // Assert
        assert!(
            results.is_empty(),
            "the check run {run} must not give a result, but gave {results:?}"
        );
    }

    #[rstest]
    #[case::a_rerun_that_passed_replaces_the_failure(
        vec![check_run(10, "testing", "completed", Some("failure")), check_run(11, "testing", "completed", Some("success"))],
        Some(GroupResult::Success)
    )]
    #[case::the_order_of_the_list_does_not_matter(
        vec![check_run(11, "testing", "completed", Some("success")), check_run(10, "testing", "completed", Some("failure"))],
        Some(GroupResult::Success)
    )]
    #[case::a_later_run_that_failed_replaces_the_success(
        vec![check_run(10, "testing", "completed", Some("success")), check_run(11, "testing", "completed", Some("failure"))],
        Some(GroupResult::Failure)
    )]
    #[case::a_later_run_in_progress_makes_the_result_missing(
        vec![check_run(10, "testing", "completed", Some("success")), check_run(11, "testing", "in_progress", None)],
        None
    )]
    #[case::a_later_skipped_run_does_not_hide_the_earlier_result(
        vec![check_run(10, "testing", "completed", Some("failure")), check_run(11, "testing", "completed", Some("skipped"))],
        Some(GroupResult::Failure)
    )]
    fn it_should_use_the_latest_run_of_a_group_that_is_not_skipped(
        #[case] runs: Vec<Value>,
        #[case] expected: Option<GroupResult>,
    ) {
        // Arrange
        let check_runs = fetch_wrapped(
            &FakeApi::new().with("p", check_runs_page(runs)),
            "p",
            |page: CheckRunsPage| page.check_runs,
        )
        .expect("valid fixture");

        // Act
        let results = group_results(&check_runs);

        // Assert
        assert_eq!(results.get(&JobGroup::Testing).copied(), expected);
    }

    #[test]
    fn it_should_give_the_result_of_every_group_when_several_groups_have_runs() {
        // Arrange
        let runs = vec![
            check_run(1, "docs-lint", "completed", Some("success")),
            check_run(2, "db-benchmarking", "completed", Some("failure")),
            check_run(3, "coverage", "completed", Some("cancelled")),
        ];
        let check_runs = fetch_wrapped(
            &FakeApi::new().with("p", check_runs_page(runs)),
            "p",
            |page: CheckRunsPage| page.check_runs,
        )
        .expect("valid fixture");

        // Act
        let results = group_results(&check_runs);

        // Assert
        assert_eq!(
            results,
            BTreeMap::from([
                (JobGroup::DocsLint, GroupResult::Success),
                (JobGroup::Coverage, GroupResult::Cancelled),
                (JobGroup::DbBenchmarking, GroupResult::Failure),
            ])
        );
    }

    #[test]
    fn it_should_find_no_override_request_when_the_label_is_not_on_the_pull_request() {
        // Arrange
        let labels = vec![Label { name: "bug".to_string() }];
        let events =
            vec![serde_json::from_value::<Event>(labeled_event("maintainer", "2026-10-21T10:00:00Z")).expect("valid event")];

        // Act
        let request = override_request(&labels, &events);

        // Assert
        assert!(
            request.is_none(),
            "an event about a label that was removed since must not count"
        );
    }

    #[test]
    fn it_should_take_the_actor_and_time_of_the_latest_labeled_event_when_the_label_was_applied_twice() {
        // Arrange
        let labels = vec![Label {
            name: OVERRIDE_LABEL.to_string(),
        }];
        let events: Vec<Event> = vec![
            labeled_event("first", "2026-10-21T10:00:00Z"),
            json!({ "event": "unlabeled", "label": { "name": "ci-override" }, "actor": { "login": "first" }, "created_at": "2026-10-21T10:30:00Z" }),
            json!({ "event": "labeled", "label": { "name": "bug" }, "actor": { "login": "someone" }, "created_at": "2026-10-21T10:45:00Z" }),
            labeled_event("second", "2026-10-21T11:00:00Z"),
        ]
        .into_iter()
        .map(|event| serde_json::from_value(event).expect("valid event"))
        .collect();

        // Act
        let request = override_request(&labels, &events).expect("the label is on the pull request");

        // Assert
        assert_eq!(
            (request.actor.as_deref(), request.applied_at.as_str()),
            (Some("second"), "2026-10-21T11:00:00Z")
        );
    }

    #[test]
    fn it_should_find_an_unknown_actor_when_the_latest_labeled_event_has_none() {
        // Arrange
        let labels = vec![Label {
            name: OVERRIDE_LABEL.to_string(),
        }];
        let events: Vec<Event> = vec![
            labeled_event("first", "2026-10-21T10:00:00Z"),
            json!({ "event": "labeled", "label": { "name": "ci-override" }, "actor": null, "created_at": "2026-10-21T11:00:00Z" }),
        ]
        .into_iter()
        .map(|event| serde_json::from_value(event).expect("valid event"))
        .collect();

        // Act
        let request = override_request(&labels, &events).expect("the label is on the pull request");

        // Assert
        assert_eq!(
            request.actor, None,
            "an earlier actor must not be used for a later label that nobody can be named for"
        );
    }

    #[rstest]
    #[case::admin_after_the_push("2026-10-21T10:49:13Z", Some("2026-10-21T10:46:06Z"), Ok("admin"), Ok(()))]
    #[case::maintain_after_the_push("2026-10-21T10:49:13Z", Some("2026-10-21T10:46:06Z"), Ok("maintain"), Ok(()))]
    #[case::one_second_after_the_push("2026-10-21T10:46:07Z", Some("2026-10-21T10:46:06Z"), Ok("admin"), Ok(()))]
    #[case::at_the_same_second_as_the_push(
        "2026-10-21T10:46:06Z",
        Some("2026-10-21T10:46:06Z"),
        Ok("admin"),
        Err("a push revoked it")
    )]
    #[case::before_the_push("2026-10-21T10:44:46Z", Some("2026-10-21T10:46:06Z"), Ok("admin"), Err("a push revoked it"))]
    #[case::write_permission(
        "2026-10-21T10:49:13Z",
        Some("2026-10-21T10:46:06Z"),
        Ok("write"),
        Err("the permission `write` is not one of admin, maintain")
    )]
    #[case::read_permission(
        "2026-10-21T10:49:13Z",
        Some("2026-10-21T10:46:06Z"),
        Ok("read"),
        Err("the permission `read` is not one of")
    )]
    #[case::permission_that_cannot_be_read(
        "2026-10-21T10:49:13Z",
        Some("2026-10-21T10:46:06Z"),
        Err("HTTP 403"),
        Err("cannot read the permission: HTTP 403")
    )]
    #[case::no_check_suite("2026-10-21T10:49:13Z", None, Ok("admin"), Err("has no check suite"))]
    #[case::a_time_that_is_not_in_github_form(
        "2026-10-21 10:49:13",
        Some("2026-10-21T10:46:06Z"),
        Ok("admin"),
        Err("are not in the form")
    )]
    fn it_should_judge_whether_an_override_still_counts(
        #[case] applied_at: &str,
        #[case] first_check_suite_at: Option<&str>,
        #[case] permission: Result<&str, &str>,
        #[case] expected: Result<(), &str>,
    ) {
        // Act
        let judgement = judge_override(applied_at, first_check_suite_at, permission);

        // Assert
        match (&judgement, expected) {
            (Ok(()), Ok(())) => {}
            (Err(reason), Err(expected_part)) => assert!(
                reason.contains(expected_part),
                "the reason should mention `{expected_part}`, but was: {reason}"
            ),
            _ => panic!("expected {expected:?}, but the judgement was {judgement:?}"),
        }
    }

    #[rstest]
    #[case::a_repository("owner/repo", HEAD_SHA, true)]
    #[case::a_repository_with_dots_and_hyphens("torrust/torrust-tracker.rs", HEAD_SHA, true)]
    #[case::no_owner("repo", HEAD_SHA, false)]
    #[case::a_path_in_the_repository("owner/repo/extra", HEAD_SHA, false)]
    #[case::a_parent_directory("owner/..", HEAD_SHA, false)]
    #[case::a_space("owner/re po", HEAD_SHA, false)]
    #[case::a_query("owner/repo?x=1", HEAD_SHA, false)]
    #[case::a_short_commit_id("owner/repo", "aaaa", false)]
    #[case::a_commit_id_that_is_not_hexadecimal("owner/repo", "gggggggggggggggggggggggggggggggggggggggg", false)]
    fn it_should_accept_only_a_repository_and_a_commit_id_that_cannot_change_the_request(
        #[case] repository: &str,
        #[case] head_sha: &str,
        #[case] valid: bool,
    ) {
        // Act
        let reference = PullRequestRef::new(repository, 7, head_sha);

        // Assert
        assert_eq!(reference.is_ok(), valid, "`{repository}` and `{head_sha}`: {reference:?}");
    }

    #[test]
    fn it_should_pass_without_looking_for_an_override_when_every_needed_group_succeeded() {
        // Arrange
        let api = FakeApi::new().with(
            CHECK_RUNS_PATH,
            check_runs_page(vec![
                check_run(1, "docs-lint", "completed", Some("success")),
                check_run(2, "db-benchmarking", "completed", Some("success")),
            ]),
        );

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (verdict.verdict.verdict, verdict.override_ignored, api.requests()),
            (Verdict::Pass, None, vec![CHECK_RUNS_PATH.to_string()]),
            "only the check runs are needed when nothing failed"
        );
    }

    #[test]
    fn it_should_fail_without_an_override_when_a_group_failed_and_the_label_is_not_on_the_pull_request() {
        // Arrange
        let api = failing_head()
            .with(LABELS_PATH, vec![json!([])])
            .with(EVENTS_PATH, vec![json!([])]);

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (
                verdict.verdict.verdict,
                verdict.verdict.overridden_by,
                verdict.override_ignored
            ),
            (Verdict::Fail, None, None)
        );
    }

    #[test]
    fn it_should_pass_naming_the_actor_when_a_maintainer_applied_the_label_after_the_push() {
        // Arrange
        let api = api_with_override(
            failing_head(),
            "josecelano",
            "2026-10-21T10:49:13Z",
            "2026-10-21T10:46:06Z",
            "admin",
        );

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            verdict.verdict.explanation,
            "overridden by `josecelano`; needed groups without a success: db-benchmarking (failure)"
        );
        assert_eq!((verdict.verdict.verdict, verdict.override_ignored), (Verdict::Pass, None));
    }

    #[test]
    fn it_should_fail_and_say_why_when_the_label_was_applied_before_the_push() {
        // Arrange
        let api = api_with_override(
            failing_head(),
            "josecelano",
            "2026-10-21T10:44:46Z",
            "2026-10-21T10:46:06Z",
            "admin",
        );

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (verdict.verdict.verdict, verdict.verdict.overridden_by),
            (Verdict::Fail, None)
        );
        let reason = verdict.override_ignored.expect("the ignored override is explained");
        assert!(
            reason.contains("`josecelano`") && reason.contains("a push revoked it") && reason.contains("apply it again"),
            "the reason should name the actor and say what to do, but was: {reason}"
        );
    }

    #[test]
    fn it_should_fail_and_name_the_permission_when_the_actor_may_not_override() {
        // Arrange
        let api = api_with_override(
            failing_head(),
            "contributor",
            "2026-10-21T10:49:13Z",
            "2026-10-21T10:46:06Z",
            "write",
        );

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (verdict.verdict.verdict, verdict.verdict.overridden_by),
            (Verdict::Fail, None)
        );
        assert!(
            verdict
                .override_ignored
                .as_deref()
                .is_some_and(|reason| reason.contains("the permission `write`")),
            "the ignored override should name the permission: {:?}",
            verdict.override_ignored
        );
    }

    #[test]
    fn it_should_fail_without_an_override_when_the_permission_cannot_be_read() {
        // Arrange
        let api = api_with_override(
            failing_head(),
            "josecelano",
            "2026-10-21T10:49:13Z",
            "2026-10-21T10:46:06Z",
            "admin",
        )
        .failing(&permission_path("josecelano"), "HTTP 403");

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (verdict.verdict.verdict, verdict.verdict.overridden_by),
            (Verdict::Fail, None)
        );
        assert!(
            verdict
                .override_ignored
                .as_deref()
                .is_some_and(|reason| reason.contains("cannot read the permission")),
            "the error should be explained: {:?}",
            verdict.override_ignored
        );
    }

    #[test]
    fn it_should_fail_without_an_override_when_the_events_cannot_be_read() {
        // Arrange
        let api = failing_head()
            .with(LABELS_PATH, vec![json!([{ "name": "ci-override" }])])
            .failing(EVENTS_PATH, "HTTP 500");

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (verdict.verdict.verdict, verdict.verdict.overridden_by),
            (Verdict::Fail, None)
        );
        assert!(
            verdict
                .override_ignored
                .as_deref()
                .is_some_and(|reason| reason.contains("cannot read the events")),
            "the error should be explained: {:?}",
            verdict.override_ignored
        );
    }

    #[test]
    fn it_should_return_an_error_instead_of_a_verdict_when_the_check_runs_cannot_be_read() {
        // Arrange
        let api = FakeApi::new().failing(CHECK_RUNS_PATH, "HTTP 502");

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking());

        // Assert
        assert_eq!(
            verdict,
            Err("cannot read the check runs of the head commit: HTTP 502".to_string()),
            "a gate that cannot see the results must fail the job, not decide"
        );
    }

    #[test]
    fn it_should_use_the_results_of_an_earlier_run_when_a_label_run_skipped_the_group_jobs() {
        // Arrange
        let api = api_with_override(
            failing_head(),
            "josecelano",
            "2026-10-21T10:49:13Z",
            "2026-10-21T10:46:06Z",
            "admin",
        )
        .with(
            CHECK_RUNS_PATH,
            check_runs_page(vec![
                check_run(1, "docs-lint", "completed", Some("success")),
                check_run(2, "db-benchmarking", "completed", Some("failure")),
                check_run(5, "matrix.group", "completed", Some("skipped")),
                check_run(6, "db-benchmarking", "completed", Some("skipped")),
                check_run(7, "CI gate", "in_progress", None),
            ]),
        );

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (
                verdict.verdict.verdict,
                verdict.verdict.problems.len(),
                verdict.verdict.overridden_by.as_deref()
            ),
            (Verdict::Pass, 1, Some("josecelano")),
            "the failure of the earlier run must still be seen, and overridden"
        );
    }

    #[test]
    fn it_should_read_every_page_of_a_paginated_response() {
        // Arrange
        let api = FakeApi::new().with(
            CHECK_RUNS_PATH,
            vec![
                json!({ "check_runs": [check_run(1, "docs-lint", "completed", Some("success"))] }),
                json!({ "check_runs": [check_run(2, "db-benchmarking", "completed", Some("success"))] }),
            ],
        );

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            verdict.verdict.verdict,
            Verdict::Pass,
            "the second group is on the second page, so reading only the first would leave it missing"
        );
    }

    #[test]
    fn it_should_take_the_earliest_check_suite_over_all_pages_when_the_override_is_judged() {
        // Arrange
        // The label was applied at 10:49:13. The first page holds only suites created after it, at 10:50:00 and
        // 10:52:00, which would make the label look stale. The earliest suite, at 10:48:00, is on the second page, so
        // the label was applied after the head commit's first suite and counts.
        let api = failing_head()
            .with(LABELS_PATH, vec![json!([{ "name": "ci-override" }])])
            .with(EVENTS_PATH, vec![json!([labeled_event("josecelano", "2026-10-21T10:49:13Z")])])
            .with(
                CHECK_SUITES_PATH,
                vec![
                    json!({ "check_suites": [{ "created_at": "2026-10-21T10:52:00Z" }, { "created_at": "2026-10-21T10:50:00Z" }] }),
                    json!({ "check_suites": [{ "created_at": "2026-10-21T10:51:00Z" }, { "created_at": "2026-10-21T10:48:00Z" }] }),
                ],
            )
            .with(&permission_path("josecelano"), vec![json!({ "permission": "admin" })]);

        // Act
        let verdict = decide_for_pull_request(&api, &pull_request(), &classification_needing_db_benchmarking()).expect("decided");

        // Assert
        assert_eq!(
            (verdict.verdict.verdict, verdict.verdict.overridden_by),
            (Verdict::Pass, Some("josecelano".to_string())),
            "the earliest suite, at 10:48:00 on the second page, is before the label, so the label counts"
        );
    }
}
