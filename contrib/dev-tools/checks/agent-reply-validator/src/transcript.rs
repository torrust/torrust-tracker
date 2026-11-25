//! The reply file a task names, read from an agent's transcript file.
//!
//! Whoever starts a task reviewer writes one line `Reply file: <path>` in the task, and writes it
//! again in each request that continues the reviewer (decision D17 of issue #2904). A stop hook
//! has no field for the task, so it reads the line from the agent's transcript: a file of JSON
//! records, one per line, whose format Claude Code does not document.
//!
//! The format used here is the one scenario M8 observed on Claude Code 2.1.318 (section V8 of the
//! issue's manual verification evidence), and the tests build records shaped like those:
//!
//! - the first request is a record of type `user` with no `isMeta`, no `origin` and the task as a
//!   string;
//! - a continued request is a record of type `user` whose `origin.kind` is `coordinator`, and whose
//!   string is a one-line prefix followed by the task;
//! - tool results, preloaded skill text and harness reminders are `user` records too, but they are
//!   neither of the above (a tool result holds blocks, the others have `isMeta` and no `origin`);
//! - records of type `api-request-blob` repeat the request text and are not requests.
//!
//! A request recorded in another way, by another caller or another Claude Code version, is not
//! found, and the task then looks like one that has no line.
//!
//! The `PreToolUse` hook input, unlike the `SubagentStop` one, names only the transcript of the
//! session that started the agent. Scenario M7 observed where Claude Code keeps the agent's own
//! file: `<folder>/<session>.jsonl` is the session's and `<folder>/<session>/subagents/agent-<id>.jsonl`
//! the agent's. [`subagent_transcript_path`] builds that path.

use std::path::{Path, PathBuf};

use serde_json::Value;

/// The start of the line that names the reply file.
const LINE_PREFIX: &str = "Reply file: ";

/// The `origin.kind` of a request that continues a finished agent.
const CONTINUATION_ORIGIN_KIND: &str = "coordinator";

/// The path in the last request of the transcript that carries a `Reply file:` line.
///
/// `transcript` is the content of the transcript file. A record that is not valid JSON, such as a
/// line cut off by a write in progress, is skipped. When a request carries several lines, the last
/// one counts. `None` means that no request carries a line.
#[must_use]
pub fn latest_reply_file(transcript: &str) -> Option<String> {
    transcript
        .lines()
        .rev()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find_map(|record| request_text(&record).and_then(reply_file_in).map(str::to_owned))
}

/// The transcript file of the agent `agent_id`, from the transcript file of the session that
/// started it.
///
/// `None` when the session's transcript has no file stem, or when the id is not a single plain
/// path component (letters, digits, `-` and `_`), so that an id never leads outside the session's
/// folder.
#[must_use]
pub fn subagent_transcript_path(session_transcript: &Path, agent_id: &str) -> Option<PathBuf> {
    let plain = !agent_id.is_empty()
        && agent_id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_');
    if !plain {
        return None;
    }
    let session = session_transcript.file_stem()?;
    Some(
        session_transcript
            .with_file_name(session)
            .join("subagents")
            .join(format!("agent-{agent_id}.jsonl")),
    )
}

/// The text of the record when it is a request from the caller, the first one or a continued one.
fn request_text(record: &Value) -> Option<&str> {
    if record.get("type")?.as_str()? != "user" {
        return None;
    }
    let text = record.get("message")?.get("content")?.as_str()?;
    match record.get("origin").filter(|origin| !origin.is_null()) {
        Some(origin) => (origin.get("kind")?.as_str()? == CONTINUATION_ORIGIN_KIND).then_some(text),
        None => (!record.get("isMeta").and_then(Value::as_bool).unwrap_or(false)).then_some(text),
    }
}

/// The path of the last line of the text that starts with the prefix; a line without a path is
/// not one. One pair of backticks or quotes around the path is not part of it, so a caller that
/// writes the path as inline code still has its reply file enforced.
fn reply_file_in(text: &str) -> Option<&str> {
    text.lines()
        .filter_map(|line| line.strip_prefix(LINE_PREFIX))
        .map(str::trim)
        .map(unwrap_pair)
        .rfind(|path| !path.is_empty())
}

/// The text without one pair of backticks or quotes around it; other text is returned as it is.
fn unwrap_pair(path: &str) -> &str {
    ['`', '"', '\'']
        .into_iter()
        .find_map(|mark| path.strip_prefix(mark)?.strip_suffix(mark))
        .unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rstest::rstest;
    use serde_json::json;

    use super::{latest_reply_file, subagent_transcript_path};

    const ROUND_1: &str = "docs/issues/open/2904-slug/agent-replies/task-reviewer-t5-r1.json";
    const ROUND_2: &str = "docs/issues/open/2904-slug/agent-replies/task-reviewer-t5-r2.json";

    /// The task of the first request, with the line that names the file.
    fn task(file: &str) -> String {
        format!("Review task T5.\nReply file: {file}\nStay within the task.")
    }

    /// The first request, as the transcript records it.
    fn first_request(text: &str) -> String {
        json!({"type": "user", "message": {"role": "user", "content": text}}).to_string()
    }

    /// A request that continues the finished agent, as the transcript records it.
    fn continued_request(text: &str) -> String {
        json!({
            "type": "user",
            "isMeta": true,
            "origin": {"kind": "coordinator"},
            "message": {"role": "user", "content": format!("The coordinator sent a message while you were working:\n{text}")}
        })
        .to_string()
    }

    /// A harness reminder: a user record that Claude Code adds, with the text as a string.
    fn harness_reminder(text: &str) -> String {
        json!({"type": "user", "isMeta": true, "message": {"role": "user", "content": text}}).to_string()
    }

    /// A preloaded skill: a user record that Claude Code adds, with the skill's command line and its
    /// body as two text blocks.
    fn preloaded_skill(body: &str) -> String {
        json!({
            "type": "user",
            "isMeta": true,
            "message": {"role": "user", "content": [
                {"type": "text", "text": "<command-name>review-task</command-name>"},
                {"type": "text", "text": body}
            ]}
        })
        .to_string()
    }

    /// The result of a tool call.
    fn tool_result(text: &str) -> String {
        json!({
            "type": "user",
            "message": {"role": "user", "content": [{"type": "tool_result", "content": text}]}
        })
        .to_string()
    }

    /// The copy of a request that the transcript keeps next to the records (its content is blocks,
    /// so the string check alone already rejects it).
    fn request_copy(text: &str) -> String {
        json!({
            "type": "api-request-blob",
            "message": {"role": "user", "content": [{"type": "text", "text": text}]}
        })
        .to_string()
    }

    /// A message of the agent itself that quotes the line, with the text as a plain string. It is the
    /// case that pins the filter on the record type: nothing else rejects it.
    fn agent_message(text: &str) -> String {
        json!({"type": "assistant", "message": {"role": "assistant", "content": text}}).to_string()
    }

    fn jsonl(records: &[String]) -> String {
        records.join("\n")
    }

    #[test]
    fn it_should_find_the_line_of_the_first_request_when_the_agent_was_not_continued() {
        // Arrange
        let transcript = jsonl(&[first_request(&task(ROUND_1))]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(file.as_deref(), Some(ROUND_1));
    }

    #[test]
    fn it_should_find_the_line_of_the_latest_request_when_the_agent_was_continued() {
        // Arrange
        let transcript = jsonl(&[first_request(&task(ROUND_1)), continued_request(&task(ROUND_2))]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(file.as_deref(), Some(ROUND_2));
    }

    #[test]
    fn it_should_keep_the_earlier_line_when_the_latest_request_has_none() {
        // Arrange
        let transcript = jsonl(&[
            first_request(&task(ROUND_1)),
            continued_request("Answer the question the reviewer asked."),
        ]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(file.as_deref(), Some(ROUND_1));
    }

    #[test]
    fn it_should_find_nothing_when_no_request_has_a_line() {
        // Arrange
        let transcript = jsonl(&[first_request("Review task T5."), continued_request("Review it again.")]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(file, None);
    }

    #[rstest]
    #[case::harness_reminder(harness_reminder(&task("docs/issues/open/9999-reminder/agent-replies/example-t1-r1.json")))]
    #[case::preloaded_skill_text(preloaded_skill(&task("docs/issues/open/9999-skill/agent-replies/example-t1-r1.json")))]
    #[case::tool_result(tool_result(&task("docs/issues/open/9999-tool/agent-replies/example-t1-r1.json")))]
    #[case::copy_of_a_request(request_copy(&task("docs/issues/open/9999-copy/agent-replies/example-t1-r1.json")))]
    #[case::message_of_the_agent(agent_message(&task("docs/issues/open/9999-agent/agent-replies/example-t1-r1.json")))]
    fn it_should_ignore_a_line_that_the_caller_did_not_write_when_it_comes_after_the_request(#[case] later_record: String) {
        // Arrange
        let transcript = jsonl(&[first_request(&task(ROUND_1)), later_record]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(
            file.as_deref(),
            Some(ROUND_1),
            "a record the caller did not write, placed after the request, must not name the file"
        );
    }

    #[test]
    fn it_should_ignore_a_request_of_an_origin_that_was_not_observed() {
        // Arrange
        let from_another_origin = json!({
            "type": "user",
            "isMeta": true,
            "origin": {"kind": "task-notification"},
            "message": {"role": "user", "content": task(ROUND_2)}
        })
        .to_string();
        let transcript = jsonl(&[first_request(&task(ROUND_1)), from_another_origin]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(
            file.as_deref(),
            Some(ROUND_1),
            "a request of an origin that was not observed must not replace the first request"
        );
    }

    /// V8 observed only an absent `origin`; a `null` one is tolerated as the same thing, as a
    /// precaution that no run has shown to be needed.
    #[test]
    fn it_should_read_the_request_when_its_origin_is_null() {
        // Arrange
        let request = json!({
            "type": "user",
            "origin": null,
            "message": {"role": "user", "content": task(ROUND_1)}
        })
        .to_string();

        // Act
        let file = latest_reply_file(&request);

        // Assert
        assert_eq!(file.as_deref(), Some(ROUND_1));
    }

    #[rstest]
    #[case::in_the_middle_of_a_sentence("Write it as the Reply file: x.json says.")]
    #[case::indented("  Reply file: x.json")]
    #[case::quoted("`Reply file: x.json`")]
    #[case::without_a_path("Reply file: ")]
    #[case::with_another_case("reply file: x.json")]
    fn it_should_not_take_a_line_that_does_not_start_with_the_prefix_and_a_path(#[case] text: &str) {
        // Arrange
        let transcript = jsonl(&[first_request(text)]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(
            file, None,
            "the text is not a line that starts with the prefix and has a path"
        );
    }

    #[rstest]
    #[case::trailing_spaces("Reply file: x.json   ")]
    #[case::lone_carriage_return("Reply file: x.json\r")]
    fn it_should_trim_the_path_of_the_line(#[case] text: &str) {
        // Arrange
        let transcript = jsonl(&[first_request(text)]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(
            file.as_deref(),
            Some("x.json"),
            "the path must not keep the characters after it"
        );
    }

    #[rstest]
    #[case::backticks("Reply file: `x.json`")]
    #[case::double_quotes("Reply file: \"x.json\"")]
    #[case::single_quotes("Reply file: 'x.json'")]
    #[case::quotes_and_trailing_space("Reply file: `x.json` ")]
    fn it_should_not_keep_one_pair_of_backticks_or_quotes_around_the_path(#[case] text: &str) {
        // Arrange
        let transcript = jsonl(&[first_request(text)]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(
            file.as_deref(),
            Some("x.json"),
            "the marks around the path are not part of it"
        );
    }

    #[test]
    fn it_should_keep_a_mark_that_has_no_pair_around_the_path() {
        // Arrange
        let transcript = jsonl(&[first_request("Reply file: `x.json")]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(
            file.as_deref(),
            Some("`x.json"),
            "a lone mark is part of the text and is not removed"
        );
    }

    #[test]
    fn it_should_take_the_last_line_when_one_request_carries_two() {
        // Arrange
        let text = format!("Reply file: {ROUND_1}\nCorrection.\nReply file: {ROUND_2}");
        let transcript = jsonl(&[first_request(&text)]);

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(file.as_deref(), Some(ROUND_2));
    }

    #[test]
    fn it_should_skip_a_record_that_is_not_json_when_a_write_was_cut_off() {
        // Arrange
        let transcript = format!(
            "{}\n{{\"type\": \"user\", \"message\": {{\"con",
            first_request(&task(ROUND_1))
        );

        // Act
        let file = latest_reply_file(&transcript);

        // Assert
        assert_eq!(file.as_deref(), Some(ROUND_1));
    }

    #[test]
    fn it_should_find_nothing_when_the_transcript_is_empty() {
        // Act
        let file = latest_reply_file("");

        // Assert
        assert_eq!(file, None);
    }

    #[test]
    fn it_should_find_the_agents_transcript_in_the_subagents_folder_of_the_session() {
        // Act
        let path = subagent_transcript_path(Path::new("/projects/p/s1.jsonl"), "a423cd4370c2ebae8");

        // Assert
        assert_eq!(
            path.as_deref(),
            Some(Path::new("/projects/p/s1/subagents/agent-a423cd4370c2ebae8.jsonl"))
        );
    }

    #[rstest]
    #[case::empty("")]
    #[case::with_a_separator("a/b")]
    #[case::with_a_parent_component("..")]
    #[case::with_a_dot("a.b")]
    #[case::with_a_space("a b")]
    #[case::with_a_backslash("a\\b")]
    #[case::with_a_newline("a\nb")]
    #[case::non_ascii("é")]
    fn it_should_find_no_transcript_when_the_agent_id_is_not_a_plain_name(#[case] agent_id: &str) {
        // Act
        let path = subagent_transcript_path(Path::new("/projects/p/s1.jsonl"), agent_id);

        // Assert
        assert_eq!(path, None, "the id `{agent_id}` must not name a file");
    }

    #[rstest]
    #[case::root("/")]
    #[case::empty("")]
    fn it_should_find_no_transcript_when_the_session_transcript_has_no_file_name(#[case] session_transcript: &str) {
        // Act
        let path = subagent_transcript_path(Path::new(session_transcript), "a1");

        // Assert
        assert_eq!(path, None);
    }

    #[rstest]
    #[case::relative("s1.jsonl", "s1/subagents/agent-a1.jsonl")]
    #[case::without_an_extension("/p/s1", "/p/s1/subagents/agent-a1.jsonl")]
    #[case::with_a_dotted_stem("/p/s.1.jsonl", "/p/s.1/subagents/agent-a1.jsonl")]
    fn it_should_keep_the_file_stem_of_the_session_transcript_as_the_session_folder(
        #[case] session_transcript: &str,
        #[case] expected: &str,
    ) {
        // Act
        let path = subagent_transcript_path(Path::new(session_transcript), "a1");

        // Assert
        assert_eq!(path.as_deref(), Some(Path::new(expected)));
    }
}
