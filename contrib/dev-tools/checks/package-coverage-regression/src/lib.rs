//! Package ownership discovery for coverage-regression workflow matrices.

use std::collections::{BTreeMap, BTreeSet};
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
    /// Repository-relative package directory.
    pub directory: PathBuf,
}

/// A comparable package with source directories from both revisions.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ComparisonPackage {
    /// Cargo package name.
    pub package: String,
    /// Repository-relative package directory at the pull request base revision.
    pub base_directory: PathBuf,
    /// Repository-relative package directory at the pull request head revision.
    pub head_directory: PathBuf,
}

/// GitHub Actions dynamic-matrix input.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PackageMatrix {
    /// Comparable packages selected for parallel coverage jobs.
    pub include: Vec<ComparisonPackage>,
}

/// A directly changed package that cannot be compared across revisions.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct UnavailablePackage {
    /// Cargo package name.
    pub package: String,
    /// Reason no base/head comparison is available.
    pub outcome: String,
}

/// Base/head package coverage work selected from a pull request diff.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PackageDiscovery {
    /// Packages that exist in both revisions and can be compared.
    pub matrix: PackageMatrix,
    /// Changed packages that exist in only one revision.
    pub unavailable: Vec<UnavailablePackage>,
}

/// Exact source-line coverage counts for one package.
#[derive(Debug, Eq, PartialEq)]
pub struct CoverageSummary {
    /// Number of instrumented source lines that have at least one execution.
    pub covered_lines: u64,
    /// Number of instrumented source lines.
    pub instrumented_lines: u64,
}

/// Exact base/head comparison result for one package.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct CoverageComparison {
    /// Coverage summary measured at the pull request base revision.
    pub base: CoverageSummary,
    /// Coverage summary measured at the pull request head revision.
    pub head: CoverageSummary,
    /// Whether the head lost more than the configured percentage-point tolerance.
    pub warning: bool,
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
            directory: package.directory.clone(),
        })
        .collect::<Vec<_>>();

    changed_packages.sort_by(|left, right| left.package.cmp(&right.package));
    changed_packages.dedup_by(|left, right| left.package == right.package);
    changed_packages
}

/// Selects comparable and unavailable packages from base and head changed paths.
#[must_use]
pub fn discover_coverage_packages(
    base_changed_paths: &[PathBuf],
    base_packages: &[WorkspacePackage],
    head_changed_paths: &[PathBuf],
    head_packages: &[WorkspacePackage],
) -> PackageDiscovery {
    let base_package_directories = package_directories(base_packages);
    let head_package_directories = package_directories(head_packages);
    let changed_package_names = directly_changed_packages(base_changed_paths, base_packages)
        .into_iter()
        .chain(directly_changed_packages(head_changed_paths, head_packages))
        .map(|package| package.package)
        .collect::<BTreeSet<_>>();
    let mut matrix = PackageMatrix { include: Vec::new() };
    let mut unavailable = Vec::new();

    for package in changed_package_names {
        match (base_package_directories.get(&package), head_package_directories.get(&package)) {
            (Some(base_directory), Some(head_directory)) => matrix.include.push(ComparisonPackage {
                package,
                base_directory: base_directory.clone(),
                head_directory: head_directory.clone(),
            }),
            (None, Some(_)) => unavailable.push(UnavailablePackage {
                package,
                outcome: String::from("new package"),
            }),
            (Some(_), None) => unavailable.push(UnavailablePackage {
                package,
                outcome: String::from("removed package"),
            }),
            (None, None) => unreachable!("changed package must be present in at least one workspace revision"),
        }
    }

    PackageDiscovery { matrix, unavailable }
}

fn package_directories(packages: &[WorkspacePackage]) -> BTreeMap<String, PathBuf> {
    packages
        .iter()
        .map(|package| (package.name.clone(), package.directory.clone()))
        .collect()
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

/// Compares exact coverage counts with a percentage-point warning tolerance.
///
/// # Errors
///
/// Returns an error when either summary has no instrumented source lines.
pub fn compare_coverage(
    base: CoverageSummary,
    head: CoverageSummary,
    tolerance_percentage_points: u64,
) -> Result<CoverageComparison, String> {
    if base.instrumented_lines == 0 || head.instrumented_lines == 0 {
        return Err(String::from("cannot compare coverage without instrumented source lines"));
    }

    let base_scaled = u128::from(base.covered_lines) * u128::from(head.instrumented_lines) * 100;
    let head_scaled = u128::from(head.covered_lines) * u128::from(base.instrumented_lines) * 100;
    let tolerance =
        u128::from(tolerance_percentage_points) * u128::from(base.instrumented_lines) * u128::from(head.instrumented_lines);

    Ok(CoverageComparison {
        base,
        head,
        warning: base_scaled > head_scaled + tolerance,
    })
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
                package: String::from("tracker-core"),
                directory: PathBuf::from("packages/tracker-core")
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
                package: String::from("tracker-core"),
                directory: PathBuf::from("packages/tracker-core")
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
                package: String::from("torrust-tracker"),
                directory: PathBuf::from(".")
            }]
        );
    }

    #[test]
    fn it_should_compare_a_moved_package_using_its_base_and_head_directories() {
        // Arrange
        let base_paths = vec![PathBuf::from("packages/old-core/src/lib.rs")];
        let head_paths = vec![PathBuf::from("packages/new-core/src/lib.rs")];
        let base_packages = vec![WorkspacePackage {
            name: String::from("tracker-core"),
            directory: PathBuf::from("packages/old-core"),
        }];
        let head_packages = vec![WorkspacePackage {
            name: String::from("tracker-core"),
            directory: PathBuf::from("packages/new-core"),
        }];

        // Act
        let discovery = discover_coverage_packages(&base_paths, &base_packages, &head_paths, &head_packages);

        // Assert
        assert_eq!(
            discovery.matrix.include,
            vec![ComparisonPackage {
                package: String::from("tracker-core"),
                base_directory: PathBuf::from("packages/old-core"),
                head_directory: PathBuf::from("packages/new-core"),
            }]
        );
        assert!(discovery.unavailable.is_empty());
    }

    #[test]
    fn it_should_report_a_new_package_as_unavailable() {
        // Arrange
        let head_paths = vec![PathBuf::from("packages/new-core/src/lib.rs")];
        let head_packages = vec![package("new-core")];

        // Act
        let discovery = discover_coverage_packages(&[], &[], &head_paths, &head_packages);

        // Assert
        assert!(discovery.matrix.include.is_empty());
        assert_eq!(
            discovery.unavailable,
            vec![UnavailablePackage {
                package: String::from("new-core"),
                outcome: String::from("new package"),
            }]
        );
    }

    #[test]
    fn it_should_report_a_removed_package_as_unavailable() {
        // Arrange
        let base_paths = vec![PathBuf::from("packages/old-core/src/lib.rs")];
        let base_packages = vec![package("old-core")];

        // Act
        let discovery = discover_coverage_packages(&base_paths, &base_packages, &[], &[]);

        // Assert
        assert!(discovery.matrix.include.is_empty());
        assert_eq!(
            discovery.unavailable,
            vec![UnavailablePackage {
                package: String::from("old-core"),
                outcome: String::from("removed package"),
            }]
        );
    }

    #[test]
    fn it_should_exclude_unchanged_packages() {
        // Arrange
        let packages = vec![package("tracker-core")];

        // Act
        let discovery = discover_coverage_packages(&[], &packages, &[], &packages);

        // Assert
        assert!(discovery.matrix.include.is_empty());
        assert!(discovery.unavailable.is_empty());
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

    #[test]
    fn it_should_warn_when_coverage_decreases_by_more_than_five_percentage_points() {
        // Arrange
        let base = CoverageSummary {
            covered_lines: 95,
            instrumented_lines: 100,
        };
        let head = CoverageSummary {
            covered_lines: 89,
            instrumented_lines: 100,
        };

        // Act
        let comparison = compare_coverage(base, head, 5).unwrap();

        // Assert
        assert!(comparison.warning);
    }

    #[test]
    fn it_should_not_warn_for_a_five_percentage_point_decrease() {
        // Arrange
        let base = CoverageSummary {
            covered_lines: 95,
            instrumented_lines: 100,
        };
        let head = CoverageSummary {
            covered_lines: 90,
            instrumented_lines: 100,
        };

        // Act
        let comparison = compare_coverage(base, head, 5).unwrap();

        // Assert
        assert!(!comparison.warning);
    }

    #[test]
    fn it_should_reject_a_comparison_without_instrumented_lines() {
        // Arrange
        let base = CoverageSummary {
            covered_lines: 0,
            instrumented_lines: 0,
        };
        let head = CoverageSummary {
            covered_lines: 1,
            instrumented_lines: 1,
        };

        // Act
        let error = compare_coverage(base, head, 5).unwrap_err();

        // Assert
        assert_eq!(error, "cannot compare coverage without instrumented source lines");
    }
}
