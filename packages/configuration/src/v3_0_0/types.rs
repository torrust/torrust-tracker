// adr: docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
// issue: #1453
//! Reusable validated value types for schema v3 configuration.
//!
//! Value invariants belong in these types, rather than in cross-field
//! configuration consistency validation.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

/// Error returned when a value is smaller than its configured lower bound.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("value must be at least {minimum}")]
pub struct ValueBelowMinimumError {
    /// Smallest accepted value.
    pub minimum: u64,
}

/// An unsigned integer guaranteed to be at least `MINIMUM`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtLeastU64<const MINIMUM: u64>(u64);

impl<const MINIMUM: u64> AtLeastU64<MINIMUM> {
    /// Creates a value after enforcing the lower bound.
    ///
    /// # Errors
    ///
    /// Returns [`ValueBelowMinimumError`] when `value` is less than `MINIMUM`.
    pub fn new(value: u64) -> Result<Self, ValueBelowMinimumError> {
        if value < MINIMUM {
            return Err(ValueBelowMinimumError { minimum: MINIMUM });
        }

        Ok(Self(value))
    }

    /// Returns the validated integer value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl<const MINIMUM: u64> TryFrom<u64> for AtLeastU64<MINIMUM> {
    type Error = ValueBelowMinimumError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MINIMUM: u64> From<AtLeastU64<MINIMUM>> for u64 {
    fn from(value: AtLeastU64<MINIMUM>) -> Self {
        value.get()
    }
}

impl<const MINIMUM: u64> Serialize for AtLeastU64<MINIMUM> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, const MINIMUM: u64> Deserialize<'de> for AtLeastU64<MINIMUM> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::AtLeastU64;

    #[test]
    fn it_should_accept_a_value_at_the_minimum() {
        assert_eq!(AtLeastU64::<60>::new(60).map(AtLeastU64::get), Ok(60));
    }

    #[test]
    fn it_should_reject_a_value_below_the_minimum() {
        let error = AtLeastU64::<60>::new(59).expect_err("a value below the minimum should be rejected");

        assert_eq!(error.to_string(), "value must be at least 60");
    }

    #[test]
    fn it_should_reject_an_invalid_value_during_deserialization() {
        #[derive(Debug, serde::Deserialize)]
        struct Fixture {
            value: AtLeastU64<60>,
        }

        let fixture: Fixture = toml::from_str("value = 60").expect("the minimum value should deserialize");

        assert_eq!(fixture.value.get(), 60);

        let error = toml::from_str::<Fixture>("value = 59").expect_err("a value below the minimum should be rejected");

        assert!(error.to_string().contains("value must be at least 60"));
    }
}
