//! Package ownership discovery for coverage-regression workflow matrices.

use std::path::{Path, PathBuf};

use serde::{Serialize, Serializer};
use serde_json::Value;

/// A workspace package that can own changed source files.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorkspacePackage {
    /// Cargo package name passed to package-scoped coverage commands.
    pub name: String,
    /// Repository-relative package directory.
    pub directory: PathBuf,
}

/// A package entry consumed by the GitHub Actions dynamic matrix.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MatrixPackage {
    /// Cargo package name.
    pub package: String,
}

/// GitHub Actions dynamic-matrix input.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PackageMatrix {
    /// Comparable packages selected for parallel coverage jobs.
    pub include: Vec<MatrixPackage>,
}

/// Exact source-line coverage counts for one package.
#[derive(Debug, Eq, PartialEq)]
pub struct CoverageSummary {
    /// Number of instrumented source lines that have at least one execution.
    pub covered_lines: u64,
    /// Number of instrumented source lines.
    pub instrumented_lines: u64,
}

impl Serialize for CoverageSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct SerializableCoverageSummary {
            covered_lines: u64,
            instrumented_lines: u64,
        }

        SerializableCoverageSummary {
            covered_lines: self.covered_lines,
            instrumented_lines: self.instrumented_lines,
        }
        .serialize(serializer)
    }
}

/// Selects workspace packages that directly own at least one changed path.
#[must_use]
pub fn directly_changed_packages(changed_paths: &[PathBuf], packages: &[WorkspacePackage]) -> Vec<MatrixPackage> {
    let mut changed_packages = changed_paths
        .iter()
        .filter_map(|changed_path| owning_package(changed_path, packages))
        .map(|package| MatrixPackage {
            package: package.name.clone(),
        })
        .collect::<Vec<_>>();

    changed_packages.sort_by(|left, right| left.package.cmp(&right.package));
    changed_packages.dedup_by(|left, right| left.package == right.package);
    changed_packages
}

fn owning_package<'a>(changed_path: &Path, packages: &'a [WorkspacePackage]) -> Option<&'a WorkspacePackage> {
    packages
        .iter()
        .filter(|package| package_owns_path(package, changed_path))
        .max_by_key(|package| package.directory.components().count())
}

fn package_owns_path(package: &WorkspacePackage, changed_path: &Path) -> bool {
    if package.directory == Path::new(".") {
        return changed_path.starts_with("src") || changed_path.starts_with("tests");
    }

    changed_path.starts_with(&package.directory)
}

/// Counts instrumented and covered lines below `source_directory` in a Codecov JSON report.
///
/// # Errors
///
/// Returns an error when the report does not follow the expected Codecov JSON format.
pub fn summarize_source_coverage(report: &Value, source_directory: &Path) -> Result<CoverageSummary, String> {
    let coverage = report
        .get("coverage")
        .and_then(Value::as_object)
        .ok_or_else(|| String::from("expected Codecov report field `coverage` to be an object"))?;
    let mut summary = CoverageSummary {
        covered_lines: 0,
        instrumented_lines: 0,
    };

    for (file_name, lines) in coverage {
        if !Path::new(file_name).starts_with(source_directory) {
            continue;
        }
        let lines = lines
            .as_object()
            .ok_or_else(|| format!("expected coverage lines for `{file_name}` to be an object"))?;

        for coverage in lines.values() {
            let hit_count = parse_hit_count(coverage)?;
            summary.instrumented_lines += 1;
            summary.covered_lines += u64::from(hit_count != 0);
        }
    }

    Ok(summary)
}

fn parse_hit_count(coverage: &Value) -> Result<u64, String> {
    let coverage = coverage
        .as_str()
        .ok_or_else(|| String::from("expected Codecov line coverage value to be a string"))?;
    let (hit_count, _) = coverage
        .split_once('/')
        .ok_or_else(|| format!("expected Codecov line coverage value `{coverage}` to contain `/`"))?;

    hit_count
        .parse::<u64>()
        .map_err(|error| format!("failed to parse Codecov line hit count `{hit_count}`: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(name: &str) -> WorkspacePackage {
        WorkspacePackage {
            name: String::from(name),
            directory: PathBuf::from(format!("packages/{name}")),
        }
    }

    fn root_package() -> WorkspacePackage {
        WorkspacePackage {
            name: String::from("torrust-tracker"),
            directory: PathBuf::from("."),
        }
    }

    #[test]
    fn it_should_select_the_package_owning_a_changed_source_file() {
        // Arrange
        let changed_paths = vec![PathBuf::from("packages/tracker-core/src/lib.rs")];
        let packages = vec![package("tracker-core"), package("udp-core")];

        // Act
        let changed_packages = directly_changed_packages(&changed_paths, &packages);

        // Assert
        assert_eq!(
            changed_packages,
            vec![MatrixPackage {
                package: String::from("tracker-core")
            }]
        );
    }

    #[test]
    fn it_should_not_select_a_package_for_a_workspace_file_change() {
        // Arrange
        let changed_paths = vec![PathBuf::from("Cargo.toml")];
        let packages = vec![root_package(), package("tracker-core")];

        // Act
        let changed_packages = directly_changed_packages(&changed_paths, &packages);

        // Assert
        assert!(changed_packages.is_empty());
    }

    #[test]
    fn it_should_select_the_most_specific_package_for_a_nested_package_change() {
        // Arrange
        let changed_paths = vec![PathBuf::from("packages/tracker-core/Cargo.toml")];
        let packages = vec![root_package(), package("tracker-core")];

        // Act
        let changed_packages = directly_changed_packages(&changed_paths, &packages);

        // Assert
        assert_eq!(
            changed_packages,
            vec![MatrixPackage {
                package: String::from("tracker-core")
            }]
        );
    }

    #[test]
    fn it_should_select_the_root_package_for_a_root_source_change() {
        // Arrange
        let changed_paths = vec![PathBuf::from("src/lib.rs")];
        let packages = vec![root_package(), package("tracker-core")];

        // Act
        let changed_packages = directly_changed_packages(&changed_paths, &packages);

        // Assert
        assert_eq!(
            changed_packages,
            vec![MatrixPackage {
                package: String::from("torrust-tracker")
            }]
        );
    }

    #[test]
    fn it_should_count_exact_coverage_for_package_source_files_only() {
        // Arrange
        let report = serde_json::json!({
            "coverage": {
                "/workspace/packages/tracker-core/src/lib.rs": { "10": "0/1", "11": "3/4" },
                "/workspace/packages/tracker-core/tests/integration.rs": { "5": "1/1" },
                "/workspace/packages/udp-core/src/lib.rs": { "7": "1/1" }
            }
        });

        // Act
        let summary = summarize_source_coverage(&report, Path::new("/workspace/packages/tracker-core/src")).unwrap();

        // Assert
        assert_eq!(
            summary,
            CoverageSummary {
                covered_lines: 1,
                instrumented_lines: 2
            }
        );
    }

    #[test]
    fn it_should_reject_a_malformed_codecov_line_coverage_value() {
        // Arrange
        let report = serde_json::json!({
            "coverage": { "/workspace/packages/tracker-core/src/lib.rs": { "10": "invalid" } }
        });

        // Act
        let error = summarize_source_coverage(&report, Path::new("/workspace/packages/tracker-core/src")).unwrap_err();

        // Assert
        assert_eq!(error, "expected Codecov line coverage value `invalid` to contain `/`");
    }
}
