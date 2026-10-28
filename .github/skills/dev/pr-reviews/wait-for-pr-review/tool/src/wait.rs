//! The wait loop: take a baseline, then poll until an event happens or the wait runs out.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::Error;
use crate::event::{Baseline, Event, detect};
use crate::gh::Gh;
use crate::snapshot::{PullRequest, Snapshot, commit_date, take_snapshot};

const SLEEP_SLICE: Duration = Duration::from_millis(100);

/// How long to wait, how often to look, and whom to listen to.
#[derive(Clone, Debug)]
pub struct Config {
    /// Logins whose reviews and comments count as events.
    pub reviewers: Vec<String>,
    /// The longest the poller waits before it reports a timeout.
    pub timeout: Duration,
    /// The pause between two looks at the pull request.
    pub poll_interval: Duration,
    /// When set, the baseline is the commit date of this commit instead of the start of the wait.
    pub since_head_sha: Option<String>,
}

/// Time, kept behind a trait so tests never sleep.
pub trait Clock {
    /// Time passed since the wait began.
    fn elapsed(&self) -> Duration;

    /// Pauses for `duration`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Interrupted`] when a signal arrives during the pause.
    fn sleep(&self, duration: Duration) -> Result<(), Error>;
}

/// The wall clock. A pause ends early with [`Error::Interrupted`] when `interrupted` is set.
#[derive(Debug)]
pub struct SystemClock {
    started: Instant,
    interrupted: Arc<AtomicBool>,
}

impl SystemClock {
    /// A clock that starts now.
    #[must_use]
    pub fn start(interrupted: Arc<AtomicBool>) -> Self {
        Self {
            started: Instant::now(),
            interrupted,
        }
    }
}

impl Clock for SystemClock {
    fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    fn sleep(&self, duration: Duration) -> Result<(), Error> {
        let deadline = Instant::now() + duration;

        loop {
            if self.interrupted.load(Ordering::Relaxed) {
                return Err(Error::Interrupted);
            }

            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Ok(());
            }

            thread::sleep(remaining.min(SLEEP_SLICE));
        }
    }
}

/// Blocks until something happens on `pull_request`, then returns what happened.
///
/// # Errors
///
/// Returns an error when `gh` is unusable, a request fails, or a signal interrupts the wait.
pub fn wait(gh: &impl Gh, clock: &impl Clock, pull_request: &PullRequest, config: &Config) -> Result<Event, Error> {
    gh.check_authenticated()?;

    let first = take_snapshot(gh, pull_request)?;
    let baseline = match &config.since_head_sha {
        Some(sha) => Baseline::since_commit(sha, commit_date(gh, pull_request, sha)?),
        None => Baseline::from_snapshot(&first),
    };

    if let Some(event) = detect(&baseline, &first, &config.reviewers) {
        return Ok(event);
    }

    poll(gh, clock, pull_request, config, &baseline, &first)
}

fn poll(
    gh: &impl Gh,
    clock: &impl Clock,
    pull_request: &PullRequest,
    config: &Config,
    baseline: &Baseline,
    first: &Snapshot,
) -> Result<Event, Error> {
    loop {
        let remaining = config.timeout.saturating_sub(clock.elapsed());
        if remaining.is_zero() {
            return Ok(Event::Timeout {
                waited_seconds: config.timeout.as_secs(),
                head_sha: first.head_sha.clone(),
            });
        }

        clock.sleep(config.poll_interval.min(remaining))?;

        let snapshot = take_snapshot(gh, pull_request)?;
        if let Some(event) = detect(baseline, &snapshot, &config.reviewers) {
            return Ok(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::time::Duration;

    use serde_json::{Value, json};

    use super::{Clock, Config, wait};
    use crate::error::Error;
    use crate::event::Event;
    use crate::snapshot::PullRequest;
    use crate::snapshot::fake::FakeGh;

    const PULL: &str = "repos/torrust/torrust-tracker/pulls/7";
    const REVIEWS: &str = "repos/torrust/torrust-tracker/pulls/7/reviews?per_page=100";
    const ISSUE_COMMENTS: &str = "repos/torrust/torrust-tracker/issues/7/comments?per_page=100";
    const REVIEW_COMMENTS: &str = "repos/torrust/torrust-tracker/pulls/7/comments?per_page=100";

    /// A clock that only advances when the loop sleeps, and can raise an interrupt after N sleeps.
    #[derive(Default)]
    struct FakeClock {
        elapsed: Cell<Duration>,
        sleeps: RefCell<Vec<Duration>>,
        interrupt_after: Option<usize>,
    }

    impl Clock for FakeClock {
        fn elapsed(&self) -> Duration {
            self.elapsed.get()
        }

        fn sleep(&self, duration: Duration) -> Result<(), Error> {
            if self.interrupt_after == Some(self.sleeps.borrow().len()) {
                return Err(Error::Interrupted);
            }
            self.sleeps.borrow_mut().push(duration);
            self.elapsed.set(self.elapsed.get() + duration);
            Ok(())
        }
    }

    fn pull_request() -> PullRequest {
        PullRequest {
            owner: "torrust".to_owned(),
            repository: "torrust-tracker".to_owned(),
            number: 7,
        }
    }

    fn config() -> Config {
        Config {
            reviewers: vec!["da2ce7".to_owned()],
            timeout: Duration::from_secs(900),
            poll_interval: Duration::from_secs(120),
            since_head_sha: None,
        }
    }

    fn pull(head: &str, state: &str, merged: bool) -> Value {
        json!({"head": {"sha": head}, "state": state, "merged": merged})
    }

    fn review(id: u64, login: &str) -> Value {
        json!({"id": id, "user": {"login": login}, "state": "COMMENTED", "commit_id": "a", "html_url": "u", "submitted_at": "2026-10-28T07:00:00Z"})
    }

    fn quiet_pull_request() -> FakeGh {
        let gh = FakeGh::new();
        gh.respond(PULL, pull("a", "open", false));
        gh.respond(REVIEWS, json!([review(1, "da2ce7")]));
        gh.respond(ISSUE_COMMENTS, json!([]));
        gh.respond(REVIEW_COMMENTS, json!([]));
        gh
    }

    #[test]
    fn it_should_report_a_timeout_after_the_configured_wait_when_nothing_happens() {
        let gh = quiet_pull_request();
        let clock = FakeClock::default();

        let event = wait(&gh, &clock, &pull_request(), &config()).unwrap();

        assert_eq!(
            event,
            Event::Timeout {
                waited_seconds: 900,
                head_sha: "a".to_owned()
            }
        );
        assert_eq!(clock.elapsed(), Duration::from_secs(900));
    }

    #[test]
    fn it_should_sleep_one_poll_interval_at_a_time_and_cut_the_last_sleep_short_at_the_deadline() {
        let gh = quiet_pull_request();
        let clock = FakeClock::default();
        let config = Config {
            timeout: Duration::from_secs(300),
            ..config()
        };

        wait(&gh, &clock, &pull_request(), &config).unwrap();

        assert_eq!(
            *clock.sleeps.borrow(),
            [Duration::from_secs(120), Duration::from_secs(120), Duration::from_secs(60)]
        );
    }

    #[test]
    fn it_should_return_the_review_that_arrives_while_waiting() {
        let gh = quiet_pull_request();
        gh.respond(PULL, pull("a", "open", false));
        gh.respond(REVIEWS, json!([review(1, "da2ce7"), review(2, "da2ce7")]));

        let event = wait(&gh, &FakeClock::default(), &pull_request(), &config()).unwrap();

        assert!(matches!(event, Event::Review { review_id: 2, .. }));
    }

    #[test]
    fn it_should_return_immediately_when_the_pull_request_is_already_merged() {
        let gh = FakeGh::new();
        gh.respond(PULL, pull("a", "closed", true));
        gh.respond(REVIEWS, json!([]));
        gh.respond(ISSUE_COMMENTS, json!([]));
        gh.respond(REVIEW_COMMENTS, json!([]));
        let clock = FakeClock::default();

        let event = wait(&gh, &clock, &pull_request(), &config()).unwrap();

        assert_eq!(
            event,
            Event::PrMerged {
                head_sha: "a".to_owned()
            }
        );
        assert!(clock.sleeps.borrow().is_empty());
    }

    #[test]
    fn it_should_return_a_review_that_predates_the_start_when_the_baseline_is_a_commit() {
        let gh = quiet_pull_request();
        gh.respond(
            "repos/torrust/torrust-tracker/commits/a",
            json!({"commit": {"committer": {"date": "2026-10-28T06:00:00Z"}}}),
        );
        let config = Config {
            since_head_sha: Some("a".to_owned()),
            ..config()
        };
        let clock = FakeClock::default();

        let event = wait(&gh, &clock, &pull_request(), &config).unwrap();

        assert!(matches!(event, Event::Review { review_id: 1, .. }));
        assert!(clock.sleeps.borrow().is_empty());
    }

    #[test]
    fn it_should_stop_with_an_error_when_gh_is_not_authenticated() {
        let mut gh = quiet_pull_request();
        gh.authenticated = false;

        let result = wait(&gh, &FakeClock::default(), &pull_request(), &config());

        assert!(matches!(result, Err(Error::GhNotAuthenticated(_))));
        assert!(
            gh.calls.borrow().is_empty(),
            "no API call should be made before authentication is confirmed"
        );
    }

    #[test]
    fn it_should_stop_with_an_error_when_a_poll_fails() {
        let gh = quiet_pull_request();
        gh.respond(PULL, pull("a", "open", false));
        gh.fail(PULL, "HTTP 502");

        let result = wait(&gh, &FakeClock::default(), &pull_request(), &config());

        assert!(matches!(result, Err(Error::GhApi { .. })));
    }

    #[test]
    fn it_should_stop_with_an_interrupted_error_when_a_signal_arrives_during_a_pause() {
        let gh = quiet_pull_request();
        let clock = FakeClock {
            interrupt_after: Some(1),
            ..FakeClock::default()
        };

        let result = wait(&gh, &clock, &pull_request(), &config());

        assert!(matches!(result, Err(Error::Interrupted)));
    }
}
