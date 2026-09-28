//! The crate-wide vocabulary for deterministic frontmatter failures.

use serde::Serialize;

/// A category for a deterministic frontmatter extraction or validation failure.
///
/// The kebab-case serialized names are the stable diagnostic contract rendered by the command.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
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
    /// A draft or open issue or EPIC spec does not use the strict v1 frontmatter.
    LegacyShape,
    /// A strict profile contains an `x-` field, which carries no contract semantics.
    ExperimentalField,
    /// A spec's `status` does not match its lifecycle folder.
    LifecycleLocationMismatch,
    /// A spec's `spec-path` is not its own repository path.
    SpecPathMismatch,
    /// A related-artifact repository path names no tracked file or directory.
    MissingArtifact,
    /// A skill link names no repository skill.
    UnknownSkill,
}

/// Whether a diagnostic fails validation or is advisory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// A failure that makes the command exit with a validation error.
    Error,
    /// An advisory finding that does not change the command's exit status.
    Warning,
}

/// A deterministic failure found while extracting or validating frontmatter.
#[derive(Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// The stable category rendered by the command.
    pub category: DiagnosticCategory,
    /// Whether the failure is an error or advisory; the library reports every failure as an error.
    pub severity: Severity,
    /// The dotted YAML path of the offending field, when the failure concerns one field.
    pub field_path: Option<String>,
    /// A human-readable description of the failure.
    pub message: String,
}

impl Diagnostic {
    pub(crate) fn new(category: DiagnosticCategory, message: impl Into<String>) -> Self {
        Self {
            category,
            severity: Severity::Error,
            field_path: None,
            message: message.into(),
        }
    }

    pub(crate) fn for_field(category: DiagnosticCategory, field_path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field_path: Some(field_path.into()),
            ..Self::new(category, message)
        }
    }

    pub(crate) fn with_severity(self, severity: Severity) -> Self {
        Self { severity, ..self }
    }
}

#[cfg(test)]
mod tests {
    // Owns the serialized category and severity names that form the command's diagnostic contract.

    use rstest::rstest;

    use super::{DiagnosticCategory, Severity};

    /// The contract names, independent of serde. No wildcard arm, so a new category fails to compile
    /// here until it has a name and a case below.
    const fn contract_name(category: DiagnosticCategory) -> &'static str {
        match category {
            DiagnosticCategory::UnclosedDelimiter => "unclosed-delimiter",
            DiagnosticCategory::MalformedYaml => "malformed-yaml",
            DiagnosticCategory::NonMappingRoot => "non-mapping-root",
            DiagnosticCategory::InvalidSemanticLinks => "invalid-semantic-links",
            DiagnosticCategory::UnknownField => "unknown-field",
            DiagnosticCategory::MissingRequiredField => "missing-required-field",
            DiagnosticCategory::WrongScalarType => "wrong-scalar-type",
            DiagnosticCategory::InvalidAllowedValue => "invalid-allowed-value",
            DiagnosticCategory::InvalidFieldValue => "invalid-field-value",
            DiagnosticCategory::InvalidReferenceSyntax => "invalid-reference-syntax",
            DiagnosticCategory::LegacyShape => "legacy-shape",
            DiagnosticCategory::ExperimentalField => "experimental-field",
            DiagnosticCategory::LifecycleLocationMismatch => "lifecycle-location-mismatch",
            DiagnosticCategory::SpecPathMismatch => "spec-path-mismatch",
            DiagnosticCategory::MissingArtifact => "missing-artifact",
            DiagnosticCategory::UnknownSkill => "unknown-skill",
        }
    }

    #[rstest]
    #[case::unclosed_delimiter(DiagnosticCategory::UnclosedDelimiter)]
    #[case::malformed_yaml(DiagnosticCategory::MalformedYaml)]
    #[case::non_mapping_root(DiagnosticCategory::NonMappingRoot)]
    #[case::invalid_semantic_links(DiagnosticCategory::InvalidSemanticLinks)]
    #[case::unknown_field(DiagnosticCategory::UnknownField)]
    #[case::missing_required_field(DiagnosticCategory::MissingRequiredField)]
    #[case::wrong_scalar_type(DiagnosticCategory::WrongScalarType)]
    #[case::invalid_allowed_value(DiagnosticCategory::InvalidAllowedValue)]
    #[case::invalid_field_value(DiagnosticCategory::InvalidFieldValue)]
    #[case::invalid_reference_syntax(DiagnosticCategory::InvalidReferenceSyntax)]
    #[case::legacy_shape(DiagnosticCategory::LegacyShape)]
    #[case::experimental_field(DiagnosticCategory::ExperimentalField)]
    #[case::lifecycle_location_mismatch(DiagnosticCategory::LifecycleLocationMismatch)]
    #[case::spec_path_mismatch(DiagnosticCategory::SpecPathMismatch)]
    #[case::missing_artifact(DiagnosticCategory::MissingArtifact)]
    #[case::unknown_skill(DiagnosticCategory::UnknownSkill)]
    fn it_should_serialize_each_category_as_its_stable_kebab_case_name(#[case] category: DiagnosticCategory) {
        // Act: serialize the category as the command will.
        let actual = serde_json::to_value(category).unwrap();

        // Assert: the rendered name is the independently specified contract name.
        assert_eq!(actual, contract_name(category));
    }

    #[rstest]
    #[case::error(Severity::Error, "error")]
    #[case::warning(Severity::Warning, "warning")]
    fn it_should_serialize_each_severity_as_its_stable_lowercase_name(#[case] severity: Severity, #[case] expected: &str) {
        // Act: serialize the severity as the command will.
        let actual = serde_json::to_value(severity).unwrap();

        // Assert: the rendered name is the independently specified contract name.
        assert_eq!(actual, expected);
    }
}
