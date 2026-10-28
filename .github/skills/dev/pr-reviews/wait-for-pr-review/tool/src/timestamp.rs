//! Parsing of the UTC timestamps that GitHub returns, so times are compared as numbers and never as strings.

/// Seconds since the Unix epoch, for a GitHub timestamp such as `2026-10-28T07:00:00Z`.
///
/// Returns `None` when the text is not a UTC timestamp in that exact shape.
#[must_use]
pub fn parse_utc(text: &str) -> Option<i64> {
    let bytes = text.as_bytes();
    let separators = [(4, b'-'), (7, b'-'), (10, b'T'), (13, b':'), (16, b':'), (19, b'Z')];
    if bytes.len() != 20 || separators.iter().any(|&(index, expected)| bytes[index] != expected) {
        return None;
    }

    let year: i64 = text.get(0..4)?.parse().ok()?;
    let month: i64 = text.get(5..7)?.parse().ok()?;
    let day: i64 = text.get(8..10)?.parse().ok()?;
    let hour: i64 = text.get(11..13)?.parse().ok()?;
    let minute: i64 = text.get(14..16)?.parse().ok()?;
    let second: i64 = text.get(17..19)?.parse().ok()?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || minute > 59 || second > 60 {
        return None;
    }

    Some(days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second)
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
