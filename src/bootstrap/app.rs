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
//! 3. Apply the logging configuration (the entry point installs the logging before the setup, see
//!    [`super::logging`]).
//! 4. Initialize the domain tracker.
use std::path::PathBuf;

use torrust_tracker_configuration::v3_0_0::Configuration;
use torrust_tracker_configuration::validator::Validator;
use tracing::instrument;

use super::config::initialize_configuration;
use super::persistence::validate_persistence_requirements;
use crate::container::AppContainer;

/// Errors encountered before tracker jobs start.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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

/// Loads the configuration and builds the application container.
///
/// # Errors
///
/// Returns a typed error when configuration, validation, or dependency composition fails.
///
// No `#[instrument]`: the logging is installed before the setup runs, so a span of the setup would
// be recorded and its name (and arguments) would prefix every record written inside it.
// issue: #2151
pub async fn setup(explicit_config_toml_path: Option<PathBuf>) -> Result<(Configuration, AppContainer), Error> {
    let configuration = initialize_configuration(explicit_config_toml_path).map_err(|source| Error::Configuration { source })?;

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

/// It initializes the global services: the clock and the logging configuration (the logging itself
/// is installed by the entry point, see [`super::logging`]).
// No `#[instrument]`: it would record the configuration, which holds secrets (the API access
// tokens), as a span field, and its `Debug` form would appear in the records written inside the span.
pub fn initialize_global_services(configuration: &Configuration) {
    initialize_static();
    super::logging::configure(&configuration.logging);
}

/// It initializes the application static values.
///
/// These values are accessible throughout the entire application:
///
/// - The time when the application started.
///
/// The UDP connection-cookie key is not a static: the application container
/// creates it and injects it into the UDP tracker services.
#[instrument(skip())]
pub fn initialize_static() {
    torrust_clock::initialize_static();
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use torrust_tracker_configuration::v3_0_0::Configuration;
    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_configuration::v3_0_0::tracker_api::HttpApi;
    use torrust_tracker_configuration::validator::{SemanticValidationError, Validator};
    use torrust_tracker_primitives::PrivateMode;
    use torrust_tracker_test_helpers::configuration::ephemeral_public;
    use tracing_subscriber::Layer;
    use tracing_subscriber::layer::{Context, SubscriberExt as _};

    use super::{Error, initialize_global_services, setup};
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

    /// The name of a span and the names of its fields.
    type SpanRecord = (String, Vec<String>);

    /// Records every span that is created.
    #[derive(Clone, Default)]
    struct SpanFields(Arc<Mutex<Vec<SpanRecord>>>);

    impl<S: tracing::Subscriber> Layer<S> for SpanFields {
        fn on_new_span(&self, attributes: &tracing::span::Attributes<'_>, _id: &tracing::span::Id, _context: Context<'_, S>) {
            let metadata = attributes.metadata();
            let fields = metadata.fields().iter().map(|field| field.name().to_owned()).collect();

            self.0
                .lock()
                .expect("the record is not poisoned")
                .push((metadata.name().to_owned(), fields));
        }
    }

    #[test]
    fn it_should_not_record_the_configuration_in_any_span_of_the_global_services_initialization() {
        // Arrange: the logging is installed before the configuration is loaded, so the spans of
        // the setup are recorded, and the configuration holds secrets (the API access tokens)
        let configuration = ephemeral_public();
        let spans = SpanFields::default();
        let subscriber = tracing_subscriber::registry().with(spans.clone());

        // Act
        tracing::subscriber::with_default(subscriber, || initialize_global_services(&configuration));

        // Assert
        let spans = spans.0.lock().expect("the record is not poisoned").clone();
        assert!(
            spans
                .iter()
                .all(|(_name, fields)| !fields.iter().any(|field| field == "configuration")),
            "a span records the configuration: {spans:?}"
        );
    }

    #[test]
    fn it_should_not_record_the_explicit_configuration_path_in_any_span_of_the_setup() {
        // Arrange
        let spans = SpanFields::default();
        let subscriber = tracing_subscriber::registry().with(spans.clone());

        // Act: the setup fails at once, because the file does not exist
        let result = tracing::subscriber::with_default(subscriber, || {
            tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("a runtime")
                .block_on(setup(Some(std::path::PathBuf::from("/nonexistent/tracker.toml"))))
        });

        // Assert
        assert!(result.is_err());
        let spans = spans.0.lock().expect("the record is not poisoned").clone();
        assert!(
            spans
                .iter()
                .all(|(_name, fields)| !fields.iter().any(|field| field == "explicit_config_toml_path")),
            "a span records the path: {spans:?}"
        );
    }
}
