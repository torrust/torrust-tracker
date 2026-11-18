//! Command-boundary tests: the built binary runs in a disposable git repository.
//!
//! Every git invocation, including the binary's, clears inherited `GIT_*` variables and sets a
//! ceiling directory, so a test run from a git hook can never read or change the real repository.

use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::{Value, json};
use tempfile::TempDir;

/// The replies folder of an open specification, where a reply file belongs.
const FOLDER: &str = "docs/issues/open/2904-slug/agent-replies";

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

/// A valid reply whose round is the given one; the agent is `task-reviewer` and the subject `t3`.
fn reply(round: u8) -> String {
    json!({
        "schema-version": 1,
        "agent": "task-reviewer",
        "model": "claude-opus-5-5",
        "subject": "t3",
        "round": round,
        "created-utc": "2026-11-18 17:25",
        "verdict": "REVIEW_PASSED",
        "summary": "Nothing blocks the task.",
        "findings": [],
        "questions": [],
        "evidence": ["cargo test -q -p agent-reply-validator passed"]
    })
    .to_string()
}

/// The exit code and the NDJSON records on stderr of one run. Stdout is checked to be empty.
struct Outcome {
    exit_code: i32,
    records: Vec<Value>,
}

impl Outcome {
    fn kinds(&self) -> Vec<&str> {
        self.records.iter().map(|record| record["kind"].as_str().unwrap()).collect()
    }

    fn last(&self) -> &Value {
        self.records.last().expect("the run wrote a record")
    }
}

/// A disposable git repository; `TempDir` removes it on drop.
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

    fn stage(&self, path: &str, content: &str) {
        self.write(path, content);
        self.git(&["add", "--", path]);
    }

    /// Stages an empty file whose name is the given bytes, which need not be UTF-8.
    #[cfg(unix)]
    fn stage_raw_name(&self, path: &[u8]) {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;
        let relative = OsStr::from_bytes(path);
        let file = self.root().join(relative);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "").unwrap();
        let status = isolated(Command::new("git"), self.root())
            .arg("add")
            .arg("--")
            .arg(relative)
            .status()
            .unwrap();
        assert!(status.success(), "git add failed");
    }

    fn git(&self, arguments: &[&str]) {
        let status = isolated(Command::new("git"), self.root()).args(arguments).status().unwrap();
        assert!(status.success(), "git {arguments:?} failed");
    }

    fn run(&self, arguments: &[&str]) -> Outcome {
        run_in(self.root(), arguments)
    }
}

fn run_in(directory: &Path, arguments: &[&str]) -> Outcome {
    let output = isolated(Command::new(env!("CARGO_BIN_EXE_agent-reply-validator")), directory)
        .args(arguments)
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&output.stdout), "", "stdout must stay empty");
    let records = std::str::from_utf8(&output.stderr)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap_or_else(|error| panic!("not NDJSON: {line}: {error}")))
        .collect();
    Outcome {
        exit_code: output.status.code().unwrap(),
        records,
    }
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

#[test]
fn it_should_exit_zero_with_only_a_summary_when_every_staged_reply_is_valid() {
    // Arrange
    let repository = Repository::new();
    repository.stage(&format!("{FOLDER}/task-reviewer-t3-r2.json"), &reply(2));
    repository.stage("docs/schemas/other.json", "{}");

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.records, vec![json!({ "kind": "summary", "files": 1, "problems": 0 })]);
}

#[test]
fn it_should_exit_one_with_each_problem_when_a_staged_reply_is_invalid() {
    // Arrange
    let repository = Repository::new();
    let path = format!("{FOLDER}/task-reviewer-t3-r2.json");
    repository.stage(&path, &reply(1));

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(
        outcome.records,
        vec![
            json!({
                "kind": "problem",
                "path": path,
                "check": "agreement",
                "field_path": "round",
                "message": "the file name says `2` but the field says `1`"
            }),
            json!({ "kind": "summary", "files": 1, "problems": 1 }),
        ]
    );
}

#[test]
fn it_should_check_the_staged_content_when_the_working_tree_differs() {
    // Arrange
    let repository = Repository::new();
    let path = format!("{FOLDER}/task-reviewer-t3-r2.json");
    repository.stage(&path, &reply(2));
    repository.write(&path, "not json");

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.last(), &json!({ "kind": "summary", "files": 1, "problems": 0 }));
}

#[test]
fn it_should_report_a_staged_reply_file_when_it_is_outside_a_replies_folder() {
    // Arrange
    let repository = Repository::new();
    repository.stage("docs/issues/open/2904-slug/task-reviewer-t3-r2.json", &reply(2));

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(
        outcome.records,
        vec![
            json!({
                "kind": "problem",
                "path": "docs/issues/open/2904-slug/task-reviewer-t3-r2.json",
                "check": "location",
                "field_path": null,
                "message": "a reply file must be in `docs/issues/<drafts|open|closed>/<specification>/agent-replies/`"
            }),
            json!({ "kind": "summary", "files": 1, "problems": 1 }),
        ]
    );
}

#[test]
fn it_should_report_a_staged_file_when_it_is_in_a_replies_folder_without_a_reply_name() {
    // Arrange
    let repository = Repository::new();
    repository.stage(&format!("{FOLDER}/notes.md"), "# Notes\n");

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["problem", "problem", "summary"]);
    assert_eq!(
        outcome.records[0],
        json!({
            "kind": "problem",
            "path": format!("{FOLDER}/notes.md"),
            "check": "name",
            "field_path": null,
            "message": "`notes.md` does not end in `.json`"
        })
    );
    assert_eq!(
        (&outcome.records[1]["check"], &outcome.records[1]["field_path"]),
        (&json!("content"), &json!("."))
    );
}

#[test]
fn it_should_check_a_given_file_on_disk_when_it_is_not_staged() {
    // Arrange
    let repository = Repository::new();
    let path = format!("{FOLDER}/task-reviewer-t3-r2.json");
    repository.write(&path, &reply(4));

    // Act
    let outcome = repository.run(&[&path]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["problem", "summary"]);
    assert_eq!(outcome.records[0]["path"], path);
    assert_eq!(outcome.records[0]["field_path"], "round");
}

#[test]
fn it_should_name_a_given_file_from_the_repository_root_when_run_in_a_subfolder() {
    // Arrange
    let repository = Repository::new();
    let path = format!("{FOLDER}/task-reviewer-t3-r2.json");
    repository.write(&path, &reply(2));

    // Act
    let outcome = run_in(
        &repository.root().join("docs/issues"),
        &["./open/2904-slug/agent-replies/../agent-replies/task-reviewer-t3-r2.json"],
    );

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.records, vec![json!({ "kind": "summary", "files": 1, "problems": 0 })]);
}

#[test]
fn it_should_fail_with_a_runtime_error_when_a_given_file_does_not_exist() {
    // Arrange
    let repository = Repository::new();

    // Act
    let outcome = repository.run(&["missing.json"]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["runtime_error"]);
    assert_eq!(outcome.records[0]["path"], "missing.json");
}

#[test]
fn it_should_fail_with_a_runtime_error_when_a_given_path_is_a_folder() {
    // Arrange
    let repository = Repository::new();
    repository.write(&format!("{FOLDER}/task-reviewer-t3-r2.json"), &reply(2));

    // Act
    let outcome = repository.run(&[FOLDER]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["runtime_error"]);
    assert_eq!(outcome.records[0]["path"], FOLDER);
}

#[test]
fn it_should_fail_with_a_runtime_error_when_a_given_file_is_outside_the_repository() {
    // Arrange
    let repository = Repository::new();
    let outside = TempDir::new().unwrap();
    let file = outside.path().join("task-reviewer-t3-r2.json");
    fs::write(&file, reply(2)).unwrap();

    // Act
    let outcome = repository.run(&[file.to_str().unwrap()]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["runtime_error"]);
    assert_eq!(outcome.records[0]["message"], "the file is not inside the repository");
}

#[test]
fn it_should_fail_with_a_runtime_error_when_run_outside_a_repository() {
    // Arrange
    let directory = TempDir::new().unwrap();

    // Act
    let outcome = run_in(directory.path(), &["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["runtime_error"]);
}

#[test]
fn it_should_exit_two_with_a_usage_error_when_no_mode_is_given() {
    // Arrange
    let repository = Repository::new();

    // Act
    let outcome = repository.run(&[]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(outcome.kinds(), vec!["usage_error"]);
}

#[test]
fn it_should_exit_two_with_a_usage_error_when_both_modes_are_given() {
    // Arrange
    let repository = Repository::new();

    // Act
    let outcome = repository.run(&["--all", "reply.json"]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(outcome.kinds(), vec!["usage_error"]);
}

#[test]
fn it_should_exit_zero_with_a_help_record_when_help_is_asked() {
    // Arrange
    let repository = Repository::new();

    // Act
    let outcome = repository.run(&["--help"]);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.kinds(), vec!["help"]);
}

#[test]
fn it_should_check_a_given_file_whatever_its_name() {
    // Arrange
    let repository = Repository::new();
    repository.write("notes.md", "# Notes\n");

    // Act
    let outcome = repository.run(&["notes.md"]);

    // Assert: the file is not selected by its name, so all three checks report it.
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["problem", "problem", "problem", "summary"]);
    let checks: Vec<_> = outcome.records.iter().filter_map(|record| record["check"].as_str()).collect();
    assert_eq!(checks, vec!["location", "name", "content"]);
}

#[cfg(unix)]
#[test]
fn it_should_fail_with_a_runtime_error_when_a_given_file_is_a_symbolic_link() {
    // Arrange: the link has a valid reply behind it, under a valid name in a replies folder.
    let repository = Repository::new();
    repository.write("drafts/reply.txt", &reply(2));
    let link = format!("{FOLDER}/task-reviewer-t3-r2.json");
    fs::create_dir_all(repository.root().join(FOLDER)).unwrap();
    std::os::unix::fs::symlink(repository.root().join("drafts/reply.txt"), repository.root().join(&link)).unwrap();

    // Act
    let outcome = repository.run(&[&link]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(
        outcome.records,
        vec![json!({
            "kind": "runtime_error",
            "path": link,
            "message": "the file is a symbolic link; a reply file must be a regular file",
            "exit_code": 1
        })]
    );
}

#[cfg(unix)]
#[test]
fn it_should_skip_a_staged_path_that_is_not_utf8_when_it_is_not_a_candidate() {
    // Arrange
    let repository = Repository::new();
    repository.stage_raw_name(b"docs/notes-\xff.md");

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.records, vec![json!({ "kind": "summary", "files": 0, "problems": 0 })]);
}

#[cfg(unix)]
#[test]
fn it_should_fail_with_a_runtime_error_when_a_staged_candidate_path_is_not_utf8() {
    // Arrange
    let repository = Repository::new();
    repository.stage_raw_name(&[FOLDER.as_bytes(), b"/notes-\xff.md"].concat());

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(outcome.kinds(), vec!["runtime_error"]);
    assert_eq!(outcome.records[0]["message"], "the path is not valid UTF-8");
}

#[test]
fn it_should_fail_with_a_runtime_error_when_a_given_file_is_not_utf8() {
    // Arrange
    let repository = Repository::new();
    let path = format!("{FOLDER}/task-reviewer-t3-r2.json");
    repository.write(&path, "");
    fs::write(repository.root().join(&path), b"\xff\xfe").unwrap();

    // Act
    let outcome = repository.run(&[&path]);

    // Assert: the keys are in the documented order.
    assert_eq!(outcome.exit_code, 1);
    assert_eq!(
        outcome.records,
        vec![json!({ "kind": "runtime_error", "path": path, "message": "the file is not valid UTF-8", "exit_code": 1 })]
    );
    let keys: Vec<_> = outcome.records[0].as_object().unwrap().keys().cloned().collect();
    assert_eq!(keys, ["kind", "path", "message", "exit_code"]);
}

#[test]
fn it_should_succeed_with_an_empty_summary_when_the_index_has_no_reply_file() {
    // Arrange
    let repository = Repository::new();

    // Act
    let outcome = repository.run(&["--all"]);

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.records, vec![json!({ "kind": "summary", "files": 0, "problems": 0 })]);
}

#[test]
fn it_should_read_the_index_that_git_index_file_names_when_it_is_set() {
    // Arrange: the reply is staged only in a second index, as `git commit <path>` prepares one.
    let repository = Repository::new();
    let path = format!("{FOLDER}/task-reviewer-t3-r2.json");
    repository.write(&path, &reply(1));
    let index = repository.root().join(".git/other-index");
    let status = isolated(Command::new("git"), repository.root())
        .env("GIT_INDEX_FILE", &index)
        .args(["add", "--", &path])
        .status()
        .unwrap();
    assert!(status.success());

    // Act
    let output = isolated(Command::new(env!("CARGO_BIN_EXE_agent-reply-validator")), repository.root())
        .env("GIT_INDEX_FILE", &index)
        .arg("--all")
        .output()
        .unwrap();

    // Assert: the reply in the second index disagrees with its name, so the run fails.
    assert_eq!(output.status.code(), Some(1));
    let first: Value = serde_json::from_str(std::str::from_utf8(&output.stderr).unwrap().lines().next().unwrap()).unwrap();
    assert_eq!(
        (&first["check"], &first["field_path"]),
        (&json!("agreement"), &json!("round"))
    );
}

#[test]
fn it_should_write_the_usage_error_keys_in_the_documented_order() {
    // Arrange
    let repository = Repository::new();

    // Act
    let outcome = repository.run(&["--version"]);

    // Assert
    assert_eq!(outcome.exit_code, 2);
    let keys: Vec<_> = outcome.records[0].as_object().unwrap().keys().cloned().collect();
    assert_eq!(keys, ["kind", "message", "exit_code"]);
}
