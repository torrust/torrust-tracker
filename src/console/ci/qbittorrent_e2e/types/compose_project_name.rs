use std::fmt;
use std::ops::Deref;

use rand::distr::Alphanumeric;
use rand::RngExt;

/// A Docker Compose project name generated for one E2E test run.
///
/// Project names follow the pattern `<prefix>-<random-suffix>` where the
/// suffix is ten lowercase alphanumeric characters, keeping each run's
/// containers, volumes, and networks isolated from one another.
///
/// Wraps a [`String`] and provides [`Deref`] to `str` so values can be
/// passed wherever `&str` is expected.
#[derive(Debug, Clone)]
pub(crate) struct ComposeProjectName(String);

impl ComposeProjectName {
    /// Generates a unique project name with the given prefix.
    ///
    /// Appends ten random lowercase alphanumeric characters to `prefix`,
    /// separated by a hyphen.
    pub(crate) fn generate(prefix: &str) -> Self {
        let suffix: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        Self(format!("{prefix}-{suffix}"))
    }

    /// Returns the project name as a `&str`.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for ComposeProjectName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ComposeProjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::ComposeProjectName;

    #[test]
    fn it_should_generate_expected_shape() {
        let name = ComposeProjectName::generate("qbt-e2e");
        let as_str = name.as_str();

        assert!(as_str.starts_with("qbt-e2e-"));
        assert_eq!(as_str.len(), "qbt-e2e-".len() + 10);

        let suffix = &as_str["qbt-e2e-".len()..];
        assert!(suffix.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));

        assert_eq!(&*name, as_str);
        assert_eq!(name.to_string(), as_str);
    }
}
