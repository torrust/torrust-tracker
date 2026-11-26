//! Command-boundary tests of the stop hook: the built `agent-reply-guard` runs in a disposable
//! git repository, receives the hook input on standard input and reads a real transcript file.

mod support;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use rstest::rstest;
use serde_json::{Value, json};
use support::isolated;
use tempfile::TempDir;

const AGENT: &str = "a423cd4370c2ebae8";
/// Where the session that started the agent keeps its files, relative to the repository's root:
/// `<SESSION>.jsonl` is the session's transcript and `<SESSION>/subagents/` holds the agents'.
const SESSION: &str = "session-folder/s1";
const SPECIFICATION: &str = "docs/issues/open/2904-slug";
const REPLY_1: &str = "docs/issues/open/2904-slug/agent-replies/task-reviewer-t5-r1.json";
const REPLY_2: &str = "docs/issues/open/2904-slug/agent-replies/task-reviewer-t5-r2.json";
/// The count of refusals for `AGENT` and `REPLY_1`.
const COUNT_1: &str = ".agents/issues/2904/reply-guard/a423cd4370c2ebae8--task-reviewer-t5-r1.json.count";
/// The count of refusals for `AGENT` and `REPLY_2`.
const COUNT_2: &str = ".agents/issues/2904/reply-guard/a423cd4370c2ebae8--task-reviewer-t5-r2.json.count";

/// A valid reply for the file `task-reviewer-t5-r<round>.json`.
fn reply(round: u8) -> String {
    json!({
        "schema-version": 1,
        "agent": "task-reviewer",
        "model": "claude-opus-5-5",
        "subject": "t5",
        "round": round,
        "created-utc": "2026-11-24 09:00",
        "verdict": "REVIEW_PASSED",
        "summary": "Nothing blocks the task.",
        "findings": [],
        "questions": [],
        "evidence": ["cargo test -q -p agent-reply-validator passed"]
    })
    .to_string()
}

/// The first request of a transcript, as Claude Code records it, naming the file.
fn first_request(file: &str) -> String {
    json!({"type": "user", "message": {"role": "user", "content": format!("Review task T5.\nReply file: {file}\n")}}).to_string()
}

/// A request that continues the finished agent, naming the file.
fn continued_request(file: &str) -> String {
    json!({
        "type": "user",
        "isMeta": true,
        "origin": {"kind": "coordinator"},
        "message": {"role": "user", "content": format!("The coordinator sent a message while you were working:\nReview the fixes.\nReply file: {file}\n")}
    })
    .to_string()
}

/// A transcript whose only request names no file.
fn request_without_a_file() -> String {
    json!({"type": "user", "message": {"role": "user", "content": "Review task T5 on a draft."}}).to_string()
}

/// What one run of the hook did.
struct Outcome {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

impl Outcome {
    /// The `systemMessage` of standard output, which must then be one JSON object.
    fn system_message(&self) -> String {
        let output: Value =
            serde_json::from_str(&self.stdout).unwrap_or_else(|error| panic!("stdout is not JSON: {error}: {}", self.stdout));
        output["systemMessage"].as_str().expect("a systemMessage").to_owned()
    }
}

/// A disposable git repository; `TempDir` removes it on drop.
struct Repository {
    directory: TempDir,
}

impl Repository {
    fn new() -> Self {
        let repository = Self {
            directory: TempDir::new().unwrap(),
        };
        let status = isolated(Command::new("git"), repository.root())
            .args(["init", "--quiet"])
            .status()
            .unwrap();
        assert!(status.success(), "git init failed");
        repository
    }

    /// A repository whose specification of issue 2904 exists.
    fn with_the_specification() -> Self {
        let repository = Self::new();
        repository.write(&format!("{SPECIFICATION}/ISSUE.md"), "# Specification\n");
        repository
    }

    fn root(&self) -> &Path {
        self.directory.path()
    }

    fn write(&self, path: &str, content: &str) {
        let file = self.root().join(path);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, content).unwrap();
    }

    fn read(&self, path: &str) -> String {
        fs::read_to_string(self.root().join(path)).unwrap()
    }

    fn exists(&self, path: &str) -> bool {
        self.root().join(path).exists()
    }

    /// Writes the transcript of the agent and runs the hook for it, in the repository's root.
    fn run(&self, transcript: &[String]) -> Outcome {
        self.run_in(self.root(), transcript)
    }

    fn run_in(&self, directory: &Path, transcript: &[String]) -> Outcome {
        self.run_from(directory, directory, transcript)
    }

    /// Runs the hook as a process started in `process_directory`, with `directory` in its input.
    fn run_from(&self, process_directory: &Path, directory: &Path, transcript: &[String]) -> Outcome {
        self.run_with_bytes(process_directory, directory, transcript.join("\n").as_bytes())
    }

    /// The same, with the transcript file given as bytes.
    fn run_with_bytes(&self, process_directory: &Path, directory: &Path, transcript: &[u8]) -> Outcome {
        fs::write(self.root().join("transcript.jsonl"), transcript).unwrap();
        let input = json!({
            "session_id": "s1",
            "hook_event_name": "SubagentStop",
            "stop_hook_active": false,
            "agent_id": AGENT,
            "agent_type": "task-reviewer",
            "agent_transcript_path": self.root().join("transcript.jsonl"),
            "cwd": directory,
            "last_assistant_message": "Done."
        });
        run_hook(process_directory, &input.to_string(), &[])
    }
}

impl Repository {
    /// Runs the hook as Claude Code runs it before the agent's hand-back tool call: the input names
    /// the transcript of the session and not the agent's, which is where Claude Code keeps it.
    /// With `agent_transcript` as `false`, the agent's transcript file is not written.
    fn run_before_the_hand_back(&self, transcript: &[String], agent_transcript: bool) -> Outcome {
        if agent_transcript {
            self.write(&agent_transcript_in_the_session(), &transcript.join("\n"));
        }
        let input = json!({
            "session_id": "s1",
            "hook_event_name": "PreToolUse",
            "tool_name": "SubagentHandback",
            "tool_input": {"message": "Done."},
            "agent_id": AGENT,
            "agent_type": "task-reviewer",
            "transcript_path": self.root().join(format!("{SESSION}.jsonl")),
            "cwd": self.root()
        });
        run_hook(self.root(), &input.to_string(), &[])
    }
}

/// Where the session keeps the transcript of `AGENT`, relative to the repository's root.
fn agent_transcript_in_the_session() -> String {
    format!("{SESSION}/subagents/agent-{AGENT}.jsonl")
}

fn run_hook(directory: &Path, input: &str, arguments: &[&str]) -> Outcome {
    let mut child = isolated(Command::new(env!("CARGO_BIN_EXE_agent-reply-guard")), directory)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    Outcome {
        exit_code: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

#[test]
fn it_should_let_the_agent_finish_in_silence_when_no_request_names_a_reply_file() {
    // Arrange
    let repository = Repository::with_the_specification();

    // Act
    let outcome = repository.run(&[request_without_a_file()]);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.stdout, "");
    assert_eq!(outcome.stderr, "");
    assert!(
        !repository.exists(".agents"),
        "no state is kept when there is no reply file to hold the agent to"
    );
}

#[test]
fn it_should_refuse_on_standard_error_and_keep_the_count_when_the_reply_file_does_not_exist() {
    // Arrange
    let repository = Repository::with_the_specification();

    // Act
    let outcome = repository.run(&[first_request(REPLY_1)]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(outcome.stdout, "");
    assert!(
        outcome.stderr.contains(REPLY_1),
        "the refusal must name the file: {}",
        outcome.stderr
    );
    assert!(
        !outcome.stderr.contains("Reply file:"),
        "the refusal must not carry the literal line, which the transcript would then hold twice: {}",
        outcome.stderr
    );
    assert_eq!(repository.read(COUNT_1), "1\n");
}

#[test]
fn it_should_refuse_three_times_and_then_let_the_agent_finish_with_a_message() {
    // Arrange
    let repository = Repository::with_the_specification();
    let transcript = [first_request(REPLY_1)];

    // Act
    let attempts: Vec<Outcome> = (0..4).map(|_| repository.run(&transcript)).collect();

    // Assert
    let exit_codes: Vec<i32> = attempts.iter().map(|attempt| attempt.exit_code).collect();
    assert_eq!(exit_codes, [2, 2, 2, 0], "three refusals, then the fourth attempt is allowed");
    assert!(
        attempts[..3]
            .iter()
            .all(|attempt| attempt.stdout.is_empty() && !attempt.stderr.is_empty())
    );
    assert!(
        attempts[3].system_message().contains(REPLY_1),
        "the message must name the file"
    );
    assert_eq!(attempts[3].stderr, "");
    assert_eq!(repository.read(COUNT_1), "3\n", "an allowed attempt is not a refusal");
}

#[test]
fn it_should_let_the_agent_finish_in_silence_and_forget_the_count_when_the_reply_file_became_valid() {
    // Arrange
    let repository = Repository::with_the_specification();
    let transcript = [first_request(REPLY_1)];
    assert_eq!(repository.run(&transcript).exit_code, 2);
    repository.write(REPLY_1, &reply(1));

    // Act
    let outcome = repository.run(&transcript);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.stdout, "");
    assert_eq!(outcome.stderr, "");
    assert!(
        !repository.exists(COUNT_1),
        "the count of a request whose reply is valid is forgotten"
    );
}

#[test]
fn it_should_refuse_when_the_reply_file_exists_but_is_not_valid() {
    // Arrange: a valid reply, but for round 2, in the file of round 1.
    let repository = Repository::with_the_specification();
    repository.write(REPLY_1, &reply(2));

    // Act
    let outcome = repository.run(&[first_request(REPLY_1)]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(outcome.stdout, "");
    assert!(
        outcome.stderr.contains(REPLY_1),
        "the refusal must name the file: {}",
        outcome.stderr
    );
    assert_eq!(repository.read(COUNT_1), "1\n");
}

#[test]
fn it_should_hold_a_continued_agent_to_the_file_of_its_latest_request_when_the_earlier_file_is_valid() {
    // Arrange: round 1 is done and valid; the agent is continued for round 2, which it has not written.
    let repository = Repository::with_the_specification();
    repository.write(REPLY_1, &reply(1));

    // Act
    let outcome = repository.run(&[first_request(REPLY_1), continued_request(REPLY_2)]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert!(outcome.stderr.contains(REPLY_2), "{}", outcome.stderr);
    assert!(!outcome.stderr.contains(REPLY_1), "{}", outcome.stderr);
    assert_eq!(repository.read(COUNT_2), "1\n");
}

#[test]
fn it_should_give_a_continued_request_its_own_three_refusals_when_the_earlier_request_used_up_its_own() {
    // Arrange
    let repository = Repository::with_the_specification();
    for _ in 0..3 {
        assert_eq!(repository.run(&[first_request(REPLY_1)]).exit_code, 2);
    }

    // Act
    let outcome = repository.run(&[first_request(REPLY_1), continued_request(REPLY_2)]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert!(outcome.stderr.contains("refusal 1 of 3"), "{}", outcome.stderr);
    assert_eq!(
        repository.read(COUNT_1),
        "3\n",
        "the count of the earlier request is untouched"
    );
}

#[rstest]
#[case::draft("docs/issues/drafts/2003-some-draft/agent-replies/task-reviewer-t5-r1.json")]
#[case::path_that_leaves_the_repository("docs/issues/open/../../../../etc/passwd")]
fn it_should_let_the_agent_finish_with_one_message_and_keep_no_state_when_the_reply_cannot_be_enforced(#[case] path: &str) {
    // Arrange: only the open specification exists.
    let repository = Repository::with_the_specification();

    // Act
    let outcome = repository.run(&[first_request(path)]);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.stderr, "");
    assert!(outcome.system_message().contains(path), "the message must name the path");
    assert!(
        !repository.exists(".agents"),
        "no count is kept when the reply is not enforced"
    );
}

#[test]
fn it_should_find_the_repository_from_a_subfolder_when_the_hook_input_names_one() {
    // Arrange
    let repository = Repository::with_the_specification();
    let subfolder = repository.root().join("docs");

    // Act
    let outcome = repository.run_from(repository.root(), &subfolder, &[first_request(REPLY_1)]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(
        repository.read(COUNT_1),
        "1\n",
        "the count is kept at the root of the repository"
    );
}

#[test]
fn it_should_find_the_repository_from_the_process_directory_when_the_hook_input_has_no_directory() {
    // Arrange
    let repository = Repository::with_the_specification();
    repository.write("transcript.jsonl", &first_request(REPLY_1));
    let input = json!({"agent_id": AGENT, "agent_transcript_path": repository.root().join("transcript.jsonl")});

    // Act
    let outcome = run_hook(repository.root(), &input.to_string(), &[]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(repository.read(COUNT_1), "1\n");
}

#[test]
fn it_should_read_the_request_when_another_record_of_the_transcript_is_not_utf8() {
    // Arrange
    let repository = Repository::with_the_specification();
    let mut transcript = b"\xff\xfe not a record\n".to_vec();
    transcript.extend_from_slice(first_request(REPLY_1).as_bytes());

    // Act
    let outcome = repository.run_with_bytes(repository.root(), repository.root(), &transcript);

    // Assert
    assert_eq!(outcome.exit_code, 2, "{}", outcome.stderr);
    assert_eq!(repository.read(COUNT_1), "1\n");
}

#[test]
fn it_should_find_the_repository_from_the_directory_in_the_hook_input_when_the_process_runs_elsewhere() {
    // Arrange
    let repository = Repository::with_the_specification();
    let elsewhere = TempDir::new().unwrap();

    // Act
    let outcome = repository.run_from(elsewhere.path(), repository.root(), &[first_request(REPLY_1)]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(repository.read(COUNT_1), "1\n");
}

#[test]
fn it_should_fail_without_holding_the_agent_when_the_transcript_cannot_be_read() {
    // Arrange
    let repository = Repository::with_the_specification();
    let input = json!({
        "agent_id": AGENT,
        "agent_transcript_path": repository.root().join("no-such-transcript.jsonl"),
        "cwd": repository.root()
    });

    // Act
    let outcome = run_hook(repository.root(), &input.to_string(), &[]);

    // Assert: exit code 2 would keep the agent working, so a failure of the hook itself is 1.
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.stdout, "");
    assert!(
        outcome.stderr.starts_with("agent-reply-guard: could not read the transcript"),
        "{}",
        outcome.stderr
    );
}

#[test]
fn it_should_fail_without_holding_the_agent_when_the_directory_is_not_in_a_repository() {
    // Arrange
    let outside = TempDir::new().unwrap();
    let transcript = outside.path().join("transcript.jsonl");
    fs::write(&transcript, first_request(REPLY_1)).unwrap();
    let input = json!({"agent_id": AGENT, "agent_transcript_path": transcript, "cwd": outside.path()});

    // Act
    let outcome = run_hook(outside.path(), &input.to_string(), &[]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.stdout, "");
    assert!(outcome.stderr.starts_with("agent-reply-guard: "), "{}", outcome.stderr);
}

#[test]
fn it_should_refuse_before_the_hand_back_and_keep_the_count_when_the_reply_file_does_not_exist() {
    // Arrange
    let repository = Repository::with_the_specification();

    // Act
    let outcome = repository.run_before_the_hand_back(&[first_request(REPLY_1)], true);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(outcome.stdout, "");
    assert!(
        outcome.stderr.contains("refusal 1 of 3"),
        "the agent must learn the count: {}",
        outcome.stderr
    );
    assert_eq!(repository.read(COUNT_1), "1\n");
}

#[test]
fn it_should_let_the_agent_hand_back_in_silence_when_the_reply_file_is_valid() {
    // Arrange
    let repository = Repository::with_the_specification();
    repository.write(REPLY_1, &reply(1));

    // Act
    let outcome = repository.run_before_the_hand_back(&[first_request(REPLY_1)], true);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.stdout, "");
    assert_eq!(outcome.stderr, "");
    assert!(!repository.exists(".agents"), "a valid reply leaves no count behind");
}

#[test]
fn it_should_count_a_refusal_before_the_hand_back_together_with_a_refusal_at_the_stop() {
    // Arrange
    let repository = Repository::with_the_specification();
    let at_the_stop = repository.run(&[first_request(REPLY_1)]);

    // Act
    let before_the_hand_back = repository.run_before_the_hand_back(&[first_request(REPLY_1)], true);

    // Assert
    assert_eq!(at_the_stop.exit_code, 2);
    assert_eq!(before_the_hand_back.exit_code, 2);
    assert!(
        before_the_hand_back.stderr.contains("refusal 2 of 3"),
        "the two attempts of one agent share a count: {}",
        before_the_hand_back.stderr
    );
    assert_eq!(repository.read(COUNT_1), "2\n");
}

#[test]
fn it_should_fail_without_holding_the_agent_when_the_transcript_of_the_agent_is_not_where_the_session_keeps_it() {
    // Arrange
    let repository = Repository::with_the_specification();

    // Act
    let outcome = repository.run_before_the_hand_back(&[first_request(REPLY_1)], false);

    // Assert
    assert_eq!(outcome.exit_code, 1, "exit code 2 would hold the agent: {}", outcome.stderr);
    assert_eq!(outcome.stdout, "");
    assert!(
        outcome.stderr.contains(&agent_transcript_in_the_session()),
        "the message must name the path the session leads to: {}",
        outcome.stderr
    );
    assert!(!repository.exists(".agents"), "a failure keeps no state");
}

#[rstest]
#[case::the_named_one_is_valid(REPLY_1, REPLY_2, 0)]
#[case::the_named_one_is_missing(REPLY_2, REPLY_1, 2)]
fn it_should_read_the_transcript_the_input_names_and_not_the_one_the_session_leads_to(
    #[case] named_reply: &str,
    #[case] session_reply: &str,
    #[case] expected_exit_code: i32,
) {
    // Arrange: round 1 is written and valid, round 2 is not written.
    let repository = Repository::with_the_specification();
    repository.write(REPLY_1, &reply(1));
    repository.write("transcript.jsonl", &first_request(named_reply));
    repository.write(&agent_transcript_in_the_session(), &first_request(session_reply));
    let input = json!({
        "hook_event_name": "SubagentStop",
        "agent_id": AGENT,
        "agent_transcript_path": repository.root().join("transcript.jsonl"),
        "transcript_path": repository.root().join(format!("{SESSION}.jsonl")),
        "cwd": repository.root()
    });

    // Act
    let outcome = run_hook(repository.root(), &input.to_string(), &[]);

    // Assert
    assert_eq!(outcome.exit_code, expected_exit_code, "{}", outcome.stderr);
    assert_eq!(outcome.stdout, "");
}

#[test]
fn it_should_release_the_agent_at_the_fourth_attempt_when_the_two_hooks_take_turns() {
    // Arrange
    let repository = Repository::with_the_specification();
    let request = [first_request(REPLY_1)];

    // Act
    let attempts = [
        repository.run_before_the_hand_back(&request, true),
        repository.run(&request),
        repository.run_before_the_hand_back(&request, true),
        repository.run(&request),
    ];

    // Assert
    let exit_codes: Vec<i32> = attempts.iter().map(|attempt| attempt.exit_code).collect();
    assert_eq!(
        exit_codes,
        [2, 2, 2, 0],
        "one budget of three refusals is shared by both hooks"
    );
    assert!(
        attempts[3].system_message().contains(REPLY_1),
        "the limit message names the file: {}",
        attempts[3].stdout
    );
}
