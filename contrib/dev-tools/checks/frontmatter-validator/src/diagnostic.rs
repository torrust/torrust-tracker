//! The crate-wide vocabulary for deterministic frontmatter failures.

/// A category for a deterministic frontmatter extraction or validation failure.
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

/// A deterministic failure found while extracting or validating frontmatter.
#[derive(Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// The stable category used by the future command adapter.
    pub category: DiagnosticCategory,
    /// A human-readable description of the failure.
    pub message: String,
}

impl Diagnostic {
    pub(crate) fn new(category: DiagnosticCategory, message: impl Into<String>) -> Self {
        Self {
            category,
            message: message.into(),
        }
    }
}
