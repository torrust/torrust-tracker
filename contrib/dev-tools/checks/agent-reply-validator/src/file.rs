//! The checks of one reply file: where it is, its name, its content, and that the name and the
//! content agree.
//!
//! The checks take the file's repository-relative path and its text, so they read nothing from
//! the disk and every outcome can be tested with plain strings.

use std::fmt;

use crate::name::ReplyFileName;
use crate::reply::Reply;

/// The folder that holds the reply files of one issue specification.
pub const REPLIES_FOLDER: &str = "agent-replies";

/// The lifecycle folders of `docs/issues/` that hold issue specifications.
const LIFECYCLE_FOLDERS: [&str; 3] = ["drafts", "open", "closed"];

/// One reason a reply file is invalid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Problem {
    /// Which check found it.
    pub check: Check,
    /// The JSON path of the value at fault, for a problem in the content.
    pub field_path: Option<String>,
    /// What is wrong.
    pub message: String,
}

/// The checks a reply file goes through.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Check {
    /// The file is in the `agent-replies/` folder of an issue specification.
    Location,
    /// The file name follows `<agent>-<subject>-r<round>.json`.
    Name,
    /// The content follows the reply format.
    Content,
    /// The name and the content give the same agent, subject and round.
    Agreement,
}

impl fmt::Display for Check {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Location => "location",
            Self::Name => "name",
            Self::Content => "content",
            Self::Agreement => "agreement",
        })
    }
}

/// Checks one reply file and returns every problem found; an empty list means the file is valid.
///
/// `path` is the file's path relative to the repository root, with `/` separators; `text` is
/// its content. The agreement of name and content is checked only when both are valid.
#[must_use]
pub fn check_reply_file(path: &str, text: &str) -> Vec<Problem> {
    let mut problems = Vec::new();
    let (folder, file_name) = path.rsplit_once('/').unwrap_or(("", path));

    if !is_replies_folder_of_a_specification(folder) {
        problems.push(Problem {
            check: Check::Location,
            field_path: None,
            message: format!(
                "a reply file must be in `docs/issues/<{}>/<specification>/{REPLIES_FOLDER}/`",
                LIFECYCLE_FOLDERS.join("|")
            ),
        });
    }

    let name = file_name.parse::<ReplyFileName>().map_err(|error| Problem {
        check: Check::Name,
        field_path: None,
        message: error.to_string(),
    });
    let reply = Reply::from_json(text).map_err(|error| Problem {
        check: Check::Content,
        field_path: Some(error.path),
        message: error.message,
    });

    match (name, reply) {
        (Ok(name), Ok(reply)) => problems.extend(disagreements(&name, &reply)),
        (name, reply) => problems.extend(name.err().into_iter().chain(reply.err())),
    }
    problems
}

/// Whether the folder is `docs/issues/<lifecycle>/<specification>/agent-replies`.
fn is_replies_folder_of_a_specification(folder: &str) -> bool {
    matches!(
        folder.split('/').collect::<Vec<_>>().as_slice(),
        ["docs", "issues", lifecycle, specification, REPLIES_FOLDER]
            if LIFECYCLE_FOLDERS.contains(lifecycle) && !matches!(*specification, "" | "." | "..")
    )
}

/// The fields whose value differs from the file name.
fn disagreements(name: &ReplyFileName, reply: &Reply) -> Vec<Problem> {
    let mismatch = |field: &str, in_name: String, in_content: String| Problem {
        check: Check::Agreement,
        field_path: Some(field.to_owned()),
        message: format!("the file name says `{in_name}` but the field says `{in_content}`"),
    };
    let mut problems = Vec::new();
    if name.agent != reply.agent {
        problems.push(mismatch("agent", name.agent.to_string(), reply.agent.to_string()));
    }
    if name.subject != reply.subject {
        problems.push(mismatch("subject", name.subject.to_string(), reply.subject.to_string()));
    }
    if name.round != reply.round {
        problems.push(mismatch(
            "round",
            name.round.number().to_string(),
            reply.round.number().to_string(),
        ));
    }
    problems
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use super::{Check, Problem, check_reply_file};

    const FOLDER: &str = "docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/agent-replies";

    /// A valid reply whose agent, subject and round are the given ones.
    fn reply(agent: &str, subject: &str, round: u8) -> String {
        json!({
            "schema-version": 1,
            "agent": agent,
            "model": "claude-opus-5-5",
            "subject": subject,
            "round": round,
            "created-utc": "2026-11-18 17:25",
            "verdict": "REVIEW_PASSED",
            "summary": "Nothing blocks the task.",
            "findings": [],
            "questions": [],
            "evidence": ["cargo test -q -p agent-reply-validator passed"]
        })
        .to_string()
    }

    fn checks(problems: &[Problem]) -> Vec<Check> {
        problems.iter().map(|problem| problem.check).collect()
    }

    #[rstest]
    #[case::draft("docs/issues/drafts/some-slug/agent-replies")]
    #[case::open(FOLDER)]
    #[case::closed("docs/issues/closed/2904-2003-migrate-custom-agents-to-claude-code/agent-replies")]
    fn it_should_find_no_problem_when_the_file_is_in_a_specification_and_name_and_content_agree(#[case] folder: &str) {
        // Arrange
        let path = format!("{folder}/task-reviewer-t3-r2.json");

        // Act
        let problems = check_reply_file(&path, &reply("task-reviewer", "t3", 2));

        // Assert
        assert_eq!(problems, vec![]);
    }

    #[rstest]
    #[case::outside_the_issues("agent-replies/task-reviewer-t3-r2.json")]
    #[case::at_the_repository_root("task-reviewer-t3-r2.json")]
    #[case::issues_outside_the_documentation("notes/issues/open/2904-slug/agent-replies/task-reviewer-t3-r2.json")]
    #[case::folder_with_another_name("docs/issues/open/2904-slug/replies/task-reviewer-t3-r2.json")]
    #[case::unknown_lifecycle("docs/issues/archive/2904-slug/agent-replies/task-reviewer-t3-r2.json")]
    #[case::directly_in_the_lifecycle_folder("docs/issues/open/agent-replies/task-reviewer-t3-r2.json")]
    #[case::empty_specification_folder("docs/issues/open//agent-replies/task-reviewer-t3-r2.json")]
    #[case::current_folder_as_specification("docs/issues/open/./agent-replies/task-reviewer-t3-r2.json")]
    #[case::parent_folder_as_specification("docs/issues/open/../agent-replies/task-reviewer-t3-r2.json")]
    #[case::one_level_too_deep("docs/issues/open/2904-slug/notes/agent-replies/task-reviewer-t3-r2.json")]
    #[case::under_another_documentation_folder("docs/refactor-plans/open/a-plan/agent-replies/task-reviewer-t3-r2.json")]
    fn it_should_report_only_the_location_when_the_file_is_outside_the_replies_folder_of_a_specification(#[case] path: &str) {
        // Act
        let problems = check_reply_file(path, &reply("task-reviewer", "t3", 2));

        // Assert
        assert_eq!(
            problems,
            vec![Problem {
                check: Check::Location,
                field_path: None,
                message: "a reply file must be in `docs/issues/<drafts|open|closed>/<specification>/agent-replies/`".to_owned(),
            }]
        );
    }

    #[test]
    fn it_should_report_the_name_and_skip_the_agreement_when_the_name_is_invalid() {
        // Arrange
        let path = format!("{FOLDER}/task-reviewer-t3-r4.json");

        // Act
        let problems = check_reply_file(&path, &reply("task-reviewer", "t3", 2));

        // Assert
        assert_eq!(
            problems,
            vec![Problem {
                check: Check::Name,
                field_path: None,
                message: "`4` is not a round: expected a number from 1 to 3, with no leading zero".to_owned(),
            }]
        );
    }

    #[test]
    fn it_should_report_the_content_with_its_path_and_skip_the_agreement_when_the_content_is_invalid() {
        // Arrange
        let path = format!("{FOLDER}/task-reviewer-t3-r2.json");

        // Act
        let problems = check_reply_file(&path, &reply("task-reviewer", "t3", 9));

        // Assert: the position at the end of the message depends on the key order, so only the
        // reason is compared.
        assert_eq!(checks(&problems), vec![Check::Content]);
        assert_eq!(problems[0].field_path.as_deref(), Some("round"));
        assert!(
            problems[0]
                .message
                .starts_with("invalid value: integer `9`, expected a round number from 1 to 3"),
            "unexpected message `{}`",
            problems[0].message
        );
    }

    #[test]
    fn it_should_report_every_problem_when_location_name_and_content_are_all_invalid() {
        // Act
        let problems = check_reply_file("notes/task-reviewer.json", "{}");

        // Assert
        assert_eq!(checks(&problems), vec![Check::Location, Check::Name, Check::Content]);
    }

    #[rstest]
    #[case::agent(
        reply("implementer", "t3", 2),
        "agent",
        "the file name says `task-reviewer` but the field says `implementer`"
    )]
    #[case::subject(
        reply("task-reviewer", "pr2942", 2),
        "subject",
        "the file name says `t3` but the field says `pr2942`"
    )]
    #[case::round(reply("task-reviewer", "t3", 1), "round", "the file name says `2` but the field says `1`")]
    fn it_should_report_the_field_that_disagrees_with_the_name(#[case] text: String, #[case] field: &str, #[case] message: &str) {
        // Arrange
        let path = format!("{FOLDER}/task-reviewer-t3-r2.json");

        // Act
        let problems = check_reply_file(&path, &text);

        // Assert
        assert_eq!(
            problems,
            vec![Problem {
                check: Check::Agreement,
                field_path: Some(field.to_owned()),
                message: message.to_owned(),
            }]
        );
    }

    #[test]
    fn it_should_report_each_disagreeing_field_when_several_disagree() {
        // Arrange
        let path = format!("{FOLDER}/task-reviewer-t3-r2.json");

        // Act
        let problems = check_reply_file(&path, &reply("researcher", "pr7", 3));

        // Assert
        let fields: Vec<_> = problems.iter().map(|problem| problem.field_path.as_deref()).collect();
        assert_eq!(fields, vec![Some("agent"), Some("subject"), Some("round")]);
    }

    #[test]
    fn it_should_report_the_location_and_the_disagreement_when_both_are_wrong() {
        // Act
        let problems = check_reply_file("task-reviewer-t3-r2.json", &reply("task-reviewer", "t3", 1));

        // Assert
        assert_eq!(checks(&problems), vec![Check::Location, Check::Agreement]);
    }
}
