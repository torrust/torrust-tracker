---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2452 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2452>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot's inline comments carry no `[Severity]` bracket. Its review overview rates both findings
`Medium severity` in badge markup; as in the PR #2423 audit, they are recorded as
`Minor (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2452-f1` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2452-f2` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - EPIC row stays IN_PROGRESS although the PR closes #2245

- PR number: 2452
- Source review ID: 5428540203
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195414808>
- Concern: The EPIC #2243 subissue row for #2245 would be stale after merge if it stays `IN_PROGRESS`; set it to `DONE` here or change the closure timing.
- Solution: No change, agreed with the maintainer. The `cleanup-completed-issues` skill sets the parent EPIC row to `DONE`, together with the new `docs/issues/closed/` path, in the archive PR that follows the merge. #2246 followed that sequence (PR #2440 left the row `IN_PROGRESS`; archive PR #2442 set it to `DONE`), and the #2440 review confirmed it. Setting `DONE` here would leave the row pointing at an `open/` path.
- Current-tree verification: `EPIC.md` line 74 lists #2245 as `IN_PROGRESS` with its `docs/issues/open/` path.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195538965>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195538965>

### F2 - Spec status stays in-progress although the PR closes #2245

- PR number: 2452
- Source review ID: 5428540203
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195414923>
- Concern: `status: in-progress` will be inconsistent once the PR merges; set it to `done`, or drop `Closes #2245`.
- Solution: No change, agreed with the maintainer. `done` is invalid for a spec under `docs/issues/open/`: the frontmatter validator allows only `planned`, `in-progress`, `blocked`, or `in-review` there. The archive PR sets `status: done` when it moves the spec to `docs/issues/closed/`. `Closes #2245` stays because this PR completes the issue's work.
- Current-tree verification: with `status: done` set temporarily, `frontmatter-validator` reported `lifecycle-location-mismatch` and exited 1; the change was reverted and the tree is clean.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195539245>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195539245>

## Processing Log

- 2026-10-06 12:46 UTC - Copilot review 5428540203 submitted findings F1 and F2.
- 2026-10-06 12:58 UTC - The maintainer agreed to decline F1 and F2 as conflicting with the archival lifecycle. Replied to both threads with the evidence and started this audit. No human review had been submitted at this time.

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
