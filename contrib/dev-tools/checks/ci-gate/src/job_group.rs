//! The job groups that the pull-request workflows run.

use serde::Serialize;

/// A group of CI jobs, one per pull-request workflow.
///
/// The variants and their order mirror the workflows under `.github/workflows/`. Change detection
/// ([`crate::change_detection`]) decides which groups a change needs.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum JobGroup {
    /// `docs-lint.yaml`: runs on every pull request.
    DocsLint,
    /// `testing.yaml`.
    Testing,
    /// `os-compatibility.yaml`.
    OsCompatibility,
    /// `generate_coverage_pr.yaml`.
    Coverage,
    /// `container.yaml`.
    Container,
    /// `db-compatibility.yaml`.
    DbCompatibility,
    /// `db-benchmarking.yaml`.
    DbBenchmarking,
    /// `security-scan.yaml`.
    SecurityScan,
}

impl JobGroup {
    /// Every group, in the order the output lists them.
    pub const ALL: [Self; 8] = [
        Self::DocsLint,
        Self::Testing,
        Self::OsCompatibility,
        Self::Coverage,
        Self::Container,
        Self::DbCompatibility,
        Self::DbBenchmarking,
        Self::SecurityScan,
    ];

    /// The workflow file that runs this group, relative to the repository root.
    #[must_use]
    pub const fn workflow_file(self) -> &'static str {
        match self {
            Self::DocsLint => ".github/workflows/docs-lint.yaml",
            Self::Testing => ".github/workflows/testing.yaml",
            Self::OsCompatibility => ".github/workflows/os-compatibility.yaml",
            Self::Coverage => ".github/workflows/generate_coverage_pr.yaml",
            Self::Container => ".github/workflows/container.yaml",
            Self::DbCompatibility => ".github/workflows/db-compatibility.yaml",
            Self::DbBenchmarking => ".github/workflows/db-benchmarking.yaml",
            Self::SecurityScan => ".github/workflows/security-scan.yaml",
        }
    }
}
