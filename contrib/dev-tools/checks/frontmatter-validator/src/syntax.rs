//! The frozen v1 value syntax shared by the Rust validator and the generated JSON Schema.
//!
//! Each regex constant is the schema projection of the predicate declared directly below it. The
//! predicate is authoritative; the constant must describe the same language.

pub const SKILL_NAME_PATTERN: &str = r"^[a-z0-9]+(?:-[a-z0-9]+)*$";

pub fn is_skill_name(value: &str) -> bool {
    is_lowercase_identifier(value)
}

pub const RELATED_ARTIFACT_PATTERN: &str = r"^(?:(?!.*(?:^|/)\.{1,2}(?:/|$))[^\s/:#]+(?:/[^\s/:#]+)*|issue #[1-9][0-9]*|review-finding:pr-[1-9][0-9]*-[a-z0-9]+(?:-[a-z0-9]+)*)$";

pub fn is_related_artifact(value: &str) -> bool {
    is_repository_relative_path(value) || is_issue_reference(value) || is_review_finding(value)
}

pub const REPOSITORY_RELATIVE_PATH_PATTERN: &str = r"^(?!.*(?:^|/)\.{1,2}(?:/|$))[^\s/:#]+(?:/[^\s/:#]+)*$";

pub fn is_repository_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && !value.contains(':')
        && !value.contains('#')
        && !value.chars().any(char::is_whitespace)
        && !value.split('/').any(|component| matches!(component, "" | "." | ".."))
}

fn is_issue_reference(value: &str) -> bool {
    value.strip_prefix("issue #").is_some_and(is_positive_integer)
}

fn is_review_finding(value: &str) -> bool {
    let Some(value) = value.strip_prefix("review-finding:pr-") else {
        return false;
    };
    let Some((pull_request, identifier)) = value.split_once('-') else {
        return false;
    };

    is_positive_integer(pull_request) && is_lowercase_identifier(identifier)
}

fn is_positive_integer(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()) && value != "0"
}

fn is_lowercase_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .split('-')
            .all(|segment| !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

pub const UTC_MINUTE_PATTERN: &str = r"^[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}$";

pub fn has_utc_minute_layout(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 16
        && bytes.get(4) == Some(&b'-')
        && bytes.get(7) == Some(&b'-')
        && bytes.get(10) == Some(&b' ')
        && bytes.get(13) == Some(&b':')
        && bytes
            .iter()
            .enumerate()
            .filter(|(index, _)| !matches!(index, 4 | 7 | 10 | 13))
            .all(|(_, byte)| byte.is_ascii_digit())
}

/// Calendar validity is beyond the regex; callers must check the layout first.
pub fn is_valid_utc_minute_calendar(value: &str) -> bool {
    debug_assert!(has_utc_minute_layout(value));
    let year = value[0..4].parse::<u16>().expect("UTC-minute layout contains ASCII digits");
    let month = value[5..7].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
    let day = value[8..10].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
    let hour = value[11..13].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
    let minute = value[14..16].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return false,
    };

    (1..=days_in_month).contains(&day) && hour < 24 && minute < 60
}

const fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

#[cfg(test)]
mod tests {
    // Owns the pure v1 value-syntax predicates; mapping-level policy is tested in `profile`.

    use super::{has_utc_minute_layout, is_valid_utc_minute_calendar};

    #[test]
    fn it_should_distinguish_utc_minute_layout_from_calendar_boundaries() {
        // Arrange: each row changes one UTC-minute layout or calendar boundary.
        let cases = [
            ("2024-02-29 23:59", true, true),
            ("2025-02-29 23:59", true, false),
            ("2026-04-31 12:00", true, false),
            ("2026-13-01 12:00", true, false),
            ("2026-01-01 24:00", true, false),
            ("2026-01-01 12:60", true, false),
            ("2026-01-01 12:0", false, false),
            ("2026-01-01 12:é0", false, false),
        ];

        for (value, expected_layout, expected_calendar) in cases {
            // Act: evaluate layout first, then calendar validity only for a safe fixed layout.
            let actual_layout = has_utc_minute_layout(value);
            let actual_calendar = if actual_layout {
                is_valid_utc_minute_calendar(value)
            } else {
                false
            };

            // Assert: each independent boundary has the documented result without panics.
            assert_eq!(actual_layout, expected_layout, "unexpected layout result for `{value}`");
            assert_eq!(actual_calendar, expected_calendar, "unexpected calendar result for `{value}`");
        }
    }
}
