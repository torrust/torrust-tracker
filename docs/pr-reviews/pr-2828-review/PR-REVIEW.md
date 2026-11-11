---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2828 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2828.

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

Rows F1-F12 come from review 5858450371 (`da2ce7`). Rows F13, R1 and R2 come from review 5858504509 (Copilot); R1 and R2 re-raise F2 and F8. Rows F19, F15, F14, F16, F17 and F18 come from the comments of review 5858813320 (`da2ce7`, approved); the reviewer's `F9` there repeats the earlier F9, so it is recorded as F19 with `Reviewer finding ID` F9. Rows F20 and F21 come from review 5859145551 (`da2ce7`, approved).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2828-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2828-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2828-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2828-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2828-f5 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2828-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2828-f7 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2828-f8 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2828-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2828-f10 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2828-f11 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2828-f12 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2828-f13 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2828-r1 | Copilot | Minor | correctness | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| R2 | review-finding:pr-2828-r2 | Copilot | Minor | testing | RE_RAISE_OF:F8 | NO_ACTION | SUPERSEDED |
| F19 | review-finding:pr-2828-f19 | Human | Minor | documentation | RE_RAISE_OF:F9 | FIXED | RESOLVED |
| F15 | review-finding:pr-2828-f15 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2828-f14 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2828-f16 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2828-f17 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2828-f18 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2828-f20 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2828-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - `peer::fixture` is deferred to an owner that is closed

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662831
- Concern: The spec and the evidence defer the uncovered `peer::fixture` lines to EPIC #2482, which is closed and did not reorganize the fixture, so no live owner exists.
- Solution: Named the F5 follow-up as the owner of any move behind a `testing` feature, planned the classification of the fixture lines as test support in the S2 `peer.rs` record, and corrected the four places.
- Current-tree verification: `grep -n 2482` over the spec and the evidence file shows only the corrected statements; the EPIC spec reads `status: done`; `peer.rs` declares `pub mod fixture` without a feature gate.
- Resolution reference: `docs(issues): [#2824] name a live owner for the peer fixture lines`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537764358

### F2 - The slice table's "Ledger rows" use the inventory numbering, not the ledger's

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662842
- Concern: The Ledger rows column of the slice table used the inventory numbers, which contradicts the ledger.
- Solution: S3 is now rows 2-6 and S4 rows 7-16.
- Current-tree verification: Read the slice table and the ledger side by side.
- Resolution reference: `docs(issues): [#2824] number the slice table rows by the ledger`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537764795

### F3 - The slicing is called proposed and awaiting approval, but the spec records it as approved

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662850
- Concern: The slice table intro said the slicing awaited approval while the checkpoint and the log record it as approved.
- Solution: The intro now says the maintainer approved the slicing on 2026-11-10.
- Current-tree verification: `grep -n awaiting` over the spec finds nothing.
- Resolution reference: `docs(issues): [#2824] mark the slicing as approved`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537765206

### F4 - The Design and Ownership Review asks questions the register already answers

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662865
- Concern: The section called five questions open although the Findings Register answers them, and the first described code that no longer exists.
- Solution: Each bullet now points at its finding F1-F5 and its outcome.
- Current-tree verification: Read the section against the register.
- Resolution reference: `docs(issues): [#2824] point the design review questions at the findings`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537765607

### F5 - `zeroed` is listed as uncovered, but it is the covered part of `swarm_metadata.rs`

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662873
- Concern: The inventory row listed `zeroed` among the uncovered groups although it is the covered part of 3 / 18 lines.
- Solution: The row now lists the accessors (41-53) and `add_assign` (75-80) as uncovered and `zeroed` (36-38) as covered.
- Current-tree verification: Read `packages/primitives/src/swarm_metadata.rs` lines 30-55 and the evidence row.
- Resolution reference: `docs(issues): [#2824] drop zeroed from the uncovered swarm_metadata lines`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537766056

### F6 - `peer.rs` has two `no_run` examples and, at this head, a `compile_fail` doctest

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662880
- Concern: The technique table counted one `no_run` example and called the examples no evidence, but `peer.rs` has two plus a `compile_fail` doctest.
- Solution: The table counts both `no_run` examples (lines 5 and 94) and records the doctest (line 117) as behavior evidence.
- Current-tree verification: `grep -n 'no_run\|compile_fail' packages/primitives/src/peer.rs` shows lines 5, 94 and 117.
- Resolution reference: `docs(issues): [#2824] count both peer.rs examples and the compile_fail doctest`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537766398

### F7 - No Acceptance Verification table, and the manual scenarios have no Status or Evidence fields

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662886
- Concern: The spec lacked the Acceptance Verification table and the Status and Evidence fields of the manual scenarios that `create-issue` and the template require.
- Solution: Added the AC1-AC12 table and the two columns.
- Current-tree verification: Read the spec sections `Manual Verification` and `Acceptance Verification`.
- Resolution reference: `docs(issues): [#2824] add the acceptance verification table and manual scenario status`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537766762

### F8 - M1 expects the announce response to carry the peer cap

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662897
- Concern: No announce response field carries the peer cap, so M1's expected result cannot be observed from one announce.
- Solution: Limited M1 to the default interval; the cap default is protected by the S3 unit test.
- Current-tree verification: Read the M1 row.
- Resolution reference: `docs(issues): [#2824] limit manual scenario M1 to the announce interval`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537767195

### F9 - The new EPIC log entry sits out of order

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662906
- Concern: The EPIC log entry was inserted in the middle of the log and stamped later than the commit that records it.
- Solution: Moved it after the last entry of the log and stamped it and `last-updated-utc` with the commit time, 2026-11-11 11:22 UTC.
- Current-tree verification: Read the EPIC log tail and `git log --format=%cI` of the commit that records it.
- Resolution reference: `docs(issues): [#2824] append the EPIC log entry in order and stamp it from the commit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537767561

### F10 - The S0 row still says `TBD` and `related-pr` is `null`

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662919
- Concern: The spec did not record the open pull request.
- Solution: `related-pr` and the S0 row now read 2828.
- Current-tree verification: Read the frontmatter and the slice table.
- Resolution reference: `docs(issues): [#2824] record the S0 pull request number`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537767858

### F11 - The first two log entries lack the time and the actor

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662925
- Concern: The first two progress-log entries did not follow the template form.
- Solution: They now carry an actor and say their time was not recorded; the third entry is stamped with the commit time.
- Current-tree verification: Read the progress log.
- Resolution reference: `docs(issues): [#2824] write the first log entries in the template form`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537768262

### F12 - The evidence frontmatter differs from the sibling coverage evidence

- PR number: 2828
- Source review ID: 5858450371
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537662933
- Concern: The evidence file used a frontmatter unlike its siblings.
- Solution: It now uses `doc-type: coverage-evidence` with `issue`, `package`, `measured-commit` and `measured-utc` (a date).
- Current-tree verification: Read the frontmatter; the markdown, yaml and spell linters passed on commit.
- Resolution reference: `docs(issues): [#2824] align the coverage evidence frontmatter with its siblings`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537768598

### F13 - Require S1 verification evidence before merging the F2 fix

- PR number: 2828
- Source review ID: 5858504509
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537706140
- Concern: S1 fixes the confirmed F2 defect, but the spec called the issue not a known defect and deferred manual verification to S4, which conflicts with the `fix-bug` workflow.
- Solution: The Bug-Fix Process and Regression Test Strategy sections now make F2 the exception and require S1 to record the reproduction, the red and green regression results and the final recheck in `manual-verification-evidence.md`; M1 and M2 stay in S4.
- Current-tree verification: Read the two sections and the slice rules.
- Resolution reference: `docs(issues): [#2824] require fix-bug verification evidence in slice S1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537768910

### R1 - Align the slice row numbers with the source ledger

- PR number: 2828
- Source review ID: 5858504509
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537706190
- Concern: The same slice row ranges as F2 (S3 = 2-6, S4 = 7-16).
- Solution: Nothing further: F2's fix already makes the ranges match the ledger.
- Current-tree verification: Read the slice table.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537769317
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537769317

### R2 - Verify the peer cap through a populated swarm

- PR number: 2828
- Source review ID: 5858504509
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537706217
- Concern: The same concern as F8: one announce response cannot show the peer cap. It proposes populating a swarm with more than 74 peers.
- Solution: Nothing further: F8's fix limits M1 to the interval (the option F8 allowed); the cap default is protected by the S3 unit test.
- Current-tree verification: Read the M1 row.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537769647
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537769647

### F19 - Partly fixed: the spec's `last-updated-utc` still reads 11:30

- PR number: 2828
- Source review ID: 5858813320
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537948631
- Concern: F9's fix left the spec's `last-updated-utc` at 11:30, which matches no commit that touched the file.
- Solution: Stamped it with the commit time of the last commit that changes the spec (12:41).
- Current-tree verification: `git log -1 --format=%cI` for that commit read 2026-11-11T12:41:47Z; the frontmatter reads `2026-11-11 12:41`.
- Resolution reference: `docs(issues): [#2824] stamp the spec last-updated-utc from its last change`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538020220

### F15 - Out of Scope still excludes production behavior changes, but S1 changes `peer.rs`

- PR number: 2828
- Source review ID: 5858813320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537948639
- Concern: Out of Scope excluded every production behavior change while slice S1 fixes F2 in `peer.rs`, and In Scope did not list the fix.
- Solution: Out of Scope now excepts the F2 fix and In Scope lists it.
- Current-tree verification: Read both bullets in the spec.
- Resolution reference: `docs(issues): [#2824] list the S1 fix as the one production change in scope`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538020504

### F14 - The intro says every finding was confirmed with a throwaway test; F4 and F5 were not

- PR number: 2828
- Source review ID: 5858813320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537948648
- Concern: The design review intro said every finding was confirmed with a throwaway test, but the register records code reading for F4 and F5.
- Solution: The intro now says a throwaway test or code reading.
- Current-tree verification: Read the intro against the register's Evidence column.
- Resolution reference: `docs(issues): [#2824] say findings were confirmed by a test or by code reading`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538020836

### F16 - AC1 is `DONE` here while its criterion is unticked and the table is reviewed "after the last slice"

- PR number: 2828
- Source review ID: 5858813320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537948664
- Concern: The verification table marked AC1 DONE while the criterion was unticked and the table is reviewed after the last slice.
- Solution: The AC1 row now reads `TODO`.
- Current-tree verification: Read the AC1 row and the criterion.
- Resolution reference: `docs(issues): [#2824] keep AC1 TODO until the final acceptance review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538021231

### F17 - The audit has no frontmatter, unlike the other recent audits

- PR number: 2828
- Source review ID: 5858813320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537948670
- Concern: The audit opened without the template's frontmatter and skill-link comment.
- Solution: Added both from the template.
- Current-tree verification: Read the first lines of this file.
- Resolution reference: `docs(pr-reviews): add the template frontmatter to the PR 2828 audit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538021657

### F18 - F1's Solution says the fixture lines were classified in the `peer.rs` record, which does not exist yet

- PR number: 2828
- Source review ID: 5858813320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4537948673
- Concern: The audit, the spec and the evidence said the fixture lines were classified in the `peer.rs` record, which does not exist yet.
- Solution: All three now say the S2 `peer.rs` record will classify them (the audit text through the commit `docs(pr-reviews): say the F1 fixture classification is planned, not done`).
- Current-tree verification: `grep -n classified` over the audit, spec and evidence shows no present-tense claim about the fixture lines.
- Resolution reference: `docs(issues): [#2824] say the fixture classification is planned for the S2 record`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538021897

### F20 - This log entry cites a PR-branch commit by its id

- PR number: 2828
- Source review ID: 5859145551
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538201992
- Concern: The 12:37 log entry named the head by a PR-branch commit id, which a rebase would make wrong.
- Solution: Appended a correction naming the head by the commit subject that added the audit; the original entry is left because the log is append-only.
- Current-tree verification: Read the log tail; the correction entry cites no commit id.
- Resolution reference: `docs(pr-reviews): correct the log entry that cites a branch commit id`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538251726

### F21 - The 12:43 entry stamps the push with the commit time; the timeline shows the push at 12:44

- PR number: 2828
- Source review ID: 5859145551
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538201999
- Concern: The 12:43 entry gave the commit time as the push time.
- Solution: Appended a correction giving 12:44 UTC from the timeline (stale-review dismissal at 12:44:56 UTC); the original entry is left because the log is append-only.
- Current-tree verification: The 12:44:56 UTC time is the reviewer-supplied timeline time of the stale-review dismissal; read the log tail.
- Resolution reference: `docs(pr-reviews): correct the log entry that stamps the push with the commit time`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2828#discussion_r4538252095

## Processing Log

- 2026-11-11 12:07 UTC - Started audit. Review 5858450371 (`da2ce7`, changes requested) arrived with 12 inline findings, F1-F12.
- 2026-11-11 12:12 UTC - Review 5858504509 (Copilot, commented) arrived with 3 inline comments: F13, R1 and R2. R1 and R2 repeat F2 and F8.
- 2026-11-11 12:15 UTC - Committed one signed fix per finding group (F1-F13); the last commit is `docs(issues): [#2824] require fix-bug verification evidence in slice S1`.
- 2026-11-11 12:18 UTC - Pushed the fixes and replied on all 15 threads (replies created between 12:18:16 and 12:18:50 UTC).
- 2026-11-11 12:19 UTC - Resolved the 15 threads; a GraphQL refresh at 12:19:37 UTC showed no unresolved thread.
- 2026-11-11 12:37 UTC - Review 5858813320 (`da2ce7`, approved) arrived on head `29dc3421d` with 6 inline findings: F19 (the reviewer's F9 again), F15, F14, F16, F17 and F18.
- 2026-11-11 12:43 UTC - Committed the fixes (the last: `docs(pr-reviews): add the template frontmatter to the PR 2828 audit`) and pushed them.
- 2026-11-11 12:45 UTC - Replied on the 6 threads (12:45:04 to 12:45:15 UTC) and resolved them; a GraphQL refresh showed no unresolved thread.
- 2026-11-11 13:03 UTC - Review 5859145551 (`da2ce7`, approved) arrived with 2 inline findings, F20 and F21, both about two Processing Log entries.
- 2026-11-11 13:05 UTC - Correction (F20): the 12:37 entry named the head by a PR-branch commit id; it means the head that added this audit, `docs(pr-reviews): add the PR 2828 review audit`. The entry is left as written, as the log is append-only.
- 2026-11-11 13:06 UTC - Correction (F21): the 12:43 entry says the fixes were committed and pushed at 12:43. The last of them was committed at 12:43:03 UTC; the push is traced in the GitHub timeline by the stale-review dismissal of review 5858813320 at 12:44:56 UTC, so the push was at 12:44. The entry is left as written, as the log is append-only.
- 2026-11-11 13:08 UTC - Replied on the 2 threads and resolved them (replies created 13:08:18 and 13:08:20 UTC); a GraphQL refresh showed no unresolved thread.
- 2026-11-11 13:28 UTC - Review 5859484596 (`da2ce7`, approved) arrived on the head that moved the round 3 log entry, with 2 inline findings: F22 (the reviewer's F20 again) and F23.
- 2026-11-11 13:30 UTC - Correction (F23): the 13:03 entry, which the commit `docs(pr-reviews): record round 3 of the PR 2828 review` appended after the 13:06 correction and which was pushed, was then moved above the 13:05 and 13:06 entries by the commit `docs(pr-reviews): order the PR 2828 round 3 log entries chronologically`. That move rewrote the pushed log in place. The log is left in the moved order, because restoring the earlier order fails the validator's order check; this entry records the move.

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
