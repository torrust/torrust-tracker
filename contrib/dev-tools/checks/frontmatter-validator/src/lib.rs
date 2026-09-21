//! Markdown frontmatter extraction and universal-envelope validation.

use serde::Deserialize;
use serde_yaml::{Mapping, Value};

pub mod profile;

/// A parsed Markdown frontmatter block.
#[derive(Debug, Eq, PartialEq)]
pub struct Frontmatter {
    /// The complete YAML mapping for later profile-specific validation.
    pub values: Mapping,
    /// The optional repository-owned universal metadata extension.
    pub semantic_links: Option<SemanticLinks>,
}

/// The universal repository-owned semantic-link extension.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct SemanticLinks {
    /// Optional names of repository skills related to this document.
    pub skill_links: Option<Vec<String>>,
    /// Optional typed or repository-relative artifacts related to this document.
    pub related_artifacts: Option<Vec<String>>,
}

/// A category for frontmatter extraction or universal-envelope failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticCategory {
    /// An opening delimiter did not have a matching closing delimiter.
    UnclosedDelimiter,
    /// The delimited content was not valid YAML.
    MalformedYaml,
    /// The YAML root value was not a mapping.
    NonMappingRoot,
    /// The universal `semantic-links` extension had an invalid shape.
    InvalidSemanticLinks,
    /// A strict profile contains an unprefixed field outside its contract.
    UnknownField,
    /// A strict profile omits a required field.
    MissingRequiredField,
    /// A strict profile field does not have its required YAML scalar type.
    WrongScalarType,
    /// A strict profile field has a scalar value outside its allowed set.
    InvalidAllowedValue,
    /// A strict profile field violates a non-enumerated value invariant.
    InvalidFieldValue,
    /// A strict profile reference does not use an approved provisional syntax.
    InvalidReferenceSyntax,
}

/// A deterministic failure found while extracting frontmatter.
#[derive(Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// The stable category used by the future command adapter.
    pub category: DiagnosticCategory,
    /// A human-readable description of the failure.
    pub message: String,
}

/// Extracts an optional opening frontmatter block and validates its universal envelope.
///
/// Documents without an opening `---` delimiter have no frontmatter and are accepted. Profile
/// validation decides later whether a particular document requires frontmatter.
///
/// # Errors
///
/// Returns a diagnostic when the opening delimiter is unclosed, the YAML is malformed, the YAML
/// root is not a mapping, or `semantic-links` does not match its universal extension shape.
pub fn extract(markdown: &str) -> Result<Option<Frontmatter>, Diagnostic> {
    let Some(yaml) = delimited_yaml(markdown)? else {
        return Ok(None);
    };
    let value = serde_yaml::from_str::<Value>(&yaml).map_err(|error| Diagnostic {
        category: DiagnosticCategory::MalformedYaml,
        message: error.to_string(),
    })?;
    let Value::Mapping(values) = value else {
        return Err(Diagnostic {
            category: DiagnosticCategory::NonMappingRoot,
            message: String::from("Markdown frontmatter must have a YAML mapping root."),
        });
    };
    let semantic_links = semantic_links(&values)?;

    Ok(Some(Frontmatter { values, semantic_links }))
}

fn delimited_yaml(markdown: &str) -> Result<Option<String>, Diagnostic> {
    let mut lines = markdown.lines();
    if lines.next() != Some("---") {
        return Ok(None);
    }

    let mut yaml = String::new();
    for line in lines {
        if line == "---" {
            return Ok(Some(yaml));
        }
        yaml.push_str(line);
        yaml.push('\n');
    }

    Err(Diagnostic {
        category: DiagnosticCategory::UnclosedDelimiter,
        message: String::from("Markdown frontmatter opening delimiter has no closing delimiter."),
    })
}

fn semantic_links(values: &Mapping) -> Result<Option<SemanticLinks>, Diagnostic> {
    let metadata_key = Value::String(String::from("metadata"));
    if is_externally_governed_document(values) {
        return values.get(&metadata_key).and_then(Value::as_mapping).map_or_else(
            || Ok(None),
            |metadata| semantic_links_from(metadata, "metadata.semantic-links"),
        );
    }

    semantic_links_from(values, "semantic-links")
}

fn is_externally_governed_document(values: &Mapping) -> bool {
    let metadata_key = Value::String(String::from("metadata"));
    let has_name = has_string_field(values, "name");
    let is_agent_skill = has_name && values.get(&metadata_key).is_some_and(Value::is_mapping);
    let is_agent_profile = has_name
        && has_string_field(values, "description")
        && (values.contains_key(Value::String(String::from("tools")))
            || values.contains_key(Value::String(String::from("argument-hint"))));

    is_agent_skill || is_agent_profile
}

fn has_string_field(values: &Mapping, field: &str) -> bool {
    values.get(Value::String(String::from(field))).is_some_and(Value::is_string)
}

fn semantic_links_from(values: &Mapping, field_path: &str) -> Result<Option<SemanticLinks>, Diagnostic> {
    let key = Value::String(String::from("semantic-links"));
    let Some(value) = values.get(&key) else {
        return Ok(None);
    };
    let semantic_links = serde_yaml::from_value::<SemanticLinks>(value.clone()).map_err(|error| Diagnostic {
        category: DiagnosticCategory::InvalidSemanticLinks,
        message: format!("`{field_path}` must be a mapping with string sequences: {error}"),
    })?;

    Ok(Some(semantic_links))
}

#[cfg(test)]
mod tests {
    use super::profile::Profile;
    use super::{DiagnosticCategory, SemanticLinks, extract};

    // Extraction owns delimiter, YAML, mapping-root, and universal-envelope shape decisions.
    // Strict profile parsing owns the v1 structural and reference-syntax decisions. External
    // metadata and repository-aware checks remain deferred to issue #2266 tasks T4-T7.

    #[test]
    fn it_should_accept_a_document_without_frontmatter() {
        // Arrange: a Markdown document has no opening frontmatter delimiter.
        let markdown = "# Plain document\n";

        // Act: extract its frontmatter.
        let frontmatter = extract(markdown).unwrap();

        // Assert: the document has no frontmatter rather than an extraction failure.
        assert_eq!(frontmatter, None);
    }

    #[test]
    fn it_should_extract_a_universal_semantic_links_envelope() {
        // Arrange: a Markdown document provides both supported universal semantic-link sequences.
        let markdown = "---\nsemantic-links:\n  skill-links:\n    - write-markdown-docs\n  related-artifacts:\n    - docs/AGENTS.md\n---\n# Document\n";

        // Act: extract and validate its universal envelope.
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Assert: the envelope retains the supported semantic-link values.
        assert_eq!(
            frontmatter.semantic_links,
            Some(SemanticLinks {
                skill_links: Some(vec![String::from("write-markdown-docs")]),
                related_artifacts: Some(vec![String::from("docs/AGENTS.md")]),
            })
        );
    }

    #[test]
    fn it_should_reject_an_unclosed_frontmatter_delimiter() {
        // Arrange: a Markdown document opens frontmatter without closing it.
        let markdown = "---\ndoc-type: issue\n";

        // Act: attempt to extract the frontmatter.
        let error = extract(markdown).unwrap_err();

        // Assert: the diagnostic identifies the missing closing delimiter.
        assert_eq!(error.category, DiagnosticCategory::UnclosedDelimiter);
    }

    #[test]
    fn it_should_reject_malformed_yaml() {
        // Arrange: a Markdown document contains syntactically invalid YAML frontmatter.
        let markdown = "---\ndoc-type: [issue\n---\n# Document\n";

        // Act: attempt to extract the frontmatter.
        let error = extract(markdown).unwrap_err();

        // Assert: the diagnostic identifies the invalid YAML content.
        assert_eq!(error.category, DiagnosticCategory::MalformedYaml);
    }

    #[test]
    fn it_should_reject_a_non_mapping_yaml_root() {
        // Arrange: a Markdown document contains a YAML sequence instead of a mapping.
        let markdown = "---\n- item\n---\n# Document\n";

        // Act: attempt to extract the frontmatter.
        let error = extract(markdown).unwrap_err();

        // Assert: the diagnostic identifies the universal envelope root violation.
        assert_eq!(error.category, DiagnosticCategory::NonMappingRoot);
    }

    #[test]
    fn it_should_reject_non_string_semantic_link_entries() {
        // Arrange: a semantic-link sequence contains an integer instead of a string.
        let markdown = "---\nsemantic-links:\n  skill-links:\n    - 2265\n---\n# Document\n";

        // Act: attempt to extract the frontmatter.
        let error = extract(markdown).unwrap_err();

        // Assert: the diagnostic identifies the malformed universal extension.
        assert_eq!(error.category, DiagnosticCategory::InvalidSemanticLinks);
    }

    #[test]
    fn it_should_reject_a_scalar_semantic_links_value() {
        // Arrange: the universal extension is a scalar instead of its required mapping.
        let markdown = "---\nsemantic-links: write-markdown-docs\n---\n# Document\n";

        // Act: attempt to extract the frontmatter.
        let error = extract(markdown).unwrap_err();

        // Assert: the diagnostic identifies the malformed universal extension.
        assert_eq!(error.category, DiagnosticCategory::InvalidSemanticLinks);
    }

    #[test]
    fn it_should_reject_an_unknown_semantic_links_field() {
        // Arrange: the universal extension contains a field outside the approved envelope.
        let markdown = "---\nsemantic-links:\n  unsupported: value\n---\n# Document\n";

        // Act: attempt to extract the frontmatter.
        let error = extract(markdown).unwrap_err();

        // Assert: the diagnostic identifies the unsupported universal field.
        assert_eq!(error.category, DiagnosticCategory::InvalidSemanticLinks);
    }

    #[test]
    fn it_should_read_semantic_links_from_external_metadata() {
        // Arrange: an externally governed document provides semantic links under metadata.
        let markdown = "---\nname: write-markdown-docs\nmetadata:\n  semantic-links:\n    skill-links:\n      - write-markdown-docs\n---\n# Skill\n";

        // Act: extract the external-document envelope.
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Assert: the nested extension is parsed without interpreting external top-level fields.
        assert_eq!(
            frontmatter.semantic_links,
            Some(SemanticLinks {
                skill_links: Some(vec![String::from("write-markdown-docs")]),
                related_artifacts: None,
            })
        );
    }

    #[test]
    fn it_should_ignore_top_level_semantic_links_when_external_metadata_exists() {
        // Arrange: an external document carries conflicting top-level and nested extensions.
        let markdown = "---\nname: write-markdown-docs\nsemantic-links: invalid\nmetadata:\n  semantic-links:\n    related-artifacts:\n      - docs/AGENTS.md\n---\n# Skill\n";

        // Act: extract the external-document envelope.
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Assert: only the nested externally owned extension determines v1 semantics.
        assert_eq!(
            frontmatter.semantic_links,
            Some(SemanticLinks {
                skill_links: None,
                related_artifacts: Some(vec![String::from("docs/AGENTS.md")]),
            })
        );
    }

    #[test]
    fn it_should_ignore_top_level_semantic_links_for_an_agent_profile_without_metadata() {
        // Arrange: an externally governed agent profile has only a legacy top-level extension.
        let markdown = "---\nname: Implementer\ndescription: Implements repository changes.\ntools: [execute, read]\nsemantic-links: invalid\n---\n# Agent\n";

        // Act: extract the agent-profile envelope.
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Assert: external top-level metadata remains outside the v1 validator's ownership.
        assert_eq!(frontmatter.semantic_links, None);
    }

    #[test]
    fn it_should_classify_the_accepted_issue_fixture_as_a_strict_issue_profile() {
        // Arrange: the predecessor's accepted issue fixture declares schema version one.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/accepted/issue.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and structurally validate the parsed frontmatter.
        let profile = super::profile::validate(&frontmatter).unwrap();

        // Assert: the canonical model recognizes the strict issue contract.
        assert!(matches!(profile, Profile::Issue(_)));
    }

    #[test]
    fn it_should_classify_the_accepted_epic_fixture_as_a_strict_epic_profile() {
        // Arrange: the predecessor's accepted EPIC fixture declares schema version one.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/accepted/epic.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and structurally validate the parsed frontmatter.
        let profile = super::profile::validate(&frontmatter).unwrap();

        // Assert: the canonical model recognizes the strict EPIC contract.
        assert!(matches!(profile, Profile::Epic(_)));
    }

    #[test]
    fn it_should_reject_the_wrong_scalar_fixture() {
        // Arrange: the predecessor fixture quotes a positive-integer issue identifier.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-wrong-scalar.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic distinguishes the exact scalar-type violation.
        assert_eq!(error.category, DiagnosticCategory::WrongScalarType);
    }

    #[test]
    fn it_should_reject_the_unknown_field_fixture() {
        // Arrange: the predecessor fixture introduces an unprefixed strict-profile field.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-unknown-field.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the unsupported field.
        assert_eq!(error.category, DiagnosticCategory::UnknownField);
    }

    #[test]
    fn it_should_reject_the_invalid_status_fixture() {
        // Arrange: the predecessor fixture uses a status outside the issue lifecycle enum.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-invalid-status.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the disallowed lifecycle value.
        assert_eq!(error.category, DiagnosticCategory::InvalidAllowedValue);
    }

    #[test]
    fn it_should_keep_a_legacy_issue_record_permissive() {
        // Arrange: an issue record has no v1 schema version and an otherwise incomplete shape.
        let markdown = "---\ndoc-type: issue\nstatus: planned\n---\n# Legacy issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and validate the legacy record.
        let profile = super::profile::validate(&frontmatter).unwrap();

        // Assert: v1 strict requirements do not rewrite historical compatibility behavior.
        assert_eq!(profile, Profile::Permissive);
    }

    #[test]
    fn it_should_reject_a_non_positive_strict_issue_identifier() {
        // Arrange: a v1 issue record supplies zero for a positive-only GitHub issue identifier.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 0\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:05\"\nsemantic-links: {}\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict issue profile.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the positive-integer invariant.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_an_invalid_strict_timestamp_shape() {
        // Arrange: a v1 EPIC record has a timestamp outside the UTC-minute format.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026/09/21\"\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the required UTC-minute string format.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_an_impossible_strict_timestamp() {
        // Arrange: a v1 EPIC record has syntactically shaped but impossible calendar and clock values.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026-99-99 99:99\"\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic rejects out-of-range date and time components.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_a_traversal_strict_specification_path() {
        // Arrange: a v1 issue record uses a parent-directory traversal as its specification path.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: ../outside/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links: {}\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict issue profile.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic requires a repository-relative specification path.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_a_non_string_document_type_for_a_v1_record() {
        // Arrange: a v1 record supplies an integer instead of a document-type string.
        let markdown = "---\nschema-version: 1\ndoc-type: 2266\n---\n# Invalid record\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and validate the frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the strict-profile dispatcher identifies the scalar-type violation.
        assert_eq!(error.category, DiagnosticCategory::WrongScalarType);
    }

    #[test]
    fn it_should_reject_the_invalid_reference_fixture() {
        // Arrange: the predecessor fixture uses an absolute URL as a related artifact.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-invalid-reference.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the provisional reference syntax violation.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }

    #[test]
    fn it_should_accept_all_provisional_related_artifact_forms() {
        // Arrange: a strict issue uses a repository path, issue reference, and review finding.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  skill-links:\n    - write-markdown-docs\n  related-artifacts:\n    - Cargo.toml\n    - issue #2264\n    - review-finding:pr-2230-f1\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let profile = super::profile::validate(&frontmatter).unwrap();

        // Assert: every approved provisional reference form remains accepted.
        assert!(matches!(profile, Profile::Issue(_)));
    }

    #[test]
    fn it_should_reject_an_unapproved_tagged_related_artifact() {
        // Arrange: a strict issue uses a tagged artifact form outside the frozen v1 union.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  related-artifacts:\n    - adr:frontmatter\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic reserves typed forms for the approved v1 union.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }

    #[test]
    fn it_should_reject_an_invalid_skill_name() {
        // Arrange: a strict issue uses uppercase letters in a frozen skill-name reference.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  skill-links:\n    - Write-Markdown-Docs\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the frozen skill-name syntax violation.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }
}
