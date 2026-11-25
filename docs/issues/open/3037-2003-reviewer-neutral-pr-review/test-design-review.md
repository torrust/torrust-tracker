# Test Design Review - Issue #3037

Review of the tests added or changed in T2 (the poller's default reviewer set), done with the
`write-unit-test` skill after the tests passed and before the commit. Toolchain: stable Rust
(`cargo test -p wait-for-pr-review`: 73 + 3 + 38 tests pass) and nightly Rust for
`cargo +nightly fmt`.

## Prose-First Arrange-Act-Assert Comparison

| Test | Arrange (the one causal difference) | Act | Assert |
| --- | --- | --- | --- |
| `event::it_should_report_a_review_from_an_account_in_no_list_when_no_reviewer_is_listed` | A review from `new-reviewer[bot]`, a login no list names | `detect` with `Reviewers::AllExceptAuthor` | A `review` event for that login |
| `event::it_should_ignore_the_authors_own_reviews_and_comments_when_no_reviewer_is_listed` | A review, a conversation comment and an inline comment by the author (one in uppercase) | `detect` with `Reviewers::AllExceptAuthor` | No event |
| `event::it_should_count_every_account_when_github_shows_no_author_and_no_reviewer_is_listed` | The snapshot has no author; a comment by the login that is the author elsewhere | `detect` with `Reviewers::AllExceptAuthor` | A `comment` event |
| `event::it_should_watch_every_account_except_the_author_when_built_from_no_logins` | No logins, then one login | `Reviewers::from_logins` | `AllExceptAuthor`, then `Only` that login |
| `event::it_should_report_a_new_commented_review_from_a_bot_that_never_approves` (renamed from the Copilot name) | A `COMMENTED` review from a listed bot | `detect` with the listed reviewers | A `review` event in state `COMMENTED` |
| `snapshot::it_should_read_the_login_of_the_pull_requests_author` | The pull response has `user.login` | `take_snapshot` | `author` is that login |
| `snapshot::it_should_have_no_author_when_github_shows_no_account` | The pull response has `user: null` | `take_snapshot` | `author` is `None` |

## Findings

- Each test changes one thing against the shared `snapshot()` fixture; the fixture now names the
  author (`AUTHOR`) and a neutral review bot (`REVIEW_BOT`) as constants, so the causal login is
  visible in the test body.
- The listed-mode tests keep `Reviewers::Only` through the `reviewers()` helper; the default-mode
  tests pass `Reviewers::AllExceptAuthor` in the Act line, so the mode under test is visible.
- No temporary prose remained after the comparison; no helper was added.
- Existing JSON fixtures without `user` still decode (`#[serde(default)]`), so the wait-loop tests
  did not change apart from the `Reviewers::Only` configuration.

## CLI Tests Added After Review (PR 3044, da2ce7 F1)

The `detect` tests above pass `Reviewers::AllExceptAuthor` directly, and the CLI fixtures had a pull
request without `user`, so no test pinned the command-line default or the `watch` field. Two tests in
`tests/cli.rs` close that gap. Toolchain: stable Rust (`cargo test -p wait-for-pr-review`: 73 + 3 + 40
tests pass).

| Test | Arrange (the one causal difference) | Act | Assert |
| --- | --- | --- | --- |
| `it_should_report_a_reviewer_no_list_names_and_skip_the_authors_own_review_when_no_reviewer_is_given` | The pull request's author is `contributor`; the author's reply review comes first in time, then a review by `new-reviewer[bot]` | The binary without `--reviewer` | A `review` event by `new-reviewer[bot]`; `wait_started` has `watch: all_except_author` and no listed reviewers |
| `it_should_watch_only_the_listed_logins_when_a_reviewer_is_given` | The same reviews | The binary with `--reviewer da2ce7` | `timeout`; `wait_started` has `watch: listed` and `["da2ce7"]` |

Mutation check, run and reverted: putting the old default (`da2ce7`,
`copilot-pull-request-reviewer[bot]`) back on `--reviewer` fails the first test (it times out after
30 s); swapping the two `watch` strings fails both. The shared fake gets one helper,
`with_later_reviews`, that only replaces the reviews the fake returns after the first poll.
