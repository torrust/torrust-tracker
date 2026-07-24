// adr: docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
// code-review: Rename `SemanticValidationError` and `Validator` to configuration-consistency names
// when a coordinated public API migration is scheduled. See the ADR above.
//! Trait to validate cross-field configuration consistency.
//!
//! Errors could involve more than one configuration option. Some configuration
//! combinations can be incompatible.
use thiserror::Error;

/// Errors that can occur while validating cross-field configuration consistency.
#[derive(Error, Debug)]
pub enum SemanticValidationError {
    #[error("Private mode section in configuration can only be included when the tracker is running in private mode.")]
    UselessPrivateModeSection,
}

pub trait Validator {
    /// # Errors
    ///
    /// Will return an error if the configuration is invalid.
    fn validate(&self) -> Result<(), SemanticValidationError>;
}
