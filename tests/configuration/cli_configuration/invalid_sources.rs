//! Executable-boundary invalid CLI configuration-source contracts.

use crate::native_tracker::{NativeTrackerExpectedFailure, NativeTrackerInvalidCliSource};

#[tokio::test]
async fn it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_missing() {
    // Arrange
    let fixture = NativeTrackerExpectedFailure::start(NativeTrackerInvalidCliSource::MissingOptionValue);

    // Act
    let failure = fixture.wait().await.expect("tracker should exit for a missing option value");

    // Assert
    assert_eq!(failure.exit_code(), 2);
    assert!(failure.output().contains("a value is required"));
}

#[tokio::test]
async fn it_should_exit_with_a_usage_error_when_the_config_toml_path_value_is_empty() {
    // Arrange
    let fixture = NativeTrackerExpectedFailure::start(NativeTrackerInvalidCliSource::EmptyOptionValue);

    // Act
    let failure = fixture.wait().await.expect("tracker should exit for an empty option value");

    // Assert
    assert_eq!(failure.exit_code(), 2);
    assert!(failure.output().contains("must not be empty"));
}

#[tokio::test]
async fn it_should_exit_without_starting_when_the_cli_configuration_file_is_missing() {
    // Arrange
    let fixture = NativeTrackerExpectedFailure::start(NativeTrackerInvalidCliSource::MissingFile);
    let expected_path = fixture
        .source_path()
        .expect("missing-file fixture should expose its source path")
        .to_string_lossy()
        .into_owned();

    // Act
    let failure = fixture.wait().await.expect("tracker should exit for a missing file");

    // Assert
    assert_eq!(failure.exit_code(), 1);
    assert!(failure.output().contains("Unable to load explicit configuration file"));
    assert!(failure.output().contains(&expected_path));
}

#[tokio::test]
async fn it_should_exit_without_starting_when_the_cli_configuration_source_is_a_directory() {
    // Arrange
    let fixture = NativeTrackerExpectedFailure::start(NativeTrackerInvalidCliSource::Directory);
    let expected_path = fixture
        .source_path()
        .expect("directory fixture should expose its source path")
        .to_string_lossy()
        .into_owned();

    // Act
    let failure = fixture.wait().await.expect("tracker should exit for a directory source");

    // Assert
    assert_eq!(failure.exit_code(), 1);
    assert!(failure.output().contains("Unable to load explicit configuration file"));
    assert!(failure.output().contains(&expected_path));
}

#[tokio::test]
async fn it_should_exit_without_starting_when_the_cli_configuration_toml_is_malformed() {
    // Arrange
    let candidate_health_port = 43158;
    let fixture = NativeTrackerExpectedFailure::start(NativeTrackerInvalidCliSource::MalformedToml { candidate_health_port });
    let expected_path = fixture
        .source_path()
        .expect("malformed-TOML fixture should expose its source path")
        .to_string_lossy()
        .into_owned();

    // Act
    let failure = fixture.wait().await.expect("tracker should exit for malformed TOML");

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
    let fixture =
        NativeTrackerExpectedFailure::start(NativeTrackerInvalidCliSource::ParentOnlyRelativeFile { candidate_health_port });

    // Act
    let failure = fixture
        .wait()
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
async fn it_should_not_panic_when_an_expected_failure_fixture_is_dropped_without_a_tokio_runtime() {
    // Arrange
    let fixture = NativeTrackerExpectedFailure::start(NativeTrackerInvalidCliSource::MissingFile);

    // Act
    let result = std::thread::spawn(move || drop(fixture)).join();

    // Assert
    assert!(result.is_ok(), "dropping the fixture outside Tokio must not panic");
}
