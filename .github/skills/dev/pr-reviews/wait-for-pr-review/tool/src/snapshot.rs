//! A point-in-time view of a pull request, assembled from the GitHub REST API.

use serde::Deserialize;
use serde_json::Value;

use crate::error::Error;
use crate::gh::Gh;
use crate::timestamp::parse_utc;

/// The pull request being watched.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PullRequest {
    /// GitHub repository owner.
    pub owner: String,
    /// GitHub repository name.
    pub repository: String,
    /// Pull-request number.
    pub number: u64,
}

impl PullRequest {
    fn endpoint(&self, suffix: &str) -> String {
        format!("repos/{}/{}/{suffix}", self.owner, self.repository)
    }
}

/// Whether the pull request still accepts changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PullState {
    /// The pull request is open.
    Open,
    /// The pull request was closed without merging.
    Closed,
    /// The pull request was merged.
    Merged,
}

/// A GitHub account, as embedded in reviews and comments. Deleted accounts appear as `null`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct User {
    /// The account login.
    pub login: String,
}

/// A submitted pull-request review.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Review {
    /// The review ID. IDs only grow, so a higher ID is a later review.
    pub id: u64,
    /// The reviewer.
    pub user: Option<User>,
    /// `APPROVED`, `COMMENTED`, `CHANGES_REQUESTED`, `DISMISSED`, or `PENDING`.
    pub state: String,
    /// The commit the review was made on.
    pub commit_id: Option<String>,
    /// The review page.
    pub html_url: String,
    /// When the review was submitted. Absent for pending reviews.
    pub submitted_at: Option<String>,
}

/// A pull-request issue comment or inline review comment.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Comment {
    /// The comment ID. IDs only grow within one comment kind.
    pub id: u64,
    /// The author.
    pub user: Option<User>,
    /// The comment page.
    pub html_url: String,
    /// When the comment was created.
    pub created_at: String,
}

/// Everything the poller needs to decide whether something happened.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    /// The current head commit.
    pub head_sha: String,
    /// Whether the pull request is open, closed, or merged.
    pub state: PullState,
    /// Every submitted review.
    pub reviews: Vec<Review>,
    /// Every pull-request conversation comment.
    pub issue_comments: Vec<Comment>,
    /// Every inline review comment.
    pub review_comments: Vec<Comment>,
}

#[derive(Deserialize)]
struct PullResponse {
    head: Head,
    state: String,
    merged: bool,
}

#[derive(Deserialize)]
struct Head {
    sha: String,
}

#[derive(Deserialize)]
struct CommitResponse {
    commit: CommitDetail,
}

#[derive(Deserialize)]
struct CommitDetail {
    committer: Option<Committer>,
}

#[derive(Deserialize)]
struct Committer {
    date: String,
}

/// Fetches the current state of `pull_request`.
///
/// # Errors
///
/// Returns an error when a request fails or a response does not have the expected shape.
pub fn take_snapshot(gh: &impl Gh, pull_request: &PullRequest) -> Result<Snapshot, Error> {
    let pull_endpoint = pull_request.endpoint(&format!("pulls/{}", pull_request.number));
    let pull: PullResponse = decode(&pull_endpoint, gh.get(&pull_endpoint)?)?;

    let state = if pull.merged {
        PullState::Merged
    } else if pull.state == "open" {
        PullState::Open
    } else {
        PullState::Closed
    };

    Ok(Snapshot {
        head_sha: pull.head.sha,
        state,
        reviews: fetch_all(gh, &format!("{pull_endpoint}/reviews?per_page=100"))?,
        issue_comments: fetch_all(
            gh,
            &pull_request.endpoint(&format!("issues/{}/comments?per_page=100", pull_request.number)),
        )?,
        review_comments: fetch_all(gh, &format!("{pull_endpoint}/comments?per_page=100"))?,
    })
}

/// The committer date of `sha`, in seconds since the Unix epoch.
///
/// # Errors
///
/// Returns an error when the request fails or the commit has no parsable committer date.
pub fn commit_date(gh: &impl Gh, pull_request: &PullRequest, sha: &str) -> Result<i64, Error> {
    let endpoint = pull_request.endpoint(&format!("commits/{sha}"));
    let commit: CommitResponse = decode(&endpoint, gh.get(&endpoint)?)?;

    commit
        .commit
        .committer
        .and_then(|committer| parse_utc(&committer.date))
        .ok_or_else(|| Error::CommitDate { sha: sha.to_owned() })
}

fn fetch_all<T: for<'de> Deserialize<'de>>(gh: &impl Gh, endpoint: &str) -> Result<Vec<T>, Error> {
    gh.get_all(endpoint)?.into_iter().map(|item| decode(endpoint, item)).collect()
}

fn decode<T: for<'de> Deserialize<'de>>(endpoint: &str, value: Value) -> Result<T, Error> {
    serde_json::from_value(value).map_err(|error| Error::MalformedResponse {
        endpoint: endpoint.to_owned(),
        reason: error.to_string(),
    })
}

#[cfg(test)]
pub mod fake {
    //! A scripted `gh`, used by the unit tests of the modules that sit above the boundary.

    use std::cell::RefCell;
    use std::collections::{HashMap, VecDeque};

    use serde_json::Value;

    use super::{Error, Gh};

    /// Returns the queued responses of an endpoint in order, then repeats the last one.
    #[derive(Default)]
    pub struct FakeGh {
        responses: RefCell<HashMap<String, VecDeque<Result<Value, String>>>>,
        pub calls: RefCell<Vec<String>>,
        pub authenticated: bool,
    }

    impl FakeGh {
        #[must_use]
        pub fn new() -> Self {
            Self {
                authenticated: true,
                ..Self::default()
            }
        }

        pub fn respond(&self, endpoint: &str, response: Value) {
            self.responses
                .borrow_mut()
                .entry(endpoint.to_owned())
                .or_default()
                .push_back(Ok(response));
        }

        pub fn fail(&self, endpoint: &str, message: &str) {
            self.responses
                .borrow_mut()
                .entry(endpoint.to_owned())
                .or_default()
                .push_back(Err(message.to_owned()));
        }

        fn next(&self, endpoint: &str) -> Result<Value, Error> {
            self.calls.borrow_mut().push(endpoint.to_owned());
            let mut responses = self.responses.borrow_mut();
            let queue = responses
                .get_mut(endpoint)
                .unwrap_or_else(|| panic!("no response scripted for {endpoint}"));
            let response = if queue.len() > 1 {
                queue.pop_front().unwrap()
            } else {
                queue.front().cloned().unwrap()
            };

            response.map_err(|message| Error::GhApi {
                endpoint: endpoint.to_owned(),
                message,
            })
        }
    }

    impl Gh for FakeGh {
        fn check_authenticated(&self) -> Result<(), Error> {
            if self.authenticated {
                Ok(())
            } else {
                Err(Error::GhNotAuthenticated("not logged in".to_owned()))
            }
        }

        fn get(&self, endpoint: &str) -> Result<Value, Error> {
            self.next(endpoint)
        }

        fn get_all(&self, endpoint: &str) -> Result<Vec<Value>, Error> {
            Ok(self.next(endpoint)?.as_array().cloned().unwrap_or_default())
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::fake::FakeGh;
    use super::{PullRequest, PullState, commit_date, take_snapshot};
    use crate::error::Error;

    fn pull_request() -> PullRequest {
        PullRequest {
            owner: "torrust".to_owned(),
            repository: "torrust-tracker".to_owned(),
            number: 7,
        }
    }

    fn script(gh: &FakeGh, pull: &serde_json::Value) {
        gh.respond("repos/torrust/torrust-tracker/pulls/7", pull.clone());
        gh.respond("repos/torrust/torrust-tracker/pulls/7/reviews?per_page=100", json!([]));
        gh.respond("repos/torrust/torrust-tracker/issues/7/comments?per_page=100", json!([]));
        gh.respond("repos/torrust/torrust-tracker/pulls/7/comments?per_page=100", json!([]));
    }

    #[test]
    fn it_should_report_a_merged_pull_request_as_merged_even_though_github_calls_it_closed() {
        let gh = FakeGh::new();
        script(&gh, &json!({"head": {"sha": "abc"}, "state": "closed", "merged": true}));

        let snapshot = take_snapshot(&gh, &pull_request()).unwrap();

        assert_eq!(snapshot.state, PullState::Merged);
        assert_eq!(snapshot.head_sha, "abc");
    }

    #[test]
    fn it_should_report_a_closed_unmerged_pull_request_as_closed() {
        let gh = FakeGh::new();
        script(&gh, &json!({"head": {"sha": "abc"}, "state": "closed", "merged": false}));

        assert_eq!(take_snapshot(&gh, &pull_request()).unwrap().state, PullState::Closed);
    }

    #[test]
    fn it_should_decode_reviews_and_comments_including_ones_from_deleted_accounts() {
        let gh = FakeGh::new();
        gh.respond(
            "repos/torrust/torrust-tracker/pulls/7",
            json!({"head": {"sha": "abc"}, "state": "open", "merged": false}),
        );
        gh.respond(
            "repos/torrust/torrust-tracker/pulls/7/reviews?per_page=100",
            json!([{"id": 1, "user": null, "state": "COMMENTED", "commit_id": "abc", "html_url": "u", "submitted_at": "2026-10-28T07:00:00Z"}]),
        );
        gh.respond("repos/torrust/torrust-tracker/issues/7/comments?per_page=100", json!([]));
        gh.respond("repos/torrust/torrust-tracker/pulls/7/comments?per_page=100", json!([]));

        let snapshot = take_snapshot(&gh, &pull_request()).unwrap();

        assert_eq!(snapshot.reviews.len(), 1);
        assert_eq!(snapshot.reviews[0].user, None);
    }

    #[test]
    fn it_should_reject_a_pull_request_response_without_a_head_commit() {
        let gh = FakeGh::new();
        script(&gh, &json!({"state": "open", "merged": false}));

        assert!(matches!(
            take_snapshot(&gh, &pull_request()),
            Err(Error::MalformedResponse { .. })
        ));
    }

    #[test]
    fn it_should_read_the_committer_date_of_a_commit() {
        let gh = FakeGh::new();
        gh.respond(
            "repos/torrust/torrust-tracker/commits/abc",
            json!({"commit": {"committer": {"date": "1970-01-01T00:01:00Z"}}}),
        );

        assert_eq!(commit_date(&gh, &pull_request(), "abc").unwrap(), 60);
    }

    #[test]
    fn it_should_reject_a_commit_without_a_parsable_committer_date() {
        let gh = FakeGh::new();
        gh.respond(
            "repos/torrust/torrust-tracker/commits/abc",
            json!({"commit": {"committer": {"date": "soon"}}}),
        );

        assert!(matches!(
            commit_date(&gh, &pull_request(), "abc"),
            Err(Error::CommitDate { .. })
        ));
    }
}
