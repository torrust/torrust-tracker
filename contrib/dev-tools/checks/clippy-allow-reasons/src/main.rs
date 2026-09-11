//! Command-line adapter for prospective Clippy `allow` rationale validation.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::{env, fs};

use clippy_allow_reasons::validate_changed_allows;
use serde::Serialize;

const EXIT_VIOLATIONS: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let exit_code = error.exit_code();
            for diagnostic in error.diagnostics() {
                if emit_diagnostic(&diagnostic).is_err() {
                    emit_output_failure(exit_code);
                    break;
                }
            }
            ExitCode::from(exit_code)
        }
    }
}

#[derive(Serialize)]
struct CliDiagnostic {
    kind: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    exit_code: u8,
}

enum CliError {
    Usage(String),
    Runtime(String),
    Violations(Vec<CliDiagnostic>),
}

impl CliError {
    const fn exit_code(&self) -> u8 {
        match self {
            Self::Usage(_) => EXIT_USAGE_ERROR,
            Self::Runtime(_) | Self::Violations(_) => EXIT_VIOLATIONS,
        }
    }

    fn diagnostics(self) -> Vec<CliDiagnostic> {
        match self {
            Self::Usage(message) => {
                vec![diagnostic("usage_error", message, None, None, EXIT_USAGE_ERROR)]
            }
            Self::Runtime(message) => {
                vec![diagnostic("runtime_error", message, None, None, EXIT_VIOLATIONS)]
            }
            Self::Violations(diagnostics) => diagnostics,
        }
    }
}

fn run() -> Result<(), CliError> {
    let arguments = arguments().map_err(CliError::Usage)?;
    let workspace_root = workspace_root().map_err(CliError::Runtime)?;
    let base_ref = match arguments.base_ref {
        Some(base_ref) => base_ref,
        None => default_base_ref(&workspace_root).map_err(CliError::Runtime)?,
    };
    let base_commit = git_output(&workspace_root, ["merge-base", "HEAD", &base_ref]).map_err(CliError::Runtime)?;
    let changed_lines = changed_rust_lines(&workspace_root, &base_commit, arguments.staged).map_err(CliError::Runtime)?;
    let mut violations = Vec::new();

    for (file, lines) in changed_lines {
        let source = source_for_validation(&workspace_root, &file, arguments.staged).map_err(CliError::Runtime)?;
        let file_violations = validate_changed_allows(&source, &lines)
            .map_err(|error| CliError::Runtime(format!("{}: failed to parse Rust source: {error}", file.display())))?;

        violations.extend(file_violations.into_iter().map(|violation| {
            diagnostic(
                "validation_error",
                violation.message.to_owned(),
                Some(file.display().to_string()),
                Some(violation.line),
                EXIT_VIOLATIONS,
            )
        }));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(CliError::Violations(violations))
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let root = git_output(
        &env::current_dir().map_err(|error| error.to_string())?,
        ["rev-parse", "--show-toplevel"],
    )?;
    Ok(PathBuf::from(root))
}

struct Arguments {
    base_ref: Option<String>,
    staged: bool,
}

fn arguments() -> Result<Arguments, String> {
    let mut arguments = env::args().skip(1);
    let mut base_ref = None;
    let mut staged = false;

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--base-ref" if base_ref.is_none() => {
                let Some(reference) = arguments.next() else {
                    return Err(String::from("usage error: `--base-ref` requires a Git reference"));
                };
                base_ref = Some(reference);
            }
            "--staged" if !staged => staged = true,
            "--base-ref" | "--staged" => return Err(format!("usage error: duplicate argument `{argument}`")),
            _ => {
                return Err(format!(
                    "usage error: unexpected argument `{argument}`; use `--base-ref <REF>` and `--staged`"
                ));
            }
        }
    }

    Ok(Arguments { base_ref, staged })
}

fn default_base_ref(workspace_root: &PathBuf) -> Result<String, String> {
    const CANDIDATES: [&str; 4] = ["origin/develop", "upstream/develop", "torrust/develop", "develop"];

    for candidate in CANDIDATES {
        if git_ref_exists(workspace_root, candidate)? {
            return Ok(String::from(candidate));
        }
    }

    Err(format!("could not resolve a base reference; tried {}", CANDIDATES.join(", ")))
}

fn git_ref_exists(workspace_root: &PathBuf, reference: &str) -> Result<bool, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "--quiet", reference])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("failed to run Git: {error}"))?;

    Ok(output.status.success())
}

fn changed_rust_lines(
    workspace_root: &PathBuf,
    base_commit: &str,
    staged: bool,
) -> Result<BTreeMap<PathBuf, BTreeSet<usize>>, String> {
    let mut command = Command::new("git");
    command.args([
        "-c",
        "diff.noprefix=false",
        "-c",
        "diff.mnemonicPrefix=false",
        "-c",
        "core.quotePath=false",
        "--no-pager",
        "diff",
        "--no-ext-diff",
        "--no-color",
        "--unified=0",
    ]);
    if staged {
        command.arg("--cached");
    }
    let output = command
        .args([base_commit, "--", "*.rs"])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("failed to run Git diff: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "Git diff command failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let diff = String::from_utf8(output.stdout).map_err(|error| format!("Git diff output was not valid UTF-8: {error}"))?;
    parse_changed_rust_lines(&diff)
}

fn source_for_validation(workspace_root: &PathBuf, file: &PathBuf, staged: bool) -> Result<String, String> {
    if staged {
        return git_output(workspace_root, ["show", &format!(":{}", file.display())]);
    }

    let source_path = workspace_root.join(file);
    fs::read_to_string(&source_path).map_err(|error| format!("{}: failed to read source: {error}", source_path.display()))
}

fn parse_changed_rust_lines(diff: &str) -> Result<BTreeMap<PathBuf, BTreeSet<usize>>, String> {
    let mut changed_lines = BTreeMap::new();
    let mut current_file = None;

    for line in diff.lines() {
        if line.starts_with("diff --git ") {
            current_file = None;
            continue;
        }

        if let Some(file) = line.strip_prefix("+++ b/") {
            current_file = Some(PathBuf::from(file));
            continue;
        }
        if let Some(file) = line.strip_prefix("+++ ") {
            if file == "/dev/null" {
                current_file = None;
                continue;
            }
            return Err(format!("unrecognized Git diff file header `{line}`"));
        }

        let Some(hunk) = line.strip_prefix("@@ ") else {
            continue;
        };
        let Some(file) = &current_file else {
            return Err(format!("Git diff hunk has no recognized Rust file header `{line}`"));
        };
        let Some(range) = hunk.split_whitespace().nth(1) else {
            continue;
        };
        let Some(added_range) = range.strip_prefix('+') else {
            continue;
        };
        let (start, count) = added_range.split_once(',').unwrap_or((added_range, "1"));
        let start = start
            .parse::<usize>()
            .map_err(|error| format!("failed to parse Git diff line range `{added_range}`: {error}"))?;
        let count = count
            .parse::<usize>()
            .map_err(|error| format!("failed to parse Git diff line range `{added_range}`: {error}"))?;

        changed_lines
            .entry(file.clone())
            .or_insert_with(BTreeSet::new)
            .extend(start..start + count);
    }

    Ok(changed_lines)
}

fn git_output<const N: usize>(workspace_root: &PathBuf, arguments: [&str; N]) -> Result<String, String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("failed to run Git: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "Git command failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    String::from_utf8(output.stdout)
        .map(|output| output.trim().to_owned())
        .map_err(|error| format!("Git output was not valid UTF-8: {error}"))
}

const fn diagnostic(
    kind: &'static str,
    message: String,
    file: Option<String>,
    line: Option<usize>,
    exit_code: u8,
) -> CliDiagnostic {
    CliDiagnostic {
        kind,
        message,
        file,
        line,
        exit_code,
    }
}

fn emit_diagnostic(diagnostic: &CliDiagnostic) -> io::Result<()> {
    let mut stderr = io::stderr().lock();
    write_diagnostic(&mut stderr, diagnostic)
}

fn emit_output_failure(exit_code: u8) {
    let mut stderr = io::stderr().lock();
    drop(stderr.write_all(b"{\"kind\":\"output_error\",\"message\":\"failed to emit diagnostic\",\"exit_code\":"));
    drop(write!(stderr, "{exit_code}"));
    drop(stderr.write_all(b"}\n"));
}

fn write_diagnostic(writer: &mut impl Write, diagnostic: &CliDiagnostic) -> io::Result<()> {
    serde_json::to_writer(&mut *writer, diagnostic)?;
    writer.write_all(b"\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("intentional write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_should_parse_added_lines_from_multiple_rust_hunks() {
        let diff = "+++ b/src/lib.rs\n@@ -1 +1,2 @@\n unchanged\n+added_one\n+added_two\n@@ -5 +7 @@\n+added_three\n";

        let changed_lines = parse_changed_rust_lines(diff).unwrap();

        assert_eq!(changed_lines[&PathBuf::from("src/lib.rs")], BTreeSet::from([1, 2, 7]));
    }

    #[test]
    fn it_should_reject_an_unrecognized_git_diff_file_header() {
        let diff = "diff --git a/src/lib.rs b/src/lib.rs\n+++ w/src/lib.rs\n@@ -0,0 +1 @@\n+#[allow(clippy::too_many_lines)]\n";

        let error = parse_changed_rust_lines(diff).unwrap_err();

        assert!(error.contains("unrecognized Git diff file header"));
    }

    #[test]
    fn it_should_report_a_diagnostic_write_failure() {
        let diagnostic = diagnostic("runtime_error", String::from("failure"), None, None, EXIT_VIOLATIONS);

        let error = write_diagnostic(&mut FailingWriter, &diagnostic).unwrap_err();

        assert_eq!(error.kind(), io::ErrorKind::Other);
    }
}
