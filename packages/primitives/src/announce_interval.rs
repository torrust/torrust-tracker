//! A re-announce interval that every delivery protocol can encode.
//!
//! The tracker sends the same configured interval to clients over every delivery protocol, and
//! each protocol encodes it differently. BEP 15 (UDP) uses a signed 32-bit integer, which is the
//! tightest limit among the supported protocols, so [`AnnounceIntervalInSecs`] rejects anything
//! above `i32::MAX` seconds (about 68 years) when it is built or deserialized. After that, no
//! protocol needs to clamp or re-validate the value.
//!
//! See the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md)
//! that records the rule.
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

/// An announce interval in seconds, never above [`AnnounceIntervalInSecs::MAX`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnounceIntervalInSecs(u32);

/// Error returned when an announce interval exceeds [`AnnounceIntervalInSecs::MAX`].
///
/// The message names the rejected value and the limit; the loader that deserializes the
/// configuration adds the name of the field.
#[derive(Debug, Error, PartialEq, Eq)]
#[error(
    "the interval {value} is above the maximum of {max} seconds that every delivery protocol can encode",
    max = AnnounceIntervalInSecs::MAX
)]
pub struct AnnounceIntervalTooLargeError {
    /// The rejected value in seconds.
    pub value: u64,
}

impl AnnounceIntervalInSecs {
    /// The largest interval in seconds: `i32::MAX`, the limit of BEP 15.
    pub const MAX: u32 = i32::MAX.cast_unsigned();

    /// Creates an interval after enforcing [`Self::MAX`].
    ///
    /// # Errors
    ///
    /// Returns [`AnnounceIntervalTooLargeError`] when `value` is above [`Self::MAX`].
    pub fn new(value: u32) -> Result<Self, AnnounceIntervalTooLargeError> {
        Self::from_u64(u64::from(value))
    }

    fn from_u64(value: u64) -> Result<Self, AnnounceIntervalTooLargeError> {
        match u32::try_from(value) {
            Ok(value) if value <= Self::MAX => Ok(Self(value)),
            _ => Err(AnnounceIntervalTooLargeError { value }),
        }
    }

    /// Returns the interval in seconds.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Returns the interval in seconds as the signed 32-bit integer BEP 15 uses.
    #[must_use]
    pub const fn get_i32(self) -> i32 {
        self.0.cast_signed()
    }
}

impl TryFrom<u32> for AnnounceIntervalInSecs {
    type Error = AnnounceIntervalTooLargeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Every `u16` is below [`AnnounceIntervalInSecs::MAX`].
impl From<u16> for AnnounceIntervalInSecs {
    fn from(value: u16) -> Self {
        Self(u32::from(value))
    }
}

impl From<AnnounceIntervalInSecs> for u32 {
    fn from(value: AnnounceIntervalInSecs) -> Self {
        value.get()
    }
}

impl From<AnnounceIntervalInSecs> for i32 {
    fn from(value: AnnounceIntervalInSecs) -> Self {
        value.get_i32()
    }
}

impl From<AnnounceIntervalInSecs> for i64 {
    fn from(value: AnnounceIntervalInSecs) -> Self {
        Self::from(value.get())
    }
}

impl fmt::Display for AnnounceIntervalInSecs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for AnnounceIntervalInSecs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AnnounceIntervalInSecs {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u64::deserialize(deserializer)?;
        Self::from_u64(value).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::{AnnounceIntervalInSecs, AnnounceIntervalTooLargeError};

    const MAX: u32 = 2_147_483_647;

    #[test]
    fn it_should_accept_the_largest_interval_every_delivery_protocol_can_encode() {
        let interval = AnnounceIntervalInSecs::new(MAX).expect("i32::MAX seconds should be accepted");

        assert_eq!(interval.get(), MAX);
    }

    #[test]
    fn it_should_reject_an_interval_one_second_above_the_limit() {
        let result = AnnounceIntervalInSecs::new(MAX + 1);

        assert_eq!(
            result,
            Err(AnnounceIntervalTooLargeError {
                value: u64::from(MAX) + 1
            })
        );
    }

    #[test]
    fn it_should_accept_a_zero_interval() {
        assert_eq!(AnnounceIntervalInSecs::new(0).expect("zero should be accepted").get(), 0);
    }

    #[test]
    fn it_should_name_the_value_and_the_limit_in_the_error_message() {
        let error = AnnounceIntervalInSecs::new(2_147_483_648).expect_err("one above the limit should be rejected");

        let message = error.to_string();

        assert!(message.contains("2147483648"), "{message}");
        assert!(message.contains("2147483647"), "{message}");
    }

    #[test]
    fn it_should_be_built_without_failing_from_a_u16() {
        assert_eq!(AnnounceIntervalInSecs::from(u16::MAX).get(), u32::from(u16::MAX));
    }

    #[test]
    fn it_should_convert_the_largest_interval_to_the_wire_types_without_clamping() {
        let interval = AnnounceIntervalInSecs::new(MAX).expect("i32::MAX seconds should be accepted");

        assert_eq!(interval.get_i32(), i32::MAX);
        assert_eq!(i32::from(interval), i32::MAX);
        assert_eq!(i64::from(interval), i64::from(i32::MAX));
    }

    #[test]
    fn it_should_deserialize_the_largest_interval() {
        let interval: AnnounceIntervalInSecs = serde_json::from_str("2147483647").expect("the limit should be accepted");

        assert_eq!(interval.get(), MAX);
    }

    #[test]
    fn it_should_reject_deserializing_an_interval_above_the_limit() {
        let error = serde_json::from_str::<AnnounceIntervalInSecs>("2147483648").expect_err("one above the limit should fail");

        let message = error.to_string();

        assert!(message.contains("2147483648"), "{message}");
        assert!(message.contains("2147483647"), "{message}");
    }

    #[test]
    fn it_should_name_the_limit_when_the_value_does_not_fit_in_a_u32() {
        let error = serde_json::from_str::<AnnounceIntervalInSecs>("5000000000").expect_err("a value above u32::MAX should fail");

        let message = error.to_string();

        assert!(message.contains("5000000000"), "{message}");
        assert!(message.contains("2147483647"), "{message}");
    }

    #[test]
    fn it_should_serialize_as_a_plain_integer() {
        let interval = AnnounceIntervalInSecs::from(120_u16);

        assert_eq!(serde_json::to_string(&interval).expect("it should serialize"), "120");
    }
}
