//! Command-line adapter for package coverage-regression workflow discovery.

use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::{env, fmt, fs};

use package_coverage_check::{
    PackageCoverageResult, PackageMatrix, UnavailablePackage, WorkspacePackage, compare_coverage, discover_coverage_packages,
    read_comparison_artifacts, render_summary, summarize_source_coverage,
};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::json;

/// Why a run failed, which decides the diagnostic record and the exit code.
enum Failure {
    TtyRefusal,
    Usage(String),
    Runtime(String),
}

impl Failure {
    const fn kind(&self) -> &'static str {
        match self {
            Self::TtyRefusal => "tty_refusal",
            Self::Usage(_) => "usage_error",
            Self::Runtime(_) => "runtime_error",
        }
    }

    fn exit_code(&self) -> ExitCode {
        match self {
            Self::TtyRefusal | Self::Usage(_) => ExitCode::from(2),
            Self::Runtime(_) => ExitCode::FAILURE,
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TtyRefusal => formatter.write_str("stdout is a TTY; pipe the output to consume result data"),
            Self::Usage(message) | Self::Runtime(message) => formatter.write_str(message),
        }
    }
}

impl From<String> for Failure {
    fn from(message: String) -> Self {
        Self::Runtime(message)
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            let record = json!({ "kind": failure.kind(), "message": failure.to_string() });
            // A failure to write to stderr cannot be reported anywhere else.
            drop(writeln!(io::stderr().lock(), "{record}"));
            failure.exit_code()
        }
    }
}

fn run() -> Result<(), Failure> {
    if io::stdout().is_terminal() {
        return Err(Failure::TtyRefusal);
    }

    let output = match arguments()? {
        Arguments::Matrix {
            base_workspace_root,
            head_workspace_root,
            base_sha,
            head_sha,
        } => matrix(&base_workspace_root, &head_workspace_root, &base_sha, &head_sha)?,
        Arguments::Compare {
            package,
            base_report,
            base_source_directory,
            head_report,
            head_source_directory,
        } => compare(
            &package,
            &base_report,
            &base_source_directory,
            &head_report,
            &head_source_directory,
        )?,
        Arguments::Summary {
            discovery,
            artifacts_directory,
        } => summary(&discovery, &artifacts_directory)?,
    };
    writeln!(io::stdout().lock(), "{output}")
        .map_err(|error| Failure::Runtime(format!("cannot write the result to stdout: {error}")))
}

enum Arguments {
    Matrix {
        base_workspace_root: PathBuf,
        head_workspace_root: PathBuf,
        base_sha: String,
        head_sha: String,
    },
    Compare {
        package: String,
        base_report: PathBuf,
        base_source_directory: PathBuf,
        head_report: PathBuf,
        head_source_directory: PathBuf,
    },
    Summary {
        discovery: PathBuf,
        artifacts_directory: PathBuf,
    },
}

fn arguments() -> Result<Arguments, Failure> {
    let mut arguments = env::args().skip(1);
    match arguments.next().as_deref() {
        Some("matrix") => {
            let base_workspace_root = arguments.next().map(PathBuf::from).ok_or_else(usage_error)?;
            let head_workspace_root = arguments.next().map(PathBuf::from).ok_or_else(usage_error)?;
            let base_sha = arguments.next().ok_or_else(usage_error)?;
            let head_sha = arguments.next().ok_or_else(usage_error)?;
            if arguments.next().is_some() {
                return Err(usage_error());
            }
            Ok(Arguments::Matrix {
                base_workspace_root,
                head_workspace_root,
                base_sha,
                head_sha,
            })
        }
        Some("compare") => {
            let package = arguments.next().ok_or_else(usage_error)?;
            let paths = (0..4)
                .map(|_| arguments.next().map(PathBuf::from).ok_or_else(usage_error))
                .collect::<Result<Vec<_>, _>>()?;
            if arguments.next().is_some() {
                return Err(usage_error());
            }
            Ok(Arguments::Compare {
                package,
                base_report: paths[0].clone(),
                base_source_directory: paths[1].clone(),
                head_report: paths[2].clone(),
                head_source_directory: paths[3].clone(),
            })
        }
        Some("summary") => {
            let discovery = arguments.next().map(PathBuf::from).ok_or_else(usage_error)?;
            let artifacts_directory = arguments.next().map(PathBuf::from).ok_or_else(usage_error)?;
            if arguments.next().is_some() {
                return Err(usage_error());
            }
            Ok(Arguments::Summary {
                discovery,
                artifacts_directory,
            })
        }
        _ => Err(usage_error()),
    }
}

fn usage_error() -> Failure {
    Failure::Usage(String::from(
        "usage: <matrix <base-workspace> <head-workspace> <base-sha> <head-sha> | compare <package> <base-report> <base-src> <head-report> <head-src> | summary <discovery-json> <artifacts-directory>>",
    ))
}

fn matrix(base_workspace_root: &Path, head_workspace_root: &Path, base_sha: &str, head_sha: &str) -> Result<String, String> {
    let base_packages = workspace_packages(&cargo_metadata(base_workspace_root)?)?;
    let head_packages = workspace_packages(&cargo_metadata(head_workspace_root)?)?;
    let changed_paths = changed_paths(head_workspace_root, base_sha, head_sha)?;
    serde_json::to_string(&discover_coverage_packages(
        &changed_paths.base,
        &base_packages,
        &changed_paths.head,
        &head_packages,
    ))
    .map_err(|error| error.to_string())
}

fn compare(
    package: &str,
    base_report: &Path,
    base_source_directory: &Path,
    head_report: &Path,
    head_source_directory: &Path,
) -> Result<String, String> {
    let base = summarize_source_coverage(&read_json(base_report)?, base_source_directory)?;
    let head = summarize_source_coverage(&read_json(head_report)?, head_source_directory)?;
    let result = PackageCoverageResult {
        package: String::from(package),
        comparison: compare_coverage(base, head, 5)?,
    };

    serde_json::to_string(&result).map_err(|error| error.to_string())
}

fn summary(discovery: &Path, artifacts_directory: &Path) -> Result<String, String> {
    let discovery = read_json::<SummaryInput>(discovery)?;
    let artifacts = read_comparison_artifacts(artifacts_directory)?;

    let markdown = render_summary(&discovery.matrix, &discovery.unavailable, &artifacts);
    serde_json::to_string(&json!({ "markdown": markdown })).map_err(|error| error.to_string())
}

#[derive(Deserialize)]
struct SummaryInput {
    matrix: PackageMatrix,
    unavailable: Vec<UnavailablePackage>,
}

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let source = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_str(&source).map_err(|error| format!("{}: invalid JSON: {error}", path.display()))
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<MetadataPackage>,
    workspace_members: Vec<String>,
    workspace_root: PathBuf,
}

#[derive(Deserialize)]
struct MetadataPackage {
    id: String,
    manifest_path: PathBuf,
    name: String,
}

fn cargo_metadata(workspace_root: &Path) -> Result<Metadata, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("failed to run cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|error| format!("failed to parse cargo metadata: {error}"))
}

fn workspace_packages(metadata: &Metadata) -> Result<Vec<WorkspacePackage>, String> {
    metadata
        .packages
        .iter()
        .filter(|package| metadata.workspace_members.contains(&package.id))
        .map(|package| {
            let directory = package
                .manifest_path
                .parent()
                .ok_or_else(|| format!("package manifest has no parent: {}", package.manifest_path.display()))?
                .strip_prefix(&metadata.workspace_root)
                .map_err(|error| format!("package manifest is outside the workspace: {error}"))?;
            Ok(WorkspacePackage {
                name: package.name.clone(),
                directory: if directory.as_os_str().is_empty() {
                    PathBuf::from(".")
                } else {
                    directory.to_path_buf()
                },
            })
        })
        .collect()
}

struct ChangedPaths {
    base: Vec<PathBuf>,
    head: Vec<PathBuf>,
}

fn changed_paths(workspace_root: &Path, base_sha: &str, head_sha: &str) -> Result<ChangedPaths, String> {
    let output = Command::new("git")
        .args(["diff", "--name-status", "--find-renames", base_sha, head_sha])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("failed to run Git diff: {error}"))?;
    if !output.status.success() {
        return Err(format!("Git diff failed: {}", String::from_utf8_lossy(&output.stderr).trim()));
    }

    let output = String::from_utf8(output.stdout)
        .map_err(|error| format!("Git diff output was not valid UTF-8: {error}"))?
        .lines()
        .map(parse_changed_path)
        .collect::<Result<Vec<_>, _>>()?;
    let mut base = Vec::new();
    let mut head = Vec::new();

    for paths in output {
        base.extend(paths.base);
        head.extend(paths.head);
    }

    Ok(ChangedPaths { base, head })
}

struct ChangedPath {
    base: Vec<PathBuf>,
    head: Vec<PathBuf>,
}

fn parse_changed_path(line: &str) -> Result<ChangedPath, String> {
    let mut fields = line.split('\t');
    let status = fields.next().ok_or_else(|| String::from("expected Git diff status"))?;
    let first_path = fields.next().ok_or_else(|| format!("expected Git diff path in `{line}`"))?;

    match status.as_bytes().first() {
        Some(b'R' | b'C') => {
            let second_path = fields
                .next()
                .ok_or_else(|| format!("expected renamed Git diff path in `{line}`"))?;
            Ok(ChangedPath {
                base: vec![PathBuf::from(first_path)],
                head: vec![PathBuf::from(second_path)],
            })
        }
        Some(b'A') => Ok(ChangedPath {
            base: Vec::new(),
            head: vec![PathBuf::from(first_path)],
        }),
        Some(b'D') => Ok(ChangedPath {
            base: vec![PathBuf::from(first_path)],
            head: Vec::new(),
        }),
        Some(_) => Ok(ChangedPath {
            base: vec![PathBuf::from(first_path)],
            head: vec![PathBuf::from(first_path)],
        }),
        None => Err(format!("expected Git diff status in `{line}`")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_map_each_failure_to_its_record_kind_and_exit_code() {
        // Arrange
        let cases = [
            (Failure::TtyRefusal, "tty_refusal", ExitCode::from(2)),
            (Failure::Usage(String::new()), "usage_error", ExitCode::from(2)),
            (Failure::Runtime(String::new()), "runtime_error", ExitCode::FAILURE),
        ];

        // Act and assert
        for (failure, kind, exit_code) in cases {
            assert_eq!(failure.kind(), kind);
            assert_eq!(failure.exit_code(), exit_code);
        }
    }

    #[test]
    fn it_should_preserve_old_and_new_paths_for_a_renamed_file() {
        // Arrange
        let status = "R100\tpackages/old-core/src/lib.rs\tpackages/new-core/src/lib.rs";

        // Act
        let paths = parse_changed_path(status).unwrap();

        // Assert
        assert_eq!(paths.base, vec![PathBuf::from("packages/old-core/src/lib.rs")]);
        assert_eq!(paths.head, vec![PathBuf::from("packages/new-core/src/lib.rs")]);
    }

    #[test]
    fn it_should_classify_a_deleted_file_as_a_base_only_path() {
        // Arrange
        let status = "D\tpackages/tracker-core/src/obsolete.rs";

        // Act
        let paths = parse_changed_path(status).unwrap();

        // Assert
        assert_eq!(paths.base, vec![PathBuf::from("packages/tracker-core/src/obsolete.rs")]);
        assert_eq!(paths.head, Vec::<PathBuf>::new());
    }
}
