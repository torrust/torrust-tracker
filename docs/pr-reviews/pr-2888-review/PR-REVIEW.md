---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2888 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2888.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2888-f1 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2888-f2 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2888-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2888-f4 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2888-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The fixture tests and the two properties carry no AAA markers

- PR number: 2888
- Source review ID: 5923877279
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588718259
- Concern: The `write-unit-test` checklist asks for `// Arrange` / `// Act` / `// Assert` markers,
  and the format test follows it; the 14 fixture tests and the two `#[quickcheck]` properties do
  not. The reviewer offered two fixes: add the markers, or record in the test plan that they were
  dropped as redundant prose.
- Solution: Recorded the decision in the test plan, for both the properties and the fixture tests:
  the markers are redundant prose for one-expression properties and one- to four-statement bodies
  (the skill's prose-first step 4), matching the other short tests in `peer.rs`.
- Current-tree verification: `grep -n "markers were left out" docs/issues/open/2876-1347-peer-id-test-helper/test-plans/qbittorrent-peer-id.md`
  finds the property note (T2 section) and the fixture-test note (T7 section).
- Resolution reference: docs(issues): [#2876] record why the short peer.rs tests carry no AAA markers
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588757231

### F2 - The amendment leaves the O1c claim that machete does not flag the self dev-dependency

- PR number: 2888
- Source review ID: 5923877279
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588718262
- Concern: The branch's own retrospective shows that `cargo machete --with-metadata` flagged the
  self dev-dependency of `primitives`, a package without `tests/` or examples, but the ADR's O1c row
  still says machete does not flag it, and the retrospective deferred the note.
- Solution: The 2026-11-16 amendment says the O1c note holds only when `tests/` or examples name
  the crate, and that a package without them lists the self dev-dependency in its machete `ignored`
  list with the reason. The retrospective's improvement 1 now points at the amendment.
- Current-tree verification: `grep -n "O1c note" docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md`
  finds the clause in the amendment of 2026-11-16.
- Resolution reference: docs(adrs): [#2876] say when cargo machete flags a testing self dev-dependency
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588757687

### F3 - Two fixture tests have four statements, not one to three

- PR number: 2888
- Source review ID: 5924274073
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4589031084
- Concern: The F1 fix described the fixture tests as one- to three-statement bodies, but
  `it_should_set_no_bytes_left_to_download_for_a_peer_that_had_bytes_left` and
  `it_should_set_the_started_event_for_a_peer_that_had_completed` have four (they first assert their
  starting fixture). The F1 reply repeats the count.
- Solution: The test plan and the F1 row now say one- to four-statement bodies, four in the two
  tests that assert their starting fixture. The F1 reply cannot be edited; this row records the
  correction.
- Current-tree verification: `grep -n "four-statement" docs/issues/open/2876-1347-peer-id-test-helper/test-plans/qbittorrent-peer-id.md docs/pr-reviews/pr-2888-review/PR-REVIEW.md`
  finds the T7 note and the F1 solution; `grep -c "three-statement"` on the test plan gives 0 (the
  audit keeps the old wording only where F3 quotes it).
- Resolution reference: docs(issues): [#2876] correct the statement count of the short peer.rs tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4589180027

### F4 - The Processing Log puts the round-1 replies one minute early

- PR number: 2888
- Source review ID: 5924274073
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4589031100
- Concern: The round-1 entry said the two thread replies were posted at 16:53 UTC; GitHub records
  16:54:26 and 16:54:29 UTC. Their order before the push is right.
- Solution: A Processing Log correction entry (17:55 UTC) states the correct time, 16:54 UTC. The round-1 entry
  keeps its original text, because the log is append-only (finding F5; round 2 had first rewritten
  that entry in place).
- Current-tree verification: `grep -n "16:54 UTC" docs/pr-reviews/pr-2888-review/PR-REVIEW.md`
  finds the correction entry; the round-1 entry still says 16:53 UTC.
- Resolution reference: docs(pr-reviews): [#2876] restore the round-1 log entry and append its correction
  (first fixed in place by `docs(pr-reviews): [#2876] record review round 2 of PR 2888`; see F5)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4589180507

### F5 - The F4 fix rewrote the round-1 Processing Log entry in place

- PR number: 2888
- Source review ID: 5924785765
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4589413418
- Concern: `process-pr-review` step 5 and the template say the Processing Log is append-only; an
  entry rewritten in place is restored and corrected by a new entry naming the rewritten content.
  The F4 fix had rewritten the round-1 entry.
- Solution: The round-1 entry is restored (`16:53 UTC,`), and a new correction entry states the
  right time and names the in-place rewrite. The F4 row now describes that fix.
- Current-tree verification: `grep -n "16:53 UTC\|16:54 UTC" docs/pr-reviews/pr-2888-review/PR-REVIEW.md`
  shows the restored round-1 entry, the F4 row and the correction entry.
- Resolution reference: docs(pr-reviews): [#2876] restore the round-1 log entry and append its correction
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4589431881

## Processing Log

- 2026-11-16 16:52 UTC - Started audit for review round 1: review 5923877279 (da2ce7, APPROVED, with
  ACK, one Nit F1 and one Suggestion F2, neither blocking). The maintainer had posted his ACK on the
  same head before the review. Both fixed by commit; the two thread replies posted at 16:53 UTC,
  before the push of the fix commits.
- 2026-11-16 17:32 UTC - Review round 2: review 5924274073 (da2ce7, APPROVED, two Nits F3 and F4,
  neither blocking) on the head with the round-1 fixes, which the maintainer ACKed. F3 fixed by
  commit, F4 in this audit; the two thread replies posted at 17:31 UTC, before the push.
- 2026-11-16 17:55 UTC - Correction (F4, applied as F5 asks): the round-1 entry above says the two
  thread replies were posted at 16:53 UTC; GitHub records 16:54:26 and 16:54:29 UTC, still before
  the push. Round 2 first fixed this by rewriting the round-1 entry in place (to "16:54 UTC
  (corrected in round 2, F4)"), which broke the append-only rule; review round 3 (review 5924785765,
  da2ce7, APPROVED, one Minor finding F5) found it, and the round-1 entry is restored to its original
  text. F5 fixed by commit; its thread reply posted before the push.

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
