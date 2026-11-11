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

Rows F1-F12 come from review 5858450371 (`da2ce7`). Rows F13, R1 and R2 come from review 5858504509 (Copilot); R1 and R2 re-raise F2 and F8.

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

## Processing Log

- 2026-11-11 12:07 UTC - Started audit. Review 5858450371 (`da2ce7`, changes requested) arrived with 12 inline findings, F1-F12.
- 2026-11-11 12:12 UTC - Review 5858504509 (Copilot, commented) arrived with 3 inline comments: F13, R1 and R2. R1 and R2 repeat F2 and F8.
- 2026-11-11 12:15 UTC - Committed one signed fix per finding group (F1-F13); the last commit is `docs(issues): [#2824] require fix-bug verification evidence in slice S1`.
- 2026-11-11 12:18 UTC - Pushed the fixes and replied on all 15 threads (replies created between 12:18:16 and 12:18:50 UTC).
- 2026-11-11 12:19 UTC - Resolved the 15 threads; a GraphQL refresh at 12:19:37 UTC showed no unresolved thread.

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
