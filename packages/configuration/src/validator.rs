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
#[non_exhaustive]
pub enum SemanticValidationError {
    #[error("Private mode section in configuration can only be included when the tracker is running in private mode.")]
    UselessPrivateModeSection,

    #[error(
        "application.shutdown.{slowest_drain_key} ({slowest_drain_in_secs} s) plus application.shutdown.event_listener_drain_in_secs ({event_listener_drain_in_secs} s) must be less than application.shutdown.deadline_in_secs ({deadline_in_secs} s): event listeners are stopped only after the HTTP and UDP servers finish draining, so both stages must fit in the deadline. Lower the drain budgets or raise the deadline (and the external stop timeout of your container runtime or service manager with it)."
    )]
    ShutdownBudgetsExceedDeadline {
        /// Key of the longer of the two server drain budgets.
        slowest_drain_key: &'static str,
        slowest_drain_in_secs: u64,
        event_listener_drain_in_secs: u64,
        deadline_in_secs: u64,
    },
}

pub trait Validator {
    /// # Errors
    ///
    /// Will return an error if the configuration is invalid.
    fn validate(&self) -> Result<(), SemanticValidationError>;
}
