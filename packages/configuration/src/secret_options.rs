//! A configuration error must not print the value of a secret option.
//!
//! `figment` reports a value of the wrong type with the value itself: a database password written
//! as `987654321` makes TOML read an integer, and the error says
//! ``invalid type: found signed int `987654321`, expected a string``. The error reaches stderr and
//! the log. [`redact_secret_values`] replaces the value in such an error by a fixed text and keeps
//! the option path and the expected type, so the user still learns what to fix.
use figment::error::{Actual, Kind};

/// How much of the configuration tree below a path holds secret values.
enum Scope {
    /// The option at the path and everything below it.
    WithEverythingBelow,
    /// Only an error about the value at the path itself: a table written as a plain string.
    TheValueItself,
}

/// The options that hold secrets, as paths in the configuration tree.
///
/// Add an option here when it becomes a `SecretString`. The test of this module loads an invalid
/// value for each entry.
const SECRET_OPTIONS: [(&[&str], Scope); 3] = [
    // The database password.
    (&["core", "database", "password"], Scope::WithEverythingBelow),
    // The REST API access tokens, a map of labels to tokens.
    (&["http_api", "access_tokens"], Scope::WithEverythingBelow),
    // The database table, when it is written as a string such as a DSN with a password.
    (&["core", "database"], Scope::TheValueItself),
];

/// What replaces the value of a secret option in an error.
const NOT_SHOWN: &str = "a value of another type (not shown: the option is a secret)";

fn is_secret_option(path: &[String]) -> bool {
    SECRET_OPTIONS.iter().any(|(secret_path, scope)| match scope {
        Scope::WithEverythingBelow => path.len() >= secret_path.len() && path.iter().zip(*secret_path).all(|(a, b)| a == b),
        Scope::TheValueItself => path.len() == secret_path.len() && path.iter().zip(*secret_path).all(|(a, b)| a == b),
    })
}

/// Replaces, in every error of `error`, the value of a secret option with a fixed text.
///
/// The option path, the expected type and the source of the value stay.
#[must_use]
pub fn redact_secret_values(error: figment::Error) -> figment::Error {
    // An error can hold several errors, one per source that failed. Iterating yields them
    // newest first, with the links between them removed, so they are chained again in the same
    // order, starting from the oldest.
    error
        .into_iter()
        .map(redact_one)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .reduce(figment::Error::chain)
        .expect("an error always yields at least itself")
}

fn redact_one(mut error: figment::Error) -> figment::Error {
    if !is_secret_option(&error.path) {
        return error;
    }

    error.kind = match error.kind {
        Kind::InvalidType(_, expected) => Kind::InvalidType(Actual::Other(NOT_SHOWN.to_string()), expected),
        Kind::InvalidValue(_, expected) => Kind::InvalidValue(Actual::Other(NOT_SHOWN.to_string()), expected),
        Kind::Unsupported(_) => Kind::Unsupported(Actual::Other(NOT_SHOWN.to_string())),
        Kind::UnsupportedKey(_, expected) => Kind::UnsupportedKey(Actual::Other(NOT_SHOWN.to_string()), expected),
        other => other,
    };

    error
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::Info;
    use crate::v3_0_0::Configuration;

    const BASE: &str = r#"
        [metadata]
        schema_version = "3.0.0"

        [logging]
        trace_filter = "info"
    "#;

    /// The error of loading `BASE` followed by `options_toml`, with environment overrides cleared.
    #[allow(
        clippy::result_large_err,
        reason = "figment::Jail::expect_with requires the framework Result callback type"
    )]
    fn load_error_for(options_toml: &str) -> (String, String) {
        let config_toml = format!("{BASE}\n{options_toml}");
        let mut shown = (String::new(), String::new());

        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            let info = Info {
                config_toml: Some(config_toml),
                config_toml_path: String::new(),
                explicit_config_toml_path: None,
            };
            let error = Configuration::load(&info).expect_err("the invalid option should be rejected");
            shown = (error.to_string(), format!("{error:?}"));
            Ok(())
        });

        shown
    }

    #[rstest]
    #[case::a_numeric_database_password(
        "[core]\nlisted = false\nprivate = false\n[core.database]\ndriver = \"mysql\"\nhost = \"127.0.0.1\"\nport = 3306\nuser = \"db_user\"\ndatabase = \"torrust\"\npassword = 987654321\n",
        "987654321",
        "core.database.password"
    )]
    #[case::a_numeric_access_token(
        "[core]\nlisted = false\nprivate = false\n[http_api.access_tokens]\nadmin = 424242\n",
        "424242",
        "http_api.access_tokens.admin"
    )]
    #[case::a_boolean_access_token(
        "[core]\nlisted = false\nprivate = false\n[http_api.access_tokens]\nadmin = true\n",
        "true",
        "http_api.access_tokens.admin"
    )]
    #[case::the_access_tokens_written_as_a_string(
        "[core]\nlisted = false\nprivate = false\n[http_api]\naccess_tokens = \"TestTokenWrittenAsAString01\"\n",
        "TestTokenWrittenAsAString01",
        "http_api.access_tokens"
    )]
    #[case::the_database_written_as_a_dsn(
        "[core]\nlisted = false\nprivate = false\ndatabase = \"mysql://db_user:TestPasswordInTheDsn01@127.0.0.1:3306/torrust\"\n",
        "TestPasswordInTheDsn01",
        "core.database"
    )]
    fn it_should_name_the_option_and_the_expected_type_and_not_print_the_value_of_a_secret_option(
        #[case] options_toml: &str,
        #[case] secret_value: &str,
        #[case] option_path: &str,
    ) {
        let (displayed, debugged) = load_error_for(options_toml);

        assert!(!displayed.contains(secret_value), "the error text holds the secret value");
        assert!(!debugged.contains(secret_value), "the error Debug holds the secret value");
        assert!(
            displayed.contains(option_path),
            "the error should name the option {option_path}: {displayed}"
        );
        assert!(
            displayed.contains("expected"),
            "the error should say what was expected: {displayed}"
        );
    }

    #[test]
    #[allow(
        clippy::result_large_err,
        reason = "figment::Jail::expect_with requires the framework Result callback type"
    )]
    fn it_should_not_print_a_numeric_database_password_that_comes_from_an_environment_override() {
        let secret_value = "987654329";
        let config_toml = format!(
            "{BASE}\n[core]\nlisted = false\nprivate = false\n[core.database]\ndriver = \"mysql\"\nhost = \"127.0.0.1\"\nport = 3306\nuser = \"db_user\"\ndatabase = \"torrust\"\npassword = \"x\"\n"
        );
        let mut displayed = String::new();

        figment::Jail::expect_with(|jail| {
            jail.clear_env();
            jail.set_env("TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__PASSWORD", secret_value);
            let info = Info {
                config_toml: Some(config_toml),
                config_toml_path: String::new(),
                explicit_config_toml_path: None,
            };
            displayed = Configuration::load(&info)
                .expect_err("a numeric password should be rejected")
                .to_string();
            Ok(())
        });

        assert!(!displayed.contains(secret_value), "the error text holds the secret value");
        // The environment source names the key in upper case.
        assert!(
            displayed.to_lowercase().contains("core.database.password"),
            "the error should name the option: {displayed}"
        );
    }

    #[test]
    fn it_should_still_print_the_value_of_an_option_that_is_not_a_secret() {
        let (displayed, _debugged) = load_error_for(
            "[core]\nlisted = false\nprivate = false\n[core.database]\ndriver = \"mysql\"\nport = \"not-a-port\"\n",
        );

        assert!(
            displayed.contains("not-a-port"),
            "a non-secret value should stay in the error: {displayed}"
        );
    }
}
