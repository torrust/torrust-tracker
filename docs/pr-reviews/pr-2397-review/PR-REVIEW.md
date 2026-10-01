---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
---

<!-- skill-link: process-pr-review -->

# PR #2397 Review Audit

Source: pull-request review `5368546961` and its inline review threads for
https://github.com/torrust/torrust-tracker/pull/2397.

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

The review overview rates F1 `Medium severity` and F2 `Low severity` in its badge markup; as in
the PR #2363 audit, they are recorded as `Minor (inferred)` and `Nit (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2397-f1` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2397-f2` | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Replace the `issue #1488` related-artifact entry with a URL

- PR number: 2397
- Source review ID: 5368546961
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2397#discussion_r4146455411>
- Concern: In the #1419 specification, `"issue #1488"` in `semantic-links.related-artifacts` is
  not an artifact path or stable link target; the reviewer suggested the GitHub issue URL.
- Solution: no change. `issue #<n>` is the canonical v1 issue reference. The frontmatter
  validator (`syntax.rs`, `is_related_artifact`) accepts only repository paths, `issue #<n>`, and
  `review-finding:pr-<n>-<id>`, and rejects URLs; this PR first used the #1488 URL and the
  validator rejected it.
- Current-tree verification: `cargo run -q --package frontmatter-validator --bin
  frontmatter-validator -- docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md`
  exits 0 at the PR head that contains `docs(issues): [#2393] quote the evidence timestamp`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2397#discussion_r4149501237>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2397#discussion_r4149501237>

### F2 - Quote the evidence timestamp

- PR number: 2397
- Source review ID: 5368546961
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2397#discussion_r4146455498>
- Concern: The #2393 manual verification evidence left `last-updated-utc` unquoted, unlike the new
  issue specifications; the reviewer also suggested `schema-version` if the validator expects it.
- Solution: quoted `last-updated-utc`. Did not add `schema-version`: manual verification evidence
  has no v1 frontmatter profile, and its repository template has none.
- Current-tree verification: the evidence frontmatter reads `last-updated-utc: "2026-09-30 15:57"`;
  the pre-commit frontmatter check passed for the fix commit.
- Resolution reference: docs(issues): [#2393] quote the evidence timestamp
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2397#discussion_r4149501421>

## Processing Log

- 2026-09-30 15:59 UTC - Started audit for Copilot review 5368546961 (submitted 15:39 UTC, Lite
  effort after an agentic-review timeout): two inline threads, no reviewer finding IDs, and a
  review body with no independently actionable assertion. Normalized as F1 and F2.
- 2026-09-30 15:59 UTC - Fixed F2 in `docs(issues): [#2393] quote the evidence timestamp`, pushed
  it, re-verified F1 with the frontmatter validator, and replied on both threads. F1 is an original
  no-change finding, so its reply names itself, as in the PR #2334 audit.

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
