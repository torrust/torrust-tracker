---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2003"
---

<!-- skill-link: process-pr-review -->

# PR #2437 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2437>.

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

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2437-f1` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Parent line describes a pending placement step

- PR number: 2437
- Source review ID: 5411959049
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2437#discussion_r4182128994>
- Concern: The parent line's second sentence said the EPIC owner would accept or reject the placement in this pull request, which goes stale once the placement is accepted; the Progress Log already carries that history.
- Solution: The parent line states the placement as a dated fact, in the suggested wording and the form of the other `2003-` drafts, and a Progress Log entry records the EPIC owner's acceptance.
- Current-tree verification: the parent line below the title of `docs/issues/drafts/2003-adopt-prose-tests-as-executable-specification/ISSUE.md` inspected; `linter markdown` and `linter cspell` passed.
- Resolution reference: `docs(issues): state the prose-tests draft's EPIC #2003 placement as a dated fact`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2437#discussion_r4183792335>

## Processing Log

- 2026-10-05 08:37 UTC - Copilot review 5411943384 posted an overview without inline findings. Its remarks (an unstable spec path for the parent relationship and no future subissue-linking checkpoints) are summary context, not rows: the EPIC path in `related-artifacts` follows the other `2003-` drafts and is repaired by `cleanup-completed-issues` when the EPIC moves, and subissue linking happens when the GitHub issue is created.
- 2026-10-05 08:38 UTC - Human review 5411959049 (da2ce7, round 1) accepted the EPIC #2003 placement and approved with one Nit, F1. Its two follow-up items are the EPIC owner's own work on PR #2366.
- 2026-10-05 12:07 UTC - Fixed F1, pushed after the pre-push suite passed, and replied on its thread before recording it here.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
