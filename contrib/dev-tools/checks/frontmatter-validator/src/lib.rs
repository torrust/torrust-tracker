//! Markdown frontmatter extraction and universal-envelope validation.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer};
use serde_yaml::{Mapping, Value};

pub mod profile;

/// A parsed Markdown frontmatter block.
#[derive(Debug, Eq, PartialEq)]
pub struct Frontmatter {
    /// The complete YAML mapping for later profile-specific validation.
    pub values: Mapping,
    /// The original YAML source for strict scalar-style validation.
    pub(crate) yaml: String,
    /// The document's known top-level schema ownership.
    pub(crate) ownership: DocumentOwnership,
    /// The optional semantic-link extension validated for this ownership mode.
    pub semantic_links: Option<SemanticLinks>,
}

/// Ownership of the document's top-level frontmatter schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentOwnership {
    /// Repository-owned or unknown documents use the top-level universal extension.
    Repository,
    /// Agent Skills and agent profiles use only the nested metadata extension.
    External,
}

/// The universal repository-owned semantic-link extension.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct SemanticLinks {
    /// Optional names of repository skills related to this document.
    #[serde(default, deserialize_with = "deserialize_optional_string_sequence")]
    pub skill_links: Option<Vec<String>>,
    /// Optional typed or repository-relative artifacts related to this document.
    #[serde(default, deserialize_with = "deserialize_optional_string_sequence")]
    pub related_artifacts: Option<Vec<String>>,
}

fn deserialize_optional_string_sequence<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let Value::Sequence(_) = value else {
        return Err(D::Error::custom("expected a sequence of strings"));
    };

    serde_yaml::from_value::<Vec<String>>(value)
        .map(Some)
        .map_err(D::Error::custom)
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
    extract_with_ownership(markdown, DocumentOwnership::Repository)
}

/// Extracts frontmatter using the document's known top-level schema ownership.
///
/// Repository-owned documents validate top-level `semantic-links`. Externally governed Agent Skill
/// and agent-profile documents validate only `metadata.semantic-links`.
///
/// # Errors
///
/// Returns the same extraction and envelope diagnostics as [`extract`].
pub fn extract_with_ownership(markdown: &str, ownership: DocumentOwnership) -> Result<Option<Frontmatter>, Diagnostic> {
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
    let semantic_links = semantic_links(&values, ownership)?;

    Ok(Some(Frontmatter {
        values,
        yaml,
        ownership,
        semantic_links,
    }))
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

fn semantic_links(values: &Mapping, ownership: DocumentOwnership) -> Result<Option<SemanticLinks>, Diagnostic> {
    let metadata_key = Value::String(String::from("metadata"));
    if ownership == DocumentOwnership::External {
        return values.get(&metadata_key).and_then(Value::as_mapping).map_or_else(
            || Ok(None),
            |metadata| semantic_links_from(metadata, "metadata.semantic-links"),
        );
    }

    semantic_links_from(values, "semantic-links")
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
    use super::{DiagnosticCategory, DocumentOwnership, SemanticLinks, extract, extract_with_ownership};

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
    fn it_should_reject_a_null_semantic_link_sequence() {
        // Arrange: a universal extension explicitly sets a link sequence to YAML null.
        let markdown = "---\nsemantic-links:\n  skill-links: null\n---\n# Document\n";

        // Act: attempt to extract the frontmatter.
        let error = extract(markdown).unwrap_err();

        // Assert: present universal link fields must be string sequences, not nulls.
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
        let markdown = "---\nname: write-markdown-docs\ndescription: Writes repository Markdown.\nmetadata:\n  semantic-links:\n    skill-links:\n      - write-markdown-docs\n---\n# Skill\n";

        // Act: extract the external-document envelope.
        let frontmatter = extract_with_ownership(markdown, DocumentOwnership::External)
            .unwrap()
            .unwrap();

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
    fn it_should_reject_invalid_semantic_links_from_external_metadata() {
        // Arrange: an external schema has a malformed nested extension.
        let markdown = "---\nname: write-markdown-docs\ndescription: Writes repository Markdown.\nmetadata:\n  semantic-links: invalid\n---\n# Skill\n";

        // Act: extract the external-document envelope.
        let error = extract_with_ownership(markdown, DocumentOwnership::External).unwrap_err();

        // Assert: external ownership still validates its owned nested extension.
        assert_eq!(error.category, DiagnosticCategory::InvalidSemanticLinks);
    }

    #[test]
    fn it_should_keep_an_external_v1_looking_document_permissive() {
        // Arrange: an external schema happens to use strict-profile field names.
        let markdown = "---\nname: external-record\ndescription: An externally governed record.\nschema-version: 1\ndoc-type: issue\nmetadata:\n  semantic-links:\n    skill-links:\n      - write-markdown-docs\n---\n# External\n";
        let frontmatter = extract_with_ownership(markdown, DocumentOwnership::External)
            .unwrap()
            .unwrap();

        // Act: validate the extracted document profile.
        let profile = super::profile::validate(&frontmatter).unwrap();

        // Assert: ownership prevents strict repository-profile interpretation.
        assert!(matches!(profile, Profile::Permissive));
    }

    #[test]
    fn it_should_ignore_top_level_semantic_links_when_external_metadata_exists() {
        // Arrange: an external document carries conflicting top-level and nested extensions.
        let markdown = "---\nname: write-markdown-docs\ndescription: Writes repository Markdown.\nsemantic-links: invalid\nmetadata:\n  semantic-links:\n    related-artifacts:\n      - docs/AGENTS.md\n---\n# Skill\n";

        // Act: extract the external-document envelope.
        let frontmatter = extract_with_ownership(markdown, DocumentOwnership::External)
            .unwrap()
            .unwrap();

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
        let frontmatter = extract_with_ownership(markdown, DocumentOwnership::External)
            .unwrap()
            .unwrap();

        // Assert: external top-level metadata remains outside the v1 validator's ownership.
        assert_eq!(frontmatter.semantic_links, None);
    }

    #[test]
    fn it_should_ignore_top_level_semantic_links_for_an_agent_skill_without_metadata() {
        // Arrange: an externally governed Agent Skill has no nested metadata extension.
        let markdown =
            "---\nname: write-markdown-docs\ndescription: Writes repository Markdown.\nsemantic-links: invalid\n---\n# Skill\n";

        // Act: extract the Agent Skill envelope.
        let frontmatter = extract_with_ownership(markdown, DocumentOwnership::External)
            .unwrap()
            .unwrap();

        // Assert: its external top-level extension remains outside v1 validation ownership.
        assert_eq!(frontmatter.semantic_links, None);
    }

    #[test]
    fn it_should_validate_top_level_semantic_links_for_an_ordinary_document_with_name_and_description() {
        // Arrange: a repository-owned document happens to contain common external-schema fields.
        let markdown = "---\nname: Evidence\ndescription: A repository-owned record.\nsemantic-links: invalid\n---\n# Evidence\n";

        // Act: extract the document under repository ownership.
        let error = extract(markdown).unwrap_err();

        // Assert: content alone cannot bypass universal top-level envelope validation.
        assert_eq!(error.category, DiagnosticCategory::InvalidSemanticLinks);
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
    fn it_should_reject_an_unquoted_strict_timestamp() {
        // Arrange: a v1 EPIC record uses a valid-looking but plain YAML timestamp scalar.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: 2026-09-21 20:35\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the strict contract requires a double-quoted timestamp string.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_accept_a_quoted_strict_timestamp_with_a_trailing_comment() {
        // Arrange: a v1 EPIC record annotates its quoted timestamp with a YAML comment.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026-09-21 20:35\" # updated\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let profile = super::profile::validate(&frontmatter).unwrap();

        // Assert: comments do not change the quoted scalar's v1 validity.
        assert!(matches!(profile, Profile::Epic(_)));
    }

    #[test]
    fn it_should_accept_a_quoted_strict_timestamp_with_a_tab_separated_comment() {
        // Arrange: a v1 EPIC record separates a quoted timestamp and YAML comment with a tab.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026-09-21 20:35\"\t# updated\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let profile = super::profile::validate(&frontmatter).unwrap();

        // Assert: any YAML whitespace before a trailing comment remains valid.
        assert!(matches!(profile, Profile::Epic(_)));
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
    fn it_should_reject_a_non_integer_schema_version_for_a_strict_issue_candidate() {
        // Arrange: an issue-shaped v1 candidate quotes its schema-version integer.
        let markdown = "---\nschema-version: \"1\"\ndoc-type: issue\n---\n# Invalid issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and validate the frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: the schema version must remain an unquoted YAML integer.
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

    #[test]
    fn it_should_reject_a_skill_name_with_consecutive_hyphens() {
        // Arrange: a strict issue uses an empty segment in a hyphen-separated skill name.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 21:10\"\nsemantic-links:\n  skill-links:\n    - write--markdown-docs\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let error = super::profile::validate(&frontmatter).unwrap_err();

        // Assert: every identifier segment must contain lowercase letters or digits.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }
}
