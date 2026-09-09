//! Command-line adapter for prospective Clippy `allow` rationale validation.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write as _};
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use std::{env, fmt, fs};

use clippy_allow_reasons::validate_changed_allows;

const EXIT_VIOLATIONS: u8 = 1;
const EXIT_USAGE_ERROR: u8 = 2;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError::Usage(error)) => {
            write_stderr(&error);
            ExitCode::from(EXIT_USAGE_ERROR)
        }
        Err(error) => {
            write_stderr(&error.to_string());
            ExitCode::from(EXIT_VIOLATIONS)
        }
    }
}
enum CliError {
    Usage(String),
    Validation(String),
}
impl From<String> for CliError {
    fn from(error: String) -> Self {
        Self::Validation(error)
    }
}
impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(error) | Self::Validation(error) => formatter.write_str(error),
        }
    }
}

fn run() -> Result<(), CliError> {
    let workspace_root = workspace_root()?;
    let base_ref = base_ref().map_err(CliError::Usage)?;
    let base_commit = git_output(&workspace_root, ["merge-base", "HEAD", &base_ref])?;
    let changed_lines = changed_rust_lines(&workspace_root, &base_commit)?;
    let mut violations = Vec::new();

    for (file, lines) in changed_lines {
        let source_path = workspace_root.join(&file);
        let source = fs::read_to_string(&source_path)
            .map_err(|error| format!("{}: failed to read source: {error}", source_path.display()))?;
        let file_violations = validate_changed_allows(&source, &lines)
            .map_err(|error| format!("{}: failed to parse Rust source: {error}", file.display()))?;

        violations.extend(
            file_violations
                .into_iter()
                .map(|violation| format!("{}:{}: {}", file.display(), violation.line, violation.message)),
        );
    }

    if violations.is_empty() {
        write_stdout("All newly added or modified Clippy allows have native reasons.");
        Ok(())
    } else {
        Err(CliError::Validation(violations.join("\n")))
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
    let diff = git_output(workspace_root, ["diff", "--unified=0", base_commit, "--", "*.rs"])?;
    parse_changed_rust_lines(&diff)
}

fn parse_changed_rust_lines(diff: &str) -> Result<BTreeMap<PathBuf, BTreeSet<usize>>, String> {
    let mut changed_lines = BTreeMap::new();
    let mut current_file = None;

    for line in diff.lines() {
        if let Some(file) = line.strip_prefix("+++ b/") {
            current_file = Some(PathBuf::from(file));
            continue;
        }

        let Some(hunk) = line.strip_prefix("@@ ") else {
            continue;
        };
        let Some(file) = &current_file else {
            continue;
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

fn write_stdout(message: &str) {
    let mut stdout = io::stdout().lock();
    drop(writeln!(stdout, "{message}"));
}

fn write_stderr(message: &str) {
    let mut stderr = io::stderr().lock();
    drop(writeln!(stderr, "{message}"));
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
}
