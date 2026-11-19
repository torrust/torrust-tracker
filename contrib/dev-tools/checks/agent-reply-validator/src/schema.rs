//! The `agent-reply-v1` JSON Schema, generated from the reply model.
//!
//! The model in [`crate::reply`] is the authority; the schema is its projection for other tools
//! and readers, tracked at [`ARTIFACT_PATH`]. The validator differs from a JSON Schema validator
//! in these places, which the schema cannot express:
//!
//! - `created-utc` must name a real date and time, not only have the layout;
//! - a number with a fraction, such as `1.0`, is not an integer, though JSON Schema counts it as
//!   one;
//! - a key given twice is rejected; a JSON Schema validator sees only the parsed value;
//! - the white space of a blank text is Unicode white space, as Rust's `trim` defines it; the
//!   pattern `\S` uses the ECMA-262 set, which differs for a few characters: a text of only
//!   U+0085 passes the schema and fails the validator, and a text of only U+FEFF fails the schema
//!   and passes the validator;
//! - the number of a subject must fit in 32 bits: the pattern accepts `t4294967296`, the
//!   validator rejects it.

use std::borrow::Cow;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};

use crate::name::{Agent, Round, Subject};
use crate::reply::{Reply, SCHEMA_VERSION, SchemaVersion, Text, UtcMinute};

/// Where the generated schema is tracked, relative to the repository root.
pub const ARTIFACT_PATH: &str = "docs/schemas/agent-reply-v1.schema.json";

/// The pattern of a subject: `t<task>` or `pr<number>`, with no leading zero.
pub const SUBJECT_PATTERN: &str = "^(t|pr)[1-9][0-9]*$";

/// The layout of a UTC time to the minute.
pub const UTC_MINUTE_PATTERN: &str = "^[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}$";

/// A text with a character that is not white space.
pub const NOT_BLANK_PATTERN: &str = "\\S";

/// The schema of a reply.
#[must_use]
pub fn reply_schema() -> Schema {
    schemars::schema_for!(Reply)
}

/// The schema as the tracked artifact holds it: pretty JSON and a final newline.
///
/// # Panics
///
/// Never in practice: a generated schema is a JSON value, which always serializes.
#[must_use]
pub fn reply_schema_json() -> String {
    let json = serde_json::to_string_pretty(&reply_schema()).expect("a schema serializes as JSON");
    format!("{json}\n")
}

impl JsonSchema for SchemaVersion {
    fn schema_name() -> Cow<'static, str> {
        "SchemaVersion".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "description": "The version of the reply format.",
            "type": "integer",
            "const": SCHEMA_VERSION
        })
    }
}

impl JsonSchema for Agent {
    fn schema_name() -> Cow<'static, str> {
        "Agent".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        let names: Vec<&str> = Self::ALL.iter().map(|agent| agent.name()).collect();
        json_schema!({
            "description": "An agent that writes reply files.",
            "type": "string",
            "enum": names
        })
    }
}

impl JsonSchema for Subject {
    fn schema_name() -> Cow<'static, str> {
        "Subject".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "description": "A task of the Implementation Plan (`t2`) or a pull request (`pr2920`).",
            "type": "string",
            "pattern": SUBJECT_PATTERN
        })
    }
}

impl JsonSchema for Round {
    fn schema_name() -> Cow<'static, str> {
        "Round".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "description": "The round of the agent on the subject.",
            "type": "integer",
            "minimum": 1,
            "maximum": Self::MAX
        })
    }
}

impl JsonSchema for Text {
    fn schema_name() -> Cow<'static, str> {
        "Text".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "description": "A text that is not blank.",
            "type": "string",
            "pattern": NOT_BLANK_PATTERN
        })
    }
}

impl JsonSchema for UtcMinute {
    fn schema_name() -> Cow<'static, str> {
        "UtcMinute".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "description": "A UTC time written `YYYY-MM-DD HH:MM`.",
            "type": "string",
            "pattern": UTC_MINUTE_PATTERN
        })
    }
}

#[cfg(test)]
mod tests {
    // The pattern tests read each pattern with the Rust `regex` engine, an approximation of the
    // ECMA-262 engine that JSON Schema names. They show that pattern and Rust check agree on the
    // samples, not how every JSON Schema validator reads the pattern; the module doc names the
    // known gap.
    use regex::Regex;
    use rstest::rstest;
    use serde_json::{Value, json};

    use super::{NOT_BLANK_PATTERN, SUBJECT_PATTERN, UTC_MINUTE_PATTERN, reply_schema};
    use crate::name::Subject;
    use crate::reply::{Text, UtcMinute};

    fn schema() -> Value {
        serde_json::to_value(reply_schema()).expect("the schema is JSON")
    }

    #[test]
    fn it_should_require_every_field_of_the_reply_and_forbid_any_other() {
        // Act
        let reply = schema();

        // Assert
        let fields = json!([
            "schema-version",
            "agent",
            "model",
            "subject",
            "round",
            "created-utc",
            "verdict",
            "summary",
            "findings",
            "questions",
            "evidence"
        ]);
        assert_eq!(
            (&reply["type"], &reply["required"], &reply["additionalProperties"]),
            (&json!("object"), &fields, &json!(false))
        );
    }

    #[test]
    fn it_should_require_every_field_of_a_finding_and_forbid_any_other() {
        // Act
        let finding = schema()["$defs"]["Finding"].clone();

        // Assert
        let fields = json!(["severity", "location", "description"]);
        assert_eq!(
            (&finding["type"], &finding["required"], &finding["additionalProperties"]),
            (&json!("object"), &fields, &json!(false))
        );
    }

    #[test]
    fn it_should_list_exactly_the_agents_that_write_reply_files() {
        // Act
        let agents = schema()["$defs"]["Agent"]["enum"].clone();

        // Assert
        assert_eq!(
            agents,
            json!(["orchestrator", "implementer", "task-reviewer", "pr-reviewer", "researcher"])
        );
    }

    #[test]
    fn it_should_bound_the_round_from_one_to_three() {
        // Act
        let round = schema()["$defs"]["Round"].clone();

        // Assert
        assert_eq!(
            (&round["type"], &round["minimum"], &round["maximum"]),
            (&json!("integer"), &json!(1), &json!(3))
        );
    }

    #[test]
    fn it_should_fix_the_schema_version_to_one() {
        // Act
        let version = schema()["$defs"]["SchemaVersion"].clone();

        // Assert
        assert_eq!((&version["type"], &version["const"]), (&json!("integer"), &json!(1)));
    }

    #[test]
    fn it_should_close_the_lists_of_verdicts_and_severities() {
        // Act
        let schema = schema();

        // Assert
        let verdicts: Vec<&Value> = schema["$defs"]["Verdict"]["oneOf"]
            .as_array()
            .expect("the verdicts are alternatives")
            .iter()
            .map(|verdict| &verdict["const"])
            .collect();
        assert_eq!(verdicts, ["REVIEW_PASSED", "REVIEW_FAILED", "DONE", "BLOCKED", "QUESTION"]);
        assert_eq!(
            schema["$defs"]["Severity"]["enum"],
            json!(["Blocker", "Major", "Minor", "Nit", "Suggestion"])
        );
    }

    #[rstest]
    #[case::task("t2")]
    #[case::pull_request("pr2920")]
    #[case::task_zero("t0")]
    #[case::leading_zero("pr02")]
    #[case::plus_sign("t+2")]
    #[case::upper_case("T2")]
    #[case::other_kind("issue2904")]
    #[case::trailing_text("t2x")]
    fn it_should_match_the_subject_pattern_when_the_parser_accepts_the_subject(#[case] subject: &str) {
        // Act
        let matched = Regex::new(SUBJECT_PATTERN).expect("the pattern compiles").is_match(subject);

        // Assert
        assert_eq!(
            matched,
            subject.parse::<Subject>().is_ok(),
            "the pattern, read by the Rust `regex` engine, and the parser disagree on `{subject}`"
        );
    }

    #[test]
    fn it_should_match_a_subject_number_too_large_for_the_parser() {
        // Arrange: the pattern has no length bound; the parser reads the number as a `u32`.
        let subject = "t4294967296";

        // Act
        let matched = Regex::new(SUBJECT_PATTERN).expect("the pattern compiles").is_match(subject);

        // Assert: a known difference, named in the module doc.
        assert!(matched);
        assert!(subject.parse::<Subject>().is_err());
    }

    #[rstest]
    #[case::valid("2026-11-18 17:25")]
    #[case::with_seconds("2026-11-18 17:25:00")]
    #[case::iso_form("2026-11-18T17:25")]
    #[case::slashes("2026/11/18 17:25")]
    #[case::plus_sign("2026-+1-18 17:25")]
    #[case::short_minute("2026-11-18 17:5")]
    fn it_should_match_the_time_pattern_when_the_time_has_the_layout(#[case] time: &str) {
        // Act: the pattern checks only the layout; the model also checks the calendar.
        let matched = Regex::new(UTC_MINUTE_PATTERN).expect("the pattern compiles").is_match(time);

        // Assert
        assert_eq!(
            matched,
            UtcMinute::new(time).is_some(),
            "the pattern, read by the Rust `regex` engine, and the model disagree on `{time}`"
        );
    }

    #[rstest]
    #[case::word("claude-opus-5-5")]
    #[case::padded(" a ")]
    #[case::empty("")]
    #[case::spaces("   ")]
    #[case::tab_and_newline("\t\n")]
    fn it_should_match_the_text_pattern_when_the_text_is_not_blank(#[case] text: &str) {
        // Act
        let matched = Regex::new(NOT_BLANK_PATTERN).expect("the pattern compiles").is_match(text);

        // Assert
        assert_eq!(
            matched,
            Text::new(text).is_some(),
            "the pattern, read by the Rust `regex` engine, and the model disagree on `{text:?}`"
        );
    }
}
