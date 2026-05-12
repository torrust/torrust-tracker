//! Application-level errors for the tracker checker binary.
//!
//! This module separates two concerns:
//! - **Delivery mechanism**: how the configuration was provided (env var, file path, …)
//! - **Error presentation**: what structured JSON the binary emits on stderr
//!
//! `ConfigSource` captures the delivery mechanism so that error messages can
//! reference it without coupling the parsing layer to delivery specifics.
//!
//! The JSON envelope emitted to stderr follows the Tracker CLI I/O Contract:
//!
//! ```json
//! { "error": { "kind": "...", "source": "...", "message": "..." } }
//! ```
use std::fmt;
use std::path::PathBuf;

/// Where the configuration content was delivered from.
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// Configuration delivered via an environment variable (stores the variable name).
    EnvVar(&'static str),
    /// Configuration delivered via a file (stores the file path).
    File(PathBuf),
}

impl fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigSource::EnvVar(name) => write!(f, "{name}"),
            ConfigSource::File(path) => write!(f, "{}", path.display()),
        }
    }
}

/// Top-level application errors for the tracker checker.
#[derive(Debug)]
pub enum AppError {
    /// The provided configuration was invalid (bad JSON, invalid URLs, etc.).
    InvalidConfig {
        /// How the configuration was delivered (env var or file path).
        source: ConfigSource,
        /// Human-readable detail from the underlying parse error.
        message: String,
    },
    /// An unexpected runtime failure occurred after configuration was accepted.
    Runtime(String),
}

impl AppError {
    /// Serializes the error to the contract JSON envelope and returns the
    /// appropriate process exit code.
    ///
    /// Exit codes:
    /// - `2` — configuration error
    /// - `1` — generic runtime failure
    #[must_use]
    pub fn to_stderr_json_and_exit_code(&self) -> (String, i32) {
        match self {
            AppError::InvalidConfig { source, message } => {
                let json =
                    format!(r#"{{"error":{{"kind":"invalid_configuration","source":"{source}","message":"{message}"}}}}"#,);
                (json, 2)
            }
            AppError::Runtime(message) => {
                let json = format!(r#"{{"error":{{"kind":"runtime_failure","source":"runtime","message":"{message}"}}}}"#);
                (json, 1)
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidConfig { source, message } => {
                write!(f, "invalid configuration from {source}: {message}")
            }
            AppError::Runtime(msg) => write!(f, "runtime failure: {msg}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_source_env_var_displays_as_variable_name() {
        let source = ConfigSource::EnvVar("TORRUST_CHECKER_CONFIG");
        assert_eq!(source.to_string(), "TORRUST_CHECKER_CONFIG");
    }

    #[test]
    fn config_source_file_displays_as_path() {
        let source = ConfigSource::File(PathBuf::from("/etc/tracker/config.json"));
        assert_eq!(source.to_string(), "/etc/tracker/config.json");
    }

    #[test]
    fn invalid_config_error_produces_exit_code_2() {
        let error = AppError::InvalidConfig {
            source: ConfigSource::EnvVar("TORRUST_CHECKER_CONFIG"),
            message: "JSON parse error: trailing comma at line 7 column 5".to_string(),
        };
        let (_, exit_code) = error.to_stderr_json_and_exit_code();
        assert_eq!(exit_code, 2);
    }

    #[test]
    fn runtime_error_produces_exit_code_1() {
        let error = AppError::Runtime("failed to bind socket".to_string());
        let (_, exit_code) = error.to_stderr_json_and_exit_code();
        assert_eq!(exit_code, 1);
    }

    #[test]
    fn invalid_config_error_json_contains_expected_fields() {
        let error = AppError::InvalidConfig {
            source: ConfigSource::EnvVar("TORRUST_CHECKER_CONFIG"),
            message: "JSON parse error: trailing comma at line 7 column 5".to_string(),
        };
        let (json, _) = error.to_stderr_json_and_exit_code();
        assert!(json.contains(r#""kind":"invalid_configuration""#));
        assert!(json.contains(r#""source":"TORRUST_CHECKER_CONFIG""#));
        assert!(json.contains("trailing comma at line 7 column 5"));
    }

    #[test]
    fn runtime_error_json_contains_expected_fields() {
        let error = AppError::Runtime("failed to bind socket".to_string());
        let (json, _) = error.to_stderr_json_and_exit_code();
        assert!(json.contains(r#""kind":"runtime_failure""#));
        assert!(json.contains(r#""source":"runtime""#));
        assert!(json.contains("failed to bind socket"));
    }

    #[test]
    fn invalid_config_error_from_file_includes_path_in_json() {
        let error = AppError::InvalidConfig {
            source: ConfigSource::File(PathBuf::from("/etc/tracker/config.json")),
            message: "JSON parse error: trailing comma at line 3 column 1".to_string(),
        };
        let (json, _) = error.to_stderr_json_and_exit_code();
        assert!(json.contains(r#""source":"/etc/tracker/config.json""#));
    }
}
