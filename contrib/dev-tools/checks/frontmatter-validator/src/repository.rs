//! Location-dependent validation policy over repository-relative paths.
//!
//! These decisions are pure: they take a path and the document text, never touch the filesystem
//! or git, and return every diagnostic for the document in contract order.

use serde_yaml::{Mapping, Value};

use crate::profile::{self, Profile};
use crate::{Diagnostic, DiagnosticCategory, DocumentOwnership, Severity, extract_with_ownership};

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
/// warnings.
#[must_use]
pub fn validate_document(path: &str, markdown: &str) -> Vec<Diagnostic> {
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
    match profile::validate(&frontmatter) {
        Ok(Profile::Issue(_) | Profile::Epic(_)) => {}
        Ok(Profile::Permissive) if ownership == DocumentOwnership::External => return diagnostics,
        Ok(Profile::Permissive) => {
            diagnostics.extend(legacy_shape(path, location, Some(&frontmatter.values)));
            return diagnostics;
        }
        Err(diagnostic) => diagnostics.push(for_location(diagnostic, location)),
    }
    diagnostics.extend(experimental_fields(&frontmatter.values));
    diagnostics
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
             in the frontmatter-validator README.",
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
    // legacy-shape rule, and experimental-field warnings. Structural profile rules are owned by
    // `profile`.

    use rstest::rstest;

    use super::{Location, ownership, validate_document};
    use crate::{DiagnosticCategory, DocumentOwnership, Severity};

    const VALID_ISSUE: &str = include_str!("../fixtures/accepted/issue.md");
    const WRONG_SCALAR_ISSUE: &str = include_str!("../fixtures/rejected/issue-wrong-scalar.md");
    const LEGACY_ISSUE: &str = "---\ndoc-type: issue\nstatus: planned\n---\n# Legacy\n";
    const LEGACY_EPIC: &str = "---\ndoc-type: epic\nstatus: planned\n---\n# Legacy\n";
    const EVIDENCE: &str = "---\ndoc-type: manual-verification-evidence\n---\n# Evidence\n";
    const PLAIN: &str = "# Plain document\n";

    /// The observable identity of each diagnostic, in order.
    fn findings(path: &str, markdown: &str) -> Vec<(DiagnosticCategory, Severity)> {
        validate_document(path, markdown)
            .into_iter()
            .map(|diagnostic| (diagnostic.category, diagnostic.severity))
            .collect()
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
    #[case::open_strict_v1_primary("docs/issues/open/1-example/ISSUE.md", VALID_ISSUE)]
    fn it_should_not_require_the_v1_shape_outside_draft_and_open_specs(#[case] path: &str, #[case] markdown: &str) {
        assert_eq!(findings(path, markdown), []);
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
        let diagnostics = validate_document("docs/issues/open/1-example/ISSUE.md", &markdown);

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
