// adr: docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
// issue: #2668
//! Application-level configuration for schema v3.
//!
//! The `[application]` table holds settings that belong to the tracker process as a whole, not to one
//! service: they are applied by the composition root (`src/app.rs` and `src/bootstrap/`) and by the
//! process entry point. Settings of one listener stay on that listener, and settings of shared
//! tracker state stay in `[core]`. See `docs/architecture/tracker-instance-architecture.md`.
//!
//! Today the only sub-table is [`shutdown`](crate::v3_0_0::shutdown::Shutdown).
use serde::{Deserialize, Serialize};

use crate::v3_0_0::shutdown::Shutdown;
use crate::validator::{SemanticValidationError, Validator};

/// The `[application]` table. Every key and sub-table is optional.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Application {
    /// How long each stage of a shutdown may take.
    #[serde(default)]
    pub shutdown: Shutdown,
}

impl Validator for Application {
    fn validate(&self) -> Result<(), SemanticValidationError> {
        self.shutdown.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::Application;
    use crate::v3_0_0::shutdown::Shutdown;

    #[test]
    fn it_should_use_the_default_shutdown_budgets_when_the_table_is_omitted() {
        // Arrange
        let empty_table = "";

        // Act
        let application: Application = toml::from_str(empty_table).expect("an empty table should deserialize");

        // Assert
        assert_eq!(application.shutdown, Shutdown::default());
    }

    #[test]
    fn it_should_reject_an_unknown_key() {
        // Arrange
        let misspelled_table = "[shutdown_policy]\ndeadline_in_secs = 30";

        // Act
        let result = toml::from_str::<Application>(misspelled_table);

        // Assert
        assert!(result.is_err(), "a table that is not an application setting must be rejected");
    }
}
