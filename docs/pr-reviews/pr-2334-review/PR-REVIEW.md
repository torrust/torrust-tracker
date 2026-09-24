---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2333-2278-fetch-all-review-threads/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2334 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2334>.

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
| F1 | `review-finding:pr-2334-f1` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2334-f2` | Copilot | Minor (inferred) | link-integrity | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - EPIC order-4 row points to a spec path the reviewer did not see in the diff

- PR number: 2334
- Source review ID: 5307261542
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2334#discussion_r4095982410>
- Concern: the EPIC links order 4 to `docs/issues/open/2333-2278-fetch-all-review-threads/ISSUE.md`,
  but the reviewer read the spec as still living under
  `docs/issues/drafts/2278-fetch-all-review-threads/ISSUE.md`.
- Solution: no change. The PR renames the spec to the `open/` path, so the EPIC reference resolves.
- Current-tree verification: at PR head `5b35f7c8` (equal to local `HEAD`),
  `git diff --name-status -M torrust/develop...HEAD` reports `R076` from the drafts path to
  `docs/issues/open/2333-2278-fetch-all-review-threads/ISSUE.md`;
  `gh api repos/torrust/torrust-tracker/pulls/2334/files` reports that file as `renamed` with the
  drafts path as `previous_filename`; `git ls-tree -r --name-only HEAD docs/issues` lists only the
  `open/` path.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2334#discussion_r4096272963>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2334#discussion_r4096272963>

### F2 - EPIC progress-log entry cites the same moved spec path

- PR number: 2334
- Source review ID: 5307261542
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2334#discussion_r4095982512>
- Concern: the same comment as F1, anchored on the 2026-09-24 16:07 UTC EPIC progress-log entry
  that records the move to `docs/issues/open/2333-2278-fetch-all-review-threads/`.
- Solution: no change; it requests the same current-tree change as F1, and the log entry is
  accurate for the reason recorded there.
- Current-tree verification: the F1 commands, run at the same head `5b35f7c8`; the log entry names
  the directory the rename targets.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2334#discussion_r4096273174>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2334#discussion_r4096273174>

## Processing Log

- 2026-09-24 17:01 UTC - Started audit for Copilot review 5307261542 (submitted 16:30 UTC, Lite
  effort after an agentic-review timeout): two inline threads, no reviewer finding IDs, and a
  review body with no independently actionable assertion. Normalized as F1 and its re-raise F2.
- 2026-09-24 17:01 UTC - Re-derived both claims at PR head `5b35f7c8` and replied on both threads
  with the prescribed `Superseded by F1:` form before resolution. F1 is an original no-change
  finding, so its reply names itself; the rule's `<FindingId>` has no other referent for that case.

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
