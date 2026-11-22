---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2988 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2988.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is two reviews of the head whose last commit is
`docs(issues): [#2902] record the S1 pull request and close the F20 follow-up`: Copilot's review
`6006223766` (2026-11-21 15:13:36 UTC, comment, three inline comments without IDs, recorded as F5
to F7 with the severity of its overview, two Medium and one Low) and da2ce7's review `6006307330`
(2026-11-21 15:32:08 UTC, approval, inline findings F1 to F4 with the reviewer's IDs).
Round 2 is da2ce7's review `6011586262` (2026-11-22 09:55:05 UTC, approval) of the head whose last
commit is `docs(pr-reviews): [#2902] record the first review round of #2988`; its inline findings
keep the reviewer's IDs F21 and F22.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2988-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2988-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2988-f3 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2988-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2988-f5 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2988-f6 | Copilot | Minor (inferred) | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2988-f7 | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F21 | review-finding:pr-2988-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2988-f22 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The S1 close entry landed under References

- PR number: 2988
- Source review ID: 6006307330
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4653399821
- Concern: The 2026-11-21 13:24 EPIC entry followed a stray 2026-11-09 line into `## References`, so the Progress Log never said that S1 closed.
- Solution: The entry ends the Progress Log; the stray #2637 entry moved, in its own commit, between the 2026-11-06 and 2026-11-11 entries.
- Current-tree verification: `sed -n '/^## References/,$p' docs/issues/open/1347-overhaul-packages-testing/EPIC.md | grep -c '^- 2026'` prints 0.
- Resolution reference: docs(issues): [#2902] move the S1 close entry into the EPIC progress log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657542178

### F2 - The table tests looped over their cases

- PR number: 2988
- Source review ID: 6006307330
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4653399827
- Concern: Three table tests iterated in a `for` loop, where the `write-unit-test` skill prescribes `rstest` cases; the first broken case hid the others.
- Solution: By the maintainer's decision of 2026-11-22, the three tables are 14 named `rstest` cases; `rstest` 0.28.0 is a dev-dependency of the package. The row 5 and row 7 plans record the change.
- Current-tree verification: `cargo test -p torrust-tracker-core --all-features --lib statistics:: -- --list` lists the 14 `case_` tests; with the `PeerRemoved` mutant only `case_4_peer_removed` fails.
- Resolution reference: test(tracker-core): [#2902] turn the statistics table tests into rstest cases
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657542313

### F3 - The test name said save where the mock fails the read

- PR number: 2988
- Source review ID: 6006307330
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4653399830
- Concern: The global failure test failed `load_global_downloads`, so no save ran, but its name and message said the global downloads could not be saved.
- Solution: The test is `it_should_not_increment_the_persisted_total_when_the_global_downloads_cannot_be_increased`, with a matching message; the plan uses the new name.
- Current-tree verification: `grep -rn 'cannot_be_saved' packages/tracker-core docs/issues/open/2902-1347-add-tests-tracker-core` prints nothing.
- Resolution reference: test(tracker-core): [#2902] name the global downloads failure test after the read it fails
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657542399

### F4 - Plan text not reconciled with the code

- PR number: 2988
- Source review ID: 6006307330
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4653399836
- Concern: The listener plan named one helper, `join_within_deadline`, and the repository plan said gauges are compared as `Option<f64>`.
- Solution: The listener plan names `LISTENER_DEADLINE`, `send` and `within_deadline`; the repository plan says `Option<Gauge>`.
- Current-tree verification: `grep -rn 'join_within_deadline\|Option<f64>' docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans` prints nothing.
- Resolution reference: docs(issues): [#2902] reconcile the listener and repository plans with their tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657542502

### F5 - The persistence check looked up one info-hash only

- PR number: 2988
- Source review ID: 6006223766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4653330510
- Concern: The persistence table asserted no row for `sample_info_hash()`, which would miss a row written under the event's own info-hash if the helper were not deterministic.
- Solution: The helper parses a fixed literal, so the old check was sound; it is still widened: each case expects no torrent row at all (`load_all_torrents_downloads()` is empty). A mutant writing a row under another info-hash fails all five cases.
- Current-tree verification: `grep -n 'load_all_torrents_downloads().await.unwrap().len()' packages/tracker-core/src/statistics/event/handler.rs` shows the assertion.
- Resolution reference: test(tracker-core): [#2902] check that no torrent row is persisted for an event other than a completion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657542603

### F6 - The same comment on the persistence table

- PR number: 2988
- Source review ID: 6006223766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4653330553
- Concern: The F5 comment, repeated at the persistence table.
- Solution: Superseded by F5, fixed by the same commit.
- Current-tree verification: As for F5.
- Resolution reference: test(tracker-core): [#2902] check that no torrent row is persisted for an event other than a completion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657542683

### F7 - One database per case

- PR number: 2988
- Source review ID: 6006223766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4653330586
- Concern: The persistence table built a fresh temporary database for each of its five cases, which may be slow.
- Solution: Superseded by F2: each case is its own `rstest` test, run in parallel; one database per test keeps the cases isolated, as the spec's ownership map requires. The five cases take 0.01 s.
- Current-tree verification: `cargo test -p torrust-tracker-core --all-features --lib it_should_persist_nothing` runs five tests.
- Resolution reference: test(tracker-core): [#2902] turn the statistics table tests into rstest cases
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657542783

### F21 - The description reported the round 1 test count

- PR number: 2988
- Source review ID: 6011586262
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657607454
- Concern: The description still said 180 unit tests and "Test code and documentation only", without the `rstest` dev-dependency that round 1 added; the merge commit copies the description.
- Solution: The Validation line gives 191 unit test cases at the head after round 1, the summary names the `rstest` dev-dependency, rows 5 and 7 say `rstest` tables, and a bullet lists round 1; the coverage sentence keeps 180 "at the measured commit".
- Current-tree verification: `gh pr view 2988 --json body --jq .body | grep -c '191 unit test cases'` prints 1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657617124
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657617124

### F22 - A Test Design row still promised the event's name in the message

- PR number: 2988
- Source review ID: 6011586262
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657607460
- Concern: The handler plan's Test Design rows still said "in a table" and "with the event's name in the message", although each event is now an `rstest` case sharing one message.
- Solution: Both rows say one `rstest` case per event, with the case name naming the event; the module plan got the same pointer.
- Current-tree verification: `grep -c "the event's name in the message" docs/issues/open/2902-1347-add-tests-tracker-core/test-refactor-plans/statistics-event-handler-tests.md` prints 0.
- Resolution reference: docs(issues): [#2902] point the table test designs at their rstest cases
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2988#discussion_r4657617254

## Processing Log

- 2026-11-22 09:22 UTC - Read both round 1 reviews; the maintainer chose `rstest` cases for F2. Fixed F2, F5, F3, F1 (and the stray #2637 line) and F4 in their own commits, then recorded the review changes in the row 5 and row 7 plans.
- 2026-11-22 09:34 UTC - Replied on all seven threads (09:34:44 to 09:34:57 UTC), before pushing the fixes and this record together; the threads are resolved after the push.
- 2026-11-22 09:58 UTC - Round 2 (approval with F21 and F22): F21 fixed in the description, F22 in its own commit; replied at 09:58:21 and 09:58:23 UTC, before pushing the fix and this record together; the threads are resolved after the push.

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
