//! Validates Markdown frontmatter and reports NDJSON diagnostics on stderr.
//!
//! Output class `no-stdout-result`: stdout stays empty; exit codes are `0` for success, `1` for
//! validation errors or runtime failures, and `2` for invalid invocation.

use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use clap::Parser;
use clap::error::ErrorKind;
use frontmatter_validator::profile::validate;
use frontmatter_validator::{Diagnostic, DiagnosticCategory, DocumentOwnership, Severity, extract_with_ownership};
use serde::Serialize;

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const EXIT_USAGE: u8 = 2;

/// Validate the frontmatter of Markdown files against the repository's v1 contract.
#[derive(Debug, Parser)]
#[command(name = "frontmatter-validator")]
struct Arguments {
    /// Markdown files to validate.
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

/// One NDJSON line on stderr. Field order and nullability are the command's output contract.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Record {
    Diagnostic {
        path: String,
        severity: Severity,
        category: DiagnosticCategory,
        field_path: Option<String>,
        message: String,
    },
    UsageError {
        message: String,
        exit_code: u8,
    },
    RuntimeError {
        path: Option<String>,
        message: String,
        exit_code: u8,
    },
    Help {
        message: String,
    },
}

impl Record {
    fn diagnostic(path: &Path, diagnostic: Diagnostic) -> Self {
        Self::Diagnostic {
            path: display_path(path),
            severity: diagnostic.severity,
            category: diagnostic.category,
            field_path: diagnostic.field_path,
            message: diagnostic.message,
        }
    }

    fn usage_error(message: impl Into<String>) -> Self {
        Self::UsageError {
            message: message.into(),
            exit_code: EXIT_USAGE,
        }
    }

    fn runtime_error(path: &Path, message: impl Into<String>) -> Self {
        Self::RuntimeError {
            path: Some(display_path(path)),
            message: message.into(),
            exit_code: EXIT_FAILURE,
        }
    }
}

fn main() -> ExitCode {
    let mut stderr = io::stderr().lock();
    ExitCode::from(run(env::args_os(), &mut stderr))
}

/// Runs the command with the program name as the first argument and returns its exit code.
fn run(arguments: impl IntoIterator<Item = OsString>, stderr: &mut impl Write) -> u8 {
    let (records, exit_code) = match Arguments::try_parse_from(arguments) {
        Ok(arguments) => validate_files(arguments.paths),
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

fn validate_files(mut paths: Vec<PathBuf>) -> (Vec<Record>, u8) {
    if let Some(missing) = paths.iter().find(|path| !path.exists()) {
        let message = format!("path `{}` does not exist", display_path(missing));
        return (vec![Record::usage_error(message)], EXIT_USAGE);
    }

    paths.sort();
    paths.dedup();

    let mut records = Vec::new();
    for path in &paths {
        match fs::read_to_string(path) {
            Ok(markdown) => {
                records.extend(validate_document(path, &markdown).map(|diagnostic| Record::diagnostic(path, diagnostic)));
            }
            Err(error) => {
                records.push(Record::runtime_error(path, format!("could not read the file: {error}")));
                return (records, EXIT_FAILURE);
            }
        }
    }

    let has_error = records.iter().any(|record| {
        matches!(
            record,
            Record::Diagnostic {
                severity: Severity::Error,
                ..
            }
        )
    });
    (records, if has_error { EXIT_FAILURE } else { EXIT_SUCCESS })
}

/// The library reports the first extraction or profile failure of a document.
fn validate_document(path: &Path, markdown: &str) -> Option<Diagnostic> {
    match extract_with_ownership(markdown, ownership(path)) {
        Ok(Some(frontmatter)) => validate(&frontmatter).err(),
        Ok(None) => None,
        Err(diagnostic) => Some(diagnostic),
    }
}

/// Agent Skills and agent profiles have an externally governed top-level schema.
fn ownership(path: &Path) -> DocumentOwnership {
    let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
    if file_name == "SKILL.md" || file_name.ends_with(".agent.md") {
        DocumentOwnership::External
    } else {
        DocumentOwnership::Repository
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn emit(records: &[Record], stderr: &mut impl Write) -> io::Result<()> {
    for record in records {
        serde_json::to_writer(&mut *stderr, record)?;
        writeln!(stderr)?;
    }
    stderr.flush()
}

#[cfg(test)]
mod tests {
    // Owns argument parsing, the D9 record catalog, record ordering, ownership dispatch, and exit codes.

    use std::ffi::OsString;
    use std::fs;
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    use rstest::rstest;
    use serde_json::{Map, Value};
    use tempfile::TempDir;

    use super::run;

    const VALID_ISSUE: &str = include_str!("../../fixtures/accepted/issue.md");
    const WRONG_SCALAR_ISSUE: &str = include_str!("../../fixtures/rejected/issue-wrong-scalar.md");

    /// The exit code and the parsed NDJSON records a run wrote to stderr.
    struct Outcome {
        exit_code: u8,
        records: Vec<Map<String, Value>>,
    }

    impl Outcome {
        fn only_record(&self) -> &Map<String, Value> {
            assert_eq!(self.records.len(), 1, "expected exactly one record: {:?}", self.records);
            &self.records[0]
        }
    }

    fn validator(arguments: &[&str]) -> Outcome {
        let mut stderr = Vec::new();
        let arguments = std::iter::once("frontmatter-validator").chain(arguments.iter().copied());
        let exit_code = run(arguments.map(OsString::from), &mut stderr);
        let records = String::from_utf8(stderr)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        Outcome { exit_code, records }
    }

    fn write_markdown(directory: &TempDir, name: &str, content: &str) -> PathBuf {
        let path = directory.path().join(name);
        fs::write(&path, content).unwrap();
        path
    }

    fn argument(path: &Path) -> &str {
        path.to_str().unwrap()
    }

    fn keys(record: &Map<String, Value>) -> Vec<&str> {
        record.keys().map(String::as_str).collect()
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
    #[case::strict_v1_issue(VALID_ISSUE)]
    #[case::no_frontmatter("# Plain document\n")]
    fn it_should_exit_zero_without_records_when_every_document_is_valid(#[case] content: &str) {
        // Arrange
        let directory = TempDir::new().unwrap();
        let document = write_markdown(&directory, "document.md", content);

        // Act
        let outcome = validator(&[argument(&document)]);

        // Assert: a clean run is silent.
        assert_eq!((outcome.exit_code, outcome.records.len()), (0, 0));
    }

    #[test]
    fn it_should_report_one_diagnostic_record_and_exit_one_for_an_invalid_document() {
        // Arrange: a strict issue quotes its positive-integer `github-issue`.
        let directory = TempDir::new().unwrap();
        let document = write_markdown(&directory, "document.md", WRONG_SCALAR_ISSUE);

        // Act
        let outcome = validator(&[argument(&document)]);

        // Assert: the record has the D9 diagnostic fields in contract order.
        let record = outcome.only_record();
        assert_eq!(outcome.exit_code, 1);
        assert_eq!(
            keys(record),
            ["kind", "path", "severity", "category", "field_path", "message"]
        );
        assert_eq!(
            (
                &record["kind"],
                &record["path"],
                &record["severity"],
                &record["category"],
                &record["field_path"]
            ),
            (
                &Value::from("diagnostic"),
                &Value::from(argument(&document)),
                &Value::from("error"),
                &Value::from("wrong-scalar-type"),
                &Value::from("github-issue"),
            )
        );
    }

    #[test]
    fn it_should_emit_a_null_field_path_when_the_failure_concerns_no_single_field() {
        // Arrange: the frontmatter is not valid YAML at all.
        let directory = TempDir::new().unwrap();
        let document = write_markdown(&directory, "document.md", "---\ndoc-type: [issue\n---\n");

        // Act
        let outcome = validator(&[argument(&document)]);

        // Assert: the nullable field is present rather than omitted.
        let record = outcome.only_record();
        assert_eq!(
            (&record["category"], record.get("field_path")),
            (&Value::from("malformed-yaml"), Some(&Value::Null))
        );
    }

    #[test]
    fn it_should_order_records_by_path_regardless_of_argument_order() {
        // Arrange: two invalid documents are passed in reverse path order.
        let directory = TempDir::new().unwrap();
        let first = write_markdown(&directory, "a.md", WRONG_SCALAR_ISSUE);
        let second = write_markdown(&directory, "b.md", WRONG_SCALAR_ISSUE);

        // Act
        let outcome = validator(&[argument(&second), argument(&first)]);

        // Assert
        let paths: Vec<&Value> = outcome.records.iter().map(|record| &record["path"]).collect();
        assert_eq!(paths, [&Value::from(argument(&first)), &Value::from(argument(&second))]);
    }

    #[rstest]
    #[case::agent_skill("SKILL.md", 0)]
    #[case::agent_profile("implementer.agent.md", 0)]
    #[case::repository_document("notes.md", 1)]
    fn it_should_validate_top_level_semantic_links_only_for_repository_owned_file_names(
        #[case] file_name: &str,
        #[case] expected_exit_code: u8,
    ) {
        // Arrange: identical content whose top-level extension is invalid; only the file name differs.
        let directory = TempDir::new().unwrap();
        let content = "---\nname: example\ndescription: Example.\nsemantic-links: invalid\n---\n";
        let document = write_markdown(&directory, file_name, content);

        // Act
        let outcome = validator(&[argument(&document)]);

        // Assert: externally governed files ignore the top-level extension.
        assert_eq!(outcome.exit_code, expected_exit_code, "records: {:?}", outcome.records);
    }

    #[rstest]
    #[case::no_arguments(&[])]
    #[case::unsupported_version_flag(&["--version"])]
    #[case::nonexistent_path(&["does/not/exist.md"])]
    fn it_should_report_a_single_usage_error_record_and_exit_two(#[case] arguments: &[&str]) {
        // Act
        let outcome = validator(arguments);

        // Assert
        let record = outcome.only_record();
        assert_eq!(outcome.exit_code, 2);
        assert_eq!(keys(record), ["kind", "message", "exit_code"]);
        assert_eq!(
            (&record["kind"], &record["exit_code"]),
            (&Value::from("usage_error"), &Value::from(2))
        );
    }

    #[test]
    fn it_should_render_help_as_a_single_help_record_and_exit_zero() {
        // Act
        let outcome = validator(&["--help"]);

        // Assert
        let record = outcome.only_record();
        assert_eq!(outcome.exit_code, 0);
        assert_eq!(keys(record), ["kind", "message"]);
        assert_eq!(record["kind"], "help");
        assert!(record["message"].as_str().unwrap().contains("Usage: frontmatter-validator"));
    }

    #[test]
    fn it_should_report_a_runtime_error_record_and_exit_one_when_a_file_cannot_be_read() {
        // Arrange: the file exists but is not UTF-8 text.
        let directory = TempDir::new().unwrap();
        let document = directory.path().join("binary.md");
        fs::write(&document, [0xff, 0xfe]).unwrap();

        // Act
        let outcome = validator(&[argument(&document)]);

        // Assert
        let record = outcome.only_record();
        assert_eq!(outcome.exit_code, 1);
        assert_eq!(keys(record), ["kind", "path", "message", "exit_code"]);
        assert_eq!(
            (&record["kind"], &record["path"], &record["exit_code"]),
            (
                &Value::from("runtime_error"),
                &Value::from(argument(&document)),
                &Value::from(1)
            )
        );
    }

    #[test]
    fn it_should_exit_one_when_stderr_cannot_be_written() {
        // Act: a usage error must be reported, but stderr rejects every write.
        let exit_code = run(["frontmatter-validator"].map(OsString::from), &mut FailingWriter);

        // Assert
        assert_eq!(exit_code, 1);
    }
}
