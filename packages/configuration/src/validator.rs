//! Trait to validate semantic errors.
//!
//! Errors could involve more than one configuration option. Some configuration
//! combinations can be incompatible.
use thiserror::Error;

/// Errors that can occur validating the configuration.
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
