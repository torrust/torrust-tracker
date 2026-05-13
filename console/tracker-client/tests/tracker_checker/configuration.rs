mod invalid_configuration_from_env_var {
    use super::super::tracker_client_check_bin;

    #[test]
    fn it_should_exit_with_code_2_on_invalid_json() {
        let output = tracker_client_check_bin()
            .env("TORRUST_CHECKER_CONFIG", r#"{"invalid json":"#)
            .output()
            .expect("Failed to run tracker_client check");

        assert_eq!(output.status.code(), Some(2), "Expected exit code 2 for invalid config");
    }

    #[test]
    fn it_should_write_json_error_to_stderr_on_invalid_json() {
        let output = tracker_client_check_bin()
            .env("TORRUST_CHECKER_CONFIG", r#"{"invalid json":"#)
            .output()
            .expect("Failed to run tracker_client check");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(r#""kind":"invalid_configuration""#),
            "Expected JSON error envelope on stderr, got: {stderr}"
        );
        assert!(
            stderr.contains(r#""source":"TORRUST_CHECKER_CONFIG""#),
            "Expected source field to identify env var, got: {stderr}"
        );
    }

    #[test]
    fn it_should_include_parse_detail_in_stderr_error_message_on_trailing_comma() {
        let config = r#"{
            "udp_trackers": [],
            "http_trackers": [
                "http://127.0.0.1:7070",
            ],
            "health_checks": []
        }"#;

        let output = tracker_client_check_bin()
            .env("TORRUST_CHECKER_CONFIG", config)
            .output()
            .expect("Failed to run tracker_client check");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "Expected exit code 2 for invalid config");
        assert!(
            stderr.contains("trailing comma"),
            "Expected 'trailing comma' detail in stderr, got: {stderr}"
        );
    }

    #[test]
    fn it_should_produce_no_output_on_stdout_on_config_error() {
        let output = tracker_client_check_bin()
            .env("TORRUST_CHECKER_CONFIG", r#"{"invalid json":"#)
            .output()
            .expect("Failed to run tracker_client check");

        // Per the I/O contract, stdout is for successful results only
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.is_empty(), "Expected no stdout on config error, got: {stdout}");
    }
}

mod invalid_configuration_from_file {
    use std::io::Write;

    use super::super::tracker_client_check_bin;

    #[test]
    fn it_should_exit_with_code_2_on_invalid_json_in_file() {
        let mut tmp = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        write!(tmp, r#"{{"invalid json":"#).unwrap();

        let output = tracker_client_check_bin()
            .env("TORRUST_CHECKER_CONFIG_PATH", tmp.path())
            .output()
            .expect("Failed to run tracker_client check");

        assert_eq!(output.status.code(), Some(2), "Expected exit code 2 for invalid config file");
    }

    #[test]
    fn it_should_include_file_path_in_stderr_source_field() {
        let mut tmp = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        write!(tmp, r#"{{"invalid json":"#).unwrap();
        let path = tmp.path().to_string_lossy().to_string();

        let output = tracker_client_check_bin()
            .env("TORRUST_CHECKER_CONFIG_PATH", &path)
            .output()
            .expect("Failed to run tracker_client check");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(&path),
            "Expected file path in stderr source field, got: {stderr}"
        );
    }

    #[test]
    fn it_should_exit_with_code_2_when_config_file_does_not_exist() {
        let output = tracker_client_check_bin()
            .env("TORRUST_CHECKER_CONFIG_PATH", "/nonexistent/path/config.json")
            .output()
            .expect("Failed to run tracker_client check");

        assert_eq!(output.status.code(), Some(2), "Expected exit code 2 for missing config file");
    }
}

mod no_configuration_provided {
    use super::super::tracker_client_check_bin;

    #[test]
    fn it_should_exit_with_code_2_when_no_config_is_provided() {
        let output = tracker_client_check_bin()
            // Ensure neither env var is set
            .env_remove("TORRUST_CHECKER_CONFIG")
            .env_remove("TORRUST_CHECKER_CONFIG_PATH")
            .output()
            .expect("Failed to run tracker_client check");

        assert_eq!(output.status.code(), Some(2), "Expected exit code 2 when no config provided");
    }

    #[test]
    fn it_should_write_json_error_to_stderr_when_no_config_is_provided() {
        let output = tracker_client_check_bin()
            .env_remove("TORRUST_CHECKER_CONFIG")
            .env_remove("TORRUST_CHECKER_CONFIG_PATH")
            .output()
            .expect("Failed to run tracker_client check");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(r#""kind":"invalid_configuration""#),
            "Expected JSON error envelope on stderr, got: {stderr}"
        );
    }
}
