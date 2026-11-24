//! The stop hook of the task reviewer: it holds the agent to a valid reply file.
//!
//! Claude Code runs it when the agent tries to finish and passes the hook input as JSON on
//! standard input. The program reads the reply file that the agent's latest request names from the
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

use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use agent_reply_validator::git::Git;
use agent_reply_validator::guard::{Outcome, decide};
use agent_reply_validator::transcript::latest_reply_file;
use agent_reply_validator::workspace::FileWorkspace;
use serde::Deserialize;
use serde_json::json;

const EXIT_FINISH: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const EXIT_REFUSE: u8 = 2;

/// The fields of the `SubagentStop` hook input that the program uses; the others are ignored.
#[derive(Debug, Deserialize)]
struct HookInput {
    /// The run of the agent; a continued run keeps it.
    agent_id: String,
    /// The agent's own transcript file.
    agent_transcript_path: PathBuf,
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
    let input: HookInput = serde_json::from_str(&text).map_err(|error| {
        format!("the hook input on standard input lacks `agent_id` or `agent_transcript_path`, or is not JSON: {error}")
    })?;
    if input.agent_id.is_empty() {
        return Err("the hook input has an empty `agent_id`".to_owned());
    }

    let git = Git::discover(input.cwd.as_deref().unwrap_or(working_directory))?;
    let transcript = fs::read(&input.agent_transcript_path).map_err(|error| {
        format!(
            "could not read the transcript `{}`: {error}",
            input.agent_transcript_path.display()
        )
    })?;
    let reply_file = latest_reply_file(&String::from_utf8_lossy(&transcript));

    Ok(decide(
        &input.agent_id,
        reply_file.as_deref(),
        &mut FileWorkspace::new(git.root().to_path_buf()),
    ))
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::Path;

    use super::{EXIT_FAILURE, run};

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
    fn it_should_fail_without_holding_the_agent_when_the_hook_input_lacks_the_transcript_path() {
        // Act
        let run = run_with(&["agent-reply-guard"], b"{\"agent_id\": \"a1\"}");

        // Assert
        assert_failure_without_holding_the_agent(&run, "`agent_transcript_path`");
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
