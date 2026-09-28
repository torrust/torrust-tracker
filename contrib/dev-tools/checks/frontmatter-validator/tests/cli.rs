//! Command-boundary tests: the built binary runs in a disposable git repository.
//!
//! Every git invocation, including the binary's, clears inherited `GIT_*` variables and sets a
//! ceiling directory, so a test run from a git hook can never read or change the real repository.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use rstest::rstest;
use serde_json::{Map, Value};
use tempfile::TempDir;

const VALID_ISSUE: &str = include_str!("../fixtures/accepted/issue.md");
const WRONG_SCALAR_ISSUE: &str = include_str!("../fixtures/rejected/issue-wrong-scalar.md");
const UNKNOWN_FIELD_ISSUE: &str = include_str!("../fixtures/rejected/issue-unknown-field.md");
const INVALID_STATUS_ISSUE: &str = include_str!("../fixtures/rejected/issue-invalid-status.md");
const INVALID_REFERENCE_ISSUE: &str = include_str!("../fixtures/rejected/issue-invalid-reference.md");
const PLAIN_DOCUMENT: &str = "# Plain document\n";
/// The `spec-path` the crate fixtures declare, so a fixture written here carries no hidden D7 defect.
const OPEN_SPEC: &str = "docs/issues/open/example/ISSUE.md";

/// A strict v1 issue at `OPEN_SPEC`; only the status and the related-artifact line vary.
fn open_strict_issue(status: &str, related_artifact: &str) -> String {
    format!(
        "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: {status}\npriority: p1\nepic: null\ngithub-issue: 1\nspec-path: {OPEN_SPEC}\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-26 11:30\"\nsemantic-links:\n  related-artifacts:\n    - {related_artifact}\n---\n"
    )
}

/// A crate fixture moved to `spec_path` with a `status` valid there, keeping its one defect.
fn relocated_fixture(fixture: &str, spec_path: &str, status: &str) -> String {
    let from_spec_path = format!("spec-path: {OPEN_SPEC}");
    assert!(
        !fixture.starts_with("---") || (fixture.contains(&from_spec_path) && fixture.contains("status: planned")),
        "fixture no longer declares `{from_spec_path}` and `status: planned`"
    );
    fixture
        .replace(&from_spec_path, &format!("spec-path: {spec_path}"))
        .replace("status: planned", &format!("status: {status}"))
}

/// Inherited variables that would redirect git away from the disposable repository.
const GIT_ENVIRONMENT: &[&str] = &[
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
    "GIT_PREFIX",
    "GIT_CONFIG_PARAMETERS",
    "GIT_CONFIG_COUNT",
    "GIT_LITERAL_PATHSPECS",
    "GIT_GLOB_PATHSPECS",
    "GIT_NOGLOB_PATHSPECS",
    "GIT_ICASE_PATHSPECS",
];

/// The exit code and the parsed NDJSON stderr records of one run. Stdout is checked to be empty.
struct Outcome {
    exit_code: i32,
    records: Vec<Map<String, Value>>,
}

impl Outcome {
    fn from(output: &Output) -> Self {
        assert_eq!(String::from_utf8_lossy(&output.stdout), "", "stdout must stay empty");
        let records = std::str::from_utf8(&output.stderr)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap_or_else(|error| panic!("not NDJSON: {line}: {error}")))
            .collect();

        Self {
            exit_code: output.status.code().unwrap(),
            records,
        }
    }

    fn only_record(&self) -> &Map<String, Value> {
        assert_eq!(self.records.len(), 1, "expected exactly one record: {:?}", self.records);
        &self.records[0]
    }

    fn paths(&self) -> Vec<&str> {
        self.records.iter().map(|record| record["path"].as_str().unwrap()).collect()
    }
}

/// A disposable git repository with an unborn branch; `TempDir` removes it on drop.
struct Repository {
    directory: TempDir,
}

impl Repository {
    fn new() -> Self {
        let repository = Self {
            directory: TempDir::new().unwrap(),
        };
        repository.git(&["init", "--quiet"]);
        repository
    }

    fn root(&self) -> &Path {
        self.directory.path()
    }

    fn write(&self, path: &str, content: &str) {
        let file = self.root().join(path);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, content).unwrap();
    }

    fn stage(&self, path: &str) {
        self.git(&["add", "--", path]);
    }

    fn commit(&self) {
        self.git(&[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.com",
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "--quiet",
            "--no-gpg-sign",
            "--message",
            "fixture",
        ]);
    }

    fn delete(&self, path: &str) {
        fs::remove_file(self.root().join(path)).unwrap();
    }

    fn validate(&self, arguments: &[&str]) -> Outcome {
        validate_in(self.root(), arguments)
    }

    fn git(&self, arguments: &[&str]) {
        let status = isolated(Command::new("git"), self.root()).args(arguments).status().unwrap();
        assert!(status.success(), "git {arguments:?} failed");
    }
}

fn validate_in(directory: &Path, arguments: &[&str]) -> Outcome {
    let output = isolated(Command::new(env!("CARGO_BIN_EXE_frontmatter-validator")), directory)
        .args(arguments)
        .output()
        .unwrap();
    Outcome::from(&output)
}

fn validate_with_git_repository_redirect(directory: &Path, redirected_repository: &Repository, arguments: &[&str]) -> Outcome {
    let output = Command::new(env!("CARGO_BIN_EXE_frontmatter-validator"))
        .current_dir(directory)
        .env("GIT_DIR", redirected_repository.root().join(".git"))
        .env("GIT_WORK_TREE", redirected_repository.root())
        .args(arguments)
        .output()
        .unwrap();
    Outcome::from(&output)
}

/// Every disposable repository lives below the temp directory, so git never searches above it.
fn isolated(mut command: Command, directory: &Path) -> Command {
    for variable in GIT_ENVIRONMENT {
        command.env_remove(variable);
    }
    command
        .current_dir(directory)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir());
    command
}

fn keys(record: &Map<String, Value>) -> Vec<&str> {
    record.keys().map(String::as_str).collect()
}

#[rstest]
#[case::strict_v1_issue(VALID_ISSUE)]
#[case::no_frontmatter(PLAIN_DOCUMENT)]
fn it_should_exit_zero_without_records_when_every_document_is_valid(#[case] content: &str) {
    // Arrange
    let repository = Repository::new();
    repository.write("document.md", content);

    // Act
    let outcome = repository.validate(&["document.md"]);

    // Assert: a clean run is silent.
    assert_eq!((outcome.exit_code, outcome.records.len()), (0, 0));
}

#[test]
fn it_should_report_one_diagnostic_record_with_a_repository_relative_path() {
    // Arrange: a strict issue quotes its positive-integer `github-issue`.
    let repository = Repository::new();
    repository.write("docs/document.md", WRONG_SCALAR_ISSUE);

    // Act: the path is given relative to a subdirectory of the repository.
    let outcome = validate_in(&repository.root().join("docs"), &["document.md"]);

    // Assert: the record has the D9 diagnostic fields in contract order and a repository path.
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
            &Value::from("docs/document.md"),
            &Value::from("error"),
            &Value::from("wrong-scalar-type"),
            &Value::from("github-issue"),
        )
    );
}

#[test]
fn it_should_emit_a_null_field_path_when_the_failure_concerns_no_single_field() {
    // Arrange: the frontmatter is not valid YAML at all.
    let repository = Repository::new();
    repository.write("document.md", "---\ndoc-type: [issue\n---\n");

    // Act
    let outcome = repository.validate(&["document.md"]);

    // Assert: the nullable field is present rather than omitted.
    let record = outcome.only_record();
    assert_eq!(
        (&record["category"], record.get("field_path")),
        (&Value::from("malformed-yaml"), Some(&Value::Null))
    );
}

#[test]
fn it_should_order_records_by_path_and_report_each_document_once() {
    // Arrange: two invalid documents are passed in reverse order, one of them twice.
    let repository = Repository::new();
    repository.write("a.md", WRONG_SCALAR_ISSUE);
    repository.write("b.md", WRONG_SCALAR_ISSUE);

    // Act
    let outcome = repository.validate(&["b.md", "a.md", "./b.md"]);

    // Assert
    assert_eq!(outcome.paths(), ["a.md", "b.md"]);
}

#[rstest]
#[case::nonexistent_path("does-not-exist.md")]
#[case::outside_the_repository("OUTSIDE")]
fn it_should_report_a_single_usage_error_for_a_path_that_cannot_be_validated(#[case] path: &str) {
    // Arrange: `OUTSIDE` stands for an existing file in another directory.
    let repository = Repository::new();
    let elsewhere = TempDir::new().unwrap();
    let outside = elsewhere.path().join("outside.md");
    fs::write(&outside, PLAIN_DOCUMENT).unwrap();
    let path = if path == "OUTSIDE" { outside.to_str().unwrap() } else { path };

    // Act
    let outcome = repository.validate(&[path]);

    // Assert
    let record = outcome.only_record();
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(keys(record), ["kind", "message", "exit_code"]);
    assert_eq!(record["kind"], "usage_error");
}

#[test]
fn it_should_report_a_runtime_error_with_a_null_path_outside_any_repository() {
    // Arrange: a directory that is not inside a git repository.
    let directory = TempDir::new().unwrap();

    // Act
    let outcome = validate_in(directory.path(), &["--all"]);

    // Assert
    let record = outcome.only_record();
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(keys(record), ["kind", "path", "message", "exit_code"]);
    assert_eq!(
        (&record["kind"], &record["path"], &record["exit_code"]),
        (&Value::from("runtime_error"), &Value::Null, &Value::from(1))
    );
}

#[test]
fn it_should_ignore_inherited_git_repository_redirects() {
    // Arrange: the environment redirects git to a different repository than the current directory.
    let repository = Repository::new();
    let redirected_repository = Repository::new();
    repository.write("document.md", WRONG_SCALAR_ISSUE);

    // Act: validate the current repository with the redirect variables inherited.
    let outcome = validate_with_git_repository_redirect(repository.root(), &redirected_repository, &["document.md"]);

    // Assert: Git discovery and reads remain scoped to the current working directory's repository.
    assert_eq!(outcome.paths(), ["document.md"]);
}

#[test]
fn it_should_report_a_runtime_error_with_the_path_when_a_document_cannot_be_read() {
    // Arrange: the file exists but is not UTF-8 text.
    let repository = Repository::new();
    fs::write(repository.root().join("binary.md"), [0xff, 0xfe]).unwrap();

    // Act
    let outcome = repository.validate(&["binary.md"]);

    // Assert
    let record = outcome.only_record();
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(
        (&record["kind"], &record["path"]),
        (&Value::from("runtime_error"), &Value::from("binary.md"))
    );
}

#[test]
fn it_should_validate_an_explicit_untracked_file() {
    // Arrange: the invalid file was never added to git.
    let repository = Repository::new();
    repository.write("draft.md", WRONG_SCALAR_ISSUE);

    // Act
    let outcome = repository.validate(&["draft.md"]);

    // Assert
    assert_eq!(outcome.paths(), ["draft.md"]);
}

#[test]
fn it_should_treat_a_directory_argument_literally_rather_than_as_a_glob() {
    // Arrange: a directory named `d*` next to a sibling the glob `d*` would also match.
    let repository = Repository::new();
    repository.write("d*/inside.md", WRONG_SCALAR_ISSUE);
    repository.write("docs/sibling.md", WRONG_SCALAR_ISSUE);
    repository.stage("d*/inside.md");
    repository.stage("docs/sibling.md");

    // Act
    let outcome = repository.validate(&["d*"]);

    // Assert
    assert_eq!(outcome.paths(), ["d*/inside.md"]);
}

#[test]
fn it_should_expand_directories_when_git_is_told_to_treat_path_patterns_literally() {
    // Arrange: an invalid tracked document; some git front ends export this variable to hooks.
    let repository = Repository::new();
    repository.write("docs/inside.md", WRONG_SCALAR_ISSUE);
    repository.stage("docs/inside.md");

    // Act
    let output = isolated(Command::new(env!("CARGO_BIN_EXE_frontmatter-validator")), repository.root())
        .env("GIT_LITERAL_PATHSPECS", "1")
        .arg("docs")
        .output()
        .unwrap();

    // Assert: the directory is still expanded instead of silently matching nothing.
    assert_eq!(Outcome::from(&output).paths(), ["docs/inside.md"]);
}

#[cfg(unix)]
#[test]
fn it_should_report_a_symlinked_file_under_its_own_path() {
    // Arrange: `link.md` is a symlink to an invalid document elsewhere in the repository.
    let repository = Repository::new();
    repository.write("target/real.md", WRONG_SCALAR_ISSUE);
    std::os::unix::fs::symlink("target/real.md", repository.root().join("link.md")).unwrap();

    // Act
    let outcome = repository.validate(&["link.md"]);

    // Assert
    assert_eq!(outcome.paths(), ["link.md"]);
}

#[test]
fn it_should_expand_a_directory_to_its_tracked_markdown_files() {
    // Arrange: every file is invalid; only `tracked.md` is both tracked and Markdown.
    let repository = Repository::new();
    repository.write("docs/tracked.md", WRONG_SCALAR_ISSUE);
    repository.write("docs/nested/also-tracked.md", WRONG_SCALAR_ISSUE);
    repository.write("docs/untracked.md", WRONG_SCALAR_ISSUE);
    repository.write("docs/notes.txt", WRONG_SCALAR_ISSUE);
    repository.write("other/tracked.md", WRONG_SCALAR_ISSUE);
    for path in [
        "docs/tracked.md",
        "docs/nested/also-tracked.md",
        "docs/notes.txt",
        "other/tracked.md",
    ] {
        repository.stage(path);
    }

    // Act
    let outcome = repository.validate(&["docs"]);

    // Assert
    assert_eq!(outcome.paths(), ["docs/nested/also-tracked.md", "docs/tracked.md"]);
}

#[test]
fn it_should_validate_every_tracked_markdown_file_that_still_exists_with_all() {
    // Arrange: two tracked invalid files, one of them deleted from the working tree only, and one
    // invalid untracked file.
    let repository = Repository::new();
    repository.write("kept.md", WRONG_SCALAR_ISSUE);
    repository.write("deleted.md", WRONG_SCALAR_ISSUE);
    repository.write("untracked.md", WRONG_SCALAR_ISSUE);
    repository.stage("kept.md");
    repository.stage("deleted.md");
    repository.delete("deleted.md");

    // Act
    let outcome = repository.validate(&["--all"]);

    // Assert
    assert_eq!(outcome.paths(), ["kept.md"]);
}

#[rstest]
#[case::staged_invalid_working_copy_fixed(WRONG_SCALAR_ISSUE, VALID_ISSUE, 1)]
#[case::staged_valid_working_copy_broken(VALID_ISSUE, WRONG_SCALAR_ISSUE, 0)]
fn it_should_validate_the_index_content_with_staged(
    #[case] staged: &str,
    #[case] working_copy: &str,
    #[case] expected_exit_code: i32,
) {
    // Arrange: the staged and working-tree versions of one file differ.
    let repository = Repository::new();
    repository.write("document.md", staged);
    repository.stage("document.md");
    repository.write("document.md", working_copy);

    // Act
    let outcome = repository.validate(&["--staged"]);

    // Assert: only the staged content decides the result.
    assert_eq!(outcome.exit_code, expected_exit_code, "records: {:?}", outcome.records);
}

#[test]
fn it_should_validate_only_staged_files_with_staged() {
    // Arrange: an invalid file is committed and unchanged; a valid file is newly staged.
    let repository = Repository::new();
    repository.write("committed.md", WRONG_SCALAR_ISSUE);
    repository.stage("committed.md");
    repository.commit();
    repository.write("new.md", WRONG_SCALAR_ISSUE);
    repository.stage("new.md");

    // Act
    let outcome = repository.validate(&["--staged"]);

    // Assert
    assert_eq!(outcome.paths(), ["new.md"]);
}

#[test]
fn it_should_ignore_a_staged_deletion_with_staged() {
    // Arrange: a committed document is deleted and the deletion is staged.
    let repository = Repository::new();
    repository.write("removed.md", VALID_ISSUE);
    repository.stage("removed.md");
    repository.commit();
    repository.git(&["rm", "--quiet", "removed.md"]);

    // Act
    let outcome = repository.validate(&["--staged"]);

    // Assert: there is no staged content to validate, and no runtime error.
    assert_eq!(
        (outcome.exit_code, outcome.records.len()),
        (0, 0),
        "records: {:?}",
        outcome.records
    );
}

#[test]
fn it_should_read_a_root_level_staged_path_that_starts_with_an_index_stage_prefix() {
    // Arrange: the staged path begins with syntax Git otherwise parses as an index stage selector.
    let repository = Repository::new();
    repository.write("1:odd.md", WRONG_SCALAR_ISSUE);
    repository.stage("1:odd.md");

    // Act
    let outcome = repository.validate(&["--staged"]);

    // Assert: staged mode reads the file rather than reporting a git runtime failure.
    assert_eq!(outcome.paths(), ["1:odd.md"]);
}

#[rstest]
#[case::unknown_field(UNKNOWN_FIELD_ISSUE.to_owned(), "unknown-field", "owner")]
#[case::wrong_scalar(WRONG_SCALAR_ISSUE.to_owned(), "wrong-scalar-type", "github-issue")]
#[case::disallowed_value(INVALID_STATUS_ISSUE.to_owned(), "invalid-allowed-value", "status")]
#[case::reference_syntax(INVALID_REFERENCE_ISSUE.to_owned(), "invalid-reference-syntax", "semantic-links.related-artifacts")]
#[case::lifecycle(open_strict_issue("done", "\"issue #1\""), "lifecycle-location-mismatch", "status")]
#[case::missing_path(
    open_strict_issue("planned", "docs/removed.md"),
    "missing-artifact",
    "semantic-links.related-artifacts"
)]
fn it_should_report_each_failure_family_as_one_error_record(
    #[case] content: String,
    #[case] expected_category: &str,
    #[case] expected_field_path: &str,
) {
    // Arrange: an open spec whose only defect is the one each case names.
    let repository = Repository::new();
    repository.write(OPEN_SPEC, &content);

    // Act
    let outcome = repository.validate(&[OPEN_SPEC]);

    // Assert
    let record = outcome.only_record();
    assert_eq!(
        (
            outcome.exit_code,
            &record["severity"],
            &record["category"],
            &record["field_path"]
        ),
        (
            1,
            &Value::from("error"),
            &Value::from(expected_category),
            &Value::from(expected_field_path)
        )
    );
}

#[rstest]
#[case::closed_spec_finding_is_advisory("docs/issues/closed/example/ISSUE.md", "done", WRONG_SCALAR_ISSUE, "warning", 0)]
#[case::open_spec_finding_is_an_error(OPEN_SPEC, "planned", WRONG_SCALAR_ISSUE, "error", 1)]
#[case::open_legacy_spec_is_an_error(OPEN_SPEC, "planned", PLAIN_DOCUMENT, "error", 1)]
fn it_should_exit_one_only_when_a_record_is_an_error(
    #[case] path: &str,
    #[case] status: &str,
    #[case] content: &str,
    #[case] expected_severity: &str,
    #[case] expected_exit_code: i32,
) {
    // Arrange: one defect; the fixture's `spec-path` and status match where it is written.
    let repository = Repository::new();
    repository.write(path, &relocated_fixture(content, path, status));

    // Act
    let outcome = repository.validate(&[path]);

    // Assert
    let record = outcome.only_record();
    assert_eq!(
        (outcome.exit_code, &record["severity"]),
        (expected_exit_code, &Value::from(expected_severity))
    );
}

#[rstest]
#[case::staged_resolves_against_the_index(&["--staged"], 0)]
#[case::explicit_path_resolves_against_the_working_tree(&["docs/issues/open/1-example/ISSUE.md"], 1)]
fn it_should_resolve_related_artifacts_against_the_index_only_with_staged(
    #[case] arguments: &[&str],
    #[case] expected_exit_code: i32,
) {
    // Arrange: the referenced file is in the index but deleted from the working tree.
    let repository = Repository::new();
    let spec = "docs/issues/open/1-example/ISSUE.md";
    repository.write(
        spec,
        &format!(
            "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 1\nspec-path: {spec}\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-26 11:30\"\nsemantic-links:\n  related-artifacts:\n    - docs/notes.md\n---\n"
        ),
    );
    repository.write("docs/notes.md", PLAIN_DOCUMENT);
    repository.stage("docs/notes.md");
    repository.stage(spec);
    repository.delete("docs/notes.md");

    // Act
    let outcome = repository.validate(arguments);

    // Assert
    assert_eq!(outcome.exit_code, expected_exit_code, "records: {:?}", outcome.records);
}

#[rstest]
#[case::explicit_file(&["docs/templates/ISSUE.md"])]
#[case::directory(&["docs"])]
#[case::all(&["--all"])]
#[case::staged(&["--staged"])]
fn it_should_skip_excluded_paths_in_every_mode(#[case] arguments: &[&str]) {
    // Arrange: invalid tracked documents under both excluded prefixes.
    let repository = Repository::new();
    for path in [
        "docs/templates/ISSUE.md",
        "contrib/dev-tools/checks/frontmatter-validator/fixtures/rejected/issue.md",
    ] {
        repository.write(path, WRONG_SCALAR_ISSUE);
        repository.stage(path);
    }

    // Act
    let outcome = repository.validate(arguments);

    // Assert
    assert_eq!(
        (outcome.exit_code, outcome.records.len()),
        (0, 0),
        "records: {:?}",
        outcome.records
    );
}
