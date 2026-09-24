//! Markdown frontmatter extraction and universal-envelope validation.
//!
//! Diagnostics live in [`diagnostic`] and strict v1 profiles in [`profile`].

use serde::de::Error as _;
use serde::{Deserialize, Deserializer};
use serde_yaml::{Mapping, Value};

pub mod diagnostic;
pub mod profile;
mod syntax;

pub use diagnostic::{Diagnostic, DiagnosticCategory};
pub use profile::{v1_schema, v1_schema_json};

/// A parsed Markdown frontmatter block.
#[derive(Debug, Eq, PartialEq)]
pub struct Frontmatter {
    /// The complete YAML mapping for later profile-specific validation.
    pub values: Mapping,
    /// The original YAML source for strict scalar-style validation.
    yaml: String,
    /// The document's known top-level schema ownership.
    pub(crate) ownership: DocumentOwnership,
    /// The optional semantic-link extension validated for this ownership mode.
    pub semantic_links: Option<SemanticLinks>,
}

impl Frontmatter {
    /// Whether the top-level `field` scalar was written with double quotes in the YAML source.
    ///
    /// YAML parsing erases scalar style, so this is the only place that still sees it.
    pub(crate) fn has_double_quoted_scalar(&self, field: &str) -> bool {
        self.yaml
            .lines()
            .find_map(|line| line.strip_prefix(field)?.trim_start().strip_prefix(':'))
            .map(str::trim)
            .map(strip_yaml_comment)
            .is_some_and(|value| value.starts_with('"') && value.ends_with('"'))
    }
}

fn strip_yaml_comment(value: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;

    for (index, character) in value.char_indices() {
        if let Some(quote_character) = quote {
            if quote_character == '"' {
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == quote_character {
                    quote = None;
                }
            } else if character == quote_character {
                quote = None;
            }
            continue;
        }

        if matches!(character, '"' | '\'') {
            quote = Some(character);
        } else if character == '#' && value[..index].chars().last().is_some_and(char::is_whitespace) {
            return value[..index].trim_end();
        }
    }

    value
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
    let value = serde_yaml::from_str::<Value>(&yaml)
        .map_err(|error| Diagnostic::new(DiagnosticCategory::MalformedYaml, error.to_string()))?;
    let Value::Mapping(values) = value else {
        return Err(Diagnostic::new(
            DiagnosticCategory::NonMappingRoot,
            "Markdown frontmatter must have a YAML mapping root.",
        ));
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

    Err(Diagnostic::new(
        DiagnosticCategory::UnclosedDelimiter,
        "Markdown frontmatter opening delimiter has no closing delimiter.",
    ))
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
    let semantic_links = serde_yaml::from_value::<SemanticLinks>(value.clone()).map_err(|error| {
        Diagnostic::new(
            DiagnosticCategory::InvalidSemanticLinks,
            format!("`{field_path}` must be a mapping with string sequences: {error}"),
        )
    })?;

    Ok(Some(semantic_links))
}

#[cfg(test)]
mod tests {
    // Owns delimiter, YAML, mapping-root, scalar-style, and universal-envelope extraction decisions.

    use super::{DiagnosticCategory, DocumentOwnership, SemanticLinks, extract, extract_with_ownership};

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
    fn it_should_detect_whether_a_top_level_scalar_was_double_quoted() {
        // Arrange: each row writes the same top-level scalar with a different YAML style.
        let cases = [
            ("---\nlast-updated-utc: \"2026-09-21 20:35\"\n---\n", true),
            ("---\nlast-updated-utc: 2026-09-21 20:35\n---\n", false),
            ("---\nlast-updated-utc: \"2026-09-21 20:35\"\t# updated\n---\n", true),
            ("---\nlast-updated-utc : \"2026-09-21 20:35\"\n---\n", true),
            ("---\nlast-updated-utc: \"2026-09-21 20:35 # updated\"\n---\n", true),
            ("---\nlast-updated-utc-draft: \"2026-09-21 20:35\"\n---\n", false),
        ];

        for (markdown, expected) in cases {
            // Act: ask the extracted frontmatter about the scalar's source style.
            let frontmatter = extract(markdown).unwrap().unwrap();
            let actual = frontmatter.has_double_quoted_scalar("last-updated-utc");

            // Assert: only an exact field name followed by a double-quoted value counts.
            assert_eq!(actual, expected, "unexpected style result for {markdown:?}");
        }
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
}
