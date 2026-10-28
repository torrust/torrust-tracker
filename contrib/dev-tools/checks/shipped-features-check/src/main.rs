//! Command-line adapter: fails when a workspace `testing` feature reaches a shipped package.
//!
//! Output follows the global CLI output contract (`no-stdout-result`): nothing on stdout, one JSON
//! object per line on stderr, exit code 0 when every shipped package is clean, 1 on a violation or a
//! runtime failure, and 2 on a usage error.

use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::process::{Command, ExitCode};

use serde::Serialize;
use shipped_features_check::{has_workspace_package, workspace_packages_with_feature};

const EXIT_FAILURE: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;
const DEFAULT_FORBIDDEN_FEATURE: &str = "testing";

fn main() -> ExitCode {
    let records = match check() {
        Ok(records) => records,
        Err(failure) => vec![failure.into_record()],
    };
    let exit_code = records.iter().map(|record| record.exit_code).max().unwrap_or(0);

    for record in &records {
        if write_record(record).is_err() {
            emit_output_failure(exit_code);
            break;
        }
    }

    ExitCode::from(exit_code)
}

/// One JSON line on stderr.
#[derive(Serialize)]
struct Record {
    kind: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipped_package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature: Option<String>,
    exit_code: u8,
}

/// A failure that stops the check before it can decide.
enum Failure {
    Usage(String),
    Runtime(String),
}

impl Failure {
    fn into_record(self) -> Record {
        let (kind, message, exit_code) = match self {
            Self::Usage(message) => ("usage_error", message, EXIT_USAGE_ERROR),
            Self::Runtime(message) => ("runtime_error", message, EXIT_FAILURE),
        };

        Record {
            kind,
            message,
            shipped_package: None,
            package: None,
            feature: None,
            exit_code,
        }
    }
}

struct Arguments {
    packages: Vec<String>,
    features: Option<String>,
    forbidden_feature: String,
}

/// Checks every shipped package and returns one record per package and per violation.
fn check() -> Result<Vec<Record>, Failure> {
    let arguments = parse_arguments(env::args().skip(1)).map_err(Failure::Usage)?;
    let mut records = Vec::new();

    for package in &arguments.packages {
        let tree = cargo_tree(package, arguments.features.as_deref()).map_err(Failure::Runtime)?;
        if !has_workspace_package(&tree, package) {
            return Err(Failure::Runtime(format!(
                "`cargo tree --package {package}` did not list `{package}` as a workspace package, so the check cannot \
                 tell whether a feature reaches it; the format of the `cargo tree` output may have changed"
            )));
        }
        let violations = workspace_packages_with_feature(&tree, &arguments.forbidden_feature);

        if violations.is_empty() {
            records.push(Record {
                kind: "checked",
                message: format!(
                    "no workspace package has the `{}` feature enabled in the dependency tree of shipped package `{package}`",
                    arguments.forbidden_feature
                ),
                shipped_package: Some(package.clone()),
                package: None,
                feature: Some(arguments.forbidden_feature.clone()),
                exit_code: 0,
            });
        }
        records.extend(violations.into_iter().map(|violation| Record {
            kind: "validation_error",
            message: format!(
                "workspace package `{}` {} has the `{}` feature enabled in the dependency tree of shipped package `{package}`",
                violation.package, violation.version, violation.feature
            ),
            shipped_package: Some(package.clone()),
            package: Some(violation.package),
            feature: Some(violation.feature),
            exit_code: EXIT_FAILURE,
        }));
    }

    Ok(records)
}

fn parse_arguments(mut arguments: impl Iterator<Item = String>) -> Result<Arguments, String> {
    let mut packages = Vec::new();
    let mut features = None;
    let mut forbidden_feature = None;

    while let Some(argument) = arguments.next() {
        let slot = match argument.as_str() {
            "--package" => None,
            "--features" if features.is_none() => Some(&mut features),
            "--forbidden-feature" if forbidden_feature.is_none() => Some(&mut forbidden_feature),
            "--features" | "--forbidden-feature" => return Err(format!("usage error: duplicate argument `{argument}`")),
            _ => {
                return Err(format!(
                    "usage error: unexpected argument `{argument}`; use `--package <NAME>` (repeatable), `--features <LIST>` and `--forbidden-feature <NAME>`"
                ));
            }
        };
        let value = arguments
            .next()
            .ok_or_else(|| format!("usage error: `{argument}` requires a value"))?;

        match slot {
            Some(slot) => *slot = Some(value),
            None => packages.push(value),
        }
    }

    if packages.is_empty() {
        return Err(String::from(
            "usage error: name at least one shipped package with `--package <NAME>`",
        ));
    }

    Ok(Arguments {
        packages,
        features,
        forbidden_feature: forbidden_feature.unwrap_or_else(|| DEFAULT_FORBIDDEN_FEATURE.to_owned()),
    })
}

/// Runs `cargo tree` with the edges and features of a `cargo build -p <package>` of the shipped package.
fn cargo_tree(package: &str, features: Option<&str>) -> Result<String, String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let mut command = Command::new(cargo);
    command.args([
        "tree",
        "--package",
        package,
        "--edges",
        "normal,build",
        "--prefix",
        "none",
        "--format",
        "{p}|{f}",
    ]);
    if let Some(features) = features {
        command.args(["--features", features]);
    }

    let output = command
        .output()
        .map_err(|error| format!("failed to run `cargo tree`: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "`cargo tree --package {package}` failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    String::from_utf8(output.stdout).map_err(|error| format!("`cargo tree` output was not valid UTF-8: {error}"))
}

fn write_record(record: &Record) -> io::Result<()> {
    let mut stderr = io::stderr().lock();
    serde_json::to_writer(&mut stderr, record)?;
    stderr.write_all(b"\n")
}

fn emit_output_failure(exit_code: u8) {
    let mut stderr = io::stderr().lock();
    drop(writeln!(
        stderr,
        "{{\"kind\":\"output_error\",\"message\":\"failed to emit diagnostic\",\"exit_code\":{exit_code}}}"
    ));
}
