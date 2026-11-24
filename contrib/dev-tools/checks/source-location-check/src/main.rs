//! Command-line adapter: fails when non-test Rust code records a source location.
//!
//! Output follows the global CLI output contract (`no-stdout-result`): nothing on stdout, one JSON object
//! per line on stderr, exit code 0 when the tree is clean, 1 on a violation or a runtime failure, and 2 on
//! a usage error. It runs from the repository root and checks the `.rs` files that Git tracks or would
//! track (`git ls-files`), so a staged new file is covered. With `--staged` (the pre-commit hook) it reads
//! the Git index instead of the working tree, so what is checked is what the commit contains.

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
    let arguments = parse_arguments(env::args().skip(1)).map_err(|message| ("usage_error", message, EXIT_USAGE_ERROR))?;
    let staged = arguments.staged;
    let allow_list_text = read_file(&arguments.allow_list, staged)
        .map_err(|error| runtime(format!("cannot read the allow-list `{}`: {error}", arguments.allow_list)))?;
    let allowed = parse_allow_list(&allow_list_text).map_err(runtime)?;

    let mut sources = Vec::new();
    for path in rust_files(staged)? {
        let source = read_file(&path.to_string_lossy(), staged)
            .map_err(|error| runtime(format!("cannot read `{}`: {error}", path.display())))?;
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

struct Arguments {
    allow_list: String,
    /// Read the Git index (the snapshot of the commit being made) instead of the working tree.
    staged: bool,
}

fn parse_arguments(mut arguments: impl Iterator<Item = String>) -> Result<Arguments, String> {
    let mut allow_list = None;
    let mut staged = false;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--allow-list" if allow_list.is_none() => {
                allow_list = Some(arguments.next().ok_or("usage error: `--allow-list` requires a value")?);
            }
            "--staged" if !staged => staged = true,
            _ => {
                return Err(format!(
                    "usage error: unexpected argument `{argument}`; use `--allow-list <FILE>` and `--staged`"
                ));
            }
        }
    }
    Ok(Arguments {
        allow_list: allow_list.unwrap_or_else(|| DEFAULT_ALLOW_LIST.to_owned()),
        staged,
    })
}

/// Reads a file from the working tree, or its staged content from the Git index (`git show :<path>`, which
/// honors `GIT_INDEX_FILE`, so the same snapshot serves the sources, the module classification and the
/// allow-list).
fn read_file(path: &str, staged: bool) -> Result<String, String> {
    if !staged {
        return fs::read_to_string(path).map_err(|error| error.to_string());
    }
    let output = Command::new("git")
        .arg("show")
        .arg(format!(":{path}"))
        .output()
        .map_err(|error| format!("failed to run `git show`: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

/// The `.rs` files to check: those that Git tracks or would track, or, when `staged`, those in the index.
fn rust_files(staged: bool) -> Result<Vec<PathBuf>, Failure> {
    let mut arguments = vec!["ls-files", "-z", "--cached"];
    if !staged {
        arguments.extend(["--others", "--exclude-standard"]);
    }
    arguments.extend(["--", "*.rs"]);
    let output = Command::new("git")
        .args(arguments)
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
        .filter(|path| staged || Path::new(path).is_file())
        .collect())
}

fn write_record(writer: &mut impl Write, record: &Record) -> io::Result<()> {
    serde_json::to_writer(&mut *writer, record)?;
    writer.write_all(b"\n")
}
