//! Persistence requirements owned by application bootstrap.
//!
//! The check is intentionally not called while the active runtime uses v2
//! configuration and its temporary database compatibility bridge. The
//! persistence-free runtime activation follow-up invokes it once bootstrap
//! receives the actual v3 configuration.
use torrust_tracker_configuration::v3_0_0::core::Core;

/// An enabled capability whose persistence requirement is unmet.
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum PersistenceRequirementError {
    /// Listing needs the whitelist persistence store.
    #[error("Configuration requires persistence for `core.listed`, but `[core.database]` is missing.")]
    ListedRequiresDatabase,

    /// Private mode needs the authentication-key persistence store.
    #[error("Configuration requires persistence for `core.private`, but `[core.database]` is missing.")]
    PrivateRequiresDatabase,

    /// Persistent completed metrics need the torrent-metrics persistence store.
    #[error(
        "Configuration requires persistence for `core.tracker_policy.persistent_torrent_completed_stat`, but `[core.database]` is missing."
    )]
    PersistentTorrentCompletedStatRequiresDatabase,
}

/// Validates persistence requirements induced by enabled tracker capabilities.
///
/// # Errors
///
/// Returns the first enabled capability that requires persistence when the v3
/// configuration omits `[core.database]`.
pub const fn validate_persistence_requirements(core: &Core) -> Result<(), PersistenceRequirementError> {
    if core.database.is_some() {
        return Ok(());
    }

    if core.listed {
        return Err(PersistenceRequirementError::ListedRequiresDatabase);
    }

    if core.private {
        return Err(PersistenceRequirementError::PrivateRequiresDatabase);
    }

    if core.tracker_policy.persistent_torrent_completed_stat {
        return Err(PersistenceRequirementError::PersistentTorrentCompletedStatRequiresDatabase);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use torrust_tracker_configuration::v3_0_0::core::Core;

    use super::{PersistenceRequirementError, validate_persistence_requirements};

    #[test]
    fn it_should_reject_listing_without_a_database() {
        // Arrange
        let core = Core {
            listed: true,
            ..Core::default()
        };

        // Act
        let result = validate_persistence_requirements(&core);

        // Assert
        let error = result.expect_err("listing should require persistence");
        assert_eq!(error, PersistenceRequirementError::ListedRequiresDatabase);
        assert_eq!(
            error.to_string(),
            "Configuration requires persistence for `core.listed`, but `[core.database]` is missing."
        );
    }

    #[test]
    fn it_should_reject_private_mode_without_a_database() {
        // Arrange
        let core = Core {
            private: true,
            ..Core::default()
        };

        // Act
        let result = validate_persistence_requirements(&core);

        // Assert
        let error = result.expect_err("private mode should require persistence");
        assert_eq!(error, PersistenceRequirementError::PrivateRequiresDatabase);
        assert_eq!(
            error.to_string(),
            "Configuration requires persistence for `core.private`, but `[core.database]` is missing."
        );
    }

    #[test]
    fn it_should_reject_persistent_completed_metrics_without_a_database() {
        // Arrange
        let mut core = Core::default();
        core.tracker_policy.persistent_torrent_completed_stat = true;

        // Act
        let result = validate_persistence_requirements(&core);

        // Assert
        let error = result.expect_err("persistent completed metrics should require persistence");
        assert_eq!(
            error,
            PersistenceRequirementError::PersistentTorrentCompletedStatRequiresDatabase
        );
        assert_eq!(
            error.to_string(),
            "Configuration requires persistence for `core.tracker_policy.persistent_torrent_completed_stat`, but `[core.database]` is missing."
        );
    }

    #[test]
    fn it_should_allow_persistence_free_core_configuration() {
        // Arrange
        let core = Core::default();

        // Act
        let result = validate_persistence_requirements(&core);

        // Assert
        assert!(result.is_ok());
    }
}
