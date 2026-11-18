//! The content of a reply file: one JSON object with the fields of the `agent-reply-v1` schema.
//!
//! Every field is required and no other field is accepted, so a reply either has the complete
//! shape or is rejected with the JSON path of the first problem.
//!
//! The fields are checked one by one; no rule joins two of them. For example a `QUESTION`
//! verdict with no questions is accepted. The orchestrator reads only the verdict, and any use of
//! the replies beyond that is out of the scope of issue #2904.

use std::fmt;
use std::ops::Range;

use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::{Deserialize, Serialize, Serializer};

use crate::name::{Agent, Round, Subject};

/// The version of the reply format this crate reads and writes.
pub const SCHEMA_VERSION: u8 = 1;

/// One reply of an agent, as stored in its reply file.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Reply {
    /// Always [`SCHEMA_VERSION`].
    pub schema_version: SchemaVersion,
    /// The agent that wrote the reply.
    pub agent: Agent,
    /// The model the agent ran on, as the harness names it, such as `claude-opus-5-5`.
    pub model: Text,
    /// What the reply is about: a task (`t2`) or a pull request (`pr2920`).
    pub subject: Subject,
    /// The round of this agent on this subject.
    pub round: Round,
    /// When the reply was written, in UTC: `YYYY-MM-DD HH:MM`.
    pub created_utc: UtcMinute,
    /// The outcome, from which the orchestrator decides the next step.
    pub verdict: Verdict,
    /// A short account of what the agent did or found.
    pub summary: Text,
    /// The problems found; empty when there are none.
    pub findings: Vec<Finding>,
    /// The questions for the contributor; empty when there are none.
    pub questions: Vec<Text>,
    /// The commands run and what they printed, or other facts the reply rests on.
    pub evidence: Vec<Text>,
}

impl Reply {
    /// Reads a reply from the text of a reply file.
    ///
    /// # Errors
    ///
    /// Returns a [`ReplyError`] with the JSON path of the first value that does not fit the
    /// format, or the position of the first syntax error.
    pub fn from_json(text: &str) -> Result<Self, ReplyError> {
        let mut deserializer = serde_json::Deserializer::from_str(text);
        let reply = serde_path_to_error::deserialize(&mut deserializer).map_err(|error| ReplyError {
            path: error.path().to_string(),
            message: error.inner().to_string(),
        })?;
        deserializer.end().map_err(|error| ReplyError {
            path: ".".to_owned(),
            message: error.to_string(),
        })?;
        Ok(reply)
    }
}

/// Why a text is not a valid reply.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{path}: {message}")]
pub struct ReplyError {
    /// The JSON path of the value that does not fit, or `.` for the whole document.
    pub path: String,
    /// What is wrong with it.
    pub message: String,
}

/// The outcome of a reply. The list is closed: the orchestrator reads only this field.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Verdict {
    /// A review found nothing that blocks the work.
    ReviewPassed,
    /// A review found something that must change.
    ReviewFailed,
    /// The agent finished the work it was given.
    Done,
    /// The agent cannot go on without something it does not have.
    Blocked,
    /// The agent needs an answer from the contributor.
    Question,
}

/// One problem found by the agent.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    /// How much the problem matters.
    pub severity: Severity,
    /// Where it is, such as `src/name.rs:42` or a section of a document.
    pub location: Text,
    /// What is wrong.
    pub description: Text,
}

/// The severity of a finding, with the same names as the pull-request review audits.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Severity {
    Blocker,
    Major,
    Minor,
    Nit,
    Suggestion,
}

/// The schema version field: the number [`SCHEMA_VERSION`] and nothing else.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchemaVersion;

impl Serialize for SchemaVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(SCHEMA_VERSION)
    }
}

impl<'de> Deserialize<'de> for SchemaVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let version = u64::deserialize(deserializer)?;
        if version == u64::from(SCHEMA_VERSION) {
            Ok(Self)
        } else {
            Err(de::Error::invalid_value(
                Unexpected::Unsigned(version),
                &"the schema version 1",
            ))
        }
    }
}

/// A text with at least one character that is not white space.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Text(String);

impl Text {
    /// The text, if it has a character that is not white space.
    #[must_use]
    pub fn new(text: impl Into<String>) -> Option<Self> {
        let text = text.into();
        if is_blank(&text) { None } else { Some(Self(text)) }
    }

    /// The text itself.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Text {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        if is_blank(&text) {
            return Err(de::Error::invalid_value(Unexpected::Str(&text), &"a text that is not blank"));
        }
        Ok(Self(text))
    }
}

/// Whether a text has no character other than white space.
fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}

/// A UTC time to the minute, written `YYYY-MM-DD HH:MM`, as in the issue specifications.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct UtcMinute(String);

impl UtcMinute {
    /// The time, if the text has the layout and names a real date and time.
    #[must_use]
    pub fn new(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();
        let layout = bytes.len() == 16
            && bytes.iter().enumerate().all(|(index, byte)| match index {
                4 | 7 => *byte == b'-',
                10 => *byte == b' ',
                13 => *byte == b':',
                _ => byte.is_ascii_digit(),
            });
        if !layout {
            return None;
        }
        let number = |range: Range<usize>| text[range].parse::<u16>().ok();
        let (year, month, day, hour, minute) = (number(0..4)?, number(5..7)?, number(8..10)?, number(11..13)?, number(14..16)?);
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => 29,
            2 => 28,
            _ => return None,
        };
        let valid = (1..=days_in_month).contains(&day) && hour < 24 && minute < 60;
        valid.then(|| Self(text.to_owned()))
    }
}

impl<'de> Deserialize<'de> for UtcMinute {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::new(&text).ok_or_else(|| de::Error::invalid_value(Unexpected::Str(&text), &"a UTC time written YYYY-MM-DD HH:MM"))
    }
}

impl Serialize for Agent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.name())
    }
}

impl<'de> Deserialize<'de> for Agent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;
        Self::ALL
            .into_iter()
            .find(|agent| agent.name() == name)
            .ok_or_else(|| de::Error::invalid_value(Unexpected::Str(&name), &"an agent that writes reply files"))
    }
}

impl Serialize for Subject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Subject {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse()
            .map_err(|_| de::Error::invalid_value(Unexpected::Str(&text), &"a subject `t<task>` or `pr<number>`"))
    }
}

impl Serialize for Round {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(self.number())
    }
}

impl<'de> Deserialize<'de> for Round {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RoundVisitor;

        impl Visitor<'_> for RoundVisitor {
            type Value = Round;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a round number from 1 to {}", Round::MAX)
            }

            fn visit_u64<E: de::Error>(self, number: u64) -> Result<Round, E> {
                u8::try_from(number)
                    .ok()
                    .and_then(Round::new)
                    .ok_or_else(|| E::invalid_value(Unexpected::Unsigned(number), &self))
            }
        }

        deserializer.deserialize_u64(RoundVisitor)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::{Value, json};

    use super::{Finding, Reply, Severity, Text, UtcMinute, Verdict};
    use crate::name::{Agent, Round, Subject};

    /// A complete reply of a task reviewer that failed a review with one finding.
    fn valid_reply() -> Value {
        json!({
            "schema-version": 1,
            "agent": "task-reviewer",
            "model": "claude-opus-5-5",
            "subject": "t3",
            "round": 2,
            "created-utc": "2026-11-18 17:25",
            "verdict": "REVIEW_FAILED",
            "summary": "The digit check has no test.",
            "findings": [
                { "severity": "Major", "location": "src/name.rs:212", "description": "A leading + is accepted." }
            ],
            "questions": [],
            "evidence": ["cargo test -q -p agent-reply-validator: 23 passed"]
        })
    }

    /// The valid reply with one top-level field replaced.
    fn reply_with(field: &str, value: Value) -> String {
        let mut reply = valid_reply();
        reply[field] = value;
        reply.to_string()
    }

    /// The valid reply without one top-level field.
    fn reply_without(field: &str) -> String {
        let mut reply = valid_reply();
        reply.as_object_mut().expect("the reply is an object").remove(field);
        reply.to_string()
    }

    fn text(value: &str) -> Text {
        Text::new(value).expect("the text is not blank")
    }

    #[test]
    fn it_should_read_every_field_when_the_reply_is_complete() {
        // Arrange
        let json = valid_reply().to_string();

        // Act
        let reply = Reply::from_json(&json).expect("the reply is valid");

        // Assert
        assert_eq!(reply.agent, Agent::TaskReviewer);
        assert_eq!(reply.model, text("claude-opus-5-5"));
        assert_eq!(reply.subject, Subject::Task(3));
        assert_eq!(reply.round, Round::new(2).expect("the round is in range"));
        assert_eq!(
            reply.created_utc,
            UtcMinute::new("2026-11-18 17:25").expect("the time is valid")
        );
        assert_eq!(reply.verdict, Verdict::ReviewFailed);
        assert_eq!(reply.summary, text("The digit check has no test."));
        assert_eq!(
            reply.findings,
            vec![Finding {
                severity: Severity::Major,
                location: text("src/name.rs:212"),
                description: text("A leading + is accepted."),
            }]
        );
        assert_eq!(reply.questions, Vec::<Text>::new());
        assert_eq!(
            reply.evidence,
            vec![text("cargo test -q -p agent-reply-validator: 23 passed")]
        );
    }

    #[test]
    fn it_should_write_the_same_json_it_read_when_serialized() {
        // Arrange
        let reply = Reply::from_json(&valid_reply().to_string()).expect("the reply is valid");

        // Act
        let written = serde_json::to_value(&reply).expect("the reply serializes");

        // Assert
        assert_eq!(written, valid_reply());
    }

    #[rstest]
    #[case::review_passed("REVIEW_PASSED", Verdict::ReviewPassed)]
    #[case::review_failed("REVIEW_FAILED", Verdict::ReviewFailed)]
    #[case::done("DONE", Verdict::Done)]
    #[case::blocked("BLOCKED", Verdict::Blocked)]
    #[case::question("QUESTION", Verdict::Question)]
    fn it_should_read_the_verdict_when_it_is_in_the_closed_list(#[case] verdict: &str, #[case] expected: Verdict) {
        // Arrange
        let json = reply_with("verdict", json!(verdict));

        // Act
        let reply = Reply::from_json(&json).expect("the reply is valid");

        // Assert
        assert_eq!(reply.verdict, expected);
    }

    #[rstest]
    #[case::schema_version("schema-version")]
    #[case::agent("agent")]
    #[case::model("model")]
    #[case::subject("subject")]
    #[case::round("round")]
    #[case::created_utc("created-utc")]
    #[case::verdict("verdict")]
    #[case::summary("summary")]
    #[case::findings("findings")]
    #[case::questions("questions")]
    #[case::evidence("evidence")]
    fn it_should_reject_the_reply_when_a_field_is_missing(#[case] field: &str) {
        // Arrange
        let json = reply_without(field);

        // Act
        let error = Reply::from_json(&json).expect_err("the reply is incomplete");

        // Assert
        assert_eq!(error.path, ".");
        assert!(
            error.message.starts_with(&format!("missing field `{field}`")),
            "unexpected message `{}`",
            error.message
        );
    }

    #[rstest]
    #[case::unknown_field(reply_with("status", json!("done")), "status", "unknown field `status`")]
    #[case::duplicate_field(valid_reply().to_string().replacen('{', "{\"agent\": \"implementer\", ", 1), ".", "duplicate field `agent`")]
    #[case::schema_version_two(reply_with("schema-version", json!(2)), "schema-version", "the schema version 1")]
    #[case::schema_version_as_text(reply_with("schema-version", json!("1")), "schema-version", "invalid type: string")]
    #[case::handler_writes_no_reply(reply_with("agent", json!("pr-review-handler")), "agent", "an agent that writes reply files")]
    #[case::not_ported_agent(reply_with("agent", json!("planner")), "agent", "an agent that writes reply files")]
    #[case::blank_model(reply_with("model", json!("  ")), "model", "a text that is not blank")]
    #[case::upper_case_subject(reply_with("subject", json!("T3")), "subject", "a subject")]
    #[case::round_zero(reply_with("round", json!(0)), "round", "a round number from 1 to 3")]
    #[case::round_past_the_limit(reply_with("round", json!(4)), "round", "a round number from 1 to 3")]
    #[case::round_that_wraps_to_one_in_a_byte(reply_with("round", json!(257)), "round", "a round number from 1 to 3")]
    #[case::round_as_text(reply_with("round", json!("2")), "round", "invalid type: string")]
    #[case::time_with_seconds(reply_with("created-utc", json!("2026-11-18 17:25:00")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_in_iso_form(reply_with("created-utc", json!("2026-11-18T17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_with_slashes(reply_with("created-utc", json!("2026/11/18 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_with_a_dot(reply_with("created-utc", json!("2026-11-18 17.25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_with_an_extra_digit(reply_with("created-utc", json!("2026-11-18 17:255")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_with_a_plus_sign(reply_with("created-utc", json!("2026-+1-18 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_on_a_missing_leap_day(reply_with("created-utc", json!("2025-02-29 12:00")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_on_a_missing_leap_day_of_a_century(reply_with("created-utc", json!("2100-02-29 12:00")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_past_the_last_hour(reply_with("created-utc", json!("2026-11-18 24:00")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_past_the_last_minute(reply_with("created-utc", json!("2026-11-18 17:60")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_in_month_thirteen(reply_with("created-utc", json!("2026-13-18 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_on_day_thirty_one_of_april(reply_with("created-utc", json!("2026-04-31 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_on_day_thirty_one_of_june(reply_with("created-utc", json!("2026-06-31 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_on_day_thirty_one_of_september(reply_with("created-utc", json!("2026-09-31 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_on_day_thirty_one_of_november(reply_with("created-utc", json!("2026-11-31 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::time_on_day_zero(reply_with("created-utc", json!("2026-11-00 17:25")), "created-utc", "a UTC time written YYYY-MM-DD HH:MM")]
    #[case::verdict_outside_the_list(reply_with("verdict", json!("PASSED")), "verdict", "unknown variant `PASSED`")]
    #[case::verdict_in_lower_case(reply_with("verdict", json!("done")), "verdict", "unknown variant `done`")]
    #[case::blank_summary(reply_with("summary", json!("")), "summary", "a text that is not blank")]
    #[case::severity_outside_the_list(reply_with("findings", json!([{ "severity": "Critical", "location": "a", "description": "b" }])), "findings[0].severity", "unknown variant `Critical`")]
    #[case::blank_location(reply_with("findings", json!([{ "severity": "Nit", "location": "", "description": "b" }])), "findings[0].location", "a text that is not blank")]
    #[case::finding_without_description(reply_with("findings", json!([{ "severity": "Nit", "location": "a" }])), "findings[0]", "missing field `description`")]
    #[case::unknown_finding_field(reply_with("findings", json!([{ "severity": "Nit", "location": "a", "description": "b", "fix": "c" }])), "findings[0].fix", "unknown field `fix`")]
    #[case::blank_question(reply_with("questions", json!(["Which branch?", " "])), "questions[1]", "a text that is not blank")]
    #[case::blank_evidence(reply_with("evidence", json!([""])), "evidence[0]", "a text that is not blank")]
    #[case::evidence_as_text(reply_with("evidence", json!("cargo test")), "evidence", "invalid type: string")]
    #[case::truncated_json(r#"{"schema-version": 1, "agent": "#.to_owned(), "agent", "EOF while parsing")]
    #[case::something_after_the_reply(format!("{} {{}}", valid_reply()), ".", "trailing characters")]
    fn it_should_reject_the_reply_with_the_path_and_reason_of_the_value_that_breaks_the_format(
        #[case] json: String,
        #[case] path: &str,
        #[case] reason: &str,
    ) {
        // Act
        let error = Reply::from_json(&json).expect_err("the reply is invalid");

        // Assert
        assert_eq!(error.path, path, "unexpected path for the error `{}`", error.message);
        assert!(
            error.message.contains(reason),
            "the error `{}` does not say `{reason}`",
            error.message
        );
    }

    #[rstest]
    #[case::leap_day_of_a_leap_year("2024-02-29 12:00")]
    #[case::leap_day_of_a_year_divisible_by_four_hundred("2000-02-29 12:00")]
    #[case::last_day_of_a_thirty_one_day_month("2026-01-31 12:00")]
    #[case::last_minute_of_the_year("2026-12-31 23:59")]
    fn it_should_accept_the_time_when_it_names_a_real_minute(#[case] time: &str) {
        // Act
        let reply = Reply::from_json(&reply_with("created-utc", json!(time)));

        // Assert
        assert!(reply.is_ok(), "`{time}` was rejected: {reply:?}");
    }
}
