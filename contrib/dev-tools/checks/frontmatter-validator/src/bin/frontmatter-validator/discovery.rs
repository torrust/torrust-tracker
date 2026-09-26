//! Selects the Markdown documents each invocation mode validates.

use std::fs;
use std::path::{Component, Path, PathBuf};

use frontmatter_validator::repository::RepositoryFiles;

use crate::git::Git;

/// Repository-relative prefixes skipped silently in every mode.
const EXCLUDED_PREFIXES: &[&str] = &["docs/templates/", "contrib/dev-tools/checks/frontmatter-validator/fixtures/"];

/// Which documents to validate.
#[derive(Debug)]
pub enum Mode {
    /// Explicit files, and tracked Markdown under explicit directories.
    Paths(Vec<PathBuf>),
    /// The staged content of staged Markdown files.
    Staged,
    /// Every tracked Markdown file in the working tree.
    All,
}

/// Why discovery could not produce a document list.
#[derive(Debug)]
pub enum DiscoveryError {
    /// The invocation named a path that cannot be validated.
    Usage(String),
    /// Git or the filesystem failed.
    Runtime(String),
}

/// A document selected for validation.
#[derive(Debug)]
pub struct Document {
    /// The repository-relative path with `/` separators.
    pub path: String,
    source: Source,
}

#[derive(Debug)]
enum Source {
    WorkingTree(PathBuf),
    Index,
}

impl Document {
    /// Reads the document from the working tree or, in staged mode, from the index.
    pub fn read(&self, git: &Git) -> Result<String, String> {
        let bytes = match &self.source {
            Source::WorkingTree(file) => fs::read(file).map_err(|error| format!("could not read the file: {error}"))?,
            Source::Index => git.index_content(&self.path)?,
        };
        String::from_utf8(bytes).map_err(|error| format!("the file is not UTF-8 text: {error}"))
    }
}

/// Returns the selected documents sorted by path, without duplicates or excluded paths.
pub fn discover(mode: Mode, git: &Git, working_directory: &Path) -> Result<Vec<Document>, DiscoveryError> {
    let mut documents = match mode {
        Mode::Paths(paths) => explicit_documents(&paths, git, working_directory)?,
        Mode::Staged => markdown(git.staged_files().map_err(DiscoveryError::Runtime)?)
            .map(|path| Document {
                path,
                source: Source::Index,
            })
            .collect(),
        Mode::All => tracked_markdown(git, None)?,
    };

    documents.retain(|document| !is_excluded(&document.path));
    documents.sort_by(|left, right| left.path.cmp(&right.path));
    documents.dedup_by(|left, right| left.path == right.path);
    Ok(documents)
}

/// The files references resolve against: index entries in staged mode, otherwise tracked files
/// that exist in the working tree.
pub fn repository_files(staged: bool, git: &Git) -> Result<RepositoryFiles, String> {
    let tracked = git.tracked_files(None)?;
    Ok(RepositoryFiles::new(
        tracked.into_iter().filter(|path| staged || git.root().join(path).is_file()),
    ))
}

fn explicit_documents(paths: &[PathBuf], git: &Git, working_directory: &Path) -> Result<Vec<Document>, DiscoveryError> {
    let root = git
        .root()
        .canonicalize()
        .map_err(|error| DiscoveryError::Runtime(format!("could not resolve the repository root: {error}")))?;

    let mut documents = Vec::new();
    for path in paths {
        let absolute = working_directory
            .join(path)
            .canonicalize()
            .map_err(|_| DiscoveryError::Usage(format!("path `{}` does not exist", path.display())))?;
        let relative = absolute
            .strip_prefix(&root)
            .map_err(|_| DiscoveryError::Usage(format!("path `{}` is outside the repository", path.display())))
            .map(repository_path)?;

        if absolute.is_dir() {
            let directory = (!relative.is_empty()).then_some(relative.as_str());
            documents.extend(tracked_markdown(git, directory)?);
        } else {
            documents.push(Document {
                path: relative,
                source: Source::WorkingTree(absolute),
            });
        }
    }
    Ok(documents)
}

/// Tracked Markdown whose working-tree file still exists; an unstaged deletion has nothing to check.
fn tracked_markdown(git: &Git, directory: Option<&str>) -> Result<Vec<Document>, DiscoveryError> {
    let tracked = git.tracked_files(directory).map_err(DiscoveryError::Runtime)?;
    Ok(markdown(tracked)
        .filter_map(|path| {
            let file = git.root().join(&path);
            file.is_file().then_some(Document {
                path,
                source: Source::WorkingTree(file),
            })
        })
        .collect())
}

fn markdown(paths: Vec<String>) -> impl Iterator<Item = String> {
    paths.into_iter().filter(|path| {
        Path::new(path)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
    })
}

fn is_excluded(path: &str) -> bool {
    EXCLUDED_PREFIXES.iter().any(|prefix| path.starts_with(prefix))
}

fn repository_path(relative: &Path) -> String {
    relative
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}
