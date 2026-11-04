---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2718 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2718>.

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

Round 1 is the Copilot review `5769555015` (submitted 2026-11-04 13:58 UTC) with one inline comment. It carries no
`[Severity][Id]` prefix, so the finding ID is audit-local and the severity is inferred. The testing workflow was green at
the time, so no CI finding exists.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2718-f1 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Match the parsed format instead of its Debug text

- PR number: 2718
- Source review ID: 5769555015
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2718#discussion_r4468429721
- Concern: The test of the stats `format` query parameter compared the `Debug` string of the parsed value (`Some(Json)`), which depends on debug formatting and variant names and is not a stable contract; the reviewer suggested matching the variant.
- Solution: `Format` has no `PartialEq`, and no production derive was added for a test, so an exhaustive `match` maps the parsed `Option<&Format>` to a small test-local `Asked` enum that the three cases compare; a new `Format` variant now breaks the build there.
- Current-tree verification: Removing `rename_all = "lowercase"` and changing it to `UPPERCASE` each still fail the format test, the new function passes clippy (a first version taking the value by value was changed to a reference), and the 156 unit and 58 integration tests pass.
- Resolution reference: test(axum-rest-api-server): [#2637] match the parsed stats format instead of its Debug text
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2718#discussion_r4468635652

## Processing Log

- 2026-11-04 13:58 UTC - Copilot review `5769555015` submitted with one inline finding. No human review existed at this point.
- 2026-11-04 14:16 UTC - Fixed F1 in one signed commit and pushed the branch (the commit subject, not an id, is cited above), replied to the thread (14:16:16 UTC) and resolved it (14:16:18 UTC); the resolve mutation returned `isResolved: true` and a refreshed fetch lists no unresolved thread.

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
