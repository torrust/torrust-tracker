---
pr-number: 2771
pr-url: https://github.com/torrust/torrust-tracker/pull/2771
last-updated-utc: "2026-11-06 18:57"
---

# PR #2771 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2771.

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

One row per independent concern from Copilot's review of this pull request (round 1: review 5804134932).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2771-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2771-f2 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - PR 2748 called both a PR and the ADR in the EPIC

- PR number: 2771
- Source review ID: 5804134932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2771#discussion_r4495442325
- Concern: The EPIC refers to #2748 as a PR in one place and as "the ADR" in another.
- Solution: Valid. The reference line now says #2748 is the merged PR that added the two ADRs; the risk line that still said it was not merged is corrected.
- Current-tree verification: `grep -n 2748 docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` shows only PR wording.
- Resolution reference: docs(issues): [#2758] describe PR 2748 consistently in the EPIC
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2771#discussion_r4496337356

### F2 - Inconsistent path formatting in a file list

- PR number: 2771
- Source review ID: 5804134932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2771#discussion_r4495442418
- Concern: A path in the "Files this issue may touch" list of the daemon logging spec lacked backticks, unlike the rest of the list.
- Solution: Valid, and the same defect was in three more specs (#2763, #2764, #2765). All four lists now wrap every path in backticks, with any note in brackets outside the code span.
- Current-tree verification: Read the "Files this issue may touch" block of each `docs/issues/open/27*-2758-*/ISSUE.md`; every path is in backticks.
- Resolution reference: docs(issues): [#2758] format the file lists of the child specs consistently
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2771#discussion_r4496337713

## Processing Log

- 2026-11-06 18:57 UTC - Started audit; round 1 (Copilot review 5804134932) processed: F1 and F2 fixed in the two commits cited above, pushed and replied. The replies were posted before the push had completed (it failed once on an SSH disconnect); the push succeeded afterwards and the threads are resolved only after it.

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
