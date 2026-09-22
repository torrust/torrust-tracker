use std::fs;

use package_coverage_check::{CoverageComparison, CoverageSummary, PackageCoverageResult, read_comparison_artifacts};
use tempfile::tempdir;

fn comparison_result(package: &str) -> PackageCoverageResult {
    PackageCoverageResult {
        package: String::from(package),
        comparison: CoverageComparison {
            base: CoverageSummary {
                covered_lines: 95,
                instrumented_lines: 100,
            },
            head: CoverageSummary {
                covered_lines: 89,
                instrumented_lines: 100,
            },
            warning: true,
        },
    }
}

#[test]
fn it_should_load_valid_and_invalid_comparison_artifacts_without_reading_unrelated_files() {
    // Arrange
    let temporary_directory = tempdir().unwrap();
    let directory = temporary_directory.path();
    let missing_directory = directory.join("missing");
    fs::write(
        directory.join("package-coverage-tracker-core.json"),
        serde_json::to_string(&comparison_result("tracker-core")).unwrap(),
    )
    .unwrap();
    fs::write(directory.join("package-coverage-invalid.json"), "invalid JSON").unwrap();
    fs::write(directory.join("unrelated.json"), "invalid JSON").unwrap();

    // Act
    let missing = read_comparison_artifacts(&missing_directory).unwrap();
    let artifacts = read_comparison_artifacts(directory).unwrap();

    // Assert
    assert_eq!(missing, Vec::new());
    assert_eq!(artifacts.len(), 2);
    assert_eq!(artifacts[0].source, "package-coverage-invalid.json");
    assert!(artifacts[0].result.is_err());
    assert_eq!(artifacts[1].source, "package-coverage-tracker-core.json");
    assert_eq!(artifacts[1].result.as_ref().unwrap().package, "tracker-core");
}
