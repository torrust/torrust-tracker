//! End-to-end tests of the `shipped-features-check` binary against scratch Cargo workspaces.
//!
//! Real crates cannot get a `testing` feature while the check is introduced, so each test builds a
//! scratch workspace of path-only crates and runs the real binary, which runs the real `cargo tree`.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

/// A scratch workspace with the members `a` (has the `testing` and `internal` features), `c` (a library
/// that depends on `a`) and `b` (the shipped package).
struct Scratch {
    directory: TempDir,
}

impl Scratch {
    fn new(b_manifest_sections: &str, c_dependencies: &str) -> Self {
        let directory = tempfile::tempdir().expect("create scratch directory");
        let root = directory.path();

        write(
            root,
            "Cargo.toml",
            "[workspace]\nmembers = [\"a\", \"b\", \"c\"]\nresolver = \"3\"\n",
        );
        write(
            root,
            "a/Cargo.toml",
            &package("a", "[features]\ntesting = []\ninternal = []\n"),
        );
        write(root, "a/src/lib.rs", "pub fn a() {}\n");
        write(
            root,
            "c/Cargo.toml",
            &package("c", &format!("[dependencies]\n{c_dependencies}")),
        );
        write(root, "c/src/lib.rs", "pub fn c() {}\n");
        write(root, "b/Cargo.toml", &package("b", b_manifest_sections));
        write(root, "b/src/main.rs", "fn main() {}\n");

        Self { directory }
    }

    fn run(&self, arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_shipped-features-check"))
            .args(arguments)
            .current_dir(self.directory.path())
            .output()
            .expect("run shipped-features-check")
    }
}

fn package(name: &str, sections: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n{sections}")
}

fn write(root: &Path, file: &str, content: &str) {
    let path = root.join(file);
    fs::create_dir_all(path.parent().expect("file has a parent")).expect("create directory");
    fs::write(path, content).expect("write scratch file");
}

fn diagnostics(output: &Output) -> Vec<Value> {
    String::from_utf8(output.stderr.clone())
        .expect("stderr is UTF-8")
        .lines()
        .map(|line| serde_json::from_str(line).expect("every stderr line is a JSON object"))
        .collect()
}

fn of_kind<'a>(diagnostics: &'a [Value], kind: &str) -> Vec<&'a Value> {
    diagnostics.iter().filter(|record| record["kind"] == kind).collect()
}

#[test]
fn it_should_fail_when_a_normal_dependency_of_the_shipped_package_enables_the_testing_feature_of_a_workspace_package() {
    // Arrange: `b` depends on `a` and on `c`, and `c` is the only one that enables `a`'s `testing` feature.
    let scratch = Scratch::new(
        "[dependencies]\na = { path = \"../a\" }\nc = { path = \"../c\" }\n",
        "a = { path = \"../a\", features = [\"testing\"] }\n",
    );

    // Act
    let output = scratch.run(&["--package", "b"]);

    // Assert
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    let records = diagnostics(&output);
    let violations = of_kind(&records, "validation_error");
    assert_eq!(violations.len(), 1, "one violation: {records:?}");
    assert_eq!(violations[0]["package"], "a");
    assert_eq!(violations[0]["shipped_package"], "b");
    assert_eq!(violations[0]["feature"], "testing");
    assert_eq!(violations[0]["exit_code"], 1);
    assert!(violations[0]["message"].as_str().unwrap().contains("`testing`"));
}

#[test]
fn it_should_fail_when_the_shipped_package_enables_the_testing_feature_in_a_build_dependency() {
    // Arrange: the feature reaches `b` only through its build script's dependencies.
    let scratch = Scratch::new(
        "[build-dependencies]\na = { path = \"../a\", features = [\"testing\"] }\n",
        "",
    );

    // Act
    let output = scratch.run(&["--package", "b"]);

    // Assert
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(of_kind(&diagnostics(&output), "validation_error").len(), 1);
}

#[test]
fn it_should_pass_when_only_a_dev_dependency_of_the_shipped_package_enables_the_testing_feature() {
    // Arrange: `b` depends on `a` plainly and enables `testing` only for its tests.
    let scratch = Scratch::new(
        "[dependencies]\na = { path = \"../a\" }\n\n[dev-dependencies]\na = { path = \"../a\", features = [\"testing\"] }\n",
        "",
    );

    // Act
    let output = scratch.run(&["--package", "b"]);

    // Assert
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"");
    let records = diagnostics(&output);
    assert_eq!(of_kind(&records, "validation_error").len(), 0);
    let checked = of_kind(&records, "checked");
    assert_eq!(checked.len(), 1, "one record per checked package: {records:?}");
    assert_eq!(checked[0]["shipped_package"], "b");
}

#[test]
fn it_should_check_the_features_that_the_shipped_build_names_explicitly() {
    // Arrange: `b`'s own feature `with-testing` turns on `a`'s `testing`; the default build does not use it.
    let manifest = "[dependencies]\na = { path = \"../a\" }\n\n[features]\nwith-testing = [\"a/testing\"]\n";
    let scratch = Scratch::new(manifest, "");

    // Act
    let default_build = scratch.run(&["--package", "b"]);
    let named_features = scratch.run(&["--package", "b", "--features", "with-testing"]);

    // Assert
    assert_eq!(default_build.status.code(), Some(0));
    assert_eq!(named_features.status.code(), Some(1));
}

#[test]
fn it_should_check_the_feature_named_by_the_forbidden_feature_option() {
    // Arrange: `b` enables `a`'s `internal` feature and not `testing`.
    let scratch = Scratch::new("[dependencies]\na = { path = \"../a\", features = [\"internal\"] }\n", "");

    // Act
    let default_feature = scratch.run(&["--package", "b"]);
    let named_feature = scratch.run(&["--package", "b", "--forbidden-feature", "internal"]);

    // Assert
    assert_eq!(default_feature.status.code(), Some(0));
    assert_eq!(named_feature.status.code(), Some(1));
    assert_eq!(
        of_kind(&diagnostics(&named_feature), "validation_error")[0]["feature"],
        "internal"
    );
}

#[test]
fn it_should_report_a_usage_error_when_no_package_is_named() {
    // Arrange
    let scratch = Scratch::new("", "");

    // Act
    let output = scratch.run(&[]);

    // Assert
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    let records = diagnostics(&output);
    assert_eq!(records[0]["kind"], "usage_error");
    assert_eq!(records[0]["exit_code"], 2);
}

#[test]
fn it_should_report_a_runtime_error_when_cargo_cannot_resolve_the_package() {
    // Arrange: the workspace has no package named `missing`.
    let scratch = Scratch::new("", "");

    // Act
    let output = scratch.run(&["--package", "missing"]);

    // Assert
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(diagnostics(&output)[0]["kind"], "runtime_error");
}
