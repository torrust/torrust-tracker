//! Read-only access to the repository that contains the working directory.

use std::path::{Path, PathBuf};
use std::process::Command;

/// A git repository, addressed by its top-level directory.
#[derive(Debug)]
pub struct Git {
    root: PathBuf,
}

impl Git {
    /// Finds the repository that contains `working_directory`.
    pub fn discover(working_directory: &Path) -> Result<Self, String> {
        let output = run_git(working_directory, &["rev-parse", "--show-toplevel"])?;
        let root = String::from_utf8(output).map_err(|error| format!("`git rev-parse` output was not UTF-8: {error}"))?;

        Ok(Self {
            root: PathBuf::from(root.trim_end_matches('\n')),
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Repository-relative paths of tracked files, optionally limited to one directory.
    pub fn tracked_files(&self, directory: Option<&str>) -> Result<Vec<String>, String> {
        let mut arguments = vec!["ls-files", "-z"];
        if let Some(directory) = directory {
            arguments.extend(["--", directory]);
        }
        null_separated(run_git(&self.root, &arguments)?)
    }

    /// Repository-relative paths of added, copied, modified, and renamed staged files.
    pub fn staged_files(&self) -> Result<Vec<String>, String> {
        null_separated(run_git(
            &self.root,
            &["diff", "--cached", "--name-only", "-z", "--diff-filter=ACMR"],
        )?)
    }

    /// The staged content of a repository-relative path.
    pub fn index_content(&self, path: &str) -> Result<Vec<u8>, String> {
        run_git(&self.root, &["show", &format!(":{path}")])
    }
}

/// `--no-optional-locks` keeps read commands from refreshing the index; `--literal-pathspecs` keeps
/// directory names from being expanded as globs, with or without `GIT_LITERAL_PATHSPECS`.
fn run_git(directory: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .args(["--no-optional-locks", "--literal-pathspecs"])
        .args(arguments)
        .current_dir(directory)
        .output()
        .map_err(|error| format!("could not run `git`: {error}"))?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "`git {}` failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn null_separated(output: Vec<u8>) -> Result<Vec<String>, String> {
    String::from_utf8(output)
        .map_err(|error| format!("git reported a path that is not UTF-8: {error}"))
        .map(|paths| paths.split('\0').filter(|path| !path.is_empty()).map(str::to_owned).collect())
}
