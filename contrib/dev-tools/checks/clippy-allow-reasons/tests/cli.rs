use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

#[test]
fn it_should_report_a_changed_allow_without_a_native_reason() {
    let workspace = FixtureRepository::new();
    write_file(
        workspace.path().join("src/lib.rs").as_path(),
        "#[allow(clippy::legacy)]\nfn legacy() {}\n",
    );
    git(workspace.path(), ["add", "src/lib.rs"]);
    git(workspace.path(), ["commit", "--quiet", "-m", "test: establish baseline"]);
    git(workspace.path(), ["switch", "--quiet", "-c", "feature"]);
    write_file(
        workspace.path().join("src/lib.rs").as_path(),
        "#[allow(clippy::legacy)]\nfn legacy() {}\n#[allow(clippy::too_many_lines)]\nfn added() {}\n",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_clippy-allow-reasons"))
        .args(["--base-ref", "develop"])
        .current_dir(workspace.path())
        .output()
        .expect("failed to run clippy-allow-reasons");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    let diagnostic = parse_single_diagnostic(&output.stderr);
    assert_eq!(diagnostic["kind"], "validation_error");
    assert_eq!(diagnostic["file"], "src/lib.rs");
    assert_eq!(diagnostic["line"], 3);
    assert!(diagnostic["message"].as_str().unwrap().contains("require `reason"));
    assert_eq!(diagnostic["exit_code"], 1);
}

#[test]
fn it_should_not_write_output_when_validation_succeeds() {
    let workspace = FixtureRepository::new();
    write_file(
        workspace.path().join("src/lib.rs").as_path(),
        "#[allow(clippy::legacy, reason = \"Legacy baseline.\")]\nfn legacy() {}\n",
    );
    git(workspace.path(), ["add", "src/lib.rs"]);
    git(workspace.path(), ["commit", "--quiet", "-m", "test: establish baseline"]);
    git(workspace.path(), ["switch", "--quiet", "-c", "feature"]);

    let output = Command::new(env!("CARGO_BIN_EXE_clippy-allow-reasons"))
        .args(["--base-ref", "develop"])
        .current_dir(workspace.path())
        .output()
        .expect("failed to run clippy-allow-reasons");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"");
}

#[test]
fn it_should_report_usage_errors_as_ndjson() {
    let directory = FixtureDirectory::new();

    let output = Command::new(env!("CARGO_BIN_EXE_clippy-allow-reasons"))
        .arg("--unexpected")
        .current_dir(directory.path())
        .output()
        .expect("failed to run clippy-allow-reasons");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    let diagnostic = parse_single_diagnostic(&output.stderr);
    assert_eq!(diagnostic["kind"], "usage_error");
    assert_eq!(diagnostic["exit_code"], 2);
}

#[test]
fn it_should_report_runtime_errors_as_ndjson() {
    let workspace = FixtureRepository::new();

    let output = Command::new(env!("CARGO_BIN_EXE_clippy-allow-reasons"))
        .args(["--base-ref", "missing-base-reference"])
        .current_dir(workspace.path())
        .output()
        .expect("failed to run clippy-allow-reasons");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    let diagnostic = parse_single_diagnostic(&output.stderr);
    assert_eq!(diagnostic["kind"], "runtime_error");
    assert_eq!(diagnostic["exit_code"], 1);
}

fn parse_single_diagnostic(stderr: &[u8]) -> Value {
    let lines = std::str::from_utf8(stderr).unwrap().lines().collect::<Vec<_>>();

    assert_eq!(lines.len(), 1);
    serde_json::from_str(lines[0]).unwrap()
}

struct FixtureRepository {
    root: std::path::PathBuf,
}

struct FixtureDirectory {
    path: std::path::PathBuf,
}

impl FixtureDirectory {
    fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("clippy-allow-reasons-no-git-{}-{timestamp}", std::process::id()));

        fs::create_dir_all(&path).expect("failed to create fixture directory");

        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.path));
    }
}

impl FixtureRepository {
    fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is before the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("clippy-allow-reasons-{}-{timestamp}", std::process::id()));

        fs::create_dir_all(root.join("src")).expect("failed to create fixture repository");
        git(root.as_path(), ["init", "--quiet", "--initial-branch=develop"]);
        git(root.as_path(), ["config", "user.email", "tests@example.com"]);
        git(root.as_path(), ["config", "user.name", "Validator tests"]);

        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }
}

impl Drop for FixtureRepository {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}

fn git<const N: usize>(directory: &Path, arguments: [&str; N]) {
    let status = Command::new("git")
        .args(arguments)
        .current_dir(directory)
        .status()
        .expect("failed to run Git");

    assert!(status.success(), "Git command failed");
}

fn write_file(path: &Path, contents: &str) {
    fs::write(path, contents).expect("failed to write fixture source");
}
