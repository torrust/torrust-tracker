use std::str::FromStr;

use chrono::{DateTime, Utc};
use torrust_tracker_primitives::DurationSinceUnixEpoch;

/// It converts a string in ISO 8601 format to a timestamp.
///
/// For example, the string `1970-01-01T00:00:00.000Z` which is the Unix Epoch
/// will be converted to a timestamp of 0: `DurationSinceUnixEpoch::ZERO`.
///
/// # Panics
///
/// Will panic if the input time cannot be converted to `DateTime::<Utc>`, internally using the `i64` type.
/// (this will naturally happen in 292.5 billion years)
#[must_use]
pub fn convert_from_iso_8601_to_timestamp(iso_8601: &str) -> DurationSinceUnixEpoch {
    convert_from_datetime_utc_to_timestamp(&DateTime::<Utc>::from_str(iso_8601).unwrap())
}

/// It converts a `DateTime::<Utc>` to a timestamp.
/// For example, the `DateTime::<Utc>` of the Unix Epoch will be converted to a
/// timestamp of 0: `DurationSinceUnixEpoch::ZERO`.
///
/// # Panics
///
/// Will panic if the input time overflows the `u64` type.
/// (this will naturally happen in 584.9 billion years)
#[must_use]
pub fn convert_from_datetime_utc_to_timestamp(datetime_utc: &DateTime<Utc>) -> DurationSinceUnixEpoch {
    DurationSinceUnixEpoch::from_secs(u64::try_from(datetime_utc.timestamp()).expect("Overflow of u64 seconds, very future!"))
}

/// It converts a timestamp to a `DateTime::<Utc>`.
/// For example, the timestamp of 0: `DurationSinceUnixEpoch::ZERO` will be
/// converted to the `DateTime::<Utc>` of the Unix Epoch.
///
/// # Panics
///
/// Will panic if the input time overflows the `u64` seconds overflows the `i64` type.
/// (this will naturally happen in 292.5 billion years)
#[must_use]
pub fn convert_from_timestamp_to_datetime_utc(duration: DurationSinceUnixEpoch) -> DateTime<Utc> {
    DateTime::from_timestamp(
        i64::try_from(duration.as_secs()).expect("Overflow of i64 seconds, very future!"),
        duration.subsec_nanos(),
    )
    .unwrap()
}

#[cfg(test)]

mod tests {
    use chrono::DateTime;
    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::conv::{
        convert_from_datetime_utc_to_timestamp, convert_from_iso_8601_to_timestamp, convert_from_timestamp_to_datetime_utc,
    };

    #[test]
    fn should_be_converted_to_datetime_utc() {
        let timestamp = DurationSinceUnixEpoch::ZERO;
        assert_eq!(
            convert_from_timestamp_to_datetime_utc(timestamp),
            DateTime::from_timestamp(0, 0).unwrap()
        );
    }

    #[test]
    fn should_be_converted_from_datetime_utc() {
        let datetime = DateTime::from_timestamp(0, 0).unwrap();
        assert_eq!(
            convert_from_datetime_utc_to_timestamp(&datetime),
            DurationSinceUnixEpoch::ZERO
        );
    }

    #[test]
    fn should_be_converted_from_datetime_utc_in_iso_8601() {
        let iso_8601 = "1970-01-01T00:00:00.000Z".to_string();
        assert_eq!(convert_from_iso_8601_to_timestamp(&iso_8601), DurationSinceUnixEpoch::ZERO);
    }
}
