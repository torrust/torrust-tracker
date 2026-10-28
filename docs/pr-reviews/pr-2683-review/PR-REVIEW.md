---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2683 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2683>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Round 1: the review of `copilot-pull-request-reviewer` (5682635980, COMMENTED, submitted 2026-10-28T12:34:22Z) on the head whose subject is `docs(issues): [#2581] elide a pull-request head commit id from the evidence`. The `wait-for-pr-review` poller this pull request adds detected it (result `review`, exit `0`). The review opens four inline threads, F1 to F4. Its body only lists the same four with the labels "Medium" for F1 to F3 and "Low" for F4, and its "What changed in this PR" summary raises no assertion of its own, so the body adds no row. The comments carry no `[Severity]` bracket, so the severities are inferred from those labels: "Medium" is recorded as Minor (inferred), as in earlier audits, and "Low" as Nit (inferred).

Round 2: the second review of `copilot-pull-request-reviewer` (5683463670, COMMENTED, submitted 2026-10-28T13:34:16Z) on the head whose subject is `docs(pr-reviews): [#2581] record the first Copilot review of PR 2683`. It was the review requested through the REST `requested_reviewers` endpoint at 12:54:26 UTC. It opens two inline threads, F5 and F6, in the advisory format `[<Severity>][<FindingId>]`: its comments carry their own severity and finding ID, and neither ID collides with F1 to F4, so both are kept. Its body lists F5 and F6 as open and F1 to F4 as resolved since the last review, and adds no other row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2683-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2683-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2683-f3 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2683-f4 | Copilot | Nit (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2683-f5 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2683-f6 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Track submitted review IDs instead of using a maximum ID

- PR number: 2683
- Source review ID: 5682635980
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400587135>
- Concern: A draft review keeps its ID when it is submitted, so a review submitted after the baseline with an ID at or below the baseline maximum is ignored for the whole wait, and a `PENDING` review present at the start is missed when it is submitted. Track the set of submitted review IDs and test both cases.
- Solution: The baseline in `event.rs` keeps the set of submitted review IDs, with `PENDING` reviews excluded, and a review counts as new when its ID is not in that set. Comments keep the maximum-ID rule because they have no draft state. Two unit tests cover a lower-ID submission and a pending-to-submitted transition.
- Current-tree verification: `cargo test -p wait-for-pr-review` passed with the two new tests `it_should_report_a_review_submitted_later_with_a_lower_id_than_one_already_submitted` and `it_should_report_a_review_that_was_pending_at_the_start_when_it_is_submitted`; `cargo clippy -p wait-for-pr-review --all-targets` reported nothing.
- Resolution reference: fix(wait-for-pr-review): [#2581] track submitted review IDs instead of a maximum
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400733804>

### F2 - Canonicalize abbreviated and uppercase SHAs before baseline comparison

- PR number: 2683
- Source review ID: 5682635980
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400587198>
- Concern: The CLI accepts abbreviated and uppercase SHAs, but the baseline stored the input verbatim and compared it with GitHub's full lowercase head SHA, so on a quiet pull request a valid abbreviation of the unchanged head returned `head_changed` at once. Return the canonical full SHA from the commit lookup and build the baseline from it.
- Solution: `snapshot.rs` returns a `CommitInfo` with the full lowercase SHA and the committer date, and `wait.rs` builds the baseline from it.
- Current-tree verification: `cargo test -p wait-for-pr-review` passed with the new tests `it_should_return_the_full_lowercase_sha_for_an_abbreviated_uppercase_one` and `it_should_not_report_a_head_change_when_the_commit_is_given_as_an_abbreviated_uppercase_sha`. A real run of the binary with the nine-character uppercase abbreviation of the unchanged head of PR #2622 as `--since-head-sha` returned a `timeout` result instead of `head_changed`.
- Resolution reference: fix(wait-for-pr-review): [#2581] compare the baseline with the canonical full commit SHA
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400734182>

### F3 - Enforce timeout deadlines and terminate stalled child processes

- PR number: 2683
- Source review ID: 5682635980
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400587241>
- Concern: A stalled `gh auth status` or API call could exceed `--timeout-seconds` indefinitely because the child-wait loop checked signals but had no deadline, and the poll loop started another snapshot after its last sleep spent the budget. Share an absolute deadline with the subprocess boundary, kill and reap the child on expiry, return the timeout result without starting further requests, and cover a hung call without sending a signal.
- Solution: `GhCli` holds an absolute deadline, refuses to start a child after it, and kills and reaps a running child when it passes. The poll loop starts no snapshot once the budget is spent. A call killed at the deadline after the baseline was taken ends the wait with the `timeout` result; before the baseline was taken nothing was observed, so it is a `deadline_exceeded` failure (exit `1`) rather than a `timeout` that would suggest a quiet pull request. That distinction goes beyond the review wording and is recorded in the spec and the skill.
- Current-tree verification: `cargo test -p wait-for-pr-review` passed with the integration tests `it_should_kill_a_hung_gh_call_at_the_deadline_without_a_signal_and_exit_one_before_the_baseline` and `it_should_report_a_timeout_when_a_gh_call_hangs_past_the_deadline_after_the_baseline`, which use a stalled child and send no signal, and with the unit tests `it_should_not_take_a_snapshot_once_the_wait_budget_is_spent`, `it_should_report_a_timeout_when_a_call_is_killed_at_the_deadline_after_the_baseline` and `it_should_stop_with_an_error_when_the_deadline_hits_before_the_baseline_is_taken`. A real run against this pull request with `--timeout-seconds 20` returned its `timeout` result after 20.002 s by `time`.
- Resolution reference: fix(wait-for-pr-review): [#2581] enforce the timeout as a hard deadline on every gh call
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400734750>

### F4 - Preserve an event cursor to avoid reprocessing unchanged feedback

- PR number: 2683
- Source review ID: 5682635980
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400587297>
- Concern: When a clean review or an informational comment needs no new commit while another reviewer is still outstanding, returning to step 1 of the skill's loop reuses the same `--since-head-sha` and returns the already-processed event at once, so the loop can process and re-request that review repeatedly. Define a resumable event cursor that excludes processed activity without losing feedback between invocations, and re-request a review only when new fixes require it.
- Solution: A `review` or `comment` result carries `cursor`, the GitHub time of the reported item, and the new `--since <utc time>` flag counts only items created strictly after it. The skill's loop uses `--since <cursor>` after an event that needed no push, re-requests a review only after a push, and reuses the same flag after a `timeout`. An item created in the same second as the cursor is skipped by the poller; the loop reads the whole pull request after every event, so it is still read. The skill says so.
- Current-tree verification: `cargo test -p wait-for-pr-review` passed with the new tests `it_should_skip_an_item_at_the_cursor_and_report_the_first_one_after_it`, `it_should_wait_for_a_head_change_against_the_head_at_the_start_when_resuming_from_a_cursor`, `it_should_carry_the_creation_time_of_a_reported_comment_as_the_cursor` and `it_should_not_report_a_review_again_when_resumed_from_its_cursor`. A real run against this pull request returned Copilot's review with cursor `2026-10-28T12:34:22Z`, and a run with `--since 2026-10-28T12:34:22Z` returned a `timeout`, so the review was not reported again.
- Resolution reference: fix(wait-for-pr-review): [#2581] add a resumable cursor so a handled event is not reported again
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4400735252>

### F5 - Synchronize the head for feedback results too

- PR number: 2683
- Source review ID: 5683463670
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4401206737>
- Concern: When a push and feedback arrive together, `detect()` prioritizes the review or comment over `head_changed`, so the skill's loop can process feedback on a stale checkout, and resuming with `--since` adopts the new head as its baseline, which hides the head change from later polls. Check the head and synchronize safely before processing feedback, not only in the `head_changed` branch.
- Solution: `--since-head-sha` and `--since` can be combined, so the agent's checkout is the head to watch and a pull request that is ahead of it is reported as `head_changed`. A `review` or `comment` result that arrives while the head differs from the watched one carries `head_moved: true`. The skill's loop requires `git fetch` and a safe fast-forward (or a rebase of unpushed commits) before processing feedback when `head_moved` is true or `head_sha` differs from the local `HEAD`, stops and tells the contributor when that is not safe, and passes both flags when resuming. The priority of reviews over head changes is unchanged, because reporting `head_changed` first would hide the review that triggered the wait.
- Current-tree verification: `cargo test -p wait-for-pr-review` passed with the new unit tests `it_should_report_a_head_change_when_resuming_from_a_cursor_with_a_checkout_that_is_not_the_head`, `it_should_flag_a_review_that_arrives_with_a_head_that_differs_from_the_checkout`, `it_should_flag_a_review_reported_while_the_head_differs_from_the_one_the_wait_watched` and `it_should_flag_a_comment_reported_while_the_head_differs_from_the_one_the_wait_watched`, and with the integration test `it_should_report_a_head_change_at_once_when_the_checked_out_commit_is_not_the_head_even_with_a_cursor`, which failed with exit `2` before the change. Against this pull request, `--since-head-sha` set to its first commit with `--since 2026-10-28T13:34:16Z` returned `head_changed` at once, the same checkout without `--since` returned Copilot's earlier review with `head_moved: true`, and a checkout equal to the head returned no `head_moved`.
- Resolution reference: fix(wait-for-pr-review): [#2581] synchronize the head before processing feedback
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4401634456>

### F6 - Reject invalid dates and signed clock fields in `--since`

- PR number: 2683
- Source review ID: 5683463670
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4401206798>
- Concern: `--since 2026-02-29T07:00:00Z` was accepted and converted to March 1, and `2026-10-28T-1:00:00Z` became the previous day's 23:00, so malformed input silently changed the review cutoff instead of producing `usage_error`. Validate digit-only components and the month's actual day count, with regression cases for both inputs.
- Solution: `parse_utc` takes digits only, checks the day against the month's real length with leap years, and rejects out-of-range clock fields, including a second of `60`. A malformed `--since` is a `usage_error` (exit `2`).
- Current-tree verification: `cargo test -p wait-for-pr-review` passed with the new tests `it_should_reject_a_day_the_month_does_not_have`, `it_should_reject_signed_or_otherwise_non_digit_fields`, `it_should_reject_clock_fields_that_are_out_of_range`, `it_should_accept_the_last_day_of_every_kind_of_month_and_both_kinds_of_february` and the integration test `it_should_exit_two_when_since_names_a_day_or_a_signed_field_that_does_not_exist`. The three rejection tests failed before the change, naming `2026-02-29T07:00:00Z`, `2026-10-28T-1:00:00Z` and `2026-10-28T07:00:60Z`. Against the real binary, `--since` with `2026-02-29T07:00:00Z`, `2026-10-28T-1:00:00Z`, `2026-04-31T00:00:00Z`, `2026-10-28T24:00:00Z` and `2026-10-28T07:00:+1Z` all exited `2` with empty stdout and a `usage_error` record; before the change, four of those five were accepted. `2028-02-29T07:00:00Z` is still accepted.
- Resolution reference: fix(wait-for-pr-review): [#2581] reject invalid dates and signed fields in --since
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4401635307>

## Processing Log

- 2026-10-28 12:35 UTC - The `wait-for-pr-review` poller returned Copilot's review 5682635980, submitted at 12:34:22 UTC (kind `review`, state `COMMENTED`, exit `0`; its stdout file was last written at 12:35:33 UTC). The review opened four inline threads, F1 to F4; all were unresolved. Each was checked against the current tree and found valid.
- 2026-10-28 12:45 UTC - The four fixes were committed, one signed commit each: F1 at 12:38, F2 at 12:41, F3 at 12:42 and F4 at 12:45 UTC (commit times from `git log --format=%cI`). The spec was refined for the changed contract at 12:47 UTC.
- 2026-10-28 12:50 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the four replies are 12:49:59, 12:50:02, 12:50:06 and 12:50:09 UTC.
- 2026-10-28 12:50 UTC - A GraphQL fetch at 12:50:24 UTC, after the last reply, reported no unresolved thread, and `github-review-threads reply-status` exited `0` for the login `josecelano`.

- 2026-10-28 13:34 UTC - Copilot's second review 5683463670 was submitted (COMMENTED, on the head with subject `docs(pr-reviews): [#2581] record the first Copilot review of PR 2683`). The `wait-for-pr-review` poller had stopped at 13:24:55 UTC with a `timeout`, so the review was read from GitHub at 14:01 UTC. It opened F5 and F6, both unresolved, and listed F1 to F4 as resolved. Each was checked against the current tree and found valid; F6 was reproduced against the real binary.
- 2026-10-28 14:10 UTC - The two fixes were committed, one signed commit each: F6 at 14:04 and F5 at 14:10 UTC (commit times from `git log --format=%cI`).
- 2026-10-28 14:13 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the two replies are 14:13:09 and 14:13:13 UTC. Copilot was requested again through the REST `requested_reviewers` endpoint at 14:13:25 UTC (timeline event `review_requested`). A GraphQL fetch at 14:13:31 UTC reported no unresolved thread.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
