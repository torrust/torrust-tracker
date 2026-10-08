---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2488 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2488>.

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

Copilot review 5456474248 raised one inline finding without a reviewer finding ID; it is recorded
as F1. The review body only summarizes that same finding and the PR changes, so it has no row of
its own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2488-f1` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ADR keeps dangling references to the deleted `.github/prompts/` directory

- PR number: 2488
- Source review ID: 5456474248
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4218839155>
- Concern: Deleting the last prompt files removes the `.github/prompts/` directory, but the AI agent governance ADR still listed it in its `semantic-links.related-artifacts` frontmatter and as "Prompt adapters" in its References section.
- Solution: Removed both references from the ADR. The dated 2026-08-21 review-log row that mentions inspecting "prompts" was left unchanged because it records historical state. Two closed issue documents that name the deleted prompt file are historical records and were also left unchanged.
- Current-tree verification: `git ls-files .github/prompts` prints nothing; a repo-wide search for `.github/prompts` matches only `docs/issues/closed/2233-2003-tune-unified-pr-review-process/code-span-path-case-inventory.tsv` and `docs/issues/closed/2295-2278-single-source-audit-roster/ISSUE.md`; the pre-commit script passes all nine steps.
- Resolution reference: `docs(adrs): drop dangling .github/prompts references`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2488#discussion_r4218893132>

## Processing Log

- 2026-10-08 12:26 UTC - Fetched review 5456474248 and its single inline thread with GraphQL; recorded F1.
- 2026-10-08 12:27 UTC - Committed and pushed `docs(adrs): drop dangling .github/prompts references`, then replied on the F1 thread.
- 2026-10-08 12:28 UTC - Created this audit record.

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
