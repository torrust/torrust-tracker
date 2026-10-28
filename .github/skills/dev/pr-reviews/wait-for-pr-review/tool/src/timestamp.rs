//! Parsing of the UTC timestamps that GitHub returns, so times are compared as numbers and never as strings.

use std::ops::Range;

/// Seconds since the Unix epoch, for a GitHub timestamp such as `2026-10-28T07:00:00Z`.
///
/// Returns `None` unless the text is a UTC timestamp in that exact shape: every field is made of
/// digits only, the day exists in its month, and the clock fields are in range. A value that would
/// roll over into another day or month is rejected, never converted.
#[must_use]
pub fn parse_utc(text: &str) -> Option<i64> {
    let bytes = text.as_bytes();
    let separators = [(4, b'-'), (7, b'-'), (10, b'T'), (13, b':'), (16, b':'), (19, b'Z')];
    if bytes.len() != 20 || separators.iter().any(|&(index, expected)| bytes[index] != expected) {
        return None;
    }

    let year = digits(text, 0..4)?;
    let month = digits(text, 5..7)?;
    let day = digits(text, 8..10)?;
    let hour = digits(text, 11..13)?;
    let minute = digits(text, 14..16)?;
    let second = digits(text, 17..19)?;

    if !(1..=12).contains(&month) || !(1..=days_in_month(year, month)).contains(&day) || hour > 23 || minute > 59 || second > 59 {
        return None;
    }

    Some(days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second)
}

/// The number written in `range` of `text`, which must be nothing but ASCII digits: `str::parse`
/// would also accept a leading `+` or `-`.
fn digits(text: &str, range: Range<usize>) -> Option<i64> {
    let field = text.get(range)?;

    if !field.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    field.parse().ok()
}

const fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 31,
    }
}

/// Days since 1970-01-01 for a proleptic Gregorian date (the well-known `days_from_civil` algorithm).
const fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year.rem_euclid(400);
    let shifted_month = (month + 9) % 12;
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

    era * 146_097 + day_of_era - 719_468
}

#[cfg(test)]
mod tests {
    use super::parse_utc;

    #[test]
    fn it_should_parse_the_unix_epoch_as_zero() {
        assert_eq!(parse_utc("1970-01-01T00:00:00Z"), Some(0));
    }

    #[test]
    fn it_should_parse_a_known_timestamp() {
        assert_eq!(parse_utc("2026-10-28T07:00:00Z"), Some(1_793_170_800));
    }

    #[test]
    fn it_should_order_timestamps_across_a_leap_day() {
        let before = parse_utc("2028-02-29T23:59:59Z").unwrap();
        let after = parse_utc("2028-03-01T00:00:00Z").unwrap();

        assert_eq!(after - before, 1);
    }

    #[test]
    fn it_should_reject_a_day_the_month_does_not_have() {
        for text in [
            "2026-02-29T07:00:00Z", // 2026 is not a leap year
            "2100-02-29T07:00:00Z", // a century that is not divisible by 400
            "2026-02-30T07:00:00Z",
            "2026-04-31T07:00:00Z",
            "2026-10-00T07:00:00Z",
        ] {
            assert_eq!(parse_utc(text), None, "{text:?} should be rejected");
        }
    }

    #[test]
    fn it_should_accept_the_last_day_of_every_kind_of_month_and_both_kinds_of_february() {
        for text in [
            "2026-01-31T07:00:00Z",
            "2026-04-30T07:00:00Z",
            "2026-02-28T07:00:00Z",
            "2028-02-29T07:00:00Z",
            "2000-02-29T07:00:00Z",
        ] {
            assert!(parse_utc(text).is_some(), "{text:?} should be accepted");
        }
    }

    #[test]
    fn it_should_reject_signed_or_otherwise_non_digit_fields() {
        for text in [
            "2026-10-28T-1:00:00Z", // would become 23:00 of the day before
            "2026-10-28T07:-1:00Z",
            "2026-10-28T07:00:+1Z",
            "+026-10-28T07:00:00Z",
            "2026-1-+8T07:00:00Z",
            "2026-10-28T 7:00:00Z",
        ] {
            assert_eq!(parse_utc(text), None, "{text:?} should be rejected");
        }
    }

    #[test]
    fn it_should_reject_clock_fields_that_are_out_of_range() {
        for text in ["2026-10-28T24:00:00Z", "2026-10-28T07:60:00Z", "2026-10-28T07:00:60Z"] {
            assert_eq!(parse_utc(text), None, "{text:?} should be rejected");
        }
    }

    #[test]
    fn it_should_reject_timestamps_that_are_not_utc_in_the_github_shape() {
        for text in [
            "",
            "2026-10-28",
            "2026-10-28T07:00:00+01:00",
            "2026-13-28T07:00:00Z",
            "2026-10-28 07:00:00Z",
        ] {
            assert_eq!(parse_utc(text), None, "{text:?} should be rejected");
        }
    }
}
