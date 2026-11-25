---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3056 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3056.

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
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is da2ce7's review `6061322366` (2026-11-25 16:38:40 UTC, changes requested) of the head whose last commit is
`docs(issues): [#2902] record the S4 measurement on the rebased branch`. Its seven inline findings carry
no reviewer IDs and are recorded as F1 to F7 in source order, with the severity of their brackets.

Round 2 is da2ce7's review `6061785377` (17:08:37 UTC, comment, no approval) of the head whose last commit is
`docs(issues): [#2902] re-measure S4 and correct its records after review`. Its three inline findings carry no
reviewer IDs and are recorded as F8 to F10. The review `6061776444` of `claude[bot]` (17:07:59 UTC, comment) found
no issue and needs no row. CodeRabbit posted its summary comment only, with no finding.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3056-f1 | da2ce7 | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3056-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3056-f3 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3056-f4 | da2ce7 | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3056-f5 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3056-f6 | da2ce7 | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3056-f7 | da2ce7 | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3056-f8 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3056-f9 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3056-f10 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The rebased coverage rows were not a measurement of the head

- PR number: 3056
- Source review ID: 6061322366
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4697845957
- Concern: The 18 head cells of the S4 coverage section equaled the rebased base plus the pre-rebase deltas, so they read as derived, not measured; the section also claimed that line counts drift between runs.
- Solution: Measured every scope again from an empty coverage target at the head and at the base (the head with the slice's changes under `packages/` reverted), rewrote the S4 section with those figures and a description of how the base is made, and updated the EPIC rows, the ledger and the log. The reply says the unit-only and aggregate lines are 88.20%.
- Current-tree verification: `grep -n 'Measured on 2026-11-25 at 16:50 UTC' docs/issues/open/2902-1347-add-tests-tracker-core/coverage-evidence.md` finds the section, and `cargo test -p torrust-tracker-core --all-features --lib` passes 349 tests.
- Resolution reference: docs(issues): [#2902] re-measure S4 and correct its records after review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698087728

### F2 - The rebase left records describing the pre-rebase tests

- PR number: 3056
- Source review ID: 6061322366
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4697845969
- Concern: Ledger rows 29 and 30 said 18 and 13 tests, plan-table row 21 listed the error texts, the peer-key plan said bytes and the key-module plan excluded debug-log lines that #3042 removed.
- Solution: Corrected the two rows (19 and 10 tests, per-file line counts after the rebase), the plan-table row, the property row of the peer-key plan and the key-module plan's coverage and exclusion text.
- Current-tree verification: `grep -n '19 tests' docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` and `grep -n 'error texts' docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/README.md` show the corrected rows.
- Resolution reference: docs(issues): [#2902] re-measure S4 and correct its records after review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698088310

### F3 - The record of the rebase disagreed with itself in three places

- PR number: 3056
- Source review ID: 6061322366
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4697845984
- Concern: The EPIC's 15:53 entry carried the rebased base and the 16:17 figures under its old stamp, a commit body said it dropped the error-text table although another commit changes `key/mod.rs`, and the description's Validation line said the pre-commit checks ran on every commit.
- Solution: Restored the EPIC's 15:53 entry to its original text and added a 16:50 entry, recorded the commit-body error in the spec's log without rewriting history, and corrected the Validation line of the description.
- Current-tree verification: `grep -n '2026-11-25 1[56]:' docs/issues/open/1347-overhaul-packages-testing/EPIC.md` shows the 15:53 entry on base `41f294c9d` and a separate 16:50 entry.
- Resolution reference: docs(issues): [#2902] re-measure S4 and correct its records after review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698088879

### F4 - The S2 and S3 slice rows still read IN_PROGRESS

- PR number: 3056
- Source review ID: 6061322366
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4697845996
- Concern: The slice table still marked S2 and S3 as in progress although #3009 and #3036 had merged.
- Solution: Marked both rows `DONE`.
- Current-tree verification: `grep -n '| S[23] ' docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` shows `DONE` in both rows.
- Resolution reference: docs(issues): [#2902] re-measure S4 and correct its records after review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698089438

### F5 - Neither bug found could be followed from the spec

- PR number: 3056
- Source review ID: 6061322366
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4697846014
- Concern: The findings procedure asks for a progress-log entry and a register row naming the draft or issue; the overflow panic had no register row and neither bug named an issue.
- Solution: Added register row F7 and named #3051 and #3052 and their spec PRs in F4, F7 and the log; the lessons register is finding F10.
- Current-tree verification: `grep -n '^| F7' docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` finds the row.
- Resolution reference: docs(issues): [#2902] re-measure S4 and correct its records after review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698090103

### F6 - The clearing done by reset_with was pinned only through the keys handler

- PR number: 3056
- Source review ID: 6061322366
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4697846028
- Concern: The repository's own reset test passed with a `reset_with` that did not clear, so a collaborator-owned rule was pinned only by the handler's tests.
- Solution: The repository's own reset test now asserts that the old key is gone, and the handler's test module says it is a collaboration test; the mutant that does not clear fails the repository's test. The two pass-through errors of the persisted repository are not tested in their own module: forwarding is not a decision the repository makes, and the handler tests pin what a caller observes.
- Current-tree verification: `cargo test -p torrust-tracker-core --all-features --lib reset_the_peer_keys` passes, and a mutant `reset_with` that does not clear fails it.
- Resolution reference: test(tracker-core): [#2902] pin the clearing of a key reset where it is owned
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698090661

### F7 - Two test names dropped the it_should_ prefix

- PR number: 3056
- Source review ID: 6061322366
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4697846033
- Concern: `adding_should_report_whether_the_hash_was_new` and `removing_should_report_whether_the_hash_was_present` lacked the `it_should_` prefix that `write-unit-test` requires.
- Solution: Renamed them `it_should_report_on_adding_whether_the_hash_was_new` and `it_should_report_on_removing_whether_the_hash_was_present`.
- Current-tree verification: `grep -n 'async fn it_should_report_on' packages/tracker-core/src/whitelist/repository/in_memory.rs` prints both.
- Resolution reference: test(tracker-core): [#2902] pin the clearing of a key reset where it is owned
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698091143

### F8 - The answered threads had no audit record

- PR number: 3056
- Source review ID: 6061785377
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698237779
- Concern: The seven review threads were answered and resolved but the branch had no `docs/pr-reviews/pr-3056-review/PR-REVIEW.md`, which `process-pr-review` requires before the first reply.
- Solution: Created this record from the template, covering both rounds, and logged the late start below.
- Current-tree verification: `python3 .claude/skills/process-pr-review/scripts/validate-audit-record.py --pr-number 3056` passes.
- Resolution reference: docs(pr-reviews): [#2902] record the first two review rounds of #3056
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_rN/A

### F9 - The 15:53 log entry gained later facts in place

- PR number: 3056
- Source review ID: 6061785377
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698237792
- Concern: The spec's 15:53 progress-log entry carried two insertions that name #3051 and #3052, which came after its stamp.
- Solution: Removed both insertions; the 16:50 entry names the issues.
- Current-tree verification: `grep -c 'now #3051' docs/issues/open/2902-1347-add-tests-tracker-core/ISSUE.md` prints 0 in the 15:53 entry.
- Resolution reference: docs(issues): [#2902] keep the S4 log entry to its stamp and add the lessons register
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_rN/A

### F10 - The findings procedure's lessons.md row was missing

- PR number: 3056
- Source review ID: 6061785377
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_r4698237813
- Concern: The fourth item of the findings procedure asks for a `lessons.md` row naming the draft or issue, and the issue folder had no `lessons.md`.
- Solution: Added `lessons.md` with the follow-up candidates (F1, F3, F4 and F7, naming #3051 and #3052) and four testing lessons of the slice.
- Current-tree verification: `ls docs/issues/open/2902-1347-add-tests-tracker-core/lessons.md` finds the file.
- Resolution reference: docs(issues): [#2902] keep the S4 log entry to its stamp and add the lessons register
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3056#discussion_rN/A

## Processing Log

- 2026-11-25 16:38 UTC - Round 1 arrived (review `6061322366`).
- 2026-11-25 16:56 UTC - Replied to the seven threads of round 1 and resolved them, without an audit record: the skill requires the record before the first reply.
- 2026-11-25 17:08 UTC - Round 2 arrived (review `6061785377`); its first finding is the missing record.
- 2026-11-25 17:12 UTC - Started this audit late from the template, covering both rounds.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
