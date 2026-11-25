//! What the stop hook of the task reviewer does with the reply file a task names.
//!
//! The hook refuses to let the reviewer finish until the reply file its task names is valid, at
//! most [`REFUSAL_LIMIT`] times per request, and lets it finish in the cases where the reviewer
//! cannot be held to a reply (decision D17 of issue #2904). This module holds those rules. It
//! reads nothing itself: the facts about the repository come through [`Workspace`], so every
//! outcome can be tested with plain values.
//!
//! The hook also lets the agent finish, with a message, when the file name the task gives is not a
//! valid reply file name: the validator refuses such a name whatever the file holds, so refusing
//! the agent could not make the request succeed.
//!
//! The text the hook returns never contains the literal line `Reply file:` that a task uses to name
//! the file, because the transcript would then hold a second line for the hook to find.

use std::fmt::Write as _;

use crate::file::{Problem, check_reply_file};
use crate::name::ReplyFileName;

/// How many times the hook refuses to let the agent finish, for one request. The fourth attempt
/// is allowed. It is the round limit of decision D18.
pub const REFUSAL_LIMIT: u8 = 3;

/// The folder under `docs/issues/` that holds draft specifications.
const DRAFTS: &str = "drafts";

/// The folder of a specification that holds its reply files.
const REPLIES_FOLDER: &str = "agent-replies";

/// The lifecycle folders under `docs/issues/` whose specifications the hook enforces.
const ENFORCED_LIFECYCLE_FOLDERS: [&str; 2] = ["open", "closed"];

/// Where the path a task names is, as far as the hook is concerned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Placement {
    /// In a draft specification: it has no implemented task to review.
    Draft,
    /// Not in a place a reply file may be in, or not a plain relative path.
    Outside,
    /// In the replies folder of a specification under `docs/issues/open/` or `docs/issues/closed/`.
    Specification {
        /// The specification's folder, such as `docs/issues/open/2904-slug`.
        folder: String,
        /// The first number of the folder's name, which keys the count of refusals.
        issue_number: Option<u64>,
    },
}

/// Says where the path is.
///
/// The path is taken from a task, so it is not trusted: an absolute path, or one with an empty, `.`
/// or `..` component, is [`Placement::Outside`]. So is a path under `open/` or `closed/` that is
/// not exactly `<specification>/agent-replies/<file>`: the reviewer cannot change the path the
/// caller wrote, and the validator would refuse that place whatever the file holds.
#[must_use]
pub fn classify(path: &str) -> Placement {
    let components: Vec<&str> = path.split('/').collect();
    if components.iter().any(|component| matches!(*component, "" | "." | "..")) {
        return Placement::Outside;
    }
    match components.as_slice() {
        ["docs", "issues", DRAFTS, _, _, ..] => Placement::Draft,
        ["docs", "issues", lifecycle, specification, REPLIES_FOLDER, _] if ENFORCED_LIFECYCLE_FOLDERS.contains(lifecycle) => {
            Placement::Specification {
                folder: format!("docs/issues/{lifecycle}/{specification}"),
                issue_number: leading_number(specification),
            }
        }
        _ => Placement::Outside,
    }
}

/// The digits that start the folder name, when a `-` or the end of the name follows them.
fn leading_number(folder_name: &str) -> Option<u64> {
    let digits = folder_name
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(folder_name.len());
    let (number, rest) = folder_name.split_at(digits);
    if number.is_empty() || !(rest.is_empty() || rest.starts_with('-')) {
        return None;
    }
    number.parse().ok()
}

/// What the hook found when it looked for the reply file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplyFile {
    /// There is no file at the path.
    Missing,
    /// The path holds something that is not a readable, regular text file; the reason says why.
    Unreadable(String),
    /// The file's content.
    Text(String),
}

/// What identifies one request in the count of refusals: the run, and the reply file it names.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RefusalKey {
    /// The issue number of the specification the reply file is in.
    pub issue_number: u64,
    /// The `agent_id` of the run. A continued run keeps it.
    pub agent_id: String,
    /// The file name of the reply file. A continued request names a new round's file.
    pub reply_file_name: String,
}

/// The facts about the repository that the hook needs, and the count of refusals it keeps.
pub trait Workspace {
    /// Whether the specification's folder exists.
    fn specification_exists(&self, folder: &str) -> bool;

    /// What is at the repository-relative path.
    fn reply_file(&self, path: &str) -> ReplyFile;

    /// The refusals so far for this request; zero when none was kept.
    fn refusals(&self, key: &RefusalKey) -> u8;

    /// Keeps the count of refusals.
    ///
    /// # Errors
    ///
    /// Returns the reason when the count cannot be kept.
    fn set_refusals(&mut self, key: &RefusalKey, count: u8) -> Result<(), String>;

    /// Forgets the count, once the reply file is valid.
    fn clear_refusals(&mut self, key: &RefusalKey);
}

/// What the hook does.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// Let the agent finish. The message, when there is one, is shown to the user.
    Finish {
        /// Why the reply was not enforced, or why the limit was reached.
        message: Option<String>,
    },
    /// Keep the agent working, and tell it what to fix.
    Refuse {
        /// What is told to the agent.
        feedback: String,
    },
}

impl Outcome {
    fn finish(message: impl Into<String>) -> Self {
        Self::Finish {
            message: Some(message.into()),
        }
    }
}

/// Decides what the hook does for the run `agent_id`, whose latest request names `reply_file`.
///
/// `reply_file` is `None` when no request names one: the reviewer was given no folder-style
/// specification, and it may finish.
pub fn decide(agent_id: &str, reply_file: Option<&str>, workspace: &mut impl Workspace) -> Outcome {
    let Some(path) = reply_file else {
        return Outcome::Finish { message: None };
    };
    match classify(path) {
        Placement::Draft => Outcome::finish(format!(
            "The reply was not enforced: `{path}` is under `docs/issues/drafts/`, and a draft specification has no implemented task to review."
        )),
        Placement::Outside => Outcome::finish(format!(
            "The reply was not enforced: `{path}` is outside the places a reply file may be in (`docs/issues/<drafts|open|closed>/<specification>/agent-replies/`), and the reviewer cannot change the path the caller wrote."
        )),
        Placement::Specification { folder, issue_number } => {
            if !workspace.specification_exists(&folder) {
                return Outcome::finish(format!(
                    "The reply was not enforced: the specification folder `{folder}` of `{path}` does not exist (close-out moves a specification to `docs/issues/closed/`, or the caller mistyped the path), and the reviewer must not create it."
                ));
            }
            let Some(issue_number) = issue_number else {
                return Outcome::finish(format!(
                    "The reply was not enforced: the name of the specification folder `{folder}` does not start with an issue number, so the count of refusals has no key."
                ));
            };
            let reply_file_name = path.rsplit('/').next().unwrap_or(path);
            if let Err(error) = reply_file_name.parse::<ReplyFileName>() {
                return Outcome::finish(format!(
                    "The reply was not enforced: the file name of `{path}` is not a valid reply file name ({error}), so the validator refuses it whatever the file holds and the reviewer cannot satisfy the request. The caller must name the file as the name rule of decision D18 says and ask again."
                ));
            }
            let key = RefusalKey {
                issue_number,
                agent_id: agent_id.to_owned(),
                reply_file_name: reply_file_name.to_owned(),
            };
            hold_to_the_reply(path, &key, workspace)
        }
    }
}

/// Refuses while the reply file is invalid and the limit is not reached.
fn hold_to_the_reply(path: &str, key: &RefusalKey, workspace: &mut impl Workspace) -> Outcome {
    let problems = problems_of(path, &workspace.reply_file(path));
    if problems.is_empty() {
        workspace.clear_refusals(key);
        return Outcome::Finish { message: None };
    }
    let refusals = workspace.refusals(key);
    if refusals >= REFUSAL_LIMIT {
        return Outcome::finish(format!(
            "The reply file `{path}` is still not valid after {REFUSAL_LIMIT} refusals, so the stop hook lets the agent finish without it. The caller finds no valid reply file and must stop and ask the contributor.{}",
            list(&problems)
        ));
    }
    let refusal = refusals + 1;
    match workspace.set_refusals(key, refusal) {
        Ok(()) => Outcome::Refuse {
            feedback: format!(
                "The review is not finished: the reply file `{path}` is not valid (refusal {refusal} of {REFUSAL_LIMIT}).{}\nWrite the reply file as the `review-task` skill describes, then finish; the hook checks it again.",
                list(&problems)
            ),
        },
        Err(reason) => Outcome::finish(format!(
            "The reply file `{path}` is not valid, but the count of refusals could not be kept ({reason}), so the stop hook lets the agent finish rather than refuse without a bound. The caller finds no valid reply file and must stop and ask the contributor.{}",
            list(&problems)
        )),
    }
}

/// The reasons the reply file is not valid; none when it is.
fn problems_of(path: &str, file: &ReplyFile) -> Vec<String> {
    match file {
        ReplyFile::Missing => vec!["the file does not exist".to_owned()],
        ReplyFile::Unreadable(reason) => vec![format!("the file cannot be read: {reason}")],
        ReplyFile::Text(text) => check_reply_file(path, text).iter().map(describe).collect(),
    }
}

/// One problem of the validator, on one line.
fn describe(problem: &Problem) -> String {
    problem.field_path.as_ref().map_or_else(
        || format!("{}: {}", problem.check, problem.message),
        |field| format!("{} (`{field}`): {}", problem.check, problem.message),
    )
}

/// The problems as a bulleted list that starts on a new line.
fn list(problems: &[String]) -> String {
    problems.iter().fold(String::new(), |mut text, problem| {
        let _ = write!(text, "\n- {problem}");
        text
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rstest::rstest;
    use serde_json::json;

    use super::{Outcome, Placement, REFUSAL_LIMIT, RefusalKey, ReplyFile, Workspace, classify, decide};
    use crate::file::check_reply_file;

    /// The number that starts the folder name of `SPECIFICATION`, which keys the count.
    const ISSUE_NUMBER: u64 = 2904;
    const SPECIFICATION: &str = "docs/issues/open/2904-2003-migrate-custom-agents";
    /// The file name of `REPLY`, which keys the count together with the agent.
    const REPLY_FILE_NAME: &str = "task-reviewer-t5-r1.json";
    const DRAFT_REPLY: &str = "docs/issues/drafts/2003-some-draft/agent-replies/task-reviewer-t1-r1.json";
    const REPLY: &str = "docs/issues/open/2904-2003-migrate-custom-agents/agent-replies/task-reviewer-t5-r1.json";
    const AGENT: &str = "a423cd4370c2ebae8";

    /// A valid reply for the file `task-reviewer-t5-r1.json`.
    fn valid_reply() -> String {
        json!({
            "schema-version": 1,
            "agent": "task-reviewer",
            "model": "claude-opus-5-5",
            "subject": "t5",
            "round": 1,
            "created-utc": "2026-11-24 09:00",
            "verdict": "REVIEW_PASSED",
            "summary": "Nothing blocks the task.",
            "findings": [],
            "questions": [],
            "evidence": ["cargo test -q -p agent-reply-validator passed"]
        })
        .to_string()
    }

    /// The messages the validator gives for a reply file with this content, as the hook must pass them on.
    fn validator_messages(text: &str) -> Vec<String> {
        check_reply_file(REPLY, text)
            .into_iter()
            .map(|problem| problem.message)
            .collect()
    }

    fn key(agent_id: &str, reply_file_name: &str) -> RefusalKey {
        RefusalKey {
            issue_number: ISSUE_NUMBER,
            agent_id: agent_id.to_owned(),
            reply_file_name: reply_file_name.to_owned(),
        }
    }

    /// The repository as the hook sees it, in memory.
    #[derive(Default)]
    struct FakeWorkspace {
        folders: Vec<String>,
        files: HashMap<String, ReplyFile>,
        counts: HashMap<RefusalKey, u8>,
        counts_cannot_be_kept: bool,
    }

    impl FakeWorkspace {
        /// A repository whose specification of issue 2904 exists, with no reply file yet.
        fn with_the_specification() -> Self {
            Self {
                folders: vec![SPECIFICATION.to_owned()],
                ..Self::default()
            }
        }

        fn with_the_reply(mut self, file: ReplyFile) -> Self {
            self.files.insert(REPLY.to_owned(), file);
            self
        }

        fn with_refusals(mut self, key: RefusalKey, count: u8) -> Self {
            self.counts.insert(key, count);
            self
        }
    }

    impl Workspace for FakeWorkspace {
        fn specification_exists(&self, folder: &str) -> bool {
            self.folders.iter().any(|existing| existing == folder)
        }

        fn reply_file(&self, path: &str) -> ReplyFile {
            self.files.get(path).cloned().unwrap_or(ReplyFile::Missing)
        }

        fn refusals(&self, key: &RefusalKey) -> u8 {
            self.counts.get(key).copied().unwrap_or(0)
        }

        fn set_refusals(&mut self, key: &RefusalKey, count: u8) -> Result<(), String> {
            if self.counts_cannot_be_kept {
                return Err("the disk is full".to_owned());
            }
            self.counts.insert(key.clone(), count);
            Ok(())
        }

        fn clear_refusals(&mut self, key: &RefusalKey) {
            self.counts.remove(key);
        }
    }

    fn specification(folder: &str, issue_number: Option<u64>) -> Placement {
        Placement::Specification {
            folder: folder.to_owned(),
            issue_number,
        }
    }

    fn finish_message(outcome: Outcome) -> String {
        match outcome {
            Outcome::Finish { message: Some(message) } => message,
            other => panic!("expected the agent to finish with a message, got {other:?}"),
        }
    }

    fn refuse_feedback(outcome: Outcome) -> String {
        match outcome {
            Outcome::Refuse { feedback } => feedback,
            other @ Outcome::Finish { .. } => panic!("expected a refusal, got {other:?}"),
        }
    }

    #[rstest]
    #[case::open(REPLY, specification(SPECIFICATION, Some(2904)))]
    #[case::closed(
        "docs/issues/closed/1859-1669-move-tracker-policy/agent-replies/task-reviewer-t1-r1.json",
        specification("docs/issues/closed/1859-1669-move-tracker-policy", Some(1859))
    )]
    #[case::folder_that_is_only_a_number(
        "docs/issues/open/2904/agent-replies/task-reviewer-t1-r1.json",
        specification("docs/issues/open/2904", Some(2904))
    )]
    #[case::folder_without_a_number(
        "docs/issues/open/some-slug/agent-replies/task-reviewer-t1-r1.json",
        specification("docs/issues/open/some-slug", None)
    )]
    #[case::number_not_followed_by_a_dash(
        "docs/issues/open/2904x-slug/agent-replies/task-reviewer-t1-r1.json",
        specification("docs/issues/open/2904x-slug", None)
    )]
    #[case::number_too_large_to_be_one(
        "docs/issues/open/99999999999999999999999-slug/agent-replies/task-reviewer-t1-r1.json",
        specification("docs/issues/open/99999999999999999999999-slug", None)
    )]
    fn it_should_classify_a_file_in_the_replies_folder_of_a_specification_with_the_first_number_of_its_folder(
        #[case] path: &str,
        #[case] expected: Placement,
    ) {
        // Act
        let placement = classify(path);

        // Assert
        assert_eq!(placement, expected);
    }

    #[rstest]
    #[case::numbered_like_its_epic(
        "docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/agent-replies/task-reviewer-t1-r1.json"
    )]
    #[case::without_a_number("docs/issues/drafts/some-slug/agent-replies/task-reviewer-t1-r1.json")]
    fn it_should_classify_a_path_under_drafts_as_a_draft_whatever_its_folder_name_starts_with(#[case] path: &str) {
        // Act
        let placement = classify(path);

        // Assert
        assert_eq!(placement, Placement::Draft);
    }

    #[rstest]
    #[case::outside_the_issues("notes/task-reviewer-t1-r1.json")]
    #[case::at_the_repository_root("task-reviewer-t1-r1.json")]
    #[case::another_documentation_folder("docs/refactor-plans/open/a-plan/agent-replies/task-reviewer-t1-r1.json")]
    #[case::unknown_lifecycle("docs/issues/archive/2904-slug/agent-replies/task-reviewer-t1-r1.json")]
    #[case::in_the_lifecycle_folder_itself("docs/issues/open")]
    #[case::directly_in_the_lifecycle_folder("docs/issues/open/task-reviewer-t1-r1.json")]
    #[case::directly_in_the_specification_folder("docs/issues/open/2904-slug/task-reviewer-t1-r1.json")]
    #[case::in_a_folder_with_another_name("docs/issues/open/2904-slug/replies/task-reviewer-t1-r1.json")]
    #[case::one_level_too_deep("docs/issues/open/2904-slug/notes/agent-replies/task-reviewer-t1-r1.json")]
    #[case::in_a_subfolder_of_the_replies_folder("docs/issues/open/2904-slug/agent-replies/notes/task-reviewer-t1-r1.json")]
    #[case::the_replies_folder_itself("docs/issues/open/2904-slug/agent-replies")]
    #[case::directly_in_the_drafts_folder("docs/issues/drafts/task-reviewer-t1-r1.json")]
    #[case::absolute_path("/docs/issues/open/2904-slug/agent-replies/task-reviewer-t1-r1.json")]
    #[case::parent_component("docs/issues/open/2904-slug/../2905-other/agent-replies/task-reviewer-t1-r1.json")]
    #[case::parent_component_out_of_the_repository("docs/issues/open/../../../etc/passwd")]
    #[case::current_folder_component("docs/issues/open/./2904-slug/agent-replies/task-reviewer-t1-r1.json")]
    #[case::empty_component("docs/issues/open//2904-slug/agent-replies/task-reviewer-t1-r1.json")]
    #[case::trailing_slash("docs/issues/open/2904-slug/agent-replies/")]
    #[case::empty_path("")]
    fn it_should_classify_a_path_as_outside_when_it_is_not_in_the_replies_folder_of_a_specification_or_is_not_a_plain_relative_path(
        #[case] path: &str,
    ) {
        // Act
        let placement = classify(path);

        // Assert
        assert_eq!(placement, Placement::Outside);
    }

    #[test]
    fn it_should_let_the_agent_finish_without_a_message_when_no_request_names_a_reply_file() {
        // Arrange
        let mut workspace = FakeWorkspace::with_the_specification();

        // Act
        let outcome = decide(AGENT, None, &mut workspace);

        // Assert
        assert_eq!(outcome, Outcome::Finish { message: None });
        assert!(
            workspace.counts.is_empty(),
            "no count is kept when there is no reply file to hold the agent to"
        );
    }

    #[test]
    fn it_should_let_the_agent_finish_with_a_message_and_keep_no_count_when_the_path_is_in_a_draft() {
        // Arrange
        let mut workspace = FakeWorkspace::default();
        let path = "docs/issues/drafts/2003-some-draft/agent-replies/task-reviewer-t1-r1.json";

        // Act
        let message = finish_message(decide(AGENT, Some(path), &mut workspace));

        // Assert
        assert!(message.contains(path), "the message must name the path: {message}");
        assert!(
            message.contains("a draft specification has no implemented task to review"),
            "the message must say why the reply was not enforced: {message}"
        );
        assert!(
            workspace.counts.is_empty(),
            "the issue number is never taken from a draft path"
        );
    }

    #[test]
    fn it_should_let_the_agent_finish_with_a_message_and_keep_no_count_when_the_path_is_outside_the_places_a_reply_file_may_be_in()
     {
        // Arrange
        let mut workspace = FakeWorkspace::with_the_specification();
        let path = "notes/task-reviewer-t5-r1.json";

        // Act
        let message = finish_message(decide(AGENT, Some(path), &mut workspace));

        // Assert
        assert!(message.contains(path), "the message must name the path: {message}");
        assert!(
            message.contains("outside the places a reply file may be in"),
            "the message must say why the reply was not enforced: {message}"
        );
        assert!(
            workspace.counts.is_empty(),
            "no count is kept for a path outside the specifications"
        );
    }

    #[test]
    fn it_should_let_the_agent_finish_with_a_message_and_keep_no_count_when_the_specification_folder_does_not_exist() {
        // Arrange: close-out moved the specification, so only `closed/` has it.
        let mut workspace = FakeWorkspace {
            folders: vec!["docs/issues/closed/2904-2003-migrate-custom-agents".to_owned()],
            ..FakeWorkspace::default()
        };

        // Act
        let message = finish_message(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(
            message.contains(&format!("`{SPECIFICATION}`")),
            "the message must name the missing folder: {message}"
        );
        assert!(
            message.contains("does not exist"),
            "the message must say the folder is missing: {message}"
        );
        assert!(workspace.counts.is_empty(), "no count is kept when the folder is missing");
    }

    #[test]
    fn it_should_let_the_agent_finish_with_a_message_when_the_specification_folder_has_no_issue_number_to_key_the_count() {
        // Arrange
        let path = "docs/issues/open/some-slug/agent-replies/task-reviewer-t5-r1.json";
        let mut workspace = FakeWorkspace {
            folders: vec!["docs/issues/open/some-slug".to_owned()],
            ..FakeWorkspace::default()
        };

        // Act
        let message = finish_message(decide(AGENT, Some(path), &mut workspace));

        // Assert
        assert!(
            message.contains("does not start with an issue number"),
            "the message must say why the reply was not enforced: {message}"
        );
        assert!(workspace.counts.is_empty(), "there is no issue number to key a count");
    }

    #[rstest]
    #[case::round_above_the_limit("task-reviewer-t5-r4.json", "not a round")]
    #[case::round_with_a_leading_zero("task-reviewer-t5-r01.json", "not a round")]
    #[case::unknown_agent("code-reviewer-t5-r1.json", "does not start with the name of an agent")]
    #[case::no_round("task-reviewer-t5.json", "has no round")]
    #[case::not_json("task-reviewer-t5-r1.md", "does not end in `.json`")]
    #[case::subject_that_is_not_a_task_or_a_pull_request("task-reviewer-x5-r1.json", "not a subject")]
    fn it_should_let_the_agent_finish_with_the_reason_and_keep_no_count_when_the_file_name_is_not_a_valid_reply_file_name(
        #[case] file_name: &str,
        #[case] reason: &str,
    ) {
        // Arrange: the folder exists and the file is missing, so a valid name would be refused.
        let path = format!("{SPECIFICATION}/agent-replies/{file_name}");
        let mut workspace = FakeWorkspace::with_the_specification();

        // Act
        let message = finish_message(decide(AGENT, Some(&path), &mut workspace));

        // Assert
        assert!(
            message.contains(file_name),
            "the message must name the path with the file name the validator rejects: {message}"
        );
        assert!(
            message.contains(reason),
            "the message must give the validator's reason: {message}"
        );
        assert!(workspace.counts.is_empty(), "no count is kept for a name nothing can satisfy");
    }

    #[test]
    fn it_should_check_the_file_name_after_the_specification_folder_and_its_issue_number() {
        // Arrange: a folder that does not exist and a bad name; the folder is reported first.
        let path = format!("{SPECIFICATION}/agent-replies/task-reviewer-t5-r4.json");
        let mut workspace = FakeWorkspace::default();

        // Act
        let message = finish_message(decide(AGENT, Some(&path), &mut workspace));

        // Assert
        assert!(
            message.contains("does not exist"),
            "the missing folder is reported before the file name: {message}"
        );
    }

    #[test]
    fn it_should_let_the_agent_finish_without_a_message_and_forget_the_count_when_the_reply_file_is_valid() {
        // Arrange
        let mut workspace = FakeWorkspace::with_the_specification()
            .with_the_reply(ReplyFile::Text(valid_reply()))
            .with_refusals(key(AGENT, REPLY_FILE_NAME), 2);

        // Act
        let outcome = decide(AGENT, Some(REPLY), &mut workspace);

        // Assert
        assert_eq!(outcome, Outcome::Finish { message: None });
        assert!(
            workspace.counts.is_empty(),
            "the count of a request whose reply is valid is forgotten"
        );
    }

    #[test]
    fn it_should_refuse_and_keep_the_first_refusal_when_the_reply_file_does_not_exist() {
        // Arrange
        let mut workspace = FakeWorkspace::with_the_specification();

        // Act
        let feedback = refuse_feedback(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(feedback.contains(REPLY), "the feedback must name the file: {feedback}");
        assert!(
            feedback.contains("the file does not exist"),
            "the feedback must say what is wrong: {feedback}"
        );
        assert!(
            feedback.contains("refusal 1 of 3"),
            "the feedback must count the refusal: {feedback}"
        );
        assert_eq!(workspace.refusals(&key(AGENT, REPLY_FILE_NAME)), 1);
    }

    #[test]
    fn it_should_refuse_with_the_problems_of_the_validator_when_the_content_is_invalid() {
        // Arrange
        let expected = validator_messages("{}");
        let mut workspace = FakeWorkspace::with_the_specification().with_the_reply(ReplyFile::Text("{}".to_owned()));

        // Act
        let feedback = refuse_feedback(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(!expected.is_empty(), "the validator must find problems in `{{}}`");
        for message in expected {
            assert!(feedback.contains(&message), "the feedback must carry `{message}`: {feedback}");
        }
    }

    #[test]
    fn it_should_refuse_with_the_reason_when_the_path_holds_something_that_cannot_be_read() {
        // Arrange
        let mut workspace =
            FakeWorkspace::with_the_specification().with_the_reply(ReplyFile::Unreadable("it is a symbolic link".to_owned()));

        // Act
        let feedback = refuse_feedback(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(
            feedback.contains("the file cannot be read: it is a symbolic link"),
            "the feedback must give the reason: {feedback}"
        );
    }

    #[rstest]
    #[case::second(1, "refusal 2 of 3")]
    #[case::third(2, "refusal 3 of 3")]
    fn it_should_count_each_refusal_up_to_the_limit(#[case] refusals_so_far: u8, #[case] expected: &str) {
        // Arrange
        let mut workspace = FakeWorkspace::with_the_specification().with_refusals(key(AGENT, REPLY_FILE_NAME), refusals_so_far);

        // Act
        let feedback = refuse_feedback(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(feedback.contains(expected), "the feedback must count the refusal: {feedback}");
        assert_eq!(workspace.refusals(&key(AGENT, REPLY_FILE_NAME)), refusals_so_far + 1);
    }

    #[rstest]
    #[case::at_the_limit(REFUSAL_LIMIT)]
    #[case::above_the_limit(200)]
    fn it_should_let_the_agent_finish_with_the_limit_reported_when_the_refusals_reached_the_limit(#[case] refusals_so_far: u8) {
        // Arrange
        let expected = validator_messages("{}");
        let mut workspace = FakeWorkspace::with_the_specification()
            .with_the_reply(ReplyFile::Text("{}".to_owned()))
            .with_refusals(key(AGENT, REPLY_FILE_NAME), refusals_so_far);

        // Act
        let message = finish_message(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(
            message.contains("still not valid after 3 refusals"),
            "the message must report the limit: {message}"
        );
        assert!(
            message.contains("must stop and ask the contributor"),
            "the message must say what the caller does: {message}"
        );
        for problem in expected {
            assert!(
                message.contains(&problem),
                "the message must still list `{problem}`: {message}"
            );
        }
        assert_eq!(
            workspace.refusals(&key(AGENT, REPLY_FILE_NAME)),
            refusals_so_far,
            "an allowed attempt is not a refusal"
        );
    }

    #[rstest]
    #[case::another_run_of_the_same_file(key("another-agent", REPLY_FILE_NAME))]
    #[case::another_file_of_the_same_run(key(AGENT, "task-reviewer-t5-r2.json"))]
    #[case::another_issue(RefusalKey { issue_number: 2905, agent_id: AGENT.to_owned(), reply_file_name: REPLY_FILE_NAME.to_owned() })]
    fn it_should_start_the_count_again_when_the_run_the_file_or_the_issue_is_another(#[case] exhausted: RefusalKey) {
        // Arrange
        let mut workspace = FakeWorkspace::with_the_specification().with_refusals(exhausted, REFUSAL_LIMIT);

        // Act
        let feedback = refuse_feedback(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(
            feedback.contains("refusal 1 of 3"),
            "the count of another request must not carry over: {feedback}"
        );
    }

    #[test]
    fn it_should_let_the_agent_finish_with_a_message_when_the_count_of_refusals_cannot_be_kept() {
        // Arrange
        let mut workspace = FakeWorkspace {
            counts_cannot_be_kept: true,
            ..FakeWorkspace::with_the_specification()
        };

        // Act
        let message = finish_message(decide(AGENT, Some(REPLY), &mut workspace));

        // Assert
        assert!(
            message.contains("could not be kept (the disk is full)"),
            "the message must give the reason: {message}"
        );
        assert!(
            message.contains("the file does not exist"),
            "the message must still list the problems: {message}"
        );
    }

    #[rstest]
    #[case::draft(Some(DRAFT_REPLY), FakeWorkspace::default())]
    #[case::outside(Some("notes/task-reviewer-t5-r1.json"), FakeWorkspace::default())]
    #[case::missing_folder(Some(REPLY), FakeWorkspace::default())]
    #[case::no_number(
        Some("docs/issues/open/some-slug/agent-replies/task-reviewer-t5-r1.json"),
        FakeWorkspace { folders: vec!["docs/issues/open/some-slug".to_owned()], ..FakeWorkspace::default() }
    )]
    #[case::missing_file(Some(REPLY), FakeWorkspace::with_the_specification())]
    #[case::invalid_content(
        Some(REPLY),
        FakeWorkspace::with_the_specification().with_the_reply(ReplyFile::Text("{}".to_owned()))
    )]
    #[case::unreadable(
        Some(REPLY),
        FakeWorkspace::with_the_specification().with_the_reply(ReplyFile::Unreadable("it is a symbolic link".to_owned()))
    )]
    #[case::limit_reached(
        Some(REPLY),
        FakeWorkspace::with_the_specification().with_refusals(key(AGENT, REPLY_FILE_NAME), REFUSAL_LIMIT)
    )]
    #[case::count_not_kept(
        Some(REPLY),
        FakeWorkspace { counts_cannot_be_kept: true, ..FakeWorkspace::with_the_specification() }
    )]
    fn it_should_never_write_the_literal_line_that_names_the_reply_file_in_any_text_it_returns(
        #[case] path: Option<&str>,
        #[case] mut workspace: FakeWorkspace,
    ) {
        // Act
        let text = match decide(AGENT, path, &mut workspace) {
            Outcome::Finish { message } => message.expect("every case in this test says something"),
            Outcome::Refuse { feedback } => feedback,
        };

        // Assert
        assert!(
            !text.contains("Reply file:"),
            "the text must not carry the literal line: {text}"
        );
    }
}
