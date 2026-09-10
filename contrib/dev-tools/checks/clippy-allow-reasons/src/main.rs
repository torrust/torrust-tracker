//! Command-line adapter for prospective Clippy `allow` rationale validation.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write as _};
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
                emit_diagnostic(&diagnostic);
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
    let base_ref = base_ref().map_err(CliError::Usage)?;
    let workspace_root = workspace_root().map_err(CliError::Runtime)?;
    let base_commit = git_output(&workspace_root, ["merge-base", "HEAD", &base_ref]).map_err(CliError::Runtime)?;
    let changed_lines = changed_rust_lines(&workspace_root, &base_commit).map_err(CliError::Runtime)?;
    let mut violations = Vec::new();

    for (file, lines) in changed_lines {
        let source_path = workspace_root.join(&file);
        let source = fs::read_to_string(&source_path)
            .map_err(|error| CliError::Runtime(format!("{}: failed to read source: {error}", source_path.display())))?;
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

fn base_ref() -> Result<String, String> {
    let mut arguments = env::args().skip(1);
    let Some(argument) = arguments.next() else {
        return Ok(String::from("torrust/develop"));
    };

    if argument != "--base-ref" {
        return Err(format!(
            "usage error: unexpected argument `{argument}`; use `--base-ref <REF>`"
        ));
    }

    let Some(reference) = arguments.next() else {
        return Err(String::from("usage error: `--base-ref` requires a Git reference"));
    };

    if arguments.next().is_some() {
        return Err(String::from("usage error: expected only `--base-ref <REF>`"));
    }

    Ok(reference)
}

fn changed_rust_lines(workspace_root: &PathBuf, base_commit: &str) -> Result<BTreeMap<PathBuf, BTreeSet<usize>>, String> {
    let output = Command::new("git")
        .args([
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
            base_commit,
            "--",
            "*.rs",
        ])
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

fn emit_diagnostic(diagnostic: &CliDiagnostic) {
    let mut stderr = io::stderr().lock();
    drop(serde_json::to_writer(&mut stderr, diagnostic));
    drop(stderr.write_all(b"\n"));
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
