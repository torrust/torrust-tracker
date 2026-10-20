//! Command-line adapter for `ci-gate`.
//!
//! The binary follows the global CLI output contract
//! (`docs/adrs/20260519000000_define_global_cli_output_contract.md`): it is a `stdout-result-data`
//! command, so on success it writes exactly one JSON object to stdout and nothing else; every
//! diagnostic is one JSON object per line on stderr; and it refuses to run when stdout is a
//! terminal.

use std::io::{self, IsTerminal, Read, Write};
use std::process::ExitCode;
use std::{env, fmt};

use ci_gate::change_detection::{classify, parse_changed_paths};
use ci_gate::cli::{USAGE, parse_arguments};
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

    let arguments: Vec<String> = env::args().skip(1).collect();
    let parsed = parse_arguments(&arguments).map_err(|message| Failure::Usage(format!("{message}. {USAGE}")))?;

    let mut input = Vec::new();
    io::stdin()
        .lock()
        .read_to_end(&mut input)
        .map_err(|error| Failure::Runtime(format!("cannot read the changed paths from stdin: {error}")))?;

    let classification = classify(&parse_changed_paths(&input), &parsed.base_branch);
    let result =
        serde_json::to_string(&classification).map_err(|error| Failure::Runtime(format!("cannot encode the result: {error}")))?;

    writeln!(io::stdout().lock(), "{result}")
        .map_err(|error| Failure::Runtime(format!("cannot write the result to stdout: {error}")))
}
