---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2278"
---

<!-- skill-link: process-pr-review -->

# PR #2494 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2494>.

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
| F1 | `review-finding:pr-2494-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2494-f2` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The EPIC's scope text omits the PR #2484 retrospective

- PR number: 2494
- Source review ID: 5460432014
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2494#discussion_r4222080328>
- Concern: The new progress-log entry makes the PR #2484 retrospective a decision source, but the Decision Record said the matrix maps "the three retrospectives", AC1 required only PRs #2270 to #2272, and References listed only those three.
- Solution: The Decision Record names "the PR review retrospectives its introduction lists", so it stays true when another arrives. AC1, its Acceptance Verification row and the References entry cover PR #2484, with a note that its retrospective reaches `develop` when PR #2484 merges, and a progress-log entry records the change.
- Current-tree verification: in `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md`, the Decision Record reads "maps each proposal from the PR review retrospectives its", AC1 reads "(PR #2270, #2271, #2272, and #2484)", and the References entry "Source retrospectives" names PR #2484; the phrase "three retrospectives" occurs in neither the EPIC nor the matrix.
- Resolution reference: `docs(issues): [#2278] cover the PR #2484 retrospective in the EPIC's scope and qualify the path sweep`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2494#discussion_r4222208629

### F2 - The pre-push sweep's path rule also rejects historical references

- PR number: 2494
- Source review ID: 5460432014
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2494#discussion_r4222080255>
- Concern: The sweep failed any tracked file naming a path the push deletes or renames, which also matches references that record history on purpose; that conflicts with the adjacent anchoring row and with EPIC #2264's distinction between current and historical paths.
- Solution: The matrix row fails the sweep only when a tracked file still names the path as a current reference (a live link, a `related-artifacts` entry, a verification claim about the current tree) and allows references marked historical (a parenthesis naming the head they held at, a dated log entry, prose about history); its rationale now cites EPIC #2264's distinction. The EPIC's row-5 note names the sweep without restating the path rule, so it needed no change.
- Current-tree verification: in `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md`, the PR #2484 sweep row under "Author Verification and Convergence" reads "as a current reference" and "while references marked historical"; `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md` reads "they may represent current files, historical files, examples" and "historical records can be valuable precisely because they preserve the theory and context".
- Resolution reference: `docs(issues): [#2278] cover the PR #2484 retrospective in the EPIC's scope and qualify the path sweep`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2494#discussion_r4222209059

## Processing Log

- 2026-10-08 17:22 UTC - Review 5460432014 posted two Minor findings, F1 and F2, each on its own inline thread.
- 2026-10-08 17:29 UTC - Fixed F1 and F2 in one commit, because both correct this pull request's record of the PR #2484 rows; the replies are drafted for posting after the push.

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
