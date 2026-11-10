---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2803 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2803.

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

Two rounds: da2ce7 review 5842604545 and Copilot review 5842654119 (round 1), da2ce7 review 5842998748 (round 2). `F13` is the second round's `F4`, a re-raise of the first round's `F4`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2803-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2803-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2803-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2803-f4 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2803-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2803-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2803-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2803-r1 | Copilot | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| R2 | review-finding:pr-2803-r2 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2803-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2803-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2803-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2803-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2803-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2803-f13 | Human | Minor | metadata | RE_RAISE_OF:F4 | FIXED | RESOLVED |

## Finding Details

### F1 - The scope list is not the list SI-18 deprecated

- PR number: 2803
- Source review ID: 5842604545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525403516
- Concern: The scope bullet and T3 omit deprecated items whose notes say SI-19 removes them.
- Solution: The scope now points to the 27 deprecated attributes and T2 to T4 name the omitted items.
- Current-tree verification: `git grep -n "start_with_health_check\|RunningHttpServer\|RunningApiServer\|RunningUdpServer" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` lists them.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525489444

### F2 - The T1 commit point is a dependency bump but the plan bumps in T4

- PR number: 2803
- Source review ID: 5842604545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525403530
- Concern: T1 and T4 disagree about where the bump lands; a bump committed first would not build.
- Solution: T1 is a wait-only task with no commit and the bump is in T4 and its commit point.
- Current-tree verification: Rows T1 and T4 of the Implementation Plan and Commit Points tables were read after the edit.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525489902

### F3 - Publishing 0.6.0 is both part of the task and out of scope

- PR number: 2803
- Source review ID: 5842604545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525403542
- Concern: The spec calls the publication part of the task, out of scope and a dependency.
- Solution: One wording: an external dependency this task waits for (corrected again by F11).
- Current-tree verification: `git grep -n "external" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` shows one wording.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525490289

### F4 - The last-updated stamp and the first log entry are later than their commits

- PR number: 2803
- Source review ID: 5842604545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525403557
- Concern: The stamps read 14:00 UTC, later than the commits that wrote them, and the second log entry has no time.
- Solution: The stamps use the commit times and every entry has a time (residue: F13).
- Current-tree verification: `git log --format=%cI` was compared with the stamps.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525490676

### F5 - The spec lacks sections the create-issue skill requires

- PR number: 2803
- Source review ID: 5842604545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525403561
- Concern: Acceptance Verification, Implementation Completion Review, a verification policy and the design review are missing.
- Solution: The sections were added to the spec.
- Current-tree verification: `grep -n "^## " docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` lists them.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525491135

### F6 - The renamed evidence file still describes the old gate

- PR number: 2803
- Source review ID: 5842604545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525403567
- Concern: The checklist asks for a support-window end date and omits SI-23 and SI-24.
- Solution: The checklist matches the gate.
- Current-tree verification: The Start-Gate Evidence section of `manual-verification-evidence.md` was read after the edit.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525491562

### F7 - The open gate item cites a working-session label

- PR number: 2803
- Source review ID: 5842604545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525403576
- Concern: The item names a session no reader can resolve.
- Solution: The item cites torrust/torrust-server-lib#5 and PR 6.
- Current-tree verification: `grep -n "server-lib-060" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525492049

### R1 - Keep the dependency bump with the final consumer removals

- PR number: 2803
- Source review ID: 5842654119
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525440797
- Concern: A standalone T1 bump would not build.
- Solution: Same change as F2.
- Current-tree verification: Same as F2.
- Resolution reference: docs(issues): [#2801] address review of the SI-19 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525505000

### R2 - Add the required implementation-completion review

- PR number: 2803
- Source review ID: 5842654119
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525440852
- Concern: The workflow never requires reviewing discoveries or deciding on a retrospective.
- Solution: A checkpoint and a requirement before closing were added.
- Current-tree verification: `grep -n "completion review" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` lists the checkpoint and the section.
- Resolution reference: docs(issues): [#2801] require the implementation completion review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525505408

### F8 - The review was processed without its audit record

- PR number: 2803
- Source review ID: 5842998748
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525702714
- Concern: The head carries no `docs/pr-reviews/pr-2803-review/PR-REVIEW.md`.
- Solution: This record was added.
- Current-tree verification: `validate-audit-record.py --pr-number 2803` was run on it.
- Resolution reference: docs(pr-reviews): [#2803] add the review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525745949

### F9 - The replies cite branch commit ids

- PR number: 2803
- Source review ID: 5842998748
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525702728
- Concern: Nine replies cite short commit ids.
- Solution: The nine replies were edited to cite commit subjects.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/pulls/2803/comments` shows subjects in the replies.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2803#issuecomment-6769290472
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525746397

### F10 - The PR description still describes the old gate

- PR number: 2803
- Source review ID: 5842998748
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525702734
- Concern: The description calls 0.6.0 an open gate item.
- Solution: The description was edited; the conversation response states it.
- Current-tree verification: `gh pr view 2803 --json body` was read after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2803#issuecomment-6769290472
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525746785

### F11 - "Not part of this task" departs from the SI-18 decision

- PR number: 2803
- Source review ID: 5842998748
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525702745
- Concern: SI-18 assigns the publication to SI-19; the spec says the opposite.
- Solution: The gate item and Out of scope say SI-18 assigned it and it was done before implementation.
- Current-tree verification: `grep -n "SI-18 counted\|SI-18 assigned" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` lists both.
- Resolution reference: docs(issues): [#2801] reconcile the SI-19 spec with the second review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525747215

### F12 - SI-18 in-scope list is not the complete list

- PR number: 2803
- Source review ID: 5842998748
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525702753
- Concern: The sentence says the SI-18 list is complete; it is not.
- Solution: The bullet states the 27 deprecated attributes are the complete list.
- Current-tree verification: `grep -n "27 shutdown-related" docs/issues/open/2801-1488-si-19-remove-legacy-shutdown-api/ISSUE.md` finds it.
- Resolution reference: docs(issues): [#2801] reconcile the SI-19 spec with the second review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525747615

### F13 - Two stamps still disagree with git log and the forge

- PR number: 2803
- Source review ID: 5842998748
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525702760
- Concern: `last-updated-utc` and the 13:04 entry disagree with `d8436d5fe` and the PR creation time.
- Solution: The entries and the stamp were corrected.
- Current-tree verification: `git log --format=%cI` and `gh pr view 2803 --json createdAt` were compared with the log.
- Resolution reference: docs(issues): [#2801] reconcile the SI-19 spec with the second review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2803#discussion_r4525747974

## Processing Log

- 2026-11-10 13:27 UTC - Round 1 review of da2ce7 received (seven findings); Copilot review at 13:31 UTC (two comments).
- 2026-11-10 13:36 UTC - Round 1 replies posted and threads resolved; fixes in the commits named above.
- 2026-11-10 13:56 UTC - Round 2 review of da2ce7 received (six findings).
- 2026-11-10 14:00 UTC - Round 2 replies posted (last at 14:00:34 UTC); the audit record was started in this round (F8).

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
