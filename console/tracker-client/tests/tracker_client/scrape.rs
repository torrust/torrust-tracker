//! `tracker_client udp|http scrape` against fake trackers that keep only the
//! first N info hashes.
//!
//! Trackers may truncate scrapes silently. Whatever the tracker's limit, the
//! client must keep stdout for the result and report the truncation as exactly
//! one warning record on stderr. See
//! `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`.
use serde_json::Value;

use super::tracker_client_bin;

/// What one `tracker_client` run produced, split by output channel.
struct Run {
    exit_code: Option<i32>,
    result: Value,
    stderr_records: Vec<Value>,
}

/// The fields of a `scrape_response_truncated` stderr record that tests check.
#[derive(Debug, PartialEq)]
struct TruncationWarning {
    requested: u64,
    returned: u64,
}

impl Run {
    fn of(args: &[String]) -> Self {
        let output = tracker_client_bin().args(args).output().expect("tracker_client should run");

        let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
        let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");

        Self {
            exit_code: output.status.code(),
            result: serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("stdout should be one JSON result: {stdout}")),
            stderr_records: stderr
                .lines()
                .map(|line| serde_json::from_str(line).unwrap_or_else(|_| panic!("stderr should be NDJSON: {line}")))
                .collect(),
        }
    }

    /// Every stderr record, read as a truncation warning; fails on any other record.
    fn stderr_warnings(&self) -> Vec<TruncationWarning> {
        self.stderr_records
            .iter()
            .map(|record| {
                let warning = &record["warning"];
                assert_eq!(
                    warning["kind"], "scrape_response_truncated",
                    "unexpected stderr record: {record}"
                );

                TruncationWarning {
                    requested: warning["requested"].as_u64().expect("requested count"),
                    returned: warning["returned"].as_u64().expect("returned count"),
                }
            })
            .collect()
    }
}

fn distinct_info_hashes(count: u8) -> Vec<String> {
    (1..=count).map(|index| format!("{index:040x}")).collect()
}

mod udp {
    use super::{Run, TruncationWarning, distinct_info_hashes};
    use crate::fake_trackers::FakeUdpTracker;

    fn scrape(tracker: &FakeUdpTracker, info_hashes: &[String]) -> Run {
        let args = ["udp".to_owned(), "scrape".to_owned(), tracker.address().to_string()];

        Run::of(&[args.as_slice(), info_hashes].concat())
    }

    fn returned_entries(run: &Run) -> usize {
        run.result["Scrape"]["torrent_stats"]
            .as_array()
            .expect("a UDP scrape result")
            .len()
    }

    #[test]
    fn it_should_warn_on_stderr_when_the_tracker_returns_fewer_entries_than_requested() {
        // Arrange
        let tracker = FakeUdpTracker::keeping_first(2);

        // Act
        let run = scrape(&tracker, &distinct_info_hashes(3));

        // Assert
        assert_eq!(run.exit_code, Some(0));
        assert_eq!(returned_entries(&run), 2);
        assert_eq!(
            run.stderr_warnings(),
            vec![TruncationWarning {
                requested: 3,
                returned: 2
            }]
        );
    }

    #[test]
    fn it_should_leave_stderr_empty_when_the_tracker_returns_every_requested_entry() {
        // Arrange
        let tracker = FakeUdpTracker::keeping_all();

        // Act
        let run = scrape(&tracker, &distinct_info_hashes(3));

        // Assert
        assert_eq!(run.exit_code, Some(0));
        assert_eq!(returned_entries(&run), 3);
        assert!(
            run.stderr_records.is_empty(),
            "stderr should be empty: {:?}",
            run.stderr_records
        );
    }
}

mod http {
    use super::{Run, TruncationWarning, distinct_info_hashes};
    use crate::fake_trackers::FakeHttpTracker;

    fn scrape(tracker: &FakeHttpTracker, info_hashes: &[String]) -> Run {
        let args = ["http".to_owned(), "scrape".to_owned(), tracker.url()];

        Run::of(&[args.as_slice(), info_hashes].concat())
    }

    fn returned_files(run: &Run) -> usize {
        run.result.as_object().expect("an HTTP scrape result").len()
    }

    #[test]
    fn it_should_warn_on_stderr_when_the_tracker_returns_fewer_files_than_requested() {
        // Arrange
        let tracker = FakeHttpTracker::keeping_first(2);

        // Act
        let run = scrape(&tracker, &distinct_info_hashes(3));

        // Assert
        assert_eq!(run.exit_code, Some(0));
        assert_eq!(returned_files(&run), 2);
        assert_eq!(
            run.stderr_warnings(),
            vec![TruncationWarning {
                requested: 3,
                returned: 2
            }]
        );
    }

    #[test]
    fn it_should_not_warn_when_repeated_info_hashes_collapse_into_one_file() {
        // Arrange
        let tracker = FakeHttpTracker::keeping_all();
        let distinct = distinct_info_hashes(2);
        let info_hashes_with_first_repeated = [distinct[0].clone(), distinct[1].clone(), distinct[0].clone()];

        // Act
        let run = scrape(&tracker, &info_hashes_with_first_repeated);

        // Assert
        assert_eq!(run.exit_code, Some(0));
        assert_eq!(returned_files(&run), 2);
        assert!(
            run.stderr_records.is_empty(),
            "stderr should be empty: {:?}",
            run.stderr_records
        );
    }
}
