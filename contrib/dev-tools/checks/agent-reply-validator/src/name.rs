//! The reply file name: `<agent>-<subject>-r<round>.json`.
//!
//! The agent is one of the agents that write reply files; the subject is a task ID of the issue's
//! Implementation Plan in lower case (`t2`) or a pull request (`pr2920`); the round starts at 1
//! and is at most [`Round::MAX`]. No other name is accepted.

use std::fmt;
use std::str::FromStr;

/// The file extension every reply file has.
const EXTENSION: &str = ".json";

/// An agent that writes reply files.
///
/// `pr-review-handler` is not listed: its audit record `PR-REVIEW.md` is its trace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Agent {
    Orchestrator,
    Implementer,
    TaskReviewer,
    PrReviewer,
    Researcher,
}

impl Agent {
    /// Every agent, in the order of the agent roster.
    pub const ALL: [Self; 5] = [
        Self::Orchestrator,
        Self::Implementer,
        Self::TaskReviewer,
        Self::PrReviewer,
        Self::Researcher,
    ];

    /// The agent's name, as its agent file and the reply file name spell it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Orchestrator => "orchestrator",
            Self::Implementer => "implementer",
            Self::TaskReviewer => "task-reviewer",
            Self::PrReviewer => "pr-reviewer",
            Self::Researcher => "researcher",
        }
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// What a reply is about.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Subject {
    /// A task of the issue's Implementation Plan, such as `t2`.
    Task(u32),
    /// A pull request, such as `pr2920`.
    PullRequest(u32),
}

impl FromStr for Subject {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let invalid = || NameError::InvalidSubject(text.to_owned());
        if let Some(number) = text.strip_prefix("pr") {
            return parse_positive(number).map(Self::PullRequest).ok_or_else(invalid);
        }
        if let Some(number) = text.strip_prefix('t') {
            return parse_positive(number).map(Self::Task).ok_or_else(invalid);
        }
        Err(invalid())
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Task(number) => write!(formatter, "t{number}"),
            Self::PullRequest(number) => write!(formatter, "pr{number}"),
        }
    }
}

/// The round of a reply: each call of an agent on a subject writes the next round.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Round(u8);

impl Round {
    /// The last round an agent gets on a subject before the orchestrator asks the contributor.
    pub const MAX: u8 = 3;

    /// The round with this number, if it is between 1 and [`Round::MAX`].
    #[must_use]
    pub const fn new(number: u8) -> Option<Self> {
        if number >= 1 && number <= Self::MAX {
            Some(Self(number))
        } else {
            None
        }
    }

    /// The round's number.
    #[must_use]
    pub const fn number(self) -> u8 {
        self.0
    }
}

impl FromStr for Round {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        parse_positive(text)
            .and_then(|number| u8::try_from(number).ok())
            .and_then(Self::new)
            .ok_or_else(|| NameError::InvalidRound(text.to_owned()))
    }
}

impl fmt::Display for Round {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "r{}", self.0)
    }
}

/// A valid reply file name, split into its parts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplyFileName {
    pub agent: Agent,
    pub subject: Subject,
    pub round: Round,
}

impl FromStr for ReplyFileName {
    type Err = NameError;

    fn from_str(file_name: &str) -> Result<Self, Self::Err> {
        let stem = file_name
            .strip_suffix(EXTENSION)
            .ok_or_else(|| NameError::NotJson(file_name.to_owned()))?;
        let (agent, rest) = Agent::ALL
            .into_iter()
            .find_map(|agent| {
                stem.strip_prefix(agent.name())
                    .and_then(|rest| rest.strip_prefix('-'))
                    .map(|rest| (agent, rest))
            })
            .ok_or_else(|| NameError::UnknownAgent(file_name.to_owned()))?;
        let (subject, round) = rest
            .rsplit_once("-r")
            .ok_or_else(|| NameError::MissingRound(file_name.to_owned()))?;
        Ok(Self {
            agent,
            subject: subject.parse()?,
            round: round.parse()?,
        })
    }
}

impl fmt::Display for ReplyFileName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}-{}-{}{EXTENSION}", self.agent, self.subject, self.round)
    }
}

/// Why a file name is not a valid reply file name. Each variant holds the text it rejects.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NameError {
    #[error("`{0}` does not end in `{EXTENSION}`")]
    NotJson(String),
    #[error("`{0}` does not start with the name of an agent that writes reply files, followed by `-`")]
    UnknownAgent(String),
    #[error("`{0}` has no round: the name must end in `-r<round>{EXTENSION}`")]
    MissingRound(String),
    #[error("`{0}` is not a subject: expected `t<task>` or `pr<number>`, with no leading zero")]
    InvalidSubject(String),
    #[error("`{0}` is not a round: expected a number from 1 to {max}, with no leading zero", max = Round::MAX)]
    InvalidRound(String),
}

/// A positive decimal number with no sign and no leading zero.
fn parse_positive(text: &str) -> Option<u32> {
    // `u32::from_str` accepts a leading `+`, so the digits are checked first.
    let well_formed = !text.starts_with('0') && text.bytes().all(|byte| byte.is_ascii_digit());
    if well_formed { text.parse().ok() } else { None }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{Agent, NameError, ReplyFileName, Round, Subject};

    fn name(agent: Agent, subject: Subject, round: u8) -> ReplyFileName {
        ReplyFileName {
            agent,
            subject,
            round: Round::new(round).expect("the round is in range"),
        }
    }

    #[rstest]
    #[case::task_reviewer_on_a_task("task-reviewer-t2-r1.json", name(Agent::TaskReviewer, Subject::Task(2), 1))]
    #[case::pr_reviewer_on_a_pull_request("pr-reviewer-pr2920-r3.json", name(Agent::PrReviewer, Subject::PullRequest(2920), 3))]
    #[case::orchestrator("orchestrator-t12-r2.json", name(Agent::Orchestrator, Subject::Task(12), 2))]
    #[case::implementer("implementer-t3-r1.json", name(Agent::Implementer, Subject::Task(3), 1))]
    #[case::researcher("researcher-pr7-r1.json", name(Agent::Researcher, Subject::PullRequest(7), 1))]
    fn it_should_split_the_name_into_agent_subject_and_round_when_it_follows_the_rule(
        #[case] file_name: &str,
        #[case] expected: ReplyFileName,
    ) {
        // Act
        let parsed: ReplyFileName = file_name.parse().expect("the name is valid");

        // Assert
        assert_eq!(parsed, expected);
    }

    #[rstest]
    #[case::zero(0, false)]
    #[case::first(1, true)]
    #[case::last(Round::MAX, true)]
    #[case::past_the_limit(Round::MAX + 1, false)]
    fn it_should_accept_a_round_number_only_when_it_is_between_one_and_the_limit(#[case] number: u8, #[case] accepted: bool) {
        // Act
        let round = Round::new(number);

        // Assert
        assert_eq!(round.map(Round::number), accepted.then_some(number));
    }

    #[rstest]
    #[case::task("implementer-t12-r3.json")]
    #[case::pull_request("task-reviewer-pr2920-r2.json")]
    fn it_should_print_the_name_it_parsed_when_displayed(#[case] file_name: &str) {
        // Arrange
        let parsed: ReplyFileName = file_name.parse().expect("the name is valid");

        // Act
        let printed = parsed.to_string();

        // Assert
        assert_eq!(printed, file_name);
    }

    #[rstest]
    #[case::markdown("task-reviewer-t2-r1.md", NameError::NotJson("task-reviewer-t2-r1.md".to_owned()))]
    #[case::no_extension("task-reviewer-t2-r1", NameError::NotJson("task-reviewer-t2-r1".to_owned()))]
    #[case::not_ported_agent("planner-t2-r1.json", NameError::UnknownAgent("planner-t2-r1.json".to_owned()))]
    #[case::handler_writes_no_reply("pr-review-handler-pr2920-r1.json", NameError::UnknownAgent("pr-review-handler-pr2920-r1.json".to_owned()))]
    #[case::agent_with_wrong_separator("task-reviewer_t2-r1.json", NameError::UnknownAgent("task-reviewer_t2-r1.json".to_owned()))]
    #[case::upper_case_agent("Task-reviewer-t2-r1.json", NameError::UnknownAgent("Task-reviewer-t2-r1.json".to_owned()))]
    #[case::no_round("task-reviewer-t2.json", NameError::MissingRound("task-reviewer-t2.json".to_owned()))]
    #[case::upper_case_task("task-reviewer-T2-r1.json", NameError::InvalidSubject("T2".to_owned()))]
    #[case::task_zero("task-reviewer-t0-r1.json", NameError::InvalidSubject("t0".to_owned()))]
    #[case::task_leading_zero("task-reviewer-t02-r1.json", NameError::InvalidSubject("t02".to_owned()))]
    #[case::task_without_number("task-reviewer-t-r1.json", NameError::InvalidSubject("t".to_owned()))]
    #[case::task_with_plus_sign("task-reviewer-t+2-r1.json", NameError::InvalidSubject("t+2".to_owned()))]
    #[case::pull_request_zero("task-reviewer-pr0-r1.json", NameError::InvalidSubject("pr0".to_owned()))]
    #[case::pull_request_leading_zero("task-reviewer-pr02-r1.json", NameError::InvalidSubject("pr02".to_owned()))]
    #[case::pull_request_with_plus_sign("task-reviewer-pr+2-r1.json", NameError::InvalidSubject("pr+2".to_owned()))]
    #[case::upper_case_pull_request("task-reviewer-PR2920-r1.json", NameError::InvalidSubject("PR2920".to_owned()))]
    #[case::pull_request_with_hash("task-reviewer-pr#2920-r1.json", NameError::InvalidSubject("pr#2920".to_owned()))]
    #[case::unknown_subject_kind("task-reviewer-issue2904-r1.json", NameError::InvalidSubject("issue2904".to_owned()))]
    #[case::round_zero("task-reviewer-t2-r0.json", NameError::InvalidRound("0".to_owned()))]
    #[case::round_past_the_limit("task-reviewer-t2-r4.json", NameError::InvalidRound("4".to_owned()))]
    #[case::round_leading_zero("task-reviewer-t2-r01.json", NameError::InvalidRound("01".to_owned()))]
    #[case::round_with_plus_sign("task-reviewer-t2-r+1.json", NameError::InvalidRound("+1".to_owned()))]
    #[case::round_that_wraps_to_one_in_a_byte("task-reviewer-t2-r257.json", NameError::InvalidRound("257".to_owned()))]
    fn it_should_reject_the_name_when_it_breaks_the_naming_rule(#[case] file_name: &str, #[case] expected: NameError) {
        // Act
        let error = file_name.parse::<ReplyFileName>().expect_err("the name is invalid");

        // Assert
        assert_eq!(error, expected);
    }
}
