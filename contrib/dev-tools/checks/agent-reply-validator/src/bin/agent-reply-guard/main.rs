//! The stop hook of the task reviewer: it holds the agent to a valid reply file.
//!
//! Claude Code runs it when the agent tries to finish, as a `SubagentStop` hook, and when the agent
//! calls the tool that hands its report back, as a `PreToolUse` hook (scenario M7: Claude Code
//! discards a stop hook's refusal when the agent ends its turn with that tool call). It passes the
//! hook input as JSON on standard input. The program reads the reply file that the agent's latest
//! request names from the
//! agent's transcript, checks it with the reply validator, and answers with the exit code:
//!
//! - `0`: the agent may finish. Standard output is empty, or one JSON object whose `systemMessage`
//!   says why the reply was not enforced or why the limit of refusals was reached; Claude Code
//!   shows it to the user.
//! - `2`: the agent may not finish yet. Standard error is the text that Claude Code gives the agent.
//! - `1`: the program itself failed, for example on input it cannot read. Claude Code treats it as
//!   an error that does not hold the agent, and standard error says what failed. A usage error is
//!   `1` too, never `2`, because `2` would keep the agent working.
//!
//! The rules are decision D17 of issue #2904, in `agent_reply_validator::guard`.
//!
//! The program follows Claude Code's hook protocol instead of the CLI output contract (plain text
//! on standard error, exit `2` as a refusal), because it is an `external-protocol` program: see
//! the ADR
//! [Let Programs Follow the Protocol of the Host That Calls Them](../../../../../../../docs/adrs/20261124120307_let_programs_follow_the_protocol_of_the_host_that_calls_them.md).
//! The protocol is the Claude Code hooks reference: <https://code.claude.com/docs/en/hooks>.

use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use agent_reply_validator::git::Git;
use agent_reply_validator::guard::{Outcome, decide};
use agent_reply_validator::transcript::{latest_reply_file, subagent_transcript_path};
use agent_reply_validator::workspace::FileWorkspace;
use serde::Deserialize;
use serde_json::json;

const EXIT_FINISH: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const EXIT_REFUSE: u8 = 2;

/// The fields of the hook input that the program uses; the others are ignored.
#[derive(Debug, Deserialize)]
struct HookInput {
    /// The run of the agent; a continued run keeps it.
    agent_id: String,
    /// The agent's own transcript file: a `SubagentStop` input has it, a `PreToolUse` one does not.
    agent_transcript_path: Option<PathBuf>,
    /// The transcript file of the session that started the agent, which both inputs have.
    transcript_path: Option<PathBuf>,
    /// The directory Claude Code runs in, which is in the repository.
    cwd: Option<PathBuf>,
}

fn main() -> ExitCode {
    let working_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    ExitCode::from(run(
        env::args_os(),
        &mut io::stdin().lock(),
        &mut io::stdout().lock(),
        &mut io::stderr().lock(),
        &working_directory,
    ))
}

/// Runs the hook with the program name as the first argument and returns its exit code.
fn run(
    arguments: impl IntoIterator<Item = OsString>,
    stdin: &mut impl Read,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
    working_directory: &Path,
) -> u8 {
    match hook(arguments, stdin, working_directory) {
        Ok(Outcome::Finish { message: None }) => EXIT_FINISH,
        Ok(Outcome::Finish { message: Some(message) }) => {
            let written = serde_json::to_writer(&mut *stdout, &json!({ "systemMessage": message }))
                .map_err(io::Error::from)
                .and_then(|()| writeln!(stdout));
            match written {
                Ok(()) => EXIT_FINISH,
                Err(error) => fail(stderr, &format!("could not write the message: {error}")),
            }
        }
        Ok(Outcome::Refuse { feedback }) => {
            // If this write fails, the exit code still keeps the agent working.
            writeln!(stderr, "{feedback}").ok();
            EXIT_REFUSE
        }
        Err(message) => fail(stderr, &message),
    }
}

fn fail(stderr: &mut impl Write, message: &str) -> u8 {
    writeln!(stderr, "agent-reply-guard: {message}").ok();
    EXIT_FAILURE
}

/// Reads the hook input and the transcript, and decides.
fn hook(
    arguments: impl IntoIterator<Item = OsString>,
    stdin: &mut impl Read,
    working_directory: &Path,
) -> Result<Outcome, String> {
    if arguments.into_iter().nth(1).is_some() {
        return Err("it takes no arguments: Claude Code passes the hook input as JSON on standard input".to_owned());
    }
    let mut text = String::new();
    stdin
        .read_to_string(&mut text)
        .map_err(|error| format!("could not read the hook input on standard input: {error}"))?;
    let input: HookInput = serde_json::from_str(&text)
        .map_err(|error| format!("the hook input on standard input lacks `agent_id`, or is not JSON: {error}"))?;
    if input.agent_id.is_empty() {
        return Err("the hook input has an empty `agent_id`".to_owned());
    }

    let transcript_path = agent_transcript_path(&input)?;
    let git = Git::discover(input.cwd.as_deref().unwrap_or(working_directory))?;
    let transcript = fs::read(&transcript_path)
        .map_err(|error| format!("could not read the transcript `{}`: {error}", transcript_path.display()))?;
    let reply_file = latest_reply_file(&String::from_utf8_lossy(&transcript));

    Ok(decide(
        &input.agent_id,
        reply_file.as_deref(),
        &mut FileWorkspace::new(git.root().to_path_buf()),
    ))
}

/// The agent's transcript file: the one the input names, or else the one the session's transcript
/// leads to.
fn agent_transcript_path(input: &HookInput) -> Result<PathBuf, String> {
    if let Some(path) = &input.agent_transcript_path {
        return Ok(path.clone());
    }
    let session = input
        .transcript_path
        .as_deref()
        .ok_or("the hook input lacks both `agent_transcript_path` and `transcript_path`")?;
    subagent_transcript_path(session, &input.agent_id).ok_or_else(|| {
        format!(
            "could not find the transcript of agent `{}` from `{}`",
            input.agent_id,
            session.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};

    use super::{EXIT_FAILURE, HookInput, agent_transcript_path, run};

    /// What one run printed, and its exit code.
    struct Run {
        exit_code: u8,
        stdout: String,
        stderr: String,
    }

    fn run_with(arguments: &[&str], stdin: &[u8]) -> Run {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let exit_code = run(
            arguments.iter().map(OsString::from),
            &mut &stdin[..],
            &mut stdout,
            &mut stderr,
            Path::new("."),
        );
        Run {
            exit_code,
            stdout: String::from_utf8(stdout).unwrap(),
            stderr: String::from_utf8(stderr).unwrap(),
        }
    }

    /// Asserts the contract for a failure of the hook itself: exit code 1, which never holds the
    /// agent (exit code 2 would), nothing on standard output, and a reason on standard error.
    fn assert_failure_without_holding_the_agent(run: &Run, reason: &str) {
        assert_eq!(run.exit_code, EXIT_FAILURE);
        assert_eq!(run.stdout, "");
        assert!(run.stderr.starts_with("agent-reply-guard: "), "{}", run.stderr);
        assert!(run.stderr.contains(reason), "the reason must say `{reason}`: {}", run.stderr);
    }

    #[test]
    fn it_should_fail_without_holding_the_agent_when_an_argument_is_given() {
        // Act
        let run = run_with(&["agent-reply-guard", "--all"], b"{}");

        // Assert
        assert_failure_without_holding_the_agent(&run, "it takes no arguments");
    }

    #[test]
    fn it_should_fail_without_holding_the_agent_when_the_hook_input_names_no_transcript() {
        // Act
        let run = run_with(&["agent-reply-guard"], b"{\"agent_id\": \"a1\"}");

        // Assert
        assert_failure_without_holding_the_agent(&run, "`agent_transcript_path` and `transcript_path`");
    }

    fn input(agent_transcript_path: Option<&str>, transcript_path: Option<&str>) -> HookInput {
        HookInput {
            agent_id: "a1".to_owned(),
            agent_transcript_path: agent_transcript_path.map(PathBuf::from),
            transcript_path: transcript_path.map(PathBuf::from),
            cwd: None,
        }
    }

    #[test]
    fn it_should_use_the_transcript_the_input_names_when_it_also_names_the_session_transcript() {
        // Act
        let path = agent_transcript_path(&input(Some("/p/agent.jsonl"), Some("/p/s1.jsonl")));

        // Assert
        assert_eq!(path, Ok(PathBuf::from("/p/agent.jsonl")));
    }

    #[test]
    fn it_should_lead_from_the_session_transcript_to_the_agents_when_the_input_names_only_that() {
        // Act
        let path = agent_transcript_path(&input(None, Some("/p/s1.jsonl")));

        // Assert
        assert_eq!(path, Ok(PathBuf::from("/p/s1/subagents/agent-a1.jsonl")));
    }

    #[test]
    fn it_should_find_no_transcript_when_the_input_names_neither() {
        // Act
        let path = agent_transcript_path(&input(None, None));

        // Assert
        assert!(path.is_err(), "{path:?}");
    }

    #[test]
    fn it_should_lead_from_the_session_transcript_when_the_agents_is_null_in_the_input() {
        // Arrange
        let text = r#"{"agent_id": "a1", "agent_transcript_path": null, "transcript_path": "/p/s1.jsonl"}"#;
        let parsed: HookInput = serde_json::from_str(text).unwrap();

        // Act
        let path = agent_transcript_path(&parsed);

        // Assert
        assert_eq!(path, Ok(PathBuf::from("/p/s1/subagents/agent-a1.jsonl")));
    }

    #[test]
    fn it_should_fail_without_holding_the_agent_when_the_agent_id_cannot_lead_to_a_transcript() {
        // Act
        let run = run_with(
            &["agent-reply-guard"],
            b"{\"agent_id\": \"../x\", \"transcript_path\": \"/p/s1.jsonl\"}",
        );

        // Assert
        assert_failure_without_holding_the_agent(&run, "could not find the transcript of agent `../x`");
    }

    #[test]
    fn it_should_fail_without_holding_the_agent_when_the_transcript_the_input_names_is_an_empty_path() {
        // Act
        let run = run_with(
            &["agent-reply-guard"],
            b"{\"agent_id\": \"a1\", \"agent_transcript_path\": \"\"}",
        );

        // Assert
        assert_failure_without_holding_the_agent(&run, "could not read the transcript ``");
    }

    #[test]
    fn it_should_fail_without_holding_the_agent_when_the_hook_input_is_empty() {
        // Act
        let run = run_with(&["agent-reply-guard"], b"");

        // Assert
        assert_failure_without_holding_the_agent(&run, "hook input");
    }

    #[test]
    fn it_should_fail_without_holding_the_agent_when_the_hook_input_is_not_utf8() {
        // Act
        let run = run_with(&["agent-reply-guard"], &[0xff, 0xfe]);

        // Assert
        assert_failure_without_holding_the_agent(&run, "could not read the hook input");
    }

    #[test]
    fn it_should_fail_without_holding_the_agent_when_the_agent_id_is_empty() {
        // Act
        let run = run_with(
            &["agent-reply-guard"],
            b"{\"agent_id\": \"\", \"agent_transcript_path\": \"t.jsonl\"}",
        );

        // Assert
        assert_failure_without_holding_the_agent(&run, "empty `agent_id`");
    }
}
