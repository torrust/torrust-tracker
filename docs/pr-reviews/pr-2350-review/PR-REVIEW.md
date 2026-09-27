---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2349-2278-contract-checker-evidence-boundary/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2350 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2350>.

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

Audit IDs `F1`-`F2` are the two Copilot review 5326193009 threads, which carry no finding IDs.
Copilot rated both "Medium", which is not in the severity vocabulary, so they are recorded as
`Minor (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2350-f1` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2350-f2` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Frontmatter `skill-links` and inline markers disagree

- PR number: 2350
- Source review ID: 5326193009
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2350#discussion_r4111640894>
- Concern: `skill-links` lists `process-pr-review` but the body carries only the `create-issue`
  inline marker; either add the marker, or drop unused skills, and consider listing
  `write-unit-test`, which the spec uses.
- Solution: added `write-unit-test` to `skill-links`, because T3 uses it. No inline
  `process-pr-review` marker was added, because the semantic skill-link convention makes
  frontmatter `skill-links` canonical and top-of-file inline markers redundant for Markdown files
  that have it; the merged #2333, #2318, and #2347 specs follow the same pattern.
- Current-tree verification: `sed -n '/skill-links:/,/related-artifacts:/p' docs/issues/open/2349-2278-contract-checker-evidence-boundary/ISSUE.md`
  lists `create-issue`, `process-pr-review`, and `write-unit-test`; `grep -n "redundant and need" docs/skills/semantic-skill-link-convention.md`
  matches the two convention sentences stating that inline markers are redundant.
- Resolution reference: `docs(issues): link write-unit-test and qualify audit paths in the #2349 spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2350#discussion_r4114323878>

### F2 - The References audit paths are not consistently qualified

- PR number: 2350
- Source review ID: 5326193009
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2350#discussion_r4111640912>
- Concern: in "Audits citing the checker", only the first path carries the `docs/pr-reviews/`
  prefix.
- Solution: all three references are now repository-root-relative.
- Current-tree verification: `grep -o "docs/pr-reviews/pr-2[23][0-9][0-9]-review/" docs/issues/open/2349-2278-contract-checker-evidence-boundary/ISSUE.md | wc -l`
  counts 3 qualified references, and `test -d` succeeds for each of the three directories.
- Resolution reference: `docs(issues): link write-unit-test and qualify audit paths in the #2349 spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2350#discussion_r4114323926>

## Processing Log

- 2026-09-27 06:24 UTC - Started audit for round 1: Copilot review 5326193009 (submitted
  2026-09-26 14:21 UTC), two inline threads and a summary with no further request. The findings
  were processed after rebasing the branch onto `develop` `e8b9c1a1` (PR #2348), which resolved one
  EPIC #2278 progress-log conflict by keeping both entries in order. Committed one fix commit for
  both findings, pushed, and replied on both threads.
- 2026-09-27 06:31 UTC - After `docs(pr-reviews): add PR #2350 review audit` was pushed,
  `reply-status --login josecelano` reported 2 of 2 threads replied, and both threads were
  resolved. A refreshed GraphQL fetch reports 2 threads, 0 unresolved.

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
