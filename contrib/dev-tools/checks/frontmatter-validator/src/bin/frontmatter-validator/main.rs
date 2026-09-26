//! Validates Markdown frontmatter and reports NDJSON diagnostics on stderr.
//!
//! Output class `no-stdout-result`: stdout stays empty; exit codes are `0` for success, `1` for
//! validation errors or runtime failures, and `2` for invalid invocation.

mod discovery;
mod git;
mod record;

use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::error::ErrorKind;
use clap::{ArgGroup, Parser};
use discovery::{DiscoveryError, Mode, discover};
use frontmatter_validator::repository::validate_document;
use git::Git;
use record::{EXIT_FAILURE, EXIT_SUCCESS, EXIT_USAGE, Record, emit};

/// Validate the frontmatter of Markdown files against the repository's v1 contract.
#[derive(Debug, Parser)]
#[command(
    name = "frontmatter-validator",
    group(ArgGroup::new("mode").required(true).args(["paths", "staged", "all"]))
)]
struct Arguments {
    /// Markdown files, or directories whose tracked Markdown files are validated.
    paths: Vec<PathBuf>,
    /// Validate the staged content of staged Markdown files.
    #[arg(long)]
    staged: bool,
    /// Validate every tracked Markdown file.
    #[arg(long)]
    all: bool,
}

impl Arguments {
    fn mode(self) -> Mode {
        if self.staged {
            Mode::Staged
        } else if self.all {
            Mode::All
        } else {
            Mode::Paths(self.paths)
        }
    }
}

fn main() -> ExitCode {
    let working_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut stderr = io::stderr().lock();
    ExitCode::from(run(env::args_os(), &working_directory, &mut stderr))
}

/// Runs the command with the program name as the first argument and returns its exit code.
fn run(arguments: impl IntoIterator<Item = OsString>, working_directory: &Path, stderr: &mut impl Write) -> u8 {
    let (records, exit_code) = match Arguments::try_parse_from(arguments) {
        Ok(arguments) => validate_mode(arguments.mode(), working_directory),
        Err(error) if error.kind() == ErrorKind::DisplayHelp => (
            vec![Record::Help {
                message: error.to_string().trim_end().to_owned(),
            }],
            EXIT_SUCCESS,
        ),
        Err(error) => (vec![Record::usage_error(error.to_string().trim_end())], EXIT_USAGE),
    };

    match emit(&records, stderr) {
        Ok(()) => exit_code,
        Err(_) => EXIT_FAILURE,
    }
}

fn validate_mode(mode: Mode, working_directory: &Path) -> (Vec<Record>, u8) {
    let git = match Git::discover(working_directory) {
        Ok(git) => git,
        Err(message) => return (vec![Record::runtime_error(None, message)], EXIT_FAILURE),
    };
    let documents = match discover(mode, &git, working_directory) {
        Ok(documents) => documents,
        Err(DiscoveryError::Usage(message)) => return (vec![Record::usage_error(message)], EXIT_USAGE),
        Err(DiscoveryError::Runtime(message)) => return (vec![Record::runtime_error(None, message)], EXIT_FAILURE),
    };

    let mut records = Vec::new();
    for document in &documents {
        match document.read(&git) {
            Ok(markdown) => {
                records.extend(
                    validate_document(&document.path, &markdown)
                        .into_iter()
                        .map(|diagnostic| Record::diagnostic(&document.path, diagnostic)),
                );
            }
            Err(message) => {
                records.push(Record::runtime_error(Some(document.path.clone()), message));
                return (records, EXIT_FAILURE);
            }
        }
    }

    let has_error = records.iter().any(Record::is_error);
    (records, if has_error { EXIT_FAILURE } else { EXIT_SUCCESS })
}

#[cfg(test)]
mod tests {
    // Owns argument parsing, help rendering, and output failure. Behavior that needs a repository
    // is tested through the built binary in `tests/cli.rs`; document policy is owned by the
    // library's `repository` module.

    use std::ffi::OsString;
    use std::io::{self, Write};
    use std::path::Path;

    use rstest::rstest;
    use serde_json::{Map, Value};

    use super::run;

    /// Records from a run that fails before any repository access.
    fn parse_only(arguments: &[&str]) -> (u8, Vec<Map<String, Value>>) {
        let mut stderr = Vec::new();
        let arguments = std::iter::once("frontmatter-validator").chain(arguments.iter().copied());
        let exit_code = run(arguments.map(OsString::from), Path::new("."), &mut stderr);
        let records = String::from_utf8(stderr)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        (exit_code, records)
    }

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("intentional write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[rstest]
    #[case::no_mode(&[])]
    #[case::staged_and_all(&["--staged", "--all"])]
    #[case::paths_and_staged(&["README.md", "--staged"])]
    #[case::unsupported_version_flag(&["--version"])]
    fn it_should_report_a_single_usage_error_record_for_an_invalid_mode_selection(#[case] arguments: &[&str]) {
        // Act
        let (exit_code, records) = parse_only(arguments);

        // Assert
        assert_eq!(exit_code, 2);
        assert_eq!(records.len(), 1, "records: {records:?}");
        assert_eq!(records[0].keys().collect::<Vec<_>>(), ["kind", "message", "exit_code"]);
        assert_eq!(
            (&records[0]["kind"], &records[0]["exit_code"]),
            (&Value::from("usage_error"), &Value::from(2))
        );
    }

    #[test]
    fn it_should_render_help_as_a_single_help_record_and_exit_zero() {
        // Act
        let (exit_code, records) = parse_only(&["--help"]);

        // Assert
        assert_eq!(exit_code, 0);
        assert_eq!(records.len(), 1, "records: {records:?}");
        assert_eq!(records[0].keys().collect::<Vec<_>>(), ["kind", "message"]);
        assert_eq!(records[0]["kind"], "help");
        assert!(
            records[0]["message"]
                .as_str()
                .unwrap()
                .contains("Usage: frontmatter-validator")
        );
    }

    #[test]
    fn it_should_exit_one_when_stderr_cannot_be_written() {
        // Act: a usage error must be reported, but stderr rejects every write.
        let exit_code = run(
            ["frontmatter-validator"].map(OsString::from),
            Path::new("."),
            &mut FailingWriter,
        );

        // Assert
        assert_eq!(exit_code, 1);
    }
}
