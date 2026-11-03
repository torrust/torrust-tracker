---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2698 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2698.

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
| F1 | review-finding:pr-2698-f1 | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2698-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Use a stable reference instead of a path that moves with the specification

- PR number: 2698
- Source review ID: 5756570435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2698#discussion_r4458088968
- Concern: The related-artifacts entry in the closed #2450 specification names the SI-21 specification by its `open/` path, which changes again when SI-21 moves to `closed/`. The review overview marked it Medium severity; the inline comment carries no severity bracket, so the severity is inferred.
- Solution: Replaced the path entry with `"issue #2697"`. The frontmatter validator accepts `issue #N` as a related-artifacts form and requires path entries to name a tracked file, so a path entry breaks at each move while the issue reference does not.
- Current-tree verification: `docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md` lists `"issue #2697"` in `semantic-links.related-artifacts`, and `cargo run -q --package frontmatter-validator --bin frontmatter-validator -- <that file>` exits 0.
- Resolution reference: docs(issues): [#2450] reference the SI-21 spec by issue number instead of its path
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2698#discussion_r4458482905

### F2 - Reconcile the stale "not yet approved" statement in the SI-21 specification

- PR number: 2698
- Source review ID: 5756570435
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2698#discussion_r4458089042
- Concern: The Design Decision section said the maintainer had not yet approved creating the issue from that text, while the checkpoints and the progress log record issue #2697 and PR #2698. The inline comment carries no severity bracket, so the severity is inferred.
- Solution: Reworded the sentence to say the maintainer approved creating the issue from this text on 2026-11-03, the date of his approval in the review chat. The spec's stamp and progress log were updated in the same commit.
- Current-tree verification: `docs/issues/open/2697-1488-si-21-mark-health-unhealthy-during-shutdown/ISSUE.md` now says "the maintainer approved creating the issue from this text on the same day". A search of that file for "not yet", "has not", "to be decided" and "draft" finds no other stale approval or decision statement; the remaining matches are the historical draft path, two log entries, the "done during drafting" note of scenario M1 and the retrospective placeholder.
- Resolution reference: docs(issues): [#2697] correct the stale approval statement in the SI-21 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2698#discussion_r4458483263

## Processing Log

- 2026-11-03 17:53 UTC - Started the audit of Copilot review 5756570435 (submitted 17:17 UTC) with findings F1 and F2, and replied on both threads after the two fix commits were pushed.
- 2026-11-03 17:54 UTC - Resolved both threads; the GraphQL thread data then showed both as resolved.

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
