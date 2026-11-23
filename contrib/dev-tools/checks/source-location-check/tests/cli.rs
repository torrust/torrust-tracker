//! End-to-end tests of the `source-location-check` binary against scratch Git repositories.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

const CALLER: &str = concat!("Location::", "caller()");

/// A `git` command that ignores the repository a Git hook runs in (`GIT_DIR`, `GIT_INDEX_FILE`, ...).
fn git() -> Command {
    let mut command = Command::new("git");
    for (name, _) in std::env::vars_os().filter(|(name, _)| name.to_string_lossy().starts_with("GIT_")) {
        command.env_remove(name);
    }
    command
}

fn scratch(files: &[(&str, &str)]) -> TempDir {
    let directory = tempfile::tempdir().expect("create scratch directory");
    let root = directory.path();
    assert!(git().arg("init").arg("--quiet").current_dir(root).status().unwrap().success());
    for (name, content) in files {
        write(root, name, content);
    }
    directory
}

fn write(root: &Path, name: &str, content: &str) {
    let path = root.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn run(directory: &TempDir) -> Output {
    binary()
        .args(["--allow-list", "allow-list.txt"])
        .current_dir(directory.path())
        .output()
        .expect("run source-location-check")
}

/// The binary under test, with the `GIT_*` variables of a running hook removed (it runs `git ls-files`).
fn binary() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_source-location-check"));
    for (name, _) in std::env::vars_os().filter(|(name, _)| name.to_string_lossy().starts_with("GIT_")) {
        command.env_remove(name);
    }
    command
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn it_should_fail_and_name_the_file_and_line_when_production_code_records_a_location() {
    let directory = scratch(&[
        ("allow-list.txt", "# empty\n"),
        ("src/lib.rs", &format!("pub fn f() {{\n    let _ = {CALLER};\n}}\n")),
    ]);

    let output = run(&directory);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("\"file\":\"src/lib.rs\",\"line\":2"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn it_should_pass_when_locations_appear_only_in_test_code() {
    let directory = scratch(&[
        ("allow-list.txt", ""),
        ("src/lib.rs", "pub fn f() {}\n#[cfg(test)]\nmod helpers;\n"),
        ("src/helpers.rs", &format!("pub fn h() {{ {CALLER}; }}\n")),
        ("tests/it.rs", &format!("fn t() {{ {CALLER}; }}\n")),
    ]);

    let output = run(&directory);

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
}

#[test]
fn it_should_pass_for_a_file_on_the_allow_list_and_fail_for_an_entry_without_reason() {
    let files = |allow: &'static str| {
        scratch(&[
            ("allow-list.txt", allow),
            ("src/lib.rs", &format!("fn f() {{ {CALLER}; }}\n")),
        ])
    };

    assert_eq!(run(&files("src/lib.rs: kept until #1\n")).status.code(), Some(0));
    assert_eq!(run(&files("src/lib.rs\n")).status.code(), Some(1));
}

#[test]
fn it_should_exit_two_on_an_unknown_argument() {
    let directory = scratch(&[]);

    let output = binary().arg("--bogus").current_dir(directory.path()).output().unwrap();

    assert_eq!(output.status.code(), Some(2));
}
