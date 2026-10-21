//! Command-line adapter for `ci-gate`.
//!
//! The binary follows the global CLI output contract
//! (`docs/adrs/20260519000000_define_global_cli_output_contract.md`): it is a `stdout-result-data`
//! command, so on success it writes exactly one JSON object to stdout and nothing else; every
//! diagnostic is one JSON object per line on stderr; and it refuses to run when stdout is a
//! terminal.

use std::io::{self, IsTerminal, Read, Write};
use std::process::ExitCode;
use std::{env, fmt, fs};

use ci_gate::change_detection::{classify, parse_changed_paths};
use ci_gate::cli::{Command, GithubVerdictArguments, USAGE, parse_arguments};
use ci_gate::gh::Gh;
use ci_gate::github::decide_for_pull_request;
use ci_gate::verdict::{VerdictInput, decide};
use serde_json::json;

/// Why a run failed, which decides the diagnostic record and the exit code.
enum Failure {
    TtyRefusal,
    Usage(String),
    Runtime(String),
}

impl Failure {
    const fn kind(&self) -> &'static str {
        match self {
            Self::TtyRefusal => "tty_refusal",
            Self::Usage(_) => "usage_error",
            Self::Runtime(_) => "runtime_error",
        }
    }

    fn exit_code(&self) -> ExitCode {
        match self {
            Self::TtyRefusal | Self::Usage(_) => ExitCode::from(2),
            Self::Runtime(_) => ExitCode::FAILURE,
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TtyRefusal => formatter.write_str("stdout is a TTY; pipe the output to consume result data"),
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            let record = json!({ "kind": failure.kind(), "message": failure.to_string() });
            // A failure to write to stderr cannot be reported anywhere else.
            drop(writeln!(io::stderr().lock(), "{record}"));
            failure.exit_code()
        }
    }
}

fn run() -> Result<(), Failure> {
    if io::stdout().is_terminal() {
        return Err(Failure::TtyRefusal);
    }

    let arguments = env::args_os()
        .skip(1)
        .map(|argument| {
            argument.into_string().map_err(|argument| {
                Failure::Usage(format!(
                    "an argument is not valid UTF-8: {}. {USAGE}",
                    argument.to_string_lossy()
                ))
            })
        })
        .collect::<Result<Vec<String>, Failure>>()?;
    let command = parse_arguments(&arguments).map_err(|message| Failure::Usage(format!("{message}. {USAGE}")))?;

    let result = match command {
        Command::Classify(arguments) => {
            let classification = classify(&parse_changed_paths(&read_stdin()?), &arguments.base_branch);
            serde_json::to_string(&classification)
        }
        Command::Verdict(arguments) => {
            let verdict_input: VerdictInput = serde_json::from_slice(&read_stdin()?)
                .map_err(|error| Failure::Runtime(format!("stdin is not a valid verdict input: {error}")))?;
            let verdict = decide(
                &verdict_input.classification,
                &verdict_input.results,
                arguments.override_actor.as_deref(),
            )
            .map_err(Failure::Runtime)?;
            serde_json::to_string(&verdict)
        }
        Command::GithubVerdict(arguments) => serde_json::to_string(&github_verdict(&arguments)?),
    }
    .map_err(|error| Failure::Runtime(format!("cannot encode the result: {error}")))?;

    writeln!(io::stdout().lock(), "{result}")
        .map_err(|error| Failure::Runtime(format!("cannot write the result to stdout: {error}")))
}

fn read_stdin() -> Result<Vec<u8>, Failure> {
    let mut input = Vec::new();
    io::stdin()
        .lock()
        .read_to_end(&mut input)
        .map_err(|error| Failure::Runtime(format!("cannot read stdin: {error}")))?;
    Ok(input)
}

fn github_verdict(arguments: &GithubVerdictArguments) -> Result<ci_gate::github::GithubVerdict, Failure> {
    let classification = fs::read(&arguments.classification_path)
        .map_err(|error| Failure::Runtime(format!("cannot read `{}`: {error}", arguments.classification_path)))
        .and_then(|bytes| {
            serde_json::from_slice(&bytes).map_err(|error| {
                Failure::Runtime(format!(
                    "`{}` is not a classification: {error}",
                    arguments.classification_path
                ))
            })
        })?;

    decide_for_pull_request(&Gh::new(&arguments.gh_program), &arguments.pull_request, &classification).map_err(Failure::Runtime)
}
