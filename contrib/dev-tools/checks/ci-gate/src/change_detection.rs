//! Change detection: which job groups a pull request needs.
//!
//! The rules reproduce, exactly, the `paths`, `paths-ignore`, and `branches` filters that the
//! pull-request workflows declare today (issue #2525, task T2). They live in one typed table, so a
//! unit test can pin each rule, instead of in six workflow files.
//!
//! GitHub's filters have three shapes, modelled by the private `PathFilter` type:
//!
//! - no filter: the workflow runs on every pull request;
//! - `paths-ignore`: the workflow runs unless **every** changed path matches an ignored pattern;
//! - `paths`: the workflow runs when **at least one** changed path matches a pattern.
//!
//! Only the patterns the workflows actually use are supported: a file extension at any depth
//! (`**/*.md`), an exact path from the repository root (`Containerfile`), and a directory subtree
//! (`packages/tracker-core/**`).

use serde::Serialize;

use crate::job_group::JobGroup;

/// A path pattern in the subset of GitHub's glob syntax that the workflows use.
#[derive(Clone, Copy, Debug)]
enum PathPattern {
    /// `**/*<extension>`: any path, at any depth, that ends with the extension.
    Extension(&'static str),
    /// A path from the repository root, matched whole.
    Exact(&'static str),
    /// `<directory>/**`: any path below the directory. The directory ends with `/`.
    Under(&'static str),
}

impl PathPattern {
    fn matches(self, path: &str) -> bool {
        match self {
            Self::Extension(extension) => path.ends_with(extension),
            Self::Exact(exact) => path == exact,
            Self::Under(directory) => path.starts_with(directory),
        }
    }

    /// The pattern as it is written in a workflow file.
    fn glob(self) -> String {
        match self {
            Self::Extension(extension) => format!("**/*{extension}"),
            Self::Exact(exact) => exact.to_string(),
            Self::Under(directory) => format!("{directory}**"),
        }
    }
}

/// How a workflow's path filter selects pull requests.
#[derive(Clone, Copy, Debug)]
enum PathFilter {
    /// No path filter.
    Always,
    /// `paths-ignore`: skipped when every changed path matches one of the patterns.
    Ignore(&'static [PathPattern]),
    /// `paths`: runs when at least one changed path matches one of the patterns.
    Only(&'static [PathPattern]),
}

/// The filters of one pull-request workflow.
#[derive(Clone, Copy, Debug)]
struct Rule {
    group: JobGroup,
    paths: PathFilter,
    /// The `branches` filter of the `pull_request` event, which matches the base branch. Empty
    /// means every base branch.
    base_branches: &'static [&'static str],
}

/// Paths that change no code: Markdown files and the spell-check dictionary.
const DOCUMENTATION_ONLY: &[PathPattern] = &[PathPattern::Extension(".md"), PathPattern::Exact("project-words.txt")];

/// One rule per group, in [`JobGroup::ALL`] order.
const RULES: [Rule; 8] = [
    Rule {
        group: JobGroup::DocsLint,
        paths: PathFilter::Always,
        base_branches: &[],
    },
    Rule {
        group: JobGroup::Testing,
        paths: PathFilter::Ignore(DOCUMENTATION_ONLY),
        base_branches: &[],
    },
    Rule {
        group: JobGroup::OsCompatibility,
        paths: PathFilter::Ignore(DOCUMENTATION_ONLY),
        base_branches: &[],
    },
    Rule {
        group: JobGroup::Coverage,
        paths: PathFilter::Ignore(DOCUMENTATION_ONLY),
        base_branches: &["develop"],
    },
    Rule {
        group: JobGroup::Container,
        paths: PathFilter::Ignore(DOCUMENTATION_ONLY),
        base_branches: &["develop", "main"],
    },
    Rule {
        group: JobGroup::DbCompatibility,
        paths: PathFilter::Only(&[
            PathPattern::Under("packages/tracker-core/"),
            PathPattern::Exact(JobGroup::DbCompatibility.workflow_file()),
        ]),
        base_branches: &[],
    },
    Rule {
        group: JobGroup::DbBenchmarking,
        paths: PathFilter::Only(&[
            PathPattern::Under("packages/tracker-core/"),
            PathPattern::Under("packages/persistence-benchmark/"),
            PathPattern::Exact(JobGroup::DbBenchmarking.workflow_file()),
        ]),
        base_branches: &[],
    },
    Rule {
        group: JobGroup::SecurityScan,
        paths: PathFilter::Only(&[
            PathPattern::Exact("Containerfile"),
            PathPattern::Exact(JobGroup::SecurityScan.workflow_file()),
        ]),
        base_branches: &[],
    },
];

/// Whether one job group is needed, and why.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GroupDecision {
    pub group: JobGroup,
    pub needed: bool,
    /// A human-readable reason, naming the path or branch that decided it.
    pub explanation: String,
}

/// The job groups a change needs.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Classification {
    pub base_branch: String,
    pub changed_path_count: usize,
    /// One decision per group, in [`JobGroup::ALL`] order.
    pub groups: Vec<GroupDecision>,
}

impl Classification {
    /// Whether the given group is needed.
    #[must_use]
    pub fn needs(&self, group: JobGroup) -> bool {
        self.groups.iter().any(|decision| decision.group == group && decision.needed)
    }
}

/// Decides which job groups a change to `changed_paths`, merged into `base_branch`, needs.
///
/// Paths are relative to the repository root, as `git diff --name-only` prints them.
#[must_use]
pub fn classify<S: AsRef<str>>(changed_paths: &[S], base_branch: &str) -> Classification {
    let paths: Vec<&str> = changed_paths.iter().map(AsRef::as_ref).collect();

    Classification {
        base_branch: base_branch.to_string(),
        changed_path_count: paths.len(),
        groups: RULES.iter().map(|rule| decide(*rule, &paths, base_branch)).collect(),
    }
}

/// Reads the changed paths from the output of `git diff --name-only -z`: records separated by NUL
/// bytes.
///
/// A record is kept exactly as it is, including leading and trailing spaces, because those can be
/// part of a file name, and `-z` also disables the C-quoting that plain `git diff --name-only`
/// applies to non-ASCII names. Only empty records are ignored (a trailing NUL leaves one). A name
/// that is not valid UTF-8 has its invalid bytes replaced, which does not change whether it matches
/// the ASCII patterns of the rules.
#[must_use]
pub fn parse_changed_paths(input: &[u8]) -> Vec<String> {
    input
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
        .map(|record| String::from_utf8_lossy(record).into_owned())
        .collect()
}

fn decide(rule: Rule, paths: &[&str], base_branch: &str) -> GroupDecision {
    let (needed, explanation) = if rule.base_branches.is_empty() || rule.base_branches.contains(&base_branch) {
        decide_by_paths(rule.paths, paths)
    } else {
        (
            false,
            format!(
                "the base branch `{base_branch}` is not one of {}",
                rule.base_branches
                    .iter()
                    .map(|branch| format!("`{branch}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )
    };

    GroupDecision {
        group: rule.group,
        needed,
        explanation,
    }
}

fn decide_by_paths(filter: PathFilter, paths: &[&str]) -> (bool, String) {
    match filter {
        PathFilter::Always => (true, "it runs on every pull request".to_string()),
        PathFilter::Ignore(patterns) => paths
            .iter()
            .find(|path| !patterns.iter().any(|pattern| pattern.matches(path)))
            .map_or_else(
                || {
                    (
                        false,
                        format!("every changed path matches an ignored pattern ({})", describe(patterns)),
                    )
                },
                |path| {
                    (
                        true,
                        format!("`{path}` matches none of the ignored patterns {}", describe(patterns)),
                    )
                },
            ),
        PathFilter::Only(patterns) => paths
            .iter()
            .find_map(|path| {
                patterns
                    .iter()
                    .find(|pattern| pattern.matches(path))
                    .map(|pattern| (path, pattern))
            })
            .map_or_else(
                || (false, format!("no changed path matches {}", describe(patterns))),
                |(path, pattern)| (true, format!("`{path}` matches `{}`", pattern.glob())),
            ),
    }
}

fn describe(patterns: &[PathPattern]) -> String {
    patterns
        .iter()
        .map(|pattern| format!("`{}`", pattern.glob()))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn it_should_always_need_the_documentation_lint_when_the_change_is_documentation_only() {
        // Arrange
        let changed_paths = ["README.md", "project-words.txt"];

        // Act
        let classification = classify(&changed_paths, "develop");

        // Assert
        assert!(
            classification.needs(JobGroup::DocsLint),
            "docs-lint is the signal for documentation-only pull requests, so it must run for {changed_paths:?}"
        );
    }

    #[test]
    fn it_should_need_the_documentation_lint_when_no_path_changed() {
        // Arrange
        let changed_paths: [&str; 0] = [];

        // Act
        let classification = classify(&changed_paths, "develop");

        // Assert
        assert!(
            classification.needs(JobGroup::DocsLint),
            "docs-lint runs on every pull request, including one with no changed path"
        );
    }

    #[rstest]
    #[case::root_markdown_file(&["README.md"], false)]
    #[case::nested_markdown_file(&["docs/adrs/index.md"], false)]
    #[case::spell_check_dictionary(&["project-words.txt"], false)]
    #[case::only_documentation_files(&["docs/a.md", "project-words.txt"], false)]
    #[case::no_changed_path(&[], false)]
    #[case::rust_source_file(&["src/main.rs"], true)]
    #[case::manifest(&["Cargo.toml"], true)]
    #[case::workflow_file(&[".github/workflows/testing.yaml"], true)]
    #[case::documentation_mixed_with_code(&["README.md", "src/main.rs"], true)]
    #[case::file_that_only_looks_like_markdown(&["notes.md.bak"], true)]
    #[case::dictionary_name_below_the_root(&["docs/project-words.txt"], true)]
    #[case::markdown_name_with_a_trailing_space(&["README.md "], true)]
    #[case::dictionary_name_with_a_leading_space(&[" project-words.txt"], true)]
    fn it_should_need_the_code_groups_only_when_a_changed_path_is_not_documentation(
        #[case] changed_paths: &[&str],
        #[case] expected_needed: bool,
        #[values(JobGroup::Testing, JobGroup::OsCompatibility, JobGroup::Coverage, JobGroup::Container)] group: JobGroup,
    ) {
        // Act
        let classification = classify(changed_paths, "develop");

        // Assert
        assert_eq!(
            classification.needs(group),
            expected_needed,
            "{group:?} with changed paths {changed_paths:?} into `develop`"
        );
    }

    #[rstest]
    #[case::develop("develop", JobGroup::Coverage, true)]
    #[case::main("main", JobGroup::Coverage, false)]
    #[case::feature_branch("feature-x", JobGroup::Coverage, false)]
    #[case::develop("develop", JobGroup::Container, true)]
    #[case::main("main", JobGroup::Container, true)]
    #[case::feature_branch("feature-x", JobGroup::Container, false)]
    fn it_should_need_the_branch_restricted_groups_only_for_their_base_branches(
        #[case] base_branch: &str,
        #[case] group: JobGroup,
        #[case] expected_needed: bool,
    ) {
        // Arrange
        let changed_paths = ["src/main.rs"];

        // Act
        let classification = classify(&changed_paths, base_branch);

        // Assert
        assert_eq!(
            classification.needs(group),
            expected_needed,
            "{group:?} for a code change into `{base_branch}`"
        );
    }

    #[rstest]
    #[case::unrestricted_group_on_a_feature_branch("feature-x", JobGroup::Testing)]
    #[case::unrestricted_group_on_main("main", JobGroup::OsCompatibility)]
    fn it_should_need_the_unrestricted_groups_for_any_base_branch(#[case] base_branch: &str, #[case] group: JobGroup) {
        // Arrange
        let changed_paths = ["src/main.rs"];

        // Act
        let classification = classify(&changed_paths, base_branch);

        // Assert
        assert!(
            classification.needs(group),
            "{group:?} has no branch filter, but was skipped for `{base_branch}`"
        );
    }

    #[rstest]
    #[case::tracker_core_source(&["packages/tracker-core/src/lib.rs"], true)]
    #[case::tracker_core_manifest(&["packages/tracker-core/Cargo.toml"], true)]
    #[case::own_workflow(&[".github/workflows/db-compatibility.yaml"], true)]
    #[case::code_elsewhere_with_one_tracker_core_file(&["src/main.rs", "packages/tracker-core/src/lib.rs"], true)]
    #[case::non_ascii_tracker_core_file(&["packages/tracker-core/src/café.rs"], true)]
    #[case::other_package(&["packages/udp-core/src/lib.rs"], false)]
    #[case::package_whose_name_starts_with_the_same_text(&["packages/tracker-core-extra/src/lib.rs"], false)]
    #[case::other_workflow(&[".github/workflows/testing.yaml"], false)]
    #[case::no_changed_path(&[], false)]
    fn it_should_need_the_database_compatibility_tests_only_for_tracker_core_or_their_workflow(
        #[case] changed_paths: &[&str],
        #[case] expected_needed: bool,
    ) {
        // Act
        let classification = classify(changed_paths, "develop");

        // Assert
        assert_eq!(
            classification.needs(JobGroup::DbCompatibility),
            expected_needed,
            "db-compatibility with changed paths {changed_paths:?}"
        );
    }

    #[rstest]
    #[case::tracker_core_source(&["packages/tracker-core/src/lib.rs"], true)]
    #[case::persistence_benchmark_source(&["packages/persistence-benchmark/src/main.rs"], true)]
    #[case::own_workflow(&[".github/workflows/db-benchmarking.yaml"], true)]
    #[case::db_compatibility_workflow(&[".github/workflows/db-compatibility.yaml"], false)]
    #[case::other_package(&["packages/udp-core/src/lib.rs"], false)]
    #[case::package_whose_name_starts_with_the_same_text(&["packages/persistence-benchmark-extra/src/main.rs"], false)]
    fn it_should_need_the_database_benchmarks_only_for_their_packages_or_workflow(
        #[case] changed_paths: &[&str],
        #[case] expected_needed: bool,
    ) {
        // Act
        let classification = classify(changed_paths, "develop");

        // Assert
        assert_eq!(
            classification.needs(JobGroup::DbBenchmarking),
            expected_needed,
            "db-benchmarking with changed paths {changed_paths:?}"
        );
    }

    #[rstest]
    #[case::containerfile(&["Containerfile"], true)]
    #[case::own_workflow(&[".github/workflows/security-scan.yaml"], true)]
    #[case::containerfile_below_the_root(&["docs/Containerfile"], false)]
    #[case::other_workflow(&[".github/workflows/container.yaml"], false)]
    #[case::rust_source_file(&["src/main.rs"], false)]
    fn it_should_need_the_security_scan_only_for_the_containerfile_or_its_workflow(
        #[case] changed_paths: &[&str],
        #[case] expected_needed: bool,
    ) {
        // Act
        let classification = classify(changed_paths, "develop");

        // Assert
        assert_eq!(
            classification.needs(JobGroup::SecurityScan),
            expected_needed,
            "security-scan with changed paths {changed_paths:?}"
        );
    }

    #[test]
    fn it_should_list_one_decision_per_group_in_a_stable_order() {
        // Arrange
        let changed_paths = ["src/main.rs"];

        // Act
        let classification = classify(&changed_paths, "develop");

        // Assert
        let listed_groups: Vec<JobGroup> = classification.groups.iter().map(|decision| decision.group).collect();
        assert_eq!(
            listed_groups,
            JobGroup::ALL,
            "the decisions must follow the declaration order of the groups"
        );
    }

    #[test]
    fn it_should_name_the_path_that_made_a_group_needed() {
        // Arrange
        let changed_paths = ["README.md", "src/main.rs"];

        // Act
        let classification = classify(&changed_paths, "develop");

        // Assert
        let testing = classification
            .groups
            .iter()
            .find(|decision| decision.group == JobGroup::Testing)
            .expect("the classification lists every group");
        assert!(
            testing.explanation.contains("`src/main.rs`"),
            "the explanation should name the path that is not documentation, but was: {}",
            testing.explanation
        );
    }

    #[test]
    fn it_should_name_the_base_branch_that_made_a_group_unneeded() {
        // Arrange
        let changed_paths = ["src/main.rs"];

        // Act
        let classification = classify(&changed_paths, "feature-x");

        // Assert
        let coverage = classification
            .groups
            .iter()
            .find(|decision| decision.group == JobGroup::Coverage)
            .expect("the classification lists every group");
        assert!(
            coverage.explanation.contains("`feature-x`") && coverage.explanation.contains("`develop`"),
            "the explanation should name the base branch and the allowed one, but was: {}",
            coverage.explanation
        );
    }

    #[test]
    fn it_should_report_the_number_of_changed_paths_and_the_base_branch() {
        // Arrange
        let changed_paths = ["README.md", "src/main.rs", "Cargo.toml"];

        // Act
        let classification = classify(&changed_paths, "develop");

        // Assert
        assert_eq!(
            (classification.changed_path_count, classification.base_branch.as_str()),
            (3, "develop")
        );
    }

    #[test]
    fn it_should_keep_the_spaces_that_belong_to_a_file_name_when_reading_changed_paths() {
        // Arrange
        let input = b"README.md \0 project-words.txt\0";

        // Act
        let changed_paths = parse_changed_paths(input);

        // Assert
        assert_eq!(
            changed_paths,
            vec!["README.md ".to_string(), " project-words.txt".to_string()]
        );
    }

    #[test]
    fn it_should_ignore_empty_records_when_reading_changed_paths() {
        // Arrange
        let input = b"README.md\0\0src/main.rs\0";

        // Act
        let changed_paths = parse_changed_paths(input);

        // Assert
        assert_eq!(changed_paths, vec!["README.md".to_string(), "src/main.rs".to_string()]);
    }

    #[test]
    fn it_should_keep_a_newline_inside_a_file_name_when_reading_changed_paths() {
        // Arrange
        let input = b"docs/first\nsecond.md\0";

        // Act
        let changed_paths = parse_changed_paths(input);

        // Assert
        assert_eq!(changed_paths, vec!["docs/first\nsecond.md".to_string()]);
    }

    #[test]
    fn it_should_read_a_non_ascii_path_as_it_is_without_git_quoting() {
        // Arrange
        let input = "packages/tracker-core/src/café.rs\0".as_bytes();

        // Act
        let changed_paths = parse_changed_paths(input);

        // Assert
        assert_eq!(changed_paths, vec!["packages/tracker-core/src/café.rs".to_string()]);
    }

    #[test]
    fn it_should_replace_the_invalid_bytes_of_a_path_that_is_not_utf8_when_reading_changed_paths() {
        // Arrange
        let input = b"packages/tracker-core/src/caf\xe9.rs\0";

        // Act
        let changed_paths = parse_changed_paths(input);

        // Assert
        assert_eq!(
            changed_paths,
            vec![format!("packages/tracker-core/src/caf{}.rs", char::REPLACEMENT_CHARACTER)]
        );
    }
}
