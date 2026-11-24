---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3035 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3035>.

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

Round 1 covers one review of the first head of the PR (the commit `chore(issues): archive closed issue #2924 spec to docs/issues/closed`): review 6044070911 (CodeRabbit, COMMENTED), with one inline finding. Its body has no other actionable assertion.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3035-f1 | Unknown | Minor | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Mark the #2924 row of the EPIC table as DONE

- PR number: 3035
- Source review ID: 6044070911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3035#discussion_r4683849494
- Concern: Row 9 of the EPIC #2392 sub-issue table points to the closed #2924 spec but still says `TODO`; the reviewer asks for `DONE`.
- Solution: Not changed in this PR. The EPIC coordinator session asked for links-only edits to `EPIC.md` in this PR and will set order 9 to DONE and write the log entry in its own spec PR after this one merges, to avoid conflicts. The maintainer chose to decline the finding on that ground.
- Current-tree verification: `grep -n '^| 9 |' docs/issues/open/2392-test-isolation/EPIC.md` shows the row with the closed path and `TODO`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3035#discussion_r4684291894
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3035#discussion_r4684291894

## Processing Log

- 2026-11-24 17:56 UTC - Started audit. Round 1: review 6044070911 (CodeRabbit, COMMENTED) with one finding, F1. Decision for F1: NO_ACTION, because the EPIC status and log belong to the coordinator's own spec PR; the maintainer approved declining it.
- 2026-11-24 17:57 UTC - Replied on the F1 thread and resolved it; the thread refresh shows no unresolved thread. The audit validator exits 0 (1 row, 0 failures). This record is committed after these events and not pushed yet.

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
