//! Executable-boundary invalid CLI configuration-source contracts.

use crate::native_tracker::{NativeTrackerFailedStart, NativeTrackerInvalidCliSource};

#[tokio::test]
async fn it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_missing() {
    // Arrange
    let failed_start = NativeTrackerFailedStart::spawn(NativeTrackerInvalidCliSource::MissingOptionValue);

    // Act
    let failure = failed_start
        .wait_for_exit()
        .await
        .expect("tracker should exit for a missing option value");

    // Assert
    assert_eq!(failure.exit_code(), 2);
    assert!(failure.output().contains("a value is required"));
}

#[tokio::test]
async fn it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_empty() {
    // Arrange
    let failed_start = NativeTrackerFailedStart::spawn(NativeTrackerInvalidCliSource::EmptyOptionValue);

    // Act
    let failure = failed_start
        .wait_for_exit()
        .await
        .expect("tracker should exit for an empty option value");

    // Assert
    assert_eq!(failure.exit_code(), 2);
    assert!(failure.output().contains("must not be empty"));
}

#[tokio::test]
async fn it_should_exit_without_starting_when_the_cli_configuration_file_is_missing() {
    // Arrange
    let failed_start = NativeTrackerFailedStart::spawn(NativeTrackerInvalidCliSource::MissingFile);
    let expected_path = failed_start
        .source_path()
        .expect("missing-file fixture should expose its source path")
        .to_string_lossy()
        .into_owned();

    // Act
    let failure = failed_start
        .wait_for_exit()
        .await
        .expect("tracker should exit for a missing file");

    // Assert
    assert_eq!(failure.exit_code(), 1);
    assert!(failure.output().contains("Unable to load explicit configuration file"));
    assert!(failure.output().contains(&expected_path));
}

#[tokio::test]
async fn it_should_exit_without_starting_when_the_cli_configuration_source_is_a_directory() {
    // Arrange
    let failed_start = NativeTrackerFailedStart::spawn(NativeTrackerInvalidCliSource::Directory);
    let expected_path = failed_start
        .source_path()
        .expect("directory fixture should expose its source path")
        .to_string_lossy()
        .into_owned();

    // Act
    let failure = failed_start
        .wait_for_exit()
        .await
        .expect("tracker should exit for a directory source");

    // Assert
    assert_eq!(failure.exit_code(), 1);
    assert!(failure.output().contains("Unable to load explicit configuration file"));
    assert!(failure.output().contains(&expected_path));
}

#[tokio::test]
async fn it_should_exit_without_starting_when_the_cli_configuration_toml_is_malformed() {
    // Arrange
    let candidate_health_port = 43158;
    let failed_start = NativeTrackerFailedStart::spawn(NativeTrackerInvalidCliSource::MalformedToml { candidate_health_port });
    let expected_path = failed_start
        .source_path()
        .expect("malformed-TOML fixture should expose its source path")
        .to_string_lossy()
        .into_owned();

    // Act
    let failure = failed_start
        .wait_for_exit()
        .await
        .expect("tracker should exit for malformed TOML");

    // Assert
    assert_eq!(failure.exit_code(), 1);
    assert!(failure.output().contains("Unable to process explicit configuration file"));
    assert!(failure.output().contains(&expected_path));
    assert_eq!(failure.candidate_port(), Some(candidate_health_port));
    failure
        .assert_candidate_port_is_bindable()
        .expect("malformed configuration must not leave its candidate health port bound");
}

#[tokio::test]
async fn it_should_not_search_parent_directories_for_a_relative_cli_configuration_file() {
    // Arrange
    let candidate_health_port = 43157;
    let failed_start =
        NativeTrackerFailedStart::spawn(NativeTrackerInvalidCliSource::ParentOnlyRelativeFile { candidate_health_port });

    // Act
    let failure = failed_start
        .wait_for_exit()
        .await
        .expect("tracker should exit rather than load the parent configuration file");

    // Assert
    assert_eq!(failure.exit_code(), 1);
    assert!(failure.output().contains("Unable to load explicit configuration file"));
    assert!(failure.output().contains("tracker.toml"));
    assert_eq!(failure.candidate_port(), Some(candidate_health_port));
    failure
        .assert_candidate_port_is_bindable()
        .expect("parent-only configuration must not leave its candidate health port bound");
}

#[tokio::test]
async fn it_should_not_panic_when_a_failed_start_is_dropped_without_a_tokio_runtime() {
    // Arrange
    let failed_start = NativeTrackerFailedStart::spawn(NativeTrackerInvalidCliSource::MissingFile);

    // Act
    let result = std::thread::spawn(move || drop(failed_start)).join();

    // Assert
    assert!(result.is_ok(), "dropping a failed start outside Tokio must not panic");
}
