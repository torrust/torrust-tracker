---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2339-review/PR-REVIEW.md
---

<!-- skill-link: process-pr-review -->

# PR #2353 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2353>.

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

Audit IDs `F1`-`F3` are the three Copilot review 5329114076 threads, which carry no finding IDs.
All three raise the same concern on different lines, so F2 and F3 are re-raises of F1. Copilot
rated them "Medium", which is not in the severity vocabulary, so they are recorded as
`Minor (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2353-f1` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2353-f2` | Copilot | Minor (inferred) | link-integrity | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2353-f3` | Copilot | Minor (inferred) | link-integrity | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Commit-subject resolution references are not permalinks

- PR number: 2353
- Source review ID: 5329114076
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114326490>
- Concern: the #2339 audit's F1 resolution reference is a commit subject only; a commit permalink
  with a SHA, or a PR link plus SHA, was suggested as more stable and navigable.
- Solution: no change. `process-pr-review` requires `FIXED` resolution references to be unique
  Conventional Commit subjects and forbids branch SHAs, and the EPIC #2278 Decision Record keeps
  that form, so references survive rebases. The row already links the merged PR through
  `Follow-up PR URL`.
- Current-tree verification: `git log --no-merges --all --fixed-strings --grep="test(dev-tools): pin review-thread query fields and explicit nulls" --format=%h | wc -l`
  prints `1`; `grep -n "resolution references are unique" .github/skills/dev/pr-reviews/process-pr-review/SKILL.md`
  matches the rule; `grep -c "Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2344>" docs/pr-reviews/pr-2339-review/PR-REVIEW.md`
  prints `3`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114637819>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114637819>

### F2 - The same concern on the #2339 audit's F2 row

- PR number: 2353
- Source review ID: 5329114076
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114326510>
- Concern: the same request as F1, anchored on the F2 resolution reference.
- Solution: no change, for the reason recorded in F1.
- Current-tree verification: the F1 commands; F2 cites the same unique subject as F1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114637859>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114637859>

### F3 - The same concern on the #2339 audit's F3 row

- PR number: 2353
- Source review ID: 5329114076
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114326525>
- Concern: the same request as F1, anchored on the F3 resolution reference.
- Solution: no change, for the reason recorded in F1.
- Current-tree verification: `git log --no-merges --all --fixed-strings --grep="docs(issues): record the exact #2333 evidence commands" --format=%h | wc -l`
  prints `1`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114637895>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2353#discussion_r4114637895>

## Processing Log

- 2026-09-27 08:23 UTC - Started audit for round 1: Copilot review 5329114076 (submitted
  06:23 UTC), three inline threads with one concern. Rebased the branch onto `develop` `6a7f1db5`
  (PR #2350) first; the rebase was clean. Verified the no-change dispositions at that tree and
  replied on all three threads with the prescribed `Superseded by F1:` form.
- 2026-09-27 08:30 UTC - After `docs(pr-reviews): add PR #2353 review audit` was pushed,
  `reply-status --login josecelano` reported 3 of 3 threads replied, and the three threads were
  resolved. A refreshed GraphQL fetch reports 3 threads, 0 unresolved.

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
