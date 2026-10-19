---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2554 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2554>.

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

Copilot's review 5550225463 (COMMENTED, 2026-10-16 15:50:46 UTC) is an overview with 0 findings, so it adds no row. da2ce7's review 5562041237 (APPROVED, 2026-10-18 08:03:48 UTC, at the pushed head) carries two inline `Nit` findings, F1 and F2. Its body raises nothing else; its "Checked, no finding" list is not a request. Both are fixed before merge and answered in one consolidated PR conversation response, <https://github.com/torrust/torrust-tracker/pull/2554#issuecomment-6264569468>.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2554-f1` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2554-f2` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - "Independent reviewer reports" is ticked, but no report exists

- PR number: 2554
- Source review ID: 5562041237
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2554#discussion_r4306271909>
- Concern: The archive commit ticked the "Independent reviewer reports" checkpoint, but the folder has no `agent-review-reports.md` and no review on #2545 wrote one. A ticked box reads as "reports recorded", and the spec had left it open through #2545 with a logged reason.
- Solution: The checkpoint is open again and marked not applicable on its own line, pointing at the 2026-10-15 21:06 UTC entry. A new Progress Log entry records the correction and the earlier archive entry stays as written. The consolidated response is <https://github.com/torrust/torrust-tracker/pull/2554#issuecomment-6264569468>.
- Current-tree verification: Line 300 of the archived `ISSUE.md` reads `- [ ] Independent reviewer reports ... (not applicable: no agent received this folder; see the 2026-10-15 21:06 UTC entry)`.
- Resolution reference: docs(issues): [#2536] correct two checkpoint ticks in the archived spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2554#discussion_r4312482353>

### F2 - "Committer verified" rests on a basis that does not check progress

- PR number: 2554
- Source review ID: 5562041237
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2554#discussion_r4306271913>
- Concern: The archive entry ticked "Committer verified spec progress is up to date before commit" on the basis that every commit passed the pre-commit hook, which checks no progress log; three spec commits during #2545 had landed without a log entry until a later one added it.
- Solution: The tick stays, and its basis is now a check of the spec: a comparison of `git log --follow` for the spec with its Progress Log. A new Progress Log entry records the result, including the one commit (`docs(issues): [#2536] record the pull request number`) that has no entry of its own. The consolidated response is <https://github.com/torrust/torrust-tracker/pull/2554#issuecomment-6264569468>.
- Current-tree verification: The newest Progress Log entry of the archived spec states the comparison and its result, and the log is current through the archive commit.
- Resolution reference: docs(issues): [#2536] correct two checkpoint ticks in the archived spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2554#discussion_r4312482605>

## Processing Log

- 2026-10-19 09:58 UTC - Started audit after da2ce7's review 5562041237 on PR #2554. Fixed F1 and F2 in one signed commit, posted the consolidated response, replied on both threads, and resolved them.

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
