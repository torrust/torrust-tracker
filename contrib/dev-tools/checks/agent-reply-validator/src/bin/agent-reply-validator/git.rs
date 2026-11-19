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

    /// The entries of the index: each file's mode and its repository-relative path, as git
    /// stores them. The path is bytes, because it need not be UTF-8.
    pub fn index_entries(&self) -> Result<Vec<IndexEntry>, String> {
        let output = run_git(&self.root, &["ls-files", "--stage", "-z"])?;
        output
            .split(|byte| *byte == 0)
            .filter(|entry| !entry.is_empty())
            .map(IndexEntry::parse)
            .collect()
    }

    /// The content of a repository-relative path in the index.
    pub fn index_content(&self, path: &str) -> Result<Vec<u8>, String> {
        run_git(&self.root, &["show", &format!(":0:{path}")])
    }
}

/// One entry of `git ls-files --stage -z`: `<mode> <object> <stage>\t<path>`.
#[derive(Debug)]
pub struct IndexEntry {
    pub mode: String,
    pub path: Vec<u8>,
}

impl IndexEntry {
    /// The mode git gives a symbolic link.
    const SYMBOLIC_LINK_MODE: &'static str = "120000";

    fn parse(entry: &[u8]) -> Result<Self, String> {
        let tab = entry
            .iter()
            .position(|byte| *byte == b'\t')
            .ok_or_else(|| "`git ls-files --stage` printed an entry without a tab".to_owned())?;
        let (fields, path) = (&entry[..tab], &entry[tab + 1..]);
        let mode = fields
            .split(|byte| *byte == b' ')
            .next()
            .and_then(|mode| std::str::from_utf8(mode).ok())
            .ok_or_else(|| "`git ls-files --stage` printed an entry without a mode".to_owned())?;
        Ok(Self {
            mode: mode.to_owned(),
            path: path.to_vec(),
        })
    }

    pub fn is_symbolic_link(&self) -> bool {
        self.mode == Self::SYMBOLIC_LINK_MODE
    }
}

/// Variables that can redirect a Git subprocess to a different repository.
///
/// `GIT_INDEX_FILE` is kept: a pre-commit hook receives it for the index `commit --all`/`--only` prepared.
const GIT_REPOSITORY_REDIRECT_ENVIRONMENT: &[&str] = &[
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// `--no-optional-locks` keeps read commands from refreshing the index; `--literal-pathspecs` keeps
/// names from being expanded as globs.
fn run_git(directory: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let mut command = Command::new("git");
    command.args(["--no-optional-locks", "--literal-pathspecs"]);
    command.args(arguments);
    command.current_dir(directory);
    for variable in GIT_REPOSITORY_REDIRECT_ENVIRONMENT {
        command.env_remove(variable);
    }
    let output = command.output().map_err(|error| format!("could not run `git`: {error}"))?;

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
