use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

#[test]
fn it_should_report_a_changed_allow_without_a_native_reason() {
    let workspace = FixtureRepository::new();
    workspace.establish_baseline();
    workspace.add_undocumented_allow();

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
fn it_should_detect_an_undocumented_allow_despite_local_git_diff_configuration() {
    let workspace = FixtureRepository::new();
    workspace.establish_baseline();
    workspace.add_undocumented_allow();

    for configuration in [
        ["diff.noprefix", "true"],
        ["diff.mnemonicPrefix", "true"],
        ["color.diff", "always"],
        ["diff.external", "false"],
    ] {
        git(workspace.path(), ["config", configuration[0], configuration[1]]);
        let output = run_validator(workspace.path(), &["--base-ref", "develop"]);

        assert_eq!(output.status.code(), Some(1));
        assert_eq!(parse_single_diagnostic(&output.stderr)["kind"], "validation_error");
    }
}

#[test]
fn it_should_not_write_output_when_validation_succeeds() {
    let workspace = FixtureRepository::new();
    workspace.establish_documented_baseline();
    workspace.add_documented_allow();

    let output = run_validator(workspace.path(), &["--base-ref", "develop"]);

    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"");
}

#[test]
fn it_should_accept_supported_documented_attribute_shapes() {
    let workspace = FixtureRepository::new();
    workspace.establish_empty_baseline();
    write_file(
        workspace.path().join("src/lib.rs").as_path(),
        "#![allow(\n    clippy::module_name_repetitions,\n    reason = \"The generated compatibility module is intentionally named.\"\n)]\n\nstruct Example;\n\nimpl Example {\n    #[allow(clippy::too_many_lines, reason = \"The generated method mirrors the protocol.\")]\n    fn method(&self) {}\n}\n\nfn statement() {\n    #[allow(clippy::let_and_return, reason = \"The binding keeps the example readable.\")]\n    let value = 1;\n    let _ = value;\n}\n",
    );

    let output = run_validator(workspace.path(), &["--base-ref", "develop"]);

    assert!(output.status.success());
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"");
}

#[test]
fn it_should_report_usage_errors_as_ndjson() {
    let directory = FixtureDirectory::new();

    let output = run_validator(directory.path(), &["--unexpected"]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    let diagnostic = parse_single_diagnostic(&output.stderr);
    assert_eq!(diagnostic["kind"], "usage_error");
    assert_eq!(diagnostic["exit_code"], 2);
}

#[test]
fn it_should_report_runtime_errors_as_ndjson() {
    let workspace = FixtureRepository::new();

    let output = run_validator(workspace.path(), &["--base-ref", "missing-base-reference"]);

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

fn run_validator(directory: &Path, arguments: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_clippy-allow-reasons"))
        .args(arguments)
        .current_dir(directory)
        .output()
        .expect("failed to run clippy-allow-reasons")
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

    fn establish_baseline(&self) {
        write_file(
            self.path().join("src/lib.rs").as_path(),
            "#[allow(clippy::legacy)]\nfn legacy() {}\n",
        );
        git(self.path(), ["add", "src/lib.rs"]);
        git(
            self.path(),
            [
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "--quiet",
                "-m",
                "test: establish baseline",
            ],
        );
        git(self.path(), ["switch", "--quiet", "-c", "feature"]);
    }

    fn establish_documented_baseline(&self) {
        write_file(
            self.path().join("src/lib.rs").as_path(),
            "#[allow(clippy::legacy, reason = \"Legacy baseline.\")]\nfn legacy() {}\n",
        );
        git(self.path(), ["add", "src/lib.rs"]);
        git(
            self.path(),
            [
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "--quiet",
                "-m",
                "test: establish documented baseline",
            ],
        );
        git(self.path(), ["switch", "--quiet", "-c", "feature"]);
    }

    fn establish_empty_baseline(&self) {
        write_file(self.path().join("src/lib.rs").as_path(), "");
        git(self.path(), ["add", "src/lib.rs"]);
        git(
            self.path(),
            [
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "--quiet",
                "-m",
                "test: establish empty baseline",
            ],
        );
        git(self.path(), ["switch", "--quiet", "-c", "feature"]);
    }

    fn add_undocumented_allow(&self) {
        write_file(
            self.path().join("src/lib.rs").as_path(),
            "#[allow(clippy::legacy)]\nfn legacy() {}\n#[allow(clippy::too_many_lines)]\nfn added() {}\n",
        );
    }

    fn add_documented_allow(&self) {
        write_file(
            self.path().join("src/lib.rs").as_path(),
            "#[allow(clippy::legacy, reason = \"Legacy baseline.\")]\nfn legacy() {}\n#[allow(clippy::too_many_lines, reason = \"The generated fixture is intentionally verbose.\")]\nfn added() {}\n",
        );
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
