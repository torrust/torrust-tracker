---
pr-number: 2820
pr-url: https://github.com/torrust/torrust-tracker/pull/2820
last-updated-utc: "2026-11-11 10:03"
---

# PR #2820 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2820.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: da2ce7 review 5856962816 (CHANGES_REQUESTED) on the head `docs(issues): [#2762] close out the package-coverage-check issue` (seven inline findings F1 to F7, one Major, four Minor, two Nit): all valid and fixed in the commit `docs(issues): [#2762] address PR 2820 review round 1`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2820-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2820-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2820-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2820-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2820-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2820-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2820-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - F24 to F26 of the PR 2810 audit name no follow-up PR, so the audit fails its validator after the merge.

- PR number: 2820
- Source review ID: 5856962816
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536470837
- Concern: F24 to F26 of the PR 2810 audit name no follow-up PR, so the audit fails its validator after the merge.
- Solution: Valid. The Follow-up PR URL of the three entries is PR 2820 and the round-5 sentence names it.
- Current-tree verification: grep -c "pull/2820" in the PR 2810 audit prints 4.
- Resolution reference: docs(issues): [#2762] address PR 2820 review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536484660

### F2 - F24 to F26 repeat the defect F24 records: FIXED and RESOLVED on replies that defer the fix.

- PR number: 2820
- Source review ID: 5856962816
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536470856
- Concern: F24 to F26 repeat the defect F24 records: FIXED and RESOLVED on replies that defer the fix.
- Solution: Valid. The rows are FOLLOW_UP and OPEN until PR 2820 merges; the F24 Solution states the real reply and commit times.
- Current-tree verification: PR 2810 audit, F24 to F26 rows and the F24 Solution.
- Resolution reference: docs(issues): [#2762] address PR 2820 review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536484288

### F3 - The Ownership line keeps Post-merge workflow approval N/A though round 5 was unaudited at the merge.

- PR number: 2820
- Source review ID: 5856962816
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536470867
- Concern: The Ownership line keeps Post-merge workflow approval N/A though round 5 was unaudited at the merge.
- Solution: Valid. The line says the request was given in the session chat and no GitHub comment URL exists; no URL was invented.
- Current-tree verification: PR 2810 audit Ownership section.
- Resolution reference: docs(issues): [#2762] address PR 2820 review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536482720

### F4 - The checkpoint "Reviewer validated acceptance criteria" is ticked though no reviewer edited the spec.

- PR number: 2820
- Source review ID: 5856962816
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536470882
- Concern: The checkpoint "Reviewer validated acceptance criteria" is ticked though no reviewer edited the spec.
- Solution: Valid. Unticked and the progress log says why.
- Current-tree verification: Closed spec Workflow Checkpoints and progress log.
- Resolution reference: docs(issues): [#2762] address PR 2820 review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536485100

### F5 - The retrospective summary still carries round-4 figures that round 5 made wrong.

- PR number: 2820
- Source review ID: 5856962816
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536470894
- Concern: The retrospective summary still carries round-4 figures that round 5 made wrong.
- Solution: Valid. The last-reply range, the rounds figure and the pattern lists are corrected.
- Current-tree verification: Retrospective Review Summary and Patterns.
- Resolution reference: docs(issues): [#2762] address PR 2820 review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536483426

### F6 - The PR title carries [#2762] for an already closed issue and the description states no validation.

- PR number: 2820
- Source review ID: 5856962816
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536470906
- Concern: The PR title carries [#2762] for an already closed issue and the description states no validation.
- Solution: Valid. The title has no issue tag, the description says Related to #2762 and lists the validation.
- Current-tree verification: PR 2820 title and description.
- Resolution reference: docs(issues): [#2762] address PR 2820 review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536482974

### F7 - The reason no implementation retrospective was written is not in the progress log.

- PR number: 2820
- Source review ID: 5856962816
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536470913
- Concern: The reason no implementation retrospective was written is not in the progress log.
- Solution: Valid. The progress log gives it.
- Current-tree verification: Closed spec progress log.
- Resolution reference: docs(issues): [#2762] address PR 2820 review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2820#discussion_r4536483810

## Processing Log

- 2026-11-11 10:03 UTC - Started audit; round 1 (da2ce7 review 5856962816, F1 to F7) processed: all seven findings fixed in one commit and replied (replies created 10:01:22Z to 10:01:35Z, before that commit). The threads are resolved after the push; their resolution times are not captured.

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
