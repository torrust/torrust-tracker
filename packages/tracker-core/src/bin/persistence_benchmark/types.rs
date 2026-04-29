use std::num::NonZeroUsize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpsCount(NonZeroUsize);

impl OpsCount {
    #[must_use]
    pub fn get(self) -> usize {
        self.0.get()
    }
}

impl FromStr for OpsCount {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .parse::<usize>()
            .map_err(|_| "ops must be a positive integer".to_string())?;

        let count = NonZeroUsize::new(parsed).ok_or_else(|| "ops must be greater than zero".to_string())?;

        Ok(Self(count))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbVersion(String);

impl DbVersion {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for DbVersion {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err("db-version must not be empty".to_string());
        }

        let is_valid = value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_'));

        if !is_valid {
            return Err("db-version contains invalid characters; allowed: letters, digits, '.', '-', '_'".to_string());
        }

        Ok(Self(value.to_string()))
    }
}

impl std::fmt::Display for DbVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{DbVersion, OpsCount};

    #[test]
    fn it_should_parse_ops_count_when_value_is_positive() {
        let ops = OpsCount::from_str("100").expect("ops count should parse");

        assert_eq!(ops.get(), 100);
    }

    #[test]
    fn it_should_reject_ops_count_when_value_is_zero() {
        let error = OpsCount::from_str("0").expect_err("zero ops count should fail");

        assert_eq!(error, "ops must be greater than zero");
    }

    #[test]
    fn it_should_reject_ops_count_when_value_is_not_numeric() {
        let error = OpsCount::from_str("abc").expect_err("non-numeric ops count should fail");

        assert_eq!(error, "ops must be a positive integer");
    }

    #[test]
    fn it_should_parse_db_version_when_value_has_allowed_characters() {
        let db_version = DbVersion::from_str("8.4-rc1").expect("db version should parse");

        assert_eq!(db_version.as_str(), "8.4-rc1");
    }

    #[test]
    fn it_should_reject_db_version_when_value_is_empty() {
        let error = DbVersion::from_str("").expect_err("empty db version should fail");

        assert_eq!(error, "db-version must not be empty");
    }

    #[test]
    fn it_should_reject_db_version_when_value_has_invalid_characters() {
        let error = DbVersion::from_str("8.4/rc1").expect_err("db version with slash should fail");

        assert_eq!(
            error,
            "db-version contains invalid characters; allowed: letters, digits, '.', '-', '_'"
        );
    }
}
