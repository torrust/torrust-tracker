//! Command-line adapter for package coverage-regression workflow discovery.

use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use package_coverage_regression::{PackageMatrix, WorkspacePackage, directly_changed_packages};
use serde::Deserialize;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let mut stderr = io::stderr().lock();
            drop(writeln!(stderr, "package-coverage-regression: {error}"));
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let arguments = arguments()?;
    let metadata = cargo_metadata()?;
    let changed_paths = changed_paths(&arguments.base_sha, &arguments.head_sha)?;
    let packages = workspace_packages(&metadata)?;
    let matrix = PackageMatrix {
        include: directly_changed_packages(&changed_paths, &packages),
    };

    let matrix = serde_json::to_string(&matrix).map_err(|error| error.to_string())?;
    writeln!(io::stdout().lock(), "{matrix}").map_err(|error| error.to_string())?;
    Ok(())
}

struct Arguments {
    base_sha: String,
    head_sha: String,
}

fn arguments() -> Result<Arguments, String> {
    let mut arguments = env::args().skip(1);
    let base_sha = arguments
        .next()
        .ok_or_else(|| String::from("usage: matrix <base-sha> <head-sha>"))?;
    let head_sha = arguments
        .next()
        .ok_or_else(|| String::from("usage: matrix <base-sha> <head-sha>"))?;
    if arguments.next().is_some() {
        return Err(String::from("usage: matrix <base-sha> <head-sha>"));
    }

    Ok(Arguments { base_sha, head_sha })
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

fn cargo_metadata() -> Result<Metadata, String> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
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

fn changed_paths(base_sha: &str, head_sha: &str) -> Result<Vec<PathBuf>, String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "--find-renames", base_sha, head_sha])
        .output()
        .map_err(|error| format!("failed to run Git diff: {error}"))?;
    if !output.status.success() {
        return Err(format!("Git diff failed: {}", String::from_utf8_lossy(&output.stderr).trim()));
    }

    String::from_utf8(output.stdout)
        .map_err(|error| format!("Git diff output was not valid UTF-8: {error}"))?
        .lines()
        .map(Path::new)
        .map(Path::to_path_buf)
        .collect::<Vec<_>>()
        .pipe(Ok)
}

trait Pipe: Sized {
    fn pipe<T>(self, function: impl FnOnce(Self) -> T) -> T {
        function(self)
    }
}

impl<T> Pipe for T {}
