//! Setup for the main tracker application.
//!
//! The [`setup`] only builds the application and its dependencies but it does not start the application.
//! In fact, there is no such thing as the main application process. When the application starts, the only thing it does is
//! starting a bunch of independent jobs. If you are looking for how things are started you should read [`app::start`](crate::app::start)
//! function documentation.
//!
//! Setup steps:
//!
//! 1. Load the global application configuration.
//! 2. Initialize static variables.
//! 3. Initialize logging.
//! 4. Initialize the domain tracker.
use torrust_tracker_configuration::v3_0_0::{Configuration, logging};
use torrust_tracker_configuration::validator::Validator;
use torrust_tracker_udp_core::crypto::keys::{self, Keeper as _};
use tracing::instrument;

use super::config::initialize_configuration;
use super::persistence::validate_persistence_requirements;
use crate::container::AppContainer;

/// Errors encountered before tracker jobs start.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tracker configuration could not be loaded. Correct the configuration source and restart: {source}")]
    Configuration { source: super::config::Error },

    #[error("Tracker configuration is inconsistent. Correct the reported settings and restart: {source}")]
    SemanticValidation {
        source: torrust_tracker_configuration::validator::SemanticValidationError,
    },

    #[error(
        "Tracker configuration has unmet persistence requirements. Configure `[core.database]` or disable the dependent capability: {source}"
    )]
    PersistenceRequirements {
        source: super::persistence::PersistenceRequirementError,
    },

    #[error("Tracker dependencies could not be composed. Correct the configured service dependencies and restart: {source}")]
    Composition { source: crate::container::Error },
}

/// It loads the configuration from the environment and builds app container.
///
/// # Errors
///
/// Returns a typed error when configuration, validation, or dependency composition fails.
///
#[instrument(skip())]
pub async fn setup() -> Result<(Configuration, AppContainer), Error> {
    #[cfg(not(test))]
    check_seed();

    let configuration = initialize_configuration().map_err(|source| Error::Configuration { source })?;

    configuration
        .validate()
        .map_err(|source| Error::SemanticValidation { source })?;

    validate_persistence_requirements(&configuration.core).map_err(|source| Error::PersistenceRequirements { source })?;

    initialize_global_services(&configuration);

    tracing::info!("Configuration:\n{}", configuration.to_redacted_json());

    let app_container = AppContainer::initialize(&configuration)
        .await
        .map_err(|source| Error::Composition { source })?;

    Ok((configuration, app_container))
}

/// checks if the seed is the instance seed in production.
///
/// # Panics
///
/// It would panic if the seed is not the instance seed.
pub fn check_seed() {
    let seed = keys::Current::get_seed();
    let instance = keys::Instance::get_seed();

    assert_eq!(seed, instance, "maybe using zeroed seed in production!?");
}

/// It initializes the global services.
#[instrument(skip())]
pub fn initialize_global_services(configuration: &Configuration) {
    initialize_static();
    logging::setup(&configuration.logging);
}

/// It initializes the application static values.
///
/// These values are accessible throughout the entire application:
///
/// - The time when the application started.
/// - An ephemeral instance random seed. This seed is used for encryption and
///   it's changed when the main application process is restarted.
#[instrument(skip())]
pub fn initialize_static() {
    torrust_clock::initialize_static();
    torrust_tracker_udp_core::initialize_static();
}

#[cfg(test)]
mod tests {
    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi;
    use torrust_tracker_configuration::validator::{SemanticValidationError, Validator};
    use torrust_tracker_primitives::PrivateMode;

    use super::Error;
    use crate::bootstrap::persistence::PersistenceRequirementError;

    #[test]
    fn it_should_preserve_the_semantic_validation_error_before_setup_composes_the_application() {
        // Arrange
        let configuration = Configuration {
            core: Core {
                private: false,
                private_mode: Some(PrivateMode::default()),
                ..Core::default()
            },
            ..Configuration::default()
        };

        // Act
        let result = configuration
            .validate()
            .map_err(|source| Error::SemanticValidation { source });

        // Assert
        assert!(matches!(
            result,
            Err(Error::SemanticValidation {
                source: SemanticValidationError::UselessPrivateModeSection
            })
        ));
    }

    #[test]
    fn it_should_preserve_the_persistence_requirement_error_before_setup_composes_the_application() {
        // Arrange
        let configuration = Configuration {
            core: Core {
                private: true,
                ..Core::default()
            },
            http_api: Some(HttpApi::default()),
            ..Configuration::default()
        };

        // Act
        let result = super::validate_persistence_requirements(&configuration.core)
            .map_err(|source| Error::PersistenceRequirements { source });

        // Assert
        assert!(matches!(
            result,
            Err(Error::PersistenceRequirements {
                source: PersistenceRequirementError::PrivateRequiresDatabase
            })
        ));
    }
}
