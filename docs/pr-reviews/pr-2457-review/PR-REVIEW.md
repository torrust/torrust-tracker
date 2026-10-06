---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2457 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2457>.

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
| F1 | review-finding:pr-2457-f1 | Human | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2457-f2 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The archived spec's parent link no longer resolves

- PR number: 2457
- Source review ID: 5431964864
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2457#discussion_r4198213591>
- Concern: the archive move turned `ISSUE.md`'s sibling link to EPIC #2411 into a path under
  `docs/issues/closed/`, where the EPIC does not exist. No gate reports it, because lychee
  excludes `docs/issues/closed/`.
- Solution: pointed the link at `../../open/2411-spam-and-abuse-resistance/EPIC.md`, the form the
  archived #2412 and #2370 specs use for a still-open parent. Refreshed the spec's stamp with a
  matching Progress Log entry.
- Current-tree verification: the link target exists; a search for `../<folder>` links in the five
  moved files finds no other sibling link.
- Resolution reference: fix(issues): [#2417] repair the archived spec's parent EPIC link
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2457#discussion_r4198848470>

### F2 - Newly frontmattered records omit required archive metadata

- PR number: 2457
- Source review ID: 5431603122
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2457#pullrequestreview-5431603122>
- Concern: Copilot's overview, which lists no findings, says the two supplementary records that
  gained frontmatter in this PR omit metadata that the archival workflow requires.
- Solution: added `spec-path` (each record's own path) and `last-updated-utc` to
  `implementation-retrospective.md` and `prose-first-test-review.md`. That fix's commit body calls
  this point "review finding F1"; the audit renumbers it F2, because the reviewer-provided F1
  belongs to review 5431964864. That review confirms the fix under "Checked, no finding".
- Current-tree verification: both records carry `spec-path`, `last-updated-utc` and
  `semantic-links`; pre-commit passed.
- Resolution reference: chore(issues): [#2417] complete the frontmatter of the newly frontmattered records
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-10-06 18:10 UTC - Started audit. Recorded F2 (Copilot review 5431603122), which was fixed
  and pushed with the rebase onto `develop` before review 5431964864. Recorded F1 (review
  5431964864), which was fixed, pushed, replied to, and resolved before this entry. Refreshed
  review threads with GraphQL: no unresolved thread remains.

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
