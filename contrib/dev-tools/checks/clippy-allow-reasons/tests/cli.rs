use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

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
    assert!(String::from_utf8_lossy(&output.stderr).contains("src/lib.rs:3: Clippy allow attributes require"));
}

struct FixtureRepository {
    root: std::path::PathBuf,
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
