//! Event detection: compares a snapshot of the pull request with the baseline taken when the poller started.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::snapshot::{Comment, PullState, Review, Snapshot};
use crate::timestamp::parse_utc;

/// Which items count as new.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Cutoff {
    /// Reviews that are not among the submitted ones present at the start, and comments with an ID
    /// above the highest comment ID present at the start.
    ///
    /// A review keeps the ID it got as a draft, so a review submitted later can have a lower ID than
    /// one already submitted. Comments have no draft state, so their IDs only grow.
    AfterIds {
        submitted_reviews: BTreeSet<u64>,
        issue_comment: u64,
        review_comment: u64,
    },
    /// Items created after this moment, in seconds since the Unix epoch.
    After(i64),
}

/// What the pull request looked like when the poller started waiting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Baseline {
    head_sha: String,
    cutoff: Cutoff,
}

impl Baseline {
    /// The baseline is the present: only what happens after `snapshot` counts.
    #[must_use]
    pub fn from_snapshot(snapshot: &Snapshot) -> Self {
        let highest = |ids: &mut dyn Iterator<Item = u64>| ids.max().unwrap_or(0);

        Self {
            head_sha: snapshot.head_sha.clone(),
            cutoff: Cutoff::AfterIds {
                submitted_reviews: snapshot
                    .reviews
                    .iter()
                    .filter(|review| review.state != "PENDING")
                    .map(|review| review.id)
                    .collect(),
                issue_comment: highest(&mut snapshot.issue_comments.iter().map(|comment| comment.id)),
                review_comment: highest(&mut snapshot.review_comments.iter().map(|comment| comment.id)),
            },
        }
    }

    /// The baseline is a moment: reviews and comments created strictly after `moment` count, so a
    /// reply that landed after a push but before the poller started is not missed, and an item
    /// already handled is not reported again. `head_sha` is the head commit to watch for changes.
    #[must_use]
    pub fn since(head_sha: &str, moment: i64) -> Self {
        Self {
            head_sha: head_sha.to_owned(),
            cutoff: Cutoff::After(moment),
        }
    }

    fn is_new_review(&self, review: &Review) -> bool {
        match &self.cutoff {
            Cutoff::AfterIds { submitted_reviews, .. } => !submitted_reviews.contains(&review.id),
            Cutoff::After(moment) => is_after(review.submitted_at.as_deref(), *moment),
        }
    }

    fn is_new_issue_comment(&self, comment: &Comment) -> bool {
        match &self.cutoff {
            Cutoff::AfterIds { issue_comment, .. } => comment.id > *issue_comment,
            Cutoff::After(moment) => is_after(Some(&comment.created_at), *moment),
        }
    }

    fn is_new_review_comment(&self, comment: &Comment) -> bool {
        match &self.cutoff {
            Cutoff::AfterIds { review_comment, .. } => comment.id > *review_comment,
            Cutoff::After(moment) => is_after(Some(&comment.created_at), *moment),
        }
    }
}

/// A timestamp that cannot be read counts as new: a spurious event is cheaper than a missed review.
fn is_after(timestamp: Option<&str>, moment: i64) -> bool {
    timestamp.and_then(parse_utc).is_none_or(|time| time > moment)
}

/// Which kind of comment arrived.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentKind {
    /// A comment in the pull-request conversation.
    Issue,
    /// An inline comment on the diff.
    Review,
}

/// The result of one wait: the single JSON object written to stdout.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    /// A watched reviewer submitted a review.
    Review {
        /// The reviewer login.
        reviewer: String,
        /// The review ID.
        review_id: u64,
        /// The review state, for example `COMMENTED` or `APPROVED`.
        state: String,
        /// The commit the review was made on.
        review_commit_id: Option<String>,
        /// The review page.
        url: String,
        /// The current head commit.
        head_sha: String,
        /// When the review was submitted. Resume with `--since` to skip it and everything before it.
        #[serde(skip_serializing_if = "Option::is_none")]
        cursor: Option<String>,
    },
    /// A watched reviewer commented.
    Comment {
        /// Whether the comment is in the conversation or inline on the diff.
        comment_kind: CommentKind,
        /// The commenter login.
        reviewer: String,
        /// The comment ID.
        comment_id: u64,
        /// The comment page.
        url: String,
        /// The current head commit.
        head_sha: String,
        /// When the comment was created. Resume with `--since` to skip it and everything before it.
        #[serde(skip_serializing_if = "Option::is_none")]
        cursor: Option<String>,
    },
    /// The head commit changed: someone pushed.
    HeadChanged {
        /// The head commit when the wait began.
        previous_head_sha: String,
        /// The current head commit.
        head_sha: String,
    },
    /// The pull request was closed without merging.
    PrClosed {
        /// The head commit at the time.
        head_sha: String,
    },
    /// The pull request was merged.
    PrMerged {
        /// The head commit at the time.
        head_sha: String,
    },
    /// Nothing happened within the wait.
    Timeout {
        /// How long the poller waited.
        waited_seconds: u64,
        /// The head commit at the start of the wait.
        head_sha: String,
    },
}

/// The first event, in priority order, that `snapshot` shows beyond `baseline`.
///
/// A closed or merged pull request wins, then a new review, then a new comment, then a new head
/// commit. Only logins in `reviewers` count, compared without regard to case.
#[must_use]
pub fn detect(baseline: &Baseline, snapshot: &Snapshot, reviewers: &[String]) -> Option<Event> {
    let head_sha = snapshot.head_sha.clone();

    match snapshot.state {
        PullState::Merged => return Some(Event::PrMerged { head_sha }),
        PullState::Closed => return Some(Event::PrClosed { head_sha }),
        PullState::Open => {}
    }

    new_review(baseline, snapshot, reviewers)
        .or_else(|| new_comment(baseline, snapshot, reviewers))
        .or_else(|| {
            (snapshot.head_sha != baseline.head_sha).then(|| Event::HeadChanged {
                previous_head_sha: baseline.head_sha.clone(),
                head_sha: snapshot.head_sha.clone(),
            })
        })
}

fn is_watched(login: Option<&str>, reviewers: &[String]) -> bool {
    login.is_some_and(|login| reviewers.iter().any(|reviewer| reviewer.eq_ignore_ascii_case(login)))
}

fn new_review(baseline: &Baseline, snapshot: &Snapshot, reviewers: &[String]) -> Option<Event> {
    snapshot
        .reviews
        .iter()
        .filter(|review| review.state != "PENDING" && baseline.is_new_review(review))
        .find(|review| is_watched(review.user.as_ref().map(|user| user.login.as_str()), reviewers))
        .map(|review| Event::Review {
            reviewer: review.user.as_ref().map(|user| user.login.clone()).unwrap_or_default(),
            review_id: review.id,
            state: review.state.clone(),
            review_commit_id: review.commit_id.clone(),
            url: review.html_url.clone(),
            head_sha: snapshot.head_sha.clone(),
            cursor: review.submitted_at.clone(),
        })
}

fn new_comment(baseline: &Baseline, snapshot: &Snapshot, reviewers: &[String]) -> Option<Event> {
    let issue = snapshot
        .issue_comments
        .iter()
        .filter(|comment| baseline.is_new_issue_comment(comment))
        .map(|comment| (CommentKind::Issue, comment));
    let review = snapshot
        .review_comments
        .iter()
        .filter(|comment| baseline.is_new_review_comment(comment))
        .map(|comment| (CommentKind::Review, comment));

    issue
        .chain(review)
        .find(|(_, comment)| is_watched(comment.user.as_ref().map(|user| user.login.as_str()), reviewers))
        .map(|(comment_kind, comment)| Event::Comment {
            comment_kind,
            reviewer: comment.user.as_ref().map(|user| user.login.clone()).unwrap_or_default(),
            comment_id: comment.id,
            url: comment.html_url.clone(),
            head_sha: snapshot.head_sha.clone(),
            cursor: Some(comment.created_at.clone()),
        })
}

#[cfg(test)]
mod tests {
    use super::{Baseline, CommentKind, Event, detect};
    use crate::snapshot::{Comment, PullState, Review, Snapshot, User};

    fn reviewers() -> Vec<String> {
        vec!["da2ce7".to_owned(), "copilot-pull-request-reviewer[bot]".to_owned()]
    }

    fn user(login: &str) -> User {
        User { login: login.to_owned() }
    }

    fn review(id: u64, login: &str, state: &str) -> Review {
        Review {
            id,
            user: Some(user(login)),
            state: state.to_owned(),
            commit_id: Some("head".to_owned()),
            html_url: format!("https://example.test/review/{id}"),
            submitted_at: Some("2026-10-28T07:00:00Z".to_owned()),
        }
    }

    fn comment(id: u64, login: &str, created_at: &str) -> Comment {
        Comment {
            id,
            user: Some(user(login)),
            html_url: format!("https://example.test/comment/{id}"),
            created_at: created_at.to_owned(),
        }
    }

    fn snapshot() -> Snapshot {
        Snapshot {
            head_sha: "head".to_owned(),
            state: PullState::Open,
            reviews: vec![review(10, "da2ce7", "APPROVED")],
            issue_comments: vec![comment(20, "da2ce7", "2026-10-28T06:00:00Z")],
            review_comments: vec![comment(30, "da2ce7", "2026-10-28T06:00:00Z")],
        }
    }

    #[test]
    fn it_should_not_report_anything_that_was_already_there_when_the_wait_began() {
        let before = snapshot();

        assert_eq!(detect(&Baseline::from_snapshot(&before), &before, &reviewers()), None);
    }

    #[test]
    fn it_should_report_a_new_review_from_a_watched_reviewer() {
        let before = snapshot();
        let mut after = before.clone();
        after.reviews.push(review(11, "da2ce7", "CHANGES_REQUESTED"));

        let event = detect(&Baseline::from_snapshot(&before), &after, &reviewers());

        assert_eq!(
            event,
            Some(Event::Review {
                reviewer: "da2ce7".to_owned(),
                review_id: 11,
                state: "CHANGES_REQUESTED".to_owned(),
                review_commit_id: Some("head".to_owned()),
                url: "https://example.test/review/11".to_owned(),
                head_sha: "head".to_owned(),
                cursor: Some("2026-10-28T07:00:00Z".to_owned()),
            })
        );
    }

    #[test]
    fn it_should_report_a_review_submitted_later_with_a_lower_id_than_one_already_submitted() {
        let mut before = snapshot();
        before
            .reviews
            .push(review(12, "copilot-pull-request-reviewer[bot]", "COMMENTED"));
        let mut after = before.clone();
        // A draft started before review 12 keeps its lower ID when it is finally submitted.
        after.reviews.push(review(11, "da2ce7", "CHANGES_REQUESTED"));

        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Review { review_id: 11, .. })
        ));
    }

    #[test]
    fn it_should_report_a_review_that_was_pending_at_the_start_when_it_is_submitted() {
        let mut before = snapshot();
        before.reviews.push(review(11, "da2ce7", "PENDING"));
        let mut after = before.clone();
        after.reviews[1] = review(11, "da2ce7", "COMMENTED");

        assert_eq!(detect(&Baseline::from_snapshot(&before), &before, &reviewers()), None);
        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Review { review_id: 11, .. })
        ));
    }

    #[test]
    fn it_should_report_a_new_copilot_review_even_though_copilot_never_approves() {
        let before = snapshot();
        let mut after = before.clone();
        after
            .reviews
            .push(review(11, "copilot-pull-request-reviewer[bot]", "COMMENTED"));

        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Review { state, .. }) if state == "COMMENTED"
        ));
    }

    #[test]
    fn it_should_match_reviewer_logins_without_regard_to_case() {
        let before = snapshot();
        let mut after = before.clone();
        after.reviews.push(review(11, "DA2CE7", "COMMENTED"));

        assert!(detect(&Baseline::from_snapshot(&before), &after, &reviewers()).is_some());
    }

    #[test]
    fn it_should_ignore_reviews_and_comments_from_accounts_outside_the_reviewer_set() {
        let before = snapshot();
        let mut after = before.clone();
        after.reviews.push(review(11, "josecelano", "COMMENTED"));
        after.issue_comments.push(comment(21, "josecelano", "2026-10-28T08:00:00Z"));
        after.review_comments.push(comment(31, "josecelano", "2026-10-28T08:00:00Z"));

        assert_eq!(detect(&Baseline::from_snapshot(&before), &after, &reviewers()), None);
    }

    #[test]
    fn it_should_ignore_reviews_from_deleted_accounts() {
        let before = snapshot();
        let mut after = before.clone();
        after.reviews.push(Review {
            user: None,
            ..review(11, "ghost", "COMMENTED")
        });

        assert_eq!(detect(&Baseline::from_snapshot(&before), &after, &reviewers()), None);
    }

    #[test]
    fn it_should_ignore_pending_reviews() {
        let before = snapshot();
        let mut after = before.clone();
        after.reviews.push(review(11, "da2ce7", "PENDING"));

        assert_eq!(detect(&Baseline::from_snapshot(&before), &after, &reviewers()), None);
    }

    #[test]
    fn it_should_report_a_new_conversation_comment_from_a_watched_reviewer() {
        let before = snapshot();
        let mut after = before.clone();
        after.issue_comments.push(comment(21, "da2ce7", "2026-10-28T08:00:00Z"));

        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Comment {
                comment_kind: CommentKind::Issue,
                comment_id: 21,
                ..
            })
        ));
    }

    #[test]
    fn it_should_carry_the_creation_time_of_a_reported_comment_as_the_cursor() {
        let before = snapshot();
        let mut after = before.clone();
        after.issue_comments.push(comment(21, "da2ce7", "2026-10-28T08:00:00Z"));

        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Comment { cursor: Some(cursor), .. }) if cursor == "2026-10-28T08:00:00Z"
        ));
    }

    #[test]
    fn it_should_report_a_new_inline_review_comment_from_a_watched_reviewer() {
        let before = snapshot();
        let mut after = before.clone();
        after.review_comments.push(comment(31, "da2ce7", "2026-10-28T08:00:00Z"));

        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Comment {
                comment_kind: CommentKind::Review,
                comment_id: 31,
                ..
            })
        ));
    }

    #[test]
    fn it_should_report_a_head_commit_change() {
        let before = snapshot();
        let mut after = before.clone();
        after.head_sha = "newer".to_owned();

        assert_eq!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::HeadChanged {
                previous_head_sha: "head".to_owned(),
                head_sha: "newer".to_owned(),
            })
        );
    }

    #[test]
    fn it_should_report_a_merged_pull_request_before_anything_else() {
        let before = snapshot();
        let mut after = before.clone();
        after.state = PullState::Merged;
        after.reviews.push(review(11, "da2ce7", "APPROVED"));

        assert_eq!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::PrMerged {
                head_sha: "head".to_owned()
            })
        );
    }

    #[test]
    fn it_should_report_a_closed_pull_request() {
        let before = snapshot();
        let mut after = before.clone();
        after.state = PullState::Closed;

        assert_eq!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::PrClosed {
                head_sha: "head".to_owned()
            })
        );
    }

    #[test]
    fn it_should_report_a_review_before_a_comment_and_a_comment_before_a_head_change() {
        let before = snapshot();
        let mut after = before.clone();
        after.head_sha = "newer".to_owned();
        after.issue_comments.push(comment(21, "da2ce7", "2026-10-28T08:00:00Z"));

        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Comment { .. })
        ));

        after.reviews.push(review(11, "da2ce7", "COMMENTED"));

        assert!(matches!(
            detect(&Baseline::from_snapshot(&before), &after, &reviewers()),
            Some(Event::Review { .. })
        ));
    }

    #[test]
    fn it_should_count_items_created_after_the_commit_date_when_the_baseline_is_a_commit() {
        let committed_at = 1_793_167_200; // 2026-10-28T06:00:00Z
        let baseline = Baseline::since("head", committed_at);
        let mut current = snapshot();
        current.reviews = vec![Review {
            submitted_at: Some("2026-10-28T06:00:01Z".to_owned()),
            ..review(10, "da2ce7", "COMMENTED")
        }];
        current.issue_comments = vec![];
        current.review_comments = vec![];

        assert!(matches!(
            detect(&baseline, &current, &reviewers()),
            Some(Event::Review { review_id: 10, .. })
        ));
    }

    #[test]
    fn it_should_not_count_items_created_at_or_before_the_commit_date_when_the_baseline_is_a_commit() {
        let committed_at = 1_793_174_400; // 2026-10-28T08:00:00Z, after every item in the fixture
        let baseline = Baseline::since("head", committed_at);

        assert_eq!(detect(&baseline, &snapshot(), &reviewers()), None);
    }
}
