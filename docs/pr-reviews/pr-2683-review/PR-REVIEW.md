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

Round 3: the third review of `copilot-pull-request-reviewer` (5684761654, COMMENTED, submitted 2026-10-28T14:58:12Z) on the head whose subject is `fix(wait-for-pr-review): [#2581] synchronize the head before processing feedback`. It was the review requested through the REST `requested_reviewers` endpoint at 14:13:25 UTC, and the `wait-for-pr-review` poller returned it. Its body says that Copilot could not run its full agentic suite, which raises no assertion of its own. It opens six inline threads and lists F5 and F6 as resolved since the last review. Its comments carry no severity or finding ID, so the audit assigns F7 to F12 in source order and infers the severities. The three comments on `reserve_free_ports` in `tests/common/native_tracker/ports.rs` (lines 22, 38 and 48) are the same concern posted three times: F10 is the original, and F11 and F12 are recorded as re-raises of it.

Round 4: the fourth review of `copilot-pull-request-reviewer` (5686403906, COMMENTED, submitted 2026-10-28T16:48:28Z) on the head whose subject is `docs(issues): [#2581] record the recheck after the third review and the progress`. It was the review requested through the REST `requested_reviewers` endpoint at 16:30:49 UTC, after everything else had been pushed, and the `wait-for-pr-review` poller returned it. Its body warns that Copilot could not run its full agentic review because it did not start before the timeout, which raises no assertion of its own. It opens four inline threads and lists F7 to F12 as resolved since the last review. Its comments carry no severity or finding ID, so the audit assigns F13 to F16 in source order and infers the severities.

Round 5: the fifth review of `copilot-pull-request-reviewer` (5687101185, COMMENTED, submitted 2026-10-28T17:40:38Z) on the head whose subject is `docs(issues): [#2581] update the review round counts in the spec progress log and the evidence`. It was the review requested through the REST `requested_reviewers` endpoint at 17:21:42 UTC, after everything else had been pushed, and the `wait-for-pr-review` poller returned it. Its body warns that Copilot could not run its full agentic review because it didn't start before the timeout, which raises no assertion of its own. It opens two inline threads and lists F13 to F16 as resolved since the last review. Its comments carry no severity or finding ID, so the audit assigns F17 and F18 in source order and infers the severities.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2683-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2683-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2683-f3 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2683-f4 | Copilot | Nit (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2683-f5 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2683-f6 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2683-f7 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2683-f8 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2683-f9 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2683-f10 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2683-f11 | Copilot | Nit (inferred) | maintainability | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-2683-f12 | Copilot | Nit (inferred) | maintainability | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2683-f13 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2683-f14 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2683-f15 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2683-f16 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2683-f17 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2683-f18 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

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

### F7 - The `Review::id` documentation contradicts the baseline design

- PR number: 2683
- Source review ID: 5684761654
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4402180076>
- Concern: The docstring on `Review::id` states that higher IDs imply later reviews, but the design, since the fix for F1, assumes that a review can have a lower ID than one already submitted because drafts keep their IDs. Update the comment so it does not contradict the baseline and event logic.
- Solution: The `Review::id` documentation in `snapshot.rs` now says the ID is unique and assigned when the review is created, that a draft keeps it, and that a higher ID is therefore not necessarily a later submission. The `Comment::id` line says IDs only grow because comments have no draft state, which matches the baseline's rule for comments.
- Current-tree verification: Both doc comments were read in `snapshot.rs`, and `cargo clippy -p wait-for-pr-review --all-targets` reported nothing.
- Resolution reference: docs(wait-for-pr-review): [#2581] stop claiming that a higher review ID is a later review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403215806>

### F8 - The CLI integration test is Unix-specific and is not gated

- PR number: 2683
- Source review ID: 5684761654
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4402180244>
- Concern: `tests/cli.rs` uses `std::os::unix::fs::PermissionsExt` and a `#!/bin/sh` fake `gh`, so it does not compile on non-Unix targets. Gate it with `#[cfg(unix)]` or provide a Windows-compatible alternative so the crate stays buildable across targets.
- Solution: The file is gated with `#![cfg(unix)]` and a comment that says why, as the tracker's native-process scenarios in `tests/configuration/cli_configuration.rs` already do. The library and binary are not gated.
- Current-tree verification: With a real Windows target (`x86_64-pc-windows-gnu`, added for this check), `cargo check -p wait-for-pr-review --tests --target x86_64-pc-windows-gnu` failed before the change with `cannot find unix in os` and `could not compile wait-for-pr-review (test "cli")`, and finished without error after it. On Linux, `cargo test -p wait-for-pr-review` still ran 52 unit tests and 19 integration tests.
- Resolution reference: fix(wait-for-pr-review): [#2581] gate the Unix-only CLI scenarios with cfg(unix)
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403216504>

### F9 - The port range comment overstates what the range guarantees

- PR number: 2683
- Source review ID: 5684761654
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4402180347>
- Concern: The comment claims that ending at `32768` prevents kernel-assigned ephemeral ports from landing in the scenario range, but Linux ephemeral port ranges are configurable and can start below `32768`. Soften the guarantee, or derive the start of the ephemeral range dynamically and choose a range below it.
- Solution: The softening option was taken. The module and constant documentation in `tests/common/native_tracker/ports.rs` say the range is below the default Linux ephemeral range, that a host which lowers `net.ipv4.ip_local_port_range` below it narrows the guarantee, and the default is named once as `DEFAULT_EPHEMERAL_START`, used by the range and by its guard test. Reading `/proc/sys/net/ipv4/ip_local_port_range` would make a test-only helper platform-specific, so it was not added.
- Current-tree verification: `cargo test -p torrust-tracker --test cli-configuration` passed with the guard test renamed to `it_should_reserve_only_ports_below_the_default_ephemeral_range`, and clippy reported nothing.
- Resolution reference: docs(tests): [#2581] state that the scenario port range assumes the default ephemeral range
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403217116>

### F10 - `reserve_free_ports` panics with a misleading message

- PR number: 2683
- Source review ID: 5684761654
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4402180447>
- Concern: `reserve_free_ports` can panic in a misleading way when `N` is larger than `MAX_DRAWS`, or when the candidate stream runs out for another reason: the panic blames the loopback interface or the range, while the real cause may be an insufficient search budget. Add an explicit precondition and/or a failure message that names the requested count and `MAX_DRAWS`.
- Solution: `reserve_free_ports` rejects `N` above `MAX_DRAWS` up front with `cannot reserve {N} ports with a budget of {MAX_DRAWS} draws`, and when the shared budget runs dry it reports how many free ports it found, the range, the budget and the requested count. The `# Panics` section says both.
- Current-tree verification: The new test `it_should_refuse_to_reserve_more_ports_than_it_can_search_for` failed before the change with the panic message `the loopback interface should have a free port below the ephemeral range`, which is the misleading message the review describes, and passes now. `cargo test -p torrust-tracker --test cli-configuration` passed (30 tests) and so did `--test lifecycle-signals` (25 tests), which shares the fixture. With ports 43152 to 43156 held by another process, the suite then passed 40 more runs.
- Resolution reference: fix(tests): [#2581] name the real cause when a port reservation cannot be satisfied
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403217683>

### F11 - The same `reserve_free_ports` panic message concern, on another line

- PR number: 2683
- Source review ID: 5684761654
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4402180542>
- Concern: The same comment as F10, posted on a second line of `reserve_free_ports`: the panic message is misleading when `N` exceeds `MAX_DRAWS` or the candidates run out.
- Solution: No separate change. The concern is identical to F10 and is resolved by its fix.
- Current-tree verification: The panic messages of `reserve_free_ports` were read in `tests/common/native_tracker/ports.rs`, and the F10 test passes.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403218690>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403218690>

### F12 - The same `reserve_free_ports` panic message concern, on a third line

- PR number: 2683
- Source review ID: 5684761654
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4402180596>
- Concern: The same comment as F10, posted on a third line of `reserve_free_ports`.
- Solution: No separate change. The concern is identical to F10 and is resolved by its fix.
- Current-tree verification: The panic messages of `reserve_free_ports` were read in `tests/common/native_tracker/ports.rs`, and the F10 test passes.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403219283>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403219283>

### F13 - A resumable cursor can skip earlier unprocessed items

- PR number: 2683
- Source review ID: 5686403906
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403476575>
- Concern: When `--since` is used as a resumable cursor, returning an arbitrary matching review or comment, chosen by iterator order, can cause earlier unprocessed items to be skipped on the next run if the returned item's `cursor` is later than other pending items. Select the earliest new item by `submitted_at` or `created_at`, or otherwise ensure a stable total order, before returning an event.
- Solution: `detect` gathers every new review, conversation comment and inline comment of a watched reviewer and reports the earliest by GitHub time, with a review before a conversation comment before an inline comment when made in the same second, then by ID. This also fixes a case the finding did not name: the old fixed priority let a review outrank an earlier comment. The skill and the spec describe the ordering. An item created in the same second as the cursor is still skipped; the review loop reads the whole pull request after every event.
- Current-tree verification: `cargo test -p wait-for-pr-review` passed with the new tests `it_should_report_the_earliest_new_item_when_a_comment_came_before_a_review`, `it_should_report_the_earliest_new_item_among_comments_of_both_kinds` and `it_should_report_the_earliest_of_two_new_reviews_whatever_order_the_api_listed_them_in`, which failed before the change, and with `it_should_report_a_review_before_a_comment_made_in_the_same_second`, which pins the tie-break. Against this pull request, watching the logins `Copilot` and `copilot-pull-request-reviewer[bot]` and resuming from each cursor in turn returned inline comment 16:48:26, inline comment 16:48:27, review 16:48:28, then `timeout`; the inline comment `4403476731` at 16:48:27 was skipped, as the same-second rule says.
- Resolution reference: fix(wait-for-pr-review): [#2581] report the earliest new review or comment so the cursor skips nothing
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403771237>

### F14 - The pipe readers of a killed gh call are not reclaimed

- PR number: 2683
- Source review ID: 5686403906
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403476660>
- Concern: On the `Err` path from `wait_for`, such as `Interrupted` or `DeadlineExceeded`, `run()` returns early and never joins the drain threads created for stdout and stderr, which risks leaking detached threads and makes failure-path behavior less predictable. Restructure `run()` to always join the handles before returning, including on error.
- Solution: Not adopted literally. A trial that joined both readers on every path made the two existing hung-call tests fail after 30.01 s, because a grandchild that inherited the pipe keeps it open after the child is killed, so an unconditional join waits on it and defeats the hard deadline of F3. The file was restored. Instead, the readers send what they read over a channel, and after a kill `run()` gives them a bounded grace period of 200 ms to finish and be collected, then lets go. In the normal case, where a killed child closes its pipes at once, they are reclaimed on every path.
- Current-tree verification: The unit tests `it_should_return_what_a_reader_sent_when_there_is_no_grace_period`, `it_should_return_what_a_reader_sent_within_the_grace_period`, `it_should_give_up_on_a_reader_that_stays_silent_once_the_grace_period_is_over` and `it_should_return_nothing_for_a_pipe_that_does_not_exist` pass; they did not compile before `collect` existed. The two hung-call integration tests pass in 2.41 s. Against the real binary, `SIGTERM` and `SIGINT` during a wait stopped it after 3.0 s with `interrupted` and exit `1`, and a 2 s budget ended after 2.0 s with `deadline_exceeded`.
- Resolution reference: fix(wait-for-pr-review): [#2581] reclaim the pipe readers of a killed gh call within a bound
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403771969>

### F15 - `reserve_free_ports` promises N distinct ports but samples with replacement

- PR number: 2683
- Source review ID: 5686403906
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403476731>
- Concern: The function promises `N` distinct ports and allows `N` up to the draw budget, but the candidate stream is sampled with replacement, so for large `N` it can contain duplicates and the function can panic even when enough free ports exist. Generate candidates without replacement, or skip duplicates while allowing more than `N` draws, and adjust the precondition and message.
- Solution: Each port gets its own draw budget (`DRAWS_PER_PORT`), so duplicate candidates cannot exhaust a budget shared by all ports, and a candidate that repeats a port already held fails to bind and is skipped. `N` is bounded by the size of the range, with the message `cannot reserve {N} ports from a range of {available} ports`. The bind step is injected (`reserve_from`) so the budget logic is tested with a fake binder and no sockets.
- Current-tree verification: `it_should_give_each_port_its_own_draw_budget_even_when_the_candidates_repeat` failed in 5 of 5 runs when the per-port budget was mutated into one that shrinks as ports are held (not staged, then restored byte for byte), and passes now; `it_should_refuse_to_reserve_more_ports_than_the_range_holds` covers the precondition. A first version of the budget test used real sockets and failed in 11 of 60 full parallel runs, because a just-released port can stay held by a tracker child that another test thread forked while the listener was open; it was replaced before the commit. After the change the full parallel suites of `cli-configuration` and `lifecycle-signals` ran 100 times each with 0 failures, and with ports 43152 to 43156 held the suite passed 40 more runs. The evidence is in section V8 of the evidence file.
- Resolution reference: fix(tests): [#2581] give each reserved port its own draw budget and bound N by the range
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403772491>

### F16 - A `usage_error` example in the evidence is truncated

- PR number: 2683
- Source review ID: 5686403906
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403476783>
- Concern: The `usage_error` example line in the manual verification evidence is cut short, missing its closing quote and brace, so it cannot be copied or validated as one complete NDJSON record. Paste the full record exactly as emitted.
- Solution: The cut came from the capture, which sliced the record at 120 characters. The complete record was captured again with the same command and pasted verbatim, with a note saying when it was re-captured. The rest of the evidence file was scanned for other records cut mid-line, and none was found.
- Current-tree verification: `target/debug/wait-for-pr-review` with no arguments exited `2` and wrote one stderr line of 215 bytes, which `python3 -c "import json; json.loads(...)"` parsed as one JSON object with `kind` `usage_error`; the evidence file now holds that line in full.
- Resolution reference: docs(issues): [#2581] record the complete usage_error record and the fixture changes
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4403773128>

### F17 - `write_diagnostic` ignores I/O and serialization errors

- PR number: 2683
- Source review ID: 5687101185
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4404026465>
- Concern: `write_diagnostic` drops I/O and serialization errors, which can silently violate the NDJSON-on-stderr contract, for example partial JSON on a short write. Handle the results of the writes, at least to return a non-zero exit code when valid diagnostics cannot be produced.
- Solution: `write_diagnostic` builds the whole record in memory and writes it with one call, so a failure cannot leave half a record behind, and it returns the error. `emit_diagnostic` exits non-zero when a record cannot be written: every failure code already is non-zero, and the help text, whose whole purpose is to be read, no longer reports success when it was never written. The `wait_started` progress record stays best effort, because the caller relies on the result on stdout or on the failure record and its exit code; the code says so.
- Current-tree verification: With `/dev/full` as stderr, which fails every write, the new test `it_should_not_report_success_for_help_when_the_help_text_cannot_be_written` failed before the change (exit `0` where `1` was expected) and passes now; `it_should_keep_the_usage_error_exit_code_when_the_diagnostic_cannot_be_written` and `it_should_still_print_the_result_when_only_the_progress_record_cannot_be_written` pin the other two cases. Against the real binary, `--help` into `/dev/full` exited `1` with empty stdout, no arguments exited `2`, a merged pull request still printed its `pr_merged` result with exit `0`, and `--help` with a normal stderr exited `0` with one valid JSON record. The three tests are gated to Linux because `/dev/full` is.
- Resolution reference: fix(wait-for-pr-review): [#2581] never report success for help that could not be written
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4404131549>

### F18 - The digit-only check of `digits` is hard to read

- PR number: 2683
- Source review ID: 5687101185
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4404026549>
- Concern: `digits` uses a nested `Option` with `then(|| field.parse().ok())?`, which is correct but hard to read. Rewrite it so the digit-only validation and the parse failure path are explicit.
- Solution: `digits` returns early when a field has a non-digit and parses otherwise. Behavior is unchanged.
- Current-tree verification: The existing parser tests passed after the rewrite, including `it_should_reject_signed_or_otherwise_non_digit_fields`, the day, clock-range and leap-year cases (60 unit tests in total), and `cargo clippy -p wait-for-pr-review --all-targets` reported nothing.
- Resolution reference: refactor(wait-for-pr-review): [#2581] make the digit-only check of the timestamp parser explicit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2683#discussion_r4404132355>

## Processing Log

- 2026-10-28 12:35 UTC - The `wait-for-pr-review` poller returned Copilot's review 5682635980, submitted at 12:34:22 UTC (kind `review`, state `COMMENTED`, exit `0`; its stdout file was last written at 12:35:33 UTC). The review opened four inline threads, F1 to F4; all were unresolved. Each was checked against the current tree and found valid.
- 2026-10-28 12:45 UTC - The four fixes were committed, one signed commit each: F1 at 12:38, F2 at 12:41, F3 at 12:42 and F4 at 12:45 UTC (commit times from `git log --format=%cI`). The spec was refined for the changed contract at 12:47 UTC.
- 2026-10-28 12:50 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the four replies are 12:49:59, 12:50:02, 12:50:06 and 12:50:09 UTC.
- 2026-10-28 12:50 UTC - A GraphQL fetch at 12:50:24 UTC, after the last reply, reported no unresolved thread, and `github-review-threads reply-status` exited `0` for the login `josecelano`.

- 2026-10-28 13:34 UTC - Copilot's second review 5683463670 was submitted (COMMENTED, on the head with subject `docs(pr-reviews): [#2581] record the first Copilot review of PR 2683`). The `wait-for-pr-review` poller had stopped at 13:24:55 UTC with a `timeout`, so the review was read from GitHub at 14:01 UTC. It opened F5 and F6, both unresolved, and listed F1 to F4 as resolved. Each was checked against the current tree and found valid; F6 was reproduced against the real binary.
- 2026-10-28 14:10 UTC - The two fixes were committed, one signed commit each: F6 at 14:04 and F5 at 14:10 UTC (commit times from `git log --format=%cI`).
- 2026-10-28 14:13 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the two replies are 14:13:09 and 14:13:13 UTC. Copilot was requested again through the REST `requested_reviewers` endpoint at 14:13:25 UTC (timeline event `review_requested`). A GraphQL fetch at 14:13:31 UTC reported no unresolved thread.

- 2026-10-28 14:59 UTC - The `wait-for-pr-review` poller returned Copilot's third review 5684761654, submitted at 14:58:12 UTC (kind `review`, state `COMMENTED`, no `head_moved`; its stdout file was last written at 14:59:15 UTC). It opened six threads, F7 to F12, all unresolved. Each was checked against the current tree and found valid; F11 and F12 repeat F10.
- 2026-10-28 16:21 UTC - The four fixes were committed, one signed commit each: F7 at 16:15, F8 at 16:17, F9 at 16:18 and F10 at 16:21 UTC (commit times from `git log --format=%cI`). The first signing attempt for F7 failed; it was rerun with the maintainer's approval and the maintainer entered the passphrase, which accounts for the gap after 14:59.
- 2026-10-28 16:24 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the six replies are 16:24:29, 16:24:32, 16:24:36, 16:24:39, 16:24:44 and 16:24:47 UTC. F11 and F12 received the prescribed `Superseded by F10: <reason>.` reply.
- 2026-10-28 16:25 UTC - A GraphQL fetch at 16:25:04 UTC, after the last reply, reported no unresolved thread.

- 2026-10-28 16:50 UTC - The `wait-for-pr-review` poller returned Copilot's fourth review 5686403906, submitted at 16:48:28 UTC (kind `review`, state `COMMENTED`, no `head_moved`; its stdout file was last written at 16:50:10 UTC). It opened four threads, F13 to F16, all unresolved. Each was checked against the current tree and found valid.
- 2026-10-28 17:13 UTC - The four fixes were committed, one signed commit each: F13 at 16:55, F14 at 16:57, F15 at 17:11 and F16 at 17:13 UTC (commit times from `git log --format=%cI`). While preparing F15, a test that re-bound just-released ports failed in 11 of 60 full parallel runs and was replaced before its commit (see the F15 detail and section V8 of the evidence file).
- 2026-10-28 17:16 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the four replies are 17:16:24, 17:16:28, 17:16:32 and 17:16:35 UTC.
- 2026-10-28 17:16 UTC - A GraphQL fetch at 17:16:48 UTC, after the last reply, reported no unresolved thread.

- 2026-10-28 17:41 UTC - The `wait-for-pr-review` poller returned Copilot's fifth review 5687101185, submitted at 17:40:38 UTC (kind `review`, state `COMMENTED`, no `head_moved`; its stdout file was last written at 17:41:08 UTC). It opened two threads, F17 and F18, both unresolved. Each was checked against the current tree and found valid.
- 2026-10-28 17:45 UTC - The two fixes were committed, one signed commit each: F17 at 17:44 and F18 at 17:45 UTC (commit times from `git log --format=%cI`). The first commit attempt for F17 was refused by pre-commit because of one word in a comment that the spell checker did not know; it was fixed and committed after the checks passed.
- 2026-10-28 17:50 UTC - After the push, each thread was replied to and resolved with `reply-and-resolve-thread.sh`; the GitHub times of the two replies are 17:50:11 and 17:50:16 UTC.
- 2026-10-28 17:50 UTC - A GraphQL fetch at 17:50:20 UTC, after the last reply, reported no unresolved thread.

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
