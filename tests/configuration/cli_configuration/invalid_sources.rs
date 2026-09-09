//! Executable-boundary invalid CLI configuration-source contracts.
//!
//! Every scenario spawns the compiled tracker with one deliberately invalid
//! `--config-toml-path` source and asserts the exit code and diagnostic. A
//! tracker that wrongly started would never exit, so the bounded wait in
//! `wait_for_exit` is itself the proof that no service was started.

use crate::native_tracker::{NativeTrackerInvalidCliSource, NativeTrackerStartAttempt};

const USAGE_ERROR_MISSING_VALUE: &str = "a value is required";
const USAGE_ERROR_EMPTY_VALUE: &str = "must not be empty";
const UNABLE_TO_LOAD_EXPLICIT_FILE: &str = "Unable to load explicit configuration file";
const UNABLE_TO_PROCESS_EXPLICIT_FILE: &str = "Unable to process explicit configuration file";

#[tokio::test]
async fn it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_missing() {
    // Arrange
    let start_attempt = NativeTrackerStartAttempt::with_invalid_cli_source(NativeTrackerInvalidCliSource::MissingOptionValue);

    // Act
    let failure = start_attempt
        .start()
        .wait_for_exit()
        .await
        .expect("tracker should exit for a missing option value");

    // Assert
    failure.assert_usage_error(USAGE_ERROR_MISSING_VALUE);
}

#[tokio::test]
async fn it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_empty() {
    // Arrange
    let start_attempt = NativeTrackerStartAttempt::with_invalid_cli_source(NativeTrackerInvalidCliSource::EmptyOptionValue);

    // Act
    let failure = start_attempt
        .start()
        .wait_for_exit()
        .await
        .expect("tracker should exit for an empty option value");

    // Assert
    failure.assert_usage_error(USAGE_ERROR_EMPTY_VALUE);
}

#[tokio::test]
async fn it_should_fail_startup_naming_the_path_when_the_cli_configuration_file_is_missing() {
    // Arrange
    let start_attempt = NativeTrackerStartAttempt::with_invalid_cli_source(NativeTrackerInvalidCliSource::MissingFile);

    // Act
    let failure = start_attempt
        .start()
        .wait_for_exit()
        .await
        .expect("tracker should exit for a missing file");

    // Assert
    failure.assert_explicit_configuration_file_load_failure();
}

#[tokio::test]
async fn it_should_fail_startup_naming_the_path_when_the_cli_configuration_source_is_a_directory() {
    // Arrange
    let start_attempt = NativeTrackerStartAttempt::with_invalid_cli_source(NativeTrackerInvalidCliSource::Directory);

    // Act
    let failure = start_attempt
        .start()
        .wait_for_exit()
        .await
        .expect("tracker should exit for a directory source");

    // Assert
    failure.assert_explicit_configuration_file_load_failure();
}

#[tokio::test]
async fn it_should_fail_startup_naming_the_path_when_the_cli_configuration_toml_is_malformed() {
    // Arrange
    let start_attempt = NativeTrackerStartAttempt::with_invalid_cli_source(NativeTrackerInvalidCliSource::MalformedToml);

    // Act
    let failure = start_attempt
        .start()
        .wait_for_exit()
        .await
        .expect("tracker should exit for malformed TOML");

    // Assert
    failure.assert_startup_failure(UNABLE_TO_PROCESS_EXPLICIT_FILE);
    failure.assert_diagnostic_names_source_path();
}

#[tokio::test]
async fn it_should_not_search_parent_directories_for_a_relative_cli_configuration_file() {
    // Arrange
    let start_attempt = NativeTrackerStartAttempt::with_invalid_cli_source(NativeTrackerInvalidCliSource::ParentOnlyRelativeFile);

    // Act
    let failure = start_attempt
        .start()
        .wait_for_exit()
        .await
        .expect("tracker should exit rather than load the parent configuration file");

    // Assert
    failure.assert_startup_failure(UNABLE_TO_LOAD_EXPLICIT_FILE);
}

#[tokio::test]
async fn it_should_fail_startup_naming_the_path_when_the_cli_configuration_file_is_unreadable() {
    // Arrange
    let Some(start_attempt) = NativeTrackerStartAttempt::with_unreadable_regular_file().enforced_or_report_skip() else {
        return;
    };

    // Act
    let failure = start_attempt
        .start()
        .wait_for_exit()
        .await
        .expect("tracker should exit for an unreadable regular file");

    // Assert
    failure.assert_explicit_configuration_file_load_failure();
}
