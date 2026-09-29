---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2375"
---

<!-- skill-link: process-pr-review -->

# PR #2377 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2377>.

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

Audit IDs `F4` and `F5` are the reviewer's IDs from Copilot review 5355913266 (round 1, at the
head `docs(issues): add issue specification for #2375`). No earlier finding exists, so no ID
collides. Each inline comment carries a `[Minor]` bracket. The overview also names an ambiguous
slug concern and an exclusion-of-closed-specs concern, but no full finding text or source thread
is retrievable for either; per the review-processing workflow, both remain non-actionable context.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F4 | `review-finding:pr-2377-f4` | Copilot | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2377-f5` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F4 - Long-lived relationships to #2003 and #2264 used movable issue-spec paths

- PR number: 2377
- Source review ID: 5355913266
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2377#discussion_r4136217682>
- Concern: The `related-artifacts` paths for the #2003 and #2264 EPIC specs will change when
  their issues close. The semantic-link convention requires a stable issue reference for durable
  issue-spec relationships.
- Solution: `docs(issues): use stable references in issue #2375` replaces those two paths with
  `"issue #2003"` and `"issue #2264"`. Paths to the issue-directory READMEs remain because they
  are stable documentation paths, not issue-spec relationships.
- Current-tree verification: `docs/skills/semantic-skill-link-convention.md` says durable
  issue-spec relationships should use `issue #NNNN`. `frontmatter-validator`
  `docs/issues/open/2375-2003-unambiguous-issue-spec-names/ISSUE.md` exits `0`, and no
  `docs/issues/open/2003...` or `docs/issues/open/2264...` reference remains in that spec.
- Resolution reference: `docs(issues): use stable references in issue #2375`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2377#discussion_r4136331087>

### F5 - The EPIC ownership metadata was stale

- PR number: 2377
- Source review ID: 5355913266
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2377#discussion_r4136217591>
- Concern: The #2003 issue records `da2ce7` as its owner, but the refreshed EPIC frontmatter still
  named `josecelano`.
- Solution: `docs(issues): align EPIC #2003 owner metadata` changes `epic-owner` to `da2ce7`.
- Current-tree verification: GitHub issue #2003 lists `da2ce7` as its assignee.
  `frontmatter-validator docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` exits
  `0`.
- Resolution reference: `docs(issues): align EPIC #2003 owner metadata`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2377#discussion_r4136331401>

## Processing Log

- 2026-09-29 17:13 UTC - Committed the F5 fix.
- 2026-09-29 17:17 UTC - Committed the F4 fix.
- 2026-09-29 17:19 UTC - Replied on the F4 and F5 threads; `reply-status` reported 2 of 2
  threads replied, and both were resolved.
- 2026-09-29 17:22 UTC - Started this audit; a GraphQL refresh shows 2 of 2 threads resolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push. For a
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
