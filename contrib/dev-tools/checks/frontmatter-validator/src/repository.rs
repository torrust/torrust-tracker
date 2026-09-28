//! Location-dependent validation policy over repository-relative paths.
//!
//! These decisions are pure: they take a path, the document text, and a snapshot of repository
//! files, never touch the filesystem or git, and return every diagnostic in contract order.

use std::collections::BTreeSet;

use serde_yaml::{Mapping, Value};

use crate::profile::{self, Profile, SkillName, StrictSemanticLinks};
use crate::{Diagnostic, DiagnosticCategory, DocumentOwnership, Severity, extract_with_ownership};

const SKILLS_DIRECTORY: &str = ".github/skills/";

/// Where a document lives, as far as the issue lifecycle is concerned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Location {
    /// `docs/issues/drafts/`
    Draft,
    /// `docs/issues/open/`
    Open,
    /// `docs/issues/closed/`
    Closed,
    /// Anywhere else in the repository.
    Elsewhere,
}

impl Location {
    /// Classifies a repository-relative path with `/` separators.
    #[must_use]
    pub fn of(path: &str) -> Self {
        if path.starts_with("docs/issues/drafts/") {
            Self::Draft
        } else if path.starts_with("docs/issues/open/") {
            Self::Open
        } else if path.starts_with("docs/issues/closed/") {
            Self::Closed
        } else {
            Self::Elsewhere
        }
    }

    const fn is_draft_or_open(self) -> bool {
        matches!(self, Self::Draft | Self::Open)
    }
}

/// The repository files that references resolve against: the index in staged mode, the working
/// tree otherwise.
#[derive(Debug, Default)]
pub struct RepositoryFiles {
    files: BTreeSet<String>,
    skills: BTreeSet<String>,
}

impl RepositoryFiles {
    /// Builds the snapshot from repository-relative file paths with `/` separators.
    pub fn new(files: impl IntoIterator<Item = String>) -> Self {
        let files: BTreeSet<String> = files.into_iter().collect();
        let skills = files
            .iter()
            .filter_map(|file| file.strip_prefix(SKILLS_DIRECTORY)?.strip_suffix("/SKILL.md"))
            .map(|directory| file_name(directory).to_owned())
            .collect();
        Self { files, skills }
    }

    /// A tracked file, or a directory that contains a tracked file.
    fn contains_path(&self, path: &str) -> bool {
        let directory = format!("{path}/");
        self.files.contains(path)
            || self
                .files
                .range(directory.clone()..)
                .next()
                .is_some_and(|file| file.starts_with(&directory))
    }

    fn contains_skill(&self, name: &str) -> bool {
        self.skills.contains(name)
    }
}

/// Agent Skills and agent profiles have an externally governed top-level schema.
#[must_use]
pub fn ownership(path: &str) -> DocumentOwnership {
    let file_name = file_name(path);
    if file_name == "SKILL.md" || file_name.ends_with(".agent.md") {
        DocumentOwnership::External
    } else {
        DocumentOwnership::Repository
    }
}

/// Validates one document and returns its diagnostics: at most one structural finding, then
/// warnings, then repository-aware findings.
#[must_use]
pub fn validate_document(path: &str, markdown: &str, repository: &RepositoryFiles) -> Vec<Diagnostic> {
    let location = Location::of(path);
    let ownership = ownership(path);
    let frontmatter = match extract_with_ownership(markdown, ownership) {
        Ok(frontmatter) => frontmatter,
        Err(diagnostic) => return vec![diagnostic],
    };
    let Some(frontmatter) = frontmatter else {
        return legacy_shape(path, location, None).into_iter().collect();
    };

    let mut diagnostics = Vec::new();
    let spec = match profile::validate(&frontmatter) {
        Ok(Profile::Issue(issue)) => Some((issue.spec_path, issue.semantic_links)),
        Ok(Profile::Epic(epic)) => Some((epic.spec_path, epic.semantic_links)),
        Ok(Profile::Permissive) if ownership == DocumentOwnership::External => return diagnostics,
        Ok(Profile::Permissive) => {
            diagnostics.extend(legacy_shape(path, location, Some(&frontmatter.values)));
            return diagnostics;
        }
        Err(diagnostic) => {
            diagnostics.push(for_location(diagnostic, location));
            None
        }
    };
    diagnostics.extend(experimental_fields(&frontmatter.values));
    if let Some((spec_path, links)) = spec {
        let status = frontmatter.values.get("status").and_then(Value::as_str).unwrap_or_default();
        diagnostics.extend(repository_findings(path, location, status, &spec_path, &links, repository));
    }
    diagnostics
}

/// D7 applies to specs in the issue lifecycle folders. Closed specs are historical: their paths go
/// stale by design, so they get only the advisory status and `spec-path` checks.
fn repository_findings(
    path: &str,
    location: Location,
    status: &str,
    spec_path: &str,
    links: &StrictSemanticLinks,
    repository: &RepositoryFiles,
) -> Vec<Diagnostic> {
    if location == Location::Elsewhere {
        return Vec::new();
    }

    let spec_path_mismatch = (spec_path != path).then(|| {
        Diagnostic::for_field(
            DiagnosticCategory::SpecPathMismatch,
            "spec-path",
            format!("`spec-path` is `{spec_path}`, but this spec is at `{path}`."),
        )
    });
    let resolves_references = location.is_draft_or_open();

    lifecycle_mismatch(location, status)
        .into_iter()
        .chain(spec_path_mismatch)
        .chain(missing_artifacts(links, repository).filter(|_| resolves_references))
        .chain(unknown_skills(links, repository).filter(|_| resolves_references))
        .map(|finding| for_location(finding, location))
        .collect()
}

fn lifecycle_mismatch(location: Location, status: &str) -> Option<Diagnostic> {
    let (matches, rule) = match location {
        Location::Draft => (status == "draft", "use `draft`"),
        Location::Open => (!matches!(status, "draft" | "done"), "not use `draft` or `done`"),
        Location::Closed => (status == "done", "use `done`"),
        Location::Elsewhere => (true, ""),
    };

    (!matches).then(|| {
        Diagnostic::for_field(
            DiagnosticCategory::LifecycleLocationMismatch,
            "status",
            format!("`status` is `{status}`, but specs in this lifecycle folder must {rule}."),
        )
    })
}

fn missing_artifacts<'a>(
    links: &'a StrictSemanticLinks,
    repository: &'a RepositoryFiles,
) -> impl Iterator<Item = Diagnostic> + 'a {
    links
        .related_artifacts
        .iter()
        .flatten()
        .filter_map(|artifact| artifact.repository_path())
        .filter(|artifact| !repository.contains_path(artifact))
        .map(|artifact| {
            Diagnostic::for_field(
                DiagnosticCategory::MissingArtifact,
                "semantic-links.related-artifacts",
                format!("`{artifact}` names no tracked file or directory."),
            )
        })
}

fn unknown_skills<'a>(links: &'a StrictSemanticLinks, repository: &'a RepositoryFiles) -> impl Iterator<Item = Diagnostic> + 'a {
    links
        .skill_links
        .iter()
        .flatten()
        .map(SkillName::as_str)
        .filter(|skill| !repository.contains_skill(skill))
        .map(|skill| {
            Diagnostic::for_field(
                DiagnosticCategory::UnknownSkill,
                "semantic-links.skill-links",
                format!("`{skill}` names no skill under `{SKILLS_DIRECTORY}`."),
            )
        })
}

/// Closed specs are historical records: their strict-profile findings are advisory.
fn for_location(diagnostic: Diagnostic, location: Location) -> Diagnostic {
    if location == Location::Closed {
        diagnostic.with_severity(Severity::Warning)
    } else {
        diagnostic
    }
}

/// Draft and open specs must use the v1 shape: primary spec files always, and other files that
/// declare an issue or EPIC document type.
fn legacy_shape(path: &str, location: Location, values: Option<&Mapping>) -> Option<Diagnostic> {
    let declares_spec = values
        .and_then(|values| values.get("doc-type"))
        .and_then(Value::as_str)
        .is_some_and(|doc_type| matches!(doc_type, "issue" | "epic"));

    (location.is_draft_or_open() && (is_primary_spec(path) || declares_spec)).then(|| {
        Diagnostic::new(
            DiagnosticCategory::LegacyShape,
            "Draft and open issue specs must use the v1 frontmatter (`schema-version: 1`); copy the shape \
             from `docs/templates/ISSUE.md` or `docs/templates/EPIC.md` and follow the migration checklist \
             in `contrib/dev-tools/checks/frontmatter-validator/README.md`.",
        )
    })
}

fn experimental_fields(values: &Mapping) -> impl Iterator<Item = Diagnostic> + '_ {
    values
        .keys()
        .filter_map(Value::as_str)
        .filter(|field| field.starts_with("x-"))
        .map(|field| {
            Diagnostic::for_field(
                DiagnosticCategory::ExperimentalField,
                field,
                format!("`{field}` is an experimental extension with no v1 contract semantics."),
            )
            .with_severity(Severity::Warning)
        })
}

fn is_primary_spec(path: &str) -> bool {
    matches!(file_name(path), "ISSUE.md" | "EPIC.md")
}

fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    // Owns location classification, ownership dispatch, location-dependent severity, the
    // legacy-shape rule, experimental-field warnings, and the D7 repository-aware checks.
    // Structural profile rules are owned by `profile`.

    use rstest::rstest;

    use super::{Location, RepositoryFiles, ownership, validate_document};
    use crate::{DiagnosticCategory, DocumentOwnership, Severity};

    const WRONG_SCALAR_ISSUE: &str = include_str!("../fixtures/rejected/issue-wrong-scalar.md");
    const LEGACY_ISSUE: &str = "---\ndoc-type: issue\nstatus: planned\n---\n# Legacy\n";
    const LEGACY_EPIC: &str = "---\ndoc-type: epic\nstatus: planned\n---\n# Legacy\n";
    const EVIDENCE: &str = "---\ndoc-type: manual-verification-evidence\n---\n# Evidence\n";
    const PLAIN: &str = "# Plain document\n";
    const OPEN_SPEC: &str = "docs/issues/open/1-example/ISSUE.md";
    const NO_LINKS: &str = " {}";

    /// A strict v1 issue; `links` is the YAML that follows `semantic-links:`.
    fn strict_issue(spec_path: &str, status: &str, links: &str) -> String {
        format!(
            "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: {status}\npriority: p1\nepic: null\ngithub-issue: 1\nspec-path: {spec_path}\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-26 11:30\"\nsemantic-links:{links}\n---\n# Issue\n"
        )
    }

    /// The observable identity of each diagnostic, in order, against an empty repository.
    fn findings(path: &str, markdown: &str) -> Vec<(DiagnosticCategory, Severity)> {
        findings_in(path, markdown, &RepositoryFiles::default())
    }

    fn findings_in(path: &str, markdown: &str, repository: &RepositoryFiles) -> Vec<(DiagnosticCategory, Severity)> {
        validate_document(path, markdown, repository)
            .into_iter()
            .map(|diagnostic| (diagnostic.category, diagnostic.severity))
            .collect()
    }

    fn repository(files: &[&str]) -> RepositoryFiles {
        RepositoryFiles::new(files.iter().map(ToString::to_string))
    }

    #[rstest]
    #[case::draft("docs/issues/drafts/1-example/ISSUE.md", Location::Draft)]
    #[case::open("docs/issues/open/1-example/ISSUE.md", Location::Open)]
    #[case::closed("docs/issues/closed/1-example/ISSUE.md", Location::Closed)]
    #[case::issues_readme("docs/issues/README.md", Location::Elsewhere)]
    #[case::similar_prefix("docs/issues/opened/ISSUE.md", Location::Elsewhere)]
    #[case::elsewhere("docs/adrs/README.md", Location::Elsewhere)]
    fn it_should_classify_the_lifecycle_location_of_a_path(#[case] path: &str, #[case] expected: Location) {
        assert_eq!(Location::of(path), expected);
    }

    #[rstest]
    #[case::agent_skill(".github/skills/dev/example/SKILL.md", DocumentOwnership::External)]
    #[case::agent_profile(".github/agents/implementer.agent.md", DocumentOwnership::External)]
    #[case::root_agent_skill("SKILL.md", DocumentOwnership::External)]
    #[case::repository_document("docs/issues/open/1-example/ISSUE.md", DocumentOwnership::Repository)]
    #[case::skill_named_directory("docs/SKILL.md/notes.md", DocumentOwnership::Repository)]
    fn it_should_dispatch_ownership_by_file_name(#[case] path: &str, #[case] expected: DocumentOwnership) {
        assert_eq!(ownership(path), expected);
    }

    #[rstest]
    #[case::open_primary_without_frontmatter("docs/issues/open/1-example/ISSUE.md", PLAIN)]
    #[case::draft_primary_without_frontmatter("docs/issues/drafts/example/ISSUE.md", PLAIN)]
    #[case::open_legacy_issue("docs/issues/open/1-example/ISSUE.md", LEGACY_ISSUE)]
    #[case::open_legacy_epic("docs/issues/open/1-example/EPIC.md", LEGACY_EPIC)]
    #[case::open_primary_with_other_doc_type("docs/issues/open/1-example/ISSUE.md", EVIDENCE)]
    #[case::open_standalone_legacy_issue("docs/issues/open/1-example.md", LEGACY_ISSUE)]
    fn it_should_reject_a_draft_or_open_spec_that_is_not_a_strict_v1_record(#[case] path: &str, #[case] markdown: &str) {
        assert_eq!(findings(path, markdown), [(DiagnosticCategory::LegacyShape, Severity::Error)]);
    }

    #[rstest]
    #[case::closed_legacy_primary("docs/issues/closed/1-example/ISSUE.md", LEGACY_ISSUE)]
    #[case::legacy_primary_elsewhere("docs/archive/ISSUE.md", LEGACY_ISSUE)]
    #[case::open_supporting_evidence("docs/issues/open/1-example/evidence.md", EVIDENCE)]
    #[case::open_supporting_plain("docs/issues/open/1-example/plan.md", PLAIN)]
    fn it_should_not_require_the_v1_shape_outside_draft_and_open_specs(#[case] path: &str, #[case] markdown: &str) {
        assert_eq!(findings(path, markdown), []);
    }

    #[test]
    fn it_should_accept_a_strict_v1_spec_that_matches_its_location() {
        // Arrange: an open spec whose status, `spec-path`, and references are all consistent.
        let markdown = strict_issue(
            OPEN_SPEC,
            "planned",
            "\n  skill-links:\n    - create-issue\n  related-artifacts:\n    - docs/AGENTS.md\n    - \"issue #1\"",
        );
        let repository = repository(&["docs/AGENTS.md", ".github/skills/dev/planning/create-issue/SKILL.md"]);

        // Act
        let actual = findings_in(OPEN_SPEC, &markdown, &repository);

        // Assert
        assert_eq!(actual, []);
    }

    #[rstest]
    #[case::draft_with_draft("docs/issues/drafts/1-example/ISSUE.md", "draft", None)]
    #[case::draft_with_planned("docs/issues/drafts/1-example/ISSUE.md", "planned", Some(Severity::Error))]
    #[case::open_with_in_progress(OPEN_SPEC, "in-progress", None)]
    #[case::open_with_draft(OPEN_SPEC, "draft", Some(Severity::Error))]
    #[case::open_with_done(OPEN_SPEC, "done", Some(Severity::Error))]
    #[case::closed_with_done("docs/issues/closed/1-example/ISSUE.md", "done", None)]
    #[case::closed_with_planned("docs/issues/closed/1-example/ISSUE.md", "planned", Some(Severity::Warning))]
    fn it_should_check_the_status_against_the_lifecycle_folder(
        #[case] path: &str,
        #[case] status: &str,
        #[case] expected: Option<Severity>,
    ) {
        // Act
        let actual = findings(path, &strict_issue(path, status, NO_LINKS));

        // Assert
        let expected: Vec<_> = expected
            .map(|severity| (DiagnosticCategory::LifecycleLocationMismatch, severity))
            .into_iter()
            .collect();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::open(OPEN_SPEC, "planned", Severity::Error)]
    #[case::closed("docs/issues/closed/1-example/ISSUE.md", "done", Severity::Warning)]
    fn it_should_report_a_spec_path_that_is_not_the_spec_location(
        #[case] path: &str,
        #[case] status: &str,
        #[case] expected: Severity,
    ) {
        // Arrange: the spec was moved but its `spec-path` still names the old location.
        let markdown = strict_issue("docs/issues/drafts/example/ISSUE.md", status, NO_LINKS);

        // Act
        let actual = findings(path, &markdown);

        // Assert
        assert_eq!(actual, [(DiagnosticCategory::SpecPathMismatch, expected)]);
    }

    #[rstest]
    #[case::tracked_file("docs/present.md", false)]
    #[case::directory_with_tracked_files("docs/tree", false)]
    #[case::untracked_file("docs/missing.md", true)]
    #[case::sibling_with_a_shared_prefix("docs/tr", true)]
    #[case::issue_reference("\"issue #1\"", false)]
    #[case::review_finding("review-finding:pr-1-f1", false)]
    fn it_should_report_a_related_artifact_path_that_names_no_tracked_file(
        #[case] artifact: &str,
        #[case] expected_missing: bool,
    ) {
        // Arrange
        let markdown = strict_issue(OPEN_SPEC, "planned", &format!("\n  related-artifacts:\n    - {artifact}"));
        let repository = repository(&["docs/present.md", "docs/tree/file.md"]);

        // Act
        let actual = findings_in(OPEN_SPEC, &markdown, &repository);

        // Assert: issue and review-finding references are syntax-only.
        let expected: Vec<_> = expected_missing
            .then_some((DiagnosticCategory::MissingArtifact, Severity::Error))
            .into_iter()
            .collect();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::nested_skill(".github/skills/dev/planning/create-issue/SKILL.md", false)]
    #[case::top_level_skill(".github/skills/create-issue/SKILL.md", false)]
    #[case::skill_outside_the_skills_directory("docs/skills/create-issue/SKILL.md", true)]
    #[case::skill_folder_without_skill_file(".github/skills/dev/create-issue/README.md", true)]
    fn it_should_report_a_skill_link_that_names_no_repository_skill(#[case] tracked_file: &str, #[case] expected_unknown: bool) {
        // Arrange
        let markdown = strict_issue(OPEN_SPEC, "planned", "\n  skill-links:\n    - create-issue");

        // Act
        let actual = findings_in(OPEN_SPEC, &markdown, &repository(&[tracked_file]));

        // Assert
        let expected: Vec<_> = expected_unknown
            .then_some((DiagnosticCategory::UnknownSkill, Severity::Error))
            .into_iter()
            .collect();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::closed("docs/issues/closed/1-example/ISSUE.md", "done")]
    #[case::elsewhere("docs/archive/ISSUE.md", "planned")]
    fn it_should_not_resolve_references_outside_draft_and_open_specs(#[case] path: &str, #[case] status: &str) {
        // Arrange: stale references in a spec whose `spec-path` and status match its location.
        let links = "\n  skill-links:\n    - retired-skill\n  related-artifacts:\n    - docs/removed.md";

        // Act
        let actual = findings(path, &strict_issue(path, status, links));

        // Assert
        assert_eq!(actual, []);
    }

    #[test]
    fn it_should_report_warnings_before_repository_findings() {
        // Arrange: an open spec with an `x-` field and a draft status.
        let markdown = strict_issue(OPEN_SPEC, "draft", NO_LINKS).replace("doc-type: issue\n", "doc-type: issue\nx-note: wip\n");

        // Act
        let actual = findings(OPEN_SPEC, &markdown);

        // Assert
        assert_eq!(
            actual,
            [
                (DiagnosticCategory::ExperimentalField, Severity::Warning),
                (DiagnosticCategory::LifecycleLocationMismatch, Severity::Error),
            ]
        );
    }

    #[rstest]
    #[case::open("docs/issues/open/1-example/ISSUE.md", Severity::Error)]
    #[case::draft("docs/issues/drafts/example/ISSUE.md", Severity::Error)]
    #[case::closed("docs/issues/closed/1-example/ISSUE.md", Severity::Warning)]
    #[case::elsewhere("docs/archive/ISSUE.md", Severity::Error)]
    fn it_should_set_strict_profile_severity_from_the_location(#[case] path: &str, #[case] expected: Severity) {
        assert_eq!(
            findings(path, WRONG_SCALAR_ISSUE),
            [(DiagnosticCategory::WrongScalarType, expected)]
        );
    }

    #[test]
    fn it_should_keep_syntax_errors_as_errors_in_closed_specs() {
        // Arrange: a closed spec whose frontmatter is not valid YAML.
        let markdown = "---\ndoc-type: [issue\n---\n";

        // Act
        let actual = findings("docs/issues/closed/1-example/ISSUE.md", markdown);

        // Assert: advisory severity covers profile findings only, never syntax.
        assert_eq!(actual, [(DiagnosticCategory::MalformedYaml, Severity::Error)]);
    }

    #[test]
    fn it_should_warn_once_per_experimental_field_after_any_structural_finding() {
        // Arrange: a strict issue with two `x-` fields and a quoted positive integer.
        let body = WRONG_SCALAR_ISSUE.strip_prefix("---\n").unwrap();
        let markdown = format!("---\nx-reviewer: alice\nx-note: draft\n{body}");

        // Act
        let diagnostics = validate_document("docs/issues/open/1-example/ISSUE.md", &markdown, &RepositoryFiles::default());

        // Assert: the structural error comes first, then one warning per field in source order.
        let actual: Vec<_> = diagnostics
            .iter()
            .map(|diagnostic| (diagnostic.category, diagnostic.severity, diagnostic.field_path.as_deref()))
            .collect();
        assert_eq!(
            actual,
            [
                (DiagnosticCategory::WrongScalarType, Severity::Error, Some("github-issue")),
                (DiagnosticCategory::ExperimentalField, Severity::Warning, Some("x-reviewer")),
                (DiagnosticCategory::ExperimentalField, Severity::Warning, Some("x-note")),
            ]
        );
    }

    #[test]
    fn it_should_ignore_the_top_level_metadata_of_externally_governed_files() {
        // Arrange: an Agent Skill in an open spec folder whose external top-level metadata happens to
        // use `doc-type: issue` and an invalid top-level extension.
        let markdown = "---\nname: example\ndescription: Example.\ndoc-type: issue\nsemantic-links: invalid\n---\n";

        // Act
        let actual = findings("docs/issues/open/1-example/SKILL.md", markdown);

        // Assert
        assert_eq!(actual, []);
    }
}
