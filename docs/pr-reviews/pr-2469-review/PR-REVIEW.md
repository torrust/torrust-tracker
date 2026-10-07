---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md
---

<!-- skill-link: process-pr-review -->

# PR #2469 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2469>.

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
| F1 | review-finding:pr-2469-f1 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2469-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Name the release the deferral waits for

- PR number: 2469
- Source review ID: 5441331718
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2469#discussion_r4206087772>
- Concern: the undated sub-issue row said "deferred until after the next major release", which
  shifts meaning once that release ships; the tree does not settle which version is meant.
- Solution: the row names v4.0.0. `v3.0.0` is tagged and #1659 tracks the v4.0.0 release, so the
  workspace `3.0.0-develop` version has not been bumped yet. A Progress Log entry records the change.
- Current-tree verification: `git tag -l v3.0.0` lists the tag; the EPIC row reads "deferred until
  after v4.0.0" and `last-updated-utc` equals the new log entry.
- Resolution reference: docs(issues): [#2411] name v4.0.0 as the rate-limiting deferral's release
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2469#discussion_r4207265787>

### F2 - "for its effort", as the source says

- PR number: 2469
- Source review ID: 5441331718
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2469#discussion_r4206087783>
- Concern: "for the effort (A1)" could be read as the EPIC's effort or A1's, and differed from
  the approval's wording used elsewhere in the PR.
- Solution: the row reads "Highest-impact mitigation for its effort (A1)".
- Current-tree verification: the EPIC row matches the Progress Log entry, commit body and PR
  description.
- Resolution reference: docs(issues): [#2411] say "for its effort" in the rate-limiting row
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2469#discussion_r4207266220>

## Processing Log

- 2026-10-07 13:08 UTC - Started audit. Recorded F1 and F2 from da2ce7 review 5441331718
  (APPROVED, two inline findings; Copilot review 5440912489 has no findings). Before this entry,
  the branch was rebased onto the latest `develop`, both fixes were committed and pushed, and both
  threads were replied to and resolved. Refreshed review threads with GraphQL: no unresolved thread
  remains.

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
