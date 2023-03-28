//! It contains helper functions related to time.
use super::DurationSinceUnixEpoch;

/// Serializes a `DurationSinceUnixEpoch` as a Unix timestamp in milliseconds.
/// # Errors
///
/// Will return `serde::Serializer::Error` if unable to serialize the `unix_time_value`.
pub fn ser_unix_time_value<S: serde::Serializer>(unix_time_value: &DurationSinceUnixEpoch, ser: S) -> Result<S::Ok, S::Error> {
    #[allow(clippy::cast_possible_truncation)]
    ser.serialize_u64(unix_time_value.as_millis() as u64)
}
