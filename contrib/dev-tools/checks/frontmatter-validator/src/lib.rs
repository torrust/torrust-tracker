//! Markdown frontmatter extraction and universal-envelope validation.

use serde::Deserialize;
use serde_yaml::{Mapping, Value};

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
    let key = Value::String(String::from("semantic-links"));
    let Some(value) = values.get(&key) else {
        return Ok(None);
    };
    let semantic_links = serde_yaml::from_value::<SemanticLinks>(value.clone()).map_err(|error| Diagnostic {
        category: DiagnosticCategory::InvalidSemanticLinks,
        message: format!("`semantic-links` must be a mapping with string sequences: {error}"),
    })?;

    Ok(Some(semantic_links))
}

#[cfg(test)]
mod tests {
    use super::{DiagnosticCategory, SemanticLinks, extract};

    // Extraction owns delimiter, YAML, mapping-root, and universal-envelope shape decisions.
    // Strict profiles, reference syntax, external metadata, and repository-aware checks are
    // deliberately deferred to issue #2266 tasks T4-T7.

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
}
