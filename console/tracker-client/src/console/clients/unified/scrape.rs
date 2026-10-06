//! Scrape result checks shared by the HTTP and UDP commands.
//!
//! Trackers may keep only the first N info hashes of a scrape and ignore the
//! rest silently. The client does not assume any tracker's limit; it reports
//! when fewer entries come back than were requested. See
//! `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`.
//!
//! The protocols differ: a UDP response has one entry per requested info hash,
//! while an HTTP `files` dictionary collapses duplicates and may omit unknown
//! torrents. So for HTTP, fewer files means truncation *or* omission, which is
//! why the warning says the tracker *may* truncate.
use std::collections::HashSet;

use torrust_info_hash::InfoHash;

/// Returns an NDJSON stderr record when the tracker returned fewer scrape
/// entries than were requested.
#[must_use]
pub fn truncation_warning(requested: usize, returned: usize) -> Option<String> {
    (returned < requested).then(|| {
        serde_json::json!({
            "warning": {
                "kind": "scrape_response_truncated",
                "requested": requested,
                "returned": returned,
                "message": format!(
                    "requested {requested} info hashes, tracker returned {returned}; the tracker may truncate scrapes"
                ),
            }
        })
        .to_string()
    })
}

/// The number of entries an HTTP scrape can return: its response dictionary
/// collapses repeated info hashes.
#[must_use]
pub fn distinct_count(info_hashes: &[InfoHash]) -> usize {
    info_hashes.iter().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use torrust_info_hash::InfoHash;

    use super::{distinct_count, truncation_warning};

    #[test]
    fn it_should_warn_with_both_counts_when_the_tracker_returns_fewer_entries_than_requested() {
        // Act
        let warning = truncation_warning(75, 74).expect("a warning");

        // Assert
        let record: serde_json::Value = serde_json::from_str(&warning).unwrap();
        assert_eq!(record["warning"]["kind"], "scrape_response_truncated");
        assert_eq!(record["warning"]["requested"], 75);
        assert_eq!(record["warning"]["returned"], 74);
    }

    #[test]
    fn it_should_not_warn_when_the_tracker_returns_every_requested_entry() {
        assert_eq!(truncation_warning(74, 74), None);
    }

    #[test]
    fn it_should_count_repeated_info_hashes_once_for_an_http_scrape() {
        // Arrange
        let first = InfoHash([1; 20]);
        let second = InfoHash([2; 20]);

        // Act
        let count = distinct_count(&[first, second, first]);

        // Assert
        assert_eq!(count, 2);
    }
}
