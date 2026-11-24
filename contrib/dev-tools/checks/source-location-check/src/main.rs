//! Command-line adapter: fails when non-test Rust code records a source location.
//!
//! Output follows the global CLI output contract (`no-stdout-result`): nothing on stdout, one JSON object
//! per line on stderr, exit code 0 when the tree is clean, 1 on a violation or a runtime failure, and 2 on
//! a usage error. It runs from the repository root and checks the `.rs` files that Git tracks or would
//! track (`git ls-files`), so a staged new file is covered.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::{env, fs};

use serde::Serialize;
use source_location_check::{is_test_file, parse_allow_list, test_only_paths, violations_in};

const EXIT_FAILURE: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;
const DEFAULT_ALLOW_LIST: &str = "contrib/dev-tools/checks/source-location-check/allow-list.txt";

#[derive(Serialize)]
struct Record {
    kind: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    exit_code: u8,
}

fn main() -> ExitCode {
    let records = match check() {
        Ok(records) => records,
        Err((kind, message, exit_code)) => vec![Record {
            kind,
            message,
            file: None,
            line: None,
            exit_code,
        }],
    };
    let exit_code = records.iter().map(|record| record.exit_code).max().unwrap_or(0);

    let mut stderr = io::stderr().lock();
    for record in &records {
        if write_record(&mut stderr, record).is_err() {
            std::process::exit(i32::from(exit_code.max(EXIT_FAILURE)));
        }
    }

    ExitCode::from(exit_code)
}

type Failure = (&'static str, String, u8);

const fn runtime(message: String) -> Failure {
    ("runtime_error", message, EXIT_FAILURE)
}

fn check() -> Result<Vec<Record>, Failure> {
    let allow_list_path = parse_arguments(env::args().skip(1)).map_err(|message| ("usage_error", message, EXIT_USAGE_ERROR))?;
    let allow_list_text = fs::read_to_string(&allow_list_path)
        .map_err(|error| runtime(format!("cannot read the allow-list `{allow_list_path}`: {error}")))?;
    let allowed = parse_allow_list(&allow_list_text).map_err(runtime)?;

    let mut sources = Vec::new();
    for path in rust_files()? {
        let source = fs::read_to_string(&path).map_err(|error| runtime(format!("cannot read `{}`: {error}", path.display())))?;
        sources.push((path, source));
    }
    let test_only = test_only_paths(sources.iter().map(|(path, source)| (path.as_path(), source.as_str())));

    let mut records = Vec::new();
    for (path, source) in &sources {
        let name = path.to_string_lossy().replace('\\', "/");
        if is_test_file(path, &test_only) || allowed.contains(&name) {
            continue;
        }
        for violation in violations_in(source) {
            records.push(Record {
                kind: "validation_error",
                message: format!(
                    "`{}` {}; errors do not record source locations (ADR 20261118045443_errors_do_not_record_source_locations)",
                    violation.pattern, violation.reason
                ),
                file: Some(name.clone()),
                line: Some(violation.line),
                exit_code: EXIT_FAILURE,
            });
        }
    }

    if records.is_empty() {
        records.push(Record {
            kind: "checked",
            message: format!(
                "no non-test Rust file records a source location ({} files read)",
                sources.len()
            ),
            file: None,
            line: None,
            exit_code: 0,
        });
    }
    Ok(records)
}

fn parse_arguments(mut arguments: impl Iterator<Item = String>) -> Result<String, String> {
    let mut allow_list = None;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--allow-list" if allow_list.is_none() => {
                allow_list = Some(arguments.next().ok_or("usage error: `--allow-list` requires a value")?);
            }
            _ => {
                return Err(format!(
                    "usage error: unexpected argument `{argument}`; use `--allow-list <FILE>`"
                ));
            }
        }
    }
    Ok(allow_list.unwrap_or_else(|| DEFAULT_ALLOW_LIST.to_owned()))
}

fn rust_files() -> Result<Vec<PathBuf>, Failure> {
    let output = Command::new("git")
        .args(["ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*.rs"])
        .output()
        .map_err(|error| runtime(format!("failed to run `git ls-files`: {error}")))?;
    if !output.status.success() {
        return Err(runtime(format!(
            "`git ls-files` failed (run the check from the repository root): {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
        .map(|name| PathBuf::from(String::from_utf8_lossy(name).into_owned()))
        .filter(|path| Path::new(path).is_file())
        .collect())
}

fn write_record(writer: &mut impl Write, record: &Record) -> io::Result<()> {
    serde_json::to_writer(&mut *writer, record)?;
    writer.write_all(b"\n")
}
