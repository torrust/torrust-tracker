---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
---

# Agent Review Reports - Issue #1349 Add Tests to the http-core Package

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-17 08:13 UTC - Task Reviewer (slice S1, first review)

- Invocation scope: Slice S1 (T1-T3, documentation only) of #1349 on branch
  `1349-1347-add-tests-http-core-inventory`: the three commits `docs(issues): [#1349] record http
  core coverage baseline and inventory`, `docs(issues): [#1349] add http core plan guidance and
  the announce test plan`, and `docs(issues): [#1349] record the approval of the announce plan and
  slices`.
- Inputs: `ISSUE.md`, `coverage-evidence.md`, `test-refactor-plans/README.md`,
  `test-refactor-plans/services-announce-tests.md`, the #1347 `EPIC.md`, the subissue guidelines,
  the raw coverage reports and `packages/http-core/src/`.
- Evidence: the diff touches only five files under `docs/issues/open`; every recomputed coverage
  number, test count and the 13-file list match; `linter markdown`, `linter cspell` and
  `linter lychee` pass; no branch commit ids or closing keywords were added.
- Findings:
  - MEDIUM: `coverage-evidence.md` classified the zero region at `listener.rs:108` as logging; it is
    the `TcpAnnounce` alternative of `event_connection_id`, which runs for every event, and no
    listener test dispatches a `TcpAnnounce` event.
  - MEDIUM: `coverage-evidence.md` described the scrape region at line 127 as the no-key path; it is
    the `Some(key)` arm.
  - MEDIUM: the ledger showed the announce row `PENDING` ("no plan yet") although its plan was
    approved, and workflow step 0 requires `IN_PROGRESS`.
  - LOW: "2 doc tests" in `statistics/repository.rs` is stale; there are 3.
  - LOW: the plan's test review missed that the tcp6 announce event test name describes a peer-IP
    change its Arrange does not set up.
  - LOW: the missed-line counts in the coverage summary exceed the listed lines by one or two in
    five files, with no note.
  - LOW (optional): the announce plan is longer than the roughly 40 lines the guidelines suggest,
    and the EPIC row mentions only T1.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Fix the first six findings and re-run the Task Reviewer. The two optional items are not blocking.

### 2026-10-17 08:21 UTC - Task Reviewer (slice S1, re-review)

- Invocation scope: Re-review of S1 after the fixes for the 2026-10-17 08:13 UTC report, in the
  commits `docs(issues): [#1349] correct the listener, scrape and doc-test evidence`,
  `docs(issues): [#1349] mark the announce ledger row in progress` and `docs(issues): [#1349]
  record the misleading tcp6 event test name in the announce plan`.
- Inputs: the same files and sources as the first review, plus the three fix commits.
- Evidence: each of the six fixes was verified against the source lines and the raw reports; the
  recount of listed against missed lines gave a gap of one or two in every case; only one ledger row
  is `IN_PROGRESS`; the diff against `torrust/develop` is still documentation only; `linter
  markdown`, `linter cspell` and `linter lychee` pass.
- Findings:
  - LOW: `ISSUE.md` `last-updated-utc` and the progress log do not record the review-fix round.
  - LOW: the plan gained the tcp6 rename after its approval, but R1's description and the plan log
    do not mention the amendment.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Record the review round in the progress log and the plan log, and add the rename to R1.

### 2026-10-17 09:14 UTC - Task Reviewer (slice S1, re-measurement)

- Invocation scope: The commit `docs(issues): [#1349] re-measure the http core baseline after #2244`
  on the S1 branch, rebased onto `develop` `a3810b813` after #2244 (merged through #2552) changed
  `statistics/metrics.rs`.
- Inputs: the S1 deliverables, the EPIC #1347 rows and log, and the raw reports of the second
  measurement in `.tmp/1349-coverage/` (`t1b-*`).
- Evidence: recomputed totals (lines 1,737 / 1,953, regions 2,335 / 2,583, functions 160 / 190);
  only `statistics/metrics.rs` differs from the first measurement and the uncovered-line lists are
  identical; every file carrying the baseline agrees; the stamps are real and chronological; the
  diff is documentation only; `linter markdown`, `linter cspell` and `linter lychee` pass.
- Findings:
  - MEDIUM: `coverage-evidence.md` still named the superseded commit `04013e7f1` as the base of the
    uncovered line numbers.
  - LOW: its reproduction commands wrote to the superseded `t1-*` reports instead of the `t1b-*`
    reports that hold the numbers.
  - LOW: "One commit did:" no longer read correctly above the added second-commit paragraph.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Fix the three items and confirm.

### 2026-10-17 09:23 UTC - Task Reviewer (slice S1, confirmation)

- Invocation scope: The commit `docs(issues): [#1349] point the coverage evidence at the
  re-measured reports`, which fixes the three findings of the 09:14 UTC report.
- Inputs: `coverage-evidence.md`, the other S1 deliverables, the EPIC, and the raw `t1b-*` reports.
- Evidence: the three fixes are present; the remaining `04013e7f1` mentions are dated history
  followed by entries that record the re-measurement; no new inconsistency; the diff against
  `develop` is documentation only; `linter markdown`, `linter cspell` and `linter lychee` pass.
- Findings:
  - None.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - None.
