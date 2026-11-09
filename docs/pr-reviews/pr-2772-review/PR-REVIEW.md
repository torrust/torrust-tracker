---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2772 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2772>.

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

Round 1 is the Copilot review `5804546552` (submitted 2026-11-06 17:49 UTC; Copilot's agentic review did not run in full, and its overview lists two open issues, both inline). The two later empty `COMMENTED` reviews are the bot's own thread replies. Neither comment carries a `[Severity][Id]` prefix, so the finding IDs are audit-local and the severities are inferred. All CI checks were green at the time, so no CI finding exists.

Round 2 is the Copilot review `5807554509` (submitted 2026-11-06 21:54 UTC; one inline comment, row F3). Round 3 is the review `5823198555` of the maintainer's reviewer `da2ce7` (CHANGES_REQUESTED, 2026-11-09 08:15 UTC) with 14 inline findings carrying `[Severity][Id]` prefixes (rows F4-F17; the reviewer's IDs F1, F2 and F3 collide with this audit's F1-F3, and the reviewer's F4 to F14 got the next audit-local ordinals in source order, so every row of that review has the original ID in its detail entry). CI at that head was green, so no CI finding exists.

Round 4 is the approving review `5825002539` of `da2ce7` (2026-11-09 11:08 UTC; its 14 inline comments are rows F18-F25, reviewer IDs F15-F22) and Copilot's review `5825043286` (11:12 UTC, rows F26-F27). Each of these rows got the next audit-local ordinal in source order; the reviewer's original ID is in the detail entry.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2772-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2772-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2772-f3 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2772-f4 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2772-f5 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2772-f6 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2772-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2772-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2772-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2772-f10 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2772-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2772-f12 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2772-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2772-f14 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2772-f15 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2772-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2772-f17 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2772-f18 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2772-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2772-f20 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2772-f21 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2772-f22 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2772-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2772-f24 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2772-f25 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2772-f26 | Copilot | Suggestion (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2772-f27 | Copilot | Suggestion (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Stale "proposed" status sentence in the wiring plan

- PR number: 2772
- Source review ID: 5804546552
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4495749132
- Concern: `wiring-modules-tests.md` still said "proposed, waiting for the maintainer's approval" although its frontmatter is `status: done` and its Results section records the approval.
- Solution: The sentence now reads "Status: done; the maintainer approved the plan on 2026-11-06 (no test is added)". The other S6 plans were checked for the same sentence and have none (their append-only log lines keep the original wording).
- Current-tree verification: `grep -n -i "proposed, \|waiting\|proposed"` over the three S6 plans returns the append-only progress-log lines ("Plan drafted; waiting for the maintainer's approval") and one sentence of `testing-support-tests.md` ("no test is proposed"), which is not a status sentence; no status sentence says the plan is still proposed. `linter markdown` passes.
- Resolution reference: docs(issues): [#2637] align two S6 plans with their final state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4496391545

### F2 - Label names of the duplicate-token plan differ from the test

- PR number: 2772
- Source review ID: 5804546552
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4495749212
- Concern: `v1-middlewares-auth-tests.md` named the labels `first` and `second`, but the test uses `first`, `copy` and `other`.
- Solution: The plan table now names `first` and `copy` (the shared value) and `other`, as implemented. The test was not changed.
- Current-tree verification: the labels in the plan match the `AccessTokens::from` fixture of `it_should_match_a_token_against_every_label_when_two_labels_share_a_value` in `auth.rs`.
- Resolution reference: docs(issues): [#2637] align two S6 plans with their final state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4496392284

### F3 - Misleading rstest case label `the_value_of_the_third_label`

- PR number: 2772
- Source review ID: 5807554509
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4498027555
- Concern: The case uses `SECOND_TOKEN` while the fixture labels are `first`, `copy` and `other`, so a failure would read ambiguously.
- Solution: The cases are now `the_value_shared_by_the_first_and_copy_labels`, `the_value_of_the_other_label` and `a_value_that_no_label_holds`; the plan names the same cases.
- Current-tree verification: `cargo test -p torrust-tracker-axum-rest-api-server --lib v1::middlewares::auth` passes (30 tests); `cargo +nightly fmt --all -- --check` passes.
- Resolution reference: test(axum-rest-api-server): [#2637] name the shared-value token test and its cases after what they check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511836042

### F4 - `Version` has a consumer: the no-consumer claim is false

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664068
- Concern: The tracker binary uses `Version::V1` (`src/app.rs`, `src/bootstrap/jobs/tracker_apis.rs`), so the claims in the lib plan (decision row, L3 result, uses), `lessons.md` and two log entries were wrong; the first search covered the packages only.
- Solution: All six statements are corrected: `Version` is used by the binary, the compiler guards its use, L3 survives only because a test run of this package does not compile the root crate, and the removal-candidate row now says it is not one on that ground.
- Current-tree verification: `grep -n "Version::V1" src/app.rs src/bootstrap/jobs/tracker_apis.rs` shows the uses; `grep -rn -i "no consumer" docs/issues/open/2637-*` finds nothing that remains.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511836634

### F5 - The final comparison misstates the S5 movement and names the wrong baseline

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664072
- Concern: Functions fall 0.24 pp, not about 0.04 pp; covered regions drop by one; the deltas are against the S1 re-measurement, not the T1 tables. The PR description repeated this.
- Solution: The paragraph of `coverage-evidence.md` now states the exact movements and the S1 baseline (naming the T1 figures separately); the PR description was corrected too.
- Current-tree verification: Recomputed from the two tables of `coverage-evidence.md` (95.72 to 95.48, 94.21 to 93.97, regions 3,490 to 3,489 and 3,463 to 3,462); the PR body was edited and re-read.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511837355

### F6 - The manual evidence records summaries, not the run

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664076
- Concern: The file lacked the template's sections and kept the configurations only in a git-ignored path.
- Solution: M1-M5 were run again one step at a time with timestamps and recorded as V1-V5 in the template form, with the configurations (the public one in full, the others as diffs), the commands, the full outputs and the shutdown logs embedded; the frontmatter follows the template.
- Current-tree verification: The run is recorded in the file itself; `linter markdown` and `linter cspell` pass.
- Resolution reference: docs(issues): [#2637] record the manual verification in the template form
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511838096

### F7 - Stale statements about #2745 and a dead artifact path in the test support plan

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664081
- Concern: The spec moved to `docs/issues/closed/`, and #2745 has merged (`lib.rs` gates the module with `cfg(feature = "testing")`).
- Solution: The related artifact path points to the closed spec and the statements about the gate are current: landed, figures re-measured and unchanged.
- Current-tree verification: `ls docs/issues/closed | grep 2745` shows the spec; `grep -n 'cfg(feature = "testing")' packages/axum-rest-api-server/src/lib.rs` shows the gate.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511838709

### F8 - "Production code changed only by documentation" is false

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664089
- Concern: S2 carried the behavior-preserving `with_layers` refactor (ledger row 3).
- Solution: The retrospective now says the behavior did not change and names the refactor; `lessons.md` says "no behavior of the production code".
- Current-tree verification: `git log --oneline --all --grep="separate the router layers"` shows the S2 commit.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511901322

### F9 - The F1 current-tree verification does not match the bytes

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664095
- Concern: The grep also returns `testing-support-tests.md:19` ("no test is proposed"), which is not a progress-log line.
- Solution: The verification of F1 now states what the grep returns, including that sentence.
- Current-tree verification: `grep -n -i "proposed\|waiting"` over the three S6 plans, re-read.
- Resolution reference: docs(pr-reviews): [#2637] record the second and third review rounds of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511901928

### F10 - The S6 row and `related-pr` do not record this pull request

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664101
- Concern: The slice row had PR `TBD` and the frontmatter `related-pr` was `null`.
- Solution: The row says `#2772` (status `IN_PROGRESS` until the merge, as for S5), `related-pr` is 2772 and a log entry records the opening.
- Current-tree verification: `grep -n "2772" docs/issues/open/2637-*/ISSUE.md`.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511902446

### F11 - The second automated review at this head is not processed

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664104
- Concern: Copilot's second review (21:54 UTC) had one open thread with no reply and no audit row.
- Solution: The thread is row F3 of this audit, fixed, replied to and resolved.
- Current-tree verification: GraphQL refresh: no unresolved thread remains.
- Resolution reference: docs(pr-reviews): [#2637] record the second and third review rounds of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511903010

### F12 - `measured-utc` of the mutation evidence dates the first run

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664108
- Concern: The file reports the final run (ended 16:52:26Z), recorded by a later commit.
- Solution: `measured-utc` is `2026-11-06T16:52:26Z`, the `end_time` of the final run.
- Current-tree verification: `outcomes.json` of the final run has `end_time` 2026-11-06T16:52:26Z.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511839497

### F13 - Two analysis statements of the lib plan disagree with the bytes

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664113
- Concern: `API_LOG_TARGET` is not pinned by a unit test but by 21 integration tests; `InfoHashParam` is also the extractor of the torrent handlers.
- Solution: Both statements are corrected in the plan.
- Current-tree verification: `grep -n "InfoHashParam" packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs`; the 21 failures of mutation L1 are recorded in the plan.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511903785

### F14 - The A2 and A3 failure counts fit a name filter, not the stated procedure

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664115
- Concern: Over the module's tests A2 fails nine and A3 twelve; four and six are the counts of a run filtered by `authenticate`.
- Solution: The plan states the module command and the module counts (1, 9, 12), and keeps the filtered counts as such.
- Current-tree verification: Re-ran A1-A3 with `cargo test -p torrust-tracker-axum-rest-api-server --lib v1::middlewares::auth`: 1, 9 and 12 failures of 30.
- Resolution reference: test(axum-rest-api-server): [#2637] name the shared-value token test and its cases after what they check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511904277

### F15 - `last-updated-utc` of the EPIC is older than its own log

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664119
- Concern: The field said 2026-11-06 10:31 while the log has a 16:37 entry.
- Solution: The field carries the time of this update.
- Current-tree verification: `grep -n last-updated-utc docs/issues/open/1347-overhaul-packages-testing/EPIC.md`.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511904937

### F16 - "Three LOW findings, fixed" counts one with nothing to fix

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664121
- Concern: The third finding of the S6 review only confirmed existing statements.
- Solution: The log entry and the PR description say two were fixed and one needed no fix.
- Current-tree verification: `grep -n "LOW/INFO" docs/issues/open/2637-*/ISSUE.md`; the PR body was edited.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511905518

### F17 - The test name describes only one of its three cases

- PR number: 2772
- Source review ID: 5823198555
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4510664124
- Concern: The function asserted a shared value, a single-label value and a rejected value but was named for the shared one.
- Solution: The function is `it_should_match_a_token_against_every_label_when_two_labels_share_a_value`, named after what all three cases share (the plan and Results note the old name); the case names are as for F3.
- Current-tree verification: As F3.
- Resolution reference: test(axum-rest-api-server): [#2637] name the shared-value token test and its cases after what they check
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4511906020

### F18 - Old test name in the current-tree verification of the audit's F2

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050866
- Concern: The verification of F2 named `it_should_authenticate_a_token_configured_under_more_than_one_label`, which no longer exists.
- Solution: It names `it_should_match_a_token_against_every_label_when_two_labels_share_a_value`. The sweep `grep -rn it_should_authenticate_a_token_configured_under_more_than_one_label docs` finds only the auth plan's Results, which names the old name on purpose.
- Current-tree verification: The grep above, re-run.
- Resolution reference: docs(pr-reviews): [#2637] record the fourth review round of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512143547

### F19 - A seventh statement of the `Version` premise remains in the lib plan

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050871
- Concern: The mutation list said "L3 rename `Version::V1` (expected to survive: unused)" without a correction.
- Solution: The line now says the prediction was wrong because the tracker binary uses `Version`, pointing at Results, as the L1 prediction does.
- Current-tree verification: `grep -n "L3" test-refactor-plans/lib-tests.md` shows lines 49 and 78 agreeing.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the approving review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512144067

### F20 - The filtered mutation counts are stale after the rename

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050873
- Concern: A run filtered by `authenticate` no longer selects the renamed test: it selects seven tests.
- Solution: Re-ran A1-A3 with `cargo test -p torrust-tracker-axum-rest-api-server --lib authenticate`: 0, 2 and 5 failures of 7. The plan says the filtered counts 1, 4 and 6 belong to the first name and gives the current ones; the module counts (1, 9, 12 of 30) stand.
- Current-tree verification: The three runs, recorded in the plan.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the approving review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512144600

### F21 - The account of the manual run disagrees with itself and with its commands

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050879
- Concern: "By hand" against a script, "cut at the length shown by the command" with no such length, and an `openssl` command without `-out` and `-keyout`.
- Solution: The Purpose says the commands were executed by a sequential shell script, states the cut lengths (120 to 330 characters per command) and that only bodies are cut, the exact `openssl` command is given, and the `-H` arguments are quoted in every transcript line (this also answers the Copilot comment F26).
- Current-tree verification: `grep -n openssl` and `grep -n 'curl.*-H ' manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the approving review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512145216

### F22 - Trailing whitespace in the embedded `diff` output

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050888
- Concern: The trailing space that `diff` prints after `>` for an empty added line.
- Solution: The blocks are recorded as `diff --suppress-blank-empty` output, without the space.
- Current-tree verification: `git diff --check` is clean for the file.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the approving review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512145648

### F23 - The body of `18c16c7fd` credits it with a mutation count change it did not make

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050895
- Concern: The commit changes only the `measured-utc` date of `mutation-evidence.md`; the failure counts were corrected by `e83aa04ad`.
- Solution: A published commit body is not rewritten; this record states the correction (see the Processing Log) and the plan carries the counts from `e83aa04ad`.
- Current-tree verification: `git show 18c16c7fd --stat -- docs/issues/open/2637-*/mutation-evidence.md` shows the date as its only change there.
- Resolution reference: docs(pr-reviews): [#2637] record the fourth review round of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512146250

### F24 - The log entry stamped 10:50 UTC records a commit and a push that had not happened

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050861
- Concern: The entry counted this audit among four commits and said the branch was pushed; the audit commit is dated 10:51:42 UTC, and two replies cite its subject before it existed.
- Solution: The entry is replaced by entries that state what had happened at each time (three fix commits pushed at 10:42, replies from 10:43, the audit committed at 10:51 and pushed at 10:54), and a correction entry names the rewritten content.
- Current-tree verification: `git log --format='%h %cI %s' -6` and `ls` of the push logs give the times.
- Resolution reference: docs(pr-reviews): [#2637] record the fourth review round of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512146712

### F25 - The collision statement does not match the ids

- PR number: 2772
- Source review ID: 5825002539
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512050905
- Concern: Only the reviewer's F1-F3 collide with this audit's F1-F3; each detail entry said the reviewer reuses F1 and F2.
- Solution: The parenthetical is removed from the 14 entries (the original ID stays) and the intro states which IDs collide.
- Current-tree verification: `grep -c "reuses the audit IDs"` of the audit returns 0.
- Resolution reference: docs(pr-reviews): [#2637] record the fourth review round of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512147348

### F26 - The `curl -H Authorization: Bearer ...` examples are not valid shell syntax

- PR number: 2772
- Source review ID: 5825043286
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512082781
- Concern: The transcript showed the header without quotes.
- Solution: Every header is quoted in the transcript (`-H "Authorization: Bearer ..."`), and the line saying they were shown after quote removal is gone.
- Current-tree verification: `grep -n 'curl.*-H ' manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the approving review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512147844

### F27 - The `tsl_config` failure is a deserialization error, not a TOML parse error

- PR number: 2772
- Source review ID: 5825043286
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512082859
- Concern: The lib plan called the rejection of the old spelling a TOML parse error.
- Solution: The plan says deserialization error (unknown field, from `deny_unknown_fields`).
- Current-tree verification: `grep -rn "parse error" docs/issues/open/2637-*` finds nothing.
- Resolution reference: docs(issues): [#2637] correct the S6 records after the approving review of PR 2772
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2772#discussion_r4512148431

## Processing Log

- 2026-11-06 17:49 UTC - Copilot review `5804546552` submitted with two inline findings on the PR opened at 16:59 UTC. No human review existed at this point.
- 2026-11-06 18:57 UTC - Fixed F1 and F2 in one signed commit and pushed the branch (the commit subject, not an id, is cited above), replied to both threads and resolved them; both resolve mutations returned `isResolved: true`, and a refresh found no unresolved thread.
- 2026-11-06 21:54 UTC - Copilot review `5807554509` submitted with one inline finding (F3) on the head after the first fixes.
- 2026-11-09 08:15 UTC - Review `5823198555` of `da2ce7` submitted with 14 inline findings (F4-F17).
- 2026-11-09 10:42 UTC - Verified each of the 15 findings of rounds 2 and 3 against the tree (all valid), fixed them in three signed commits (a test rename, the manual evidence redone in the template form, the record corrections), pushed the branch (the push finished at 10:42:40 UTC) and corrected the PR description.
- 2026-11-09 10:43 UTC - Replied to the threads of F3, F4, F5, F6, F7 and F12 and resolved all of them except F12 (10:43:03 to 10:43:28 UTC); then the bot token was rejected by GitHub and the remaining replies could not be posted.
- 2026-11-09 10:50 UTC - After a retry the token worked: replied to the threads of F8-F11 and F13-F17 and resolved them, and resolved F12 (10:50:25 to 10:50:57 UTC). The replies of F9 and F11 cite the commit `docs(pr-reviews): [#2637] record the second and third review rounds of PR 2772` before it existed.
- 2026-11-09 10:51 UTC - Committed this audit (10:51:42 UTC) and pushed it at 10:54 UTC (push finished 10:54:33 UTC).
- 2026-11-09 11:08 UTC - Review `5825002539` of `da2ce7` approved the head with eight open items (F18-F25); at 11:12 UTC Copilot review `5825043286` added two comments (F26-F27).
- 2026-11-09 11:16 UTC - Correction (F24, F23): the entry stamped 2026-11-09 10:50 UTC in the commit `docs(pr-reviews): [#2637] record the second and third review rounds of PR 2772` described four commits and a push that had not happened yet; it was replaced by the four entries above, which state what happened at each time. The body of the commit `docs(issues): [#2637] correct the S6 records after the review of PR 2772` says it makes the mutation counts agree with the bytes; that commit changed only the `measured-utc` date of `mutation-evidence.md`, and the failure counts were corrected by `test(axum-rest-api-server): [#2637] name the shared-value token test and its cases after what they check`.
- 2026-11-09 11:19 UTC - Verified F18-F27 against the tree (all valid), fixed them in the commit `docs(issues): [#2637] correct the S6 records after the approving review of PR 2772` (pushed before the replies) and in the commit that adds this entry; replied to the ten threads and resolved them (11:19 to 11:20 UTC), each resolve returning `isResolved: true`.

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
