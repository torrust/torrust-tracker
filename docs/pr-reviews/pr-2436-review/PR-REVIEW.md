---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2435"
---

<!-- skill-link: process-pr-review -->

# PR #2436 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2436>.

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
| F1 | `review-finding:pr-2436-f1` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2436-f2` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Classification recorded as an exception to the bug rule

- PR number: 2436
- Source review ID: 5411949365
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2436#discussion_r4182122100>
- Concern: The Bug-Fix Process paragraph read as an override of a bug rule the spec conceded applied, although that rule covers observed behavior and nothing is observable here. Its "nothing to act on" also contradicted the Regression Test Strategy, where AC3 is the regression guard.
- Solution: The paragraph now states that the bug rule in the `create-issue` and `fix-bug` skills covers observed behavior, that none is observable because the registry's error type is `Infallible`, and that AC3 is the regression protection. It follows the suggested wording, without line numbers that a skill edit would invalidate.
- Current-tree verification: the Bug-Fix Process paragraph of `docs/issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md` inspected; `linter markdown` and `linter cspell` passed.
- Resolution reference: `docs(issues): [#2435] record the task classification as outside the bug rule`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2436#discussion_r4182399935>

### F2 - Review not recorded in an audit and F1 thread unanswered

- PR number: 2436
- Source review ID: 5412275418
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2436#discussion_r4182364047>
- Concern: At the round-2 head the PR had acted on F1 by commit but had no audit record, and F1's thread had no reply.
- Solution: The audit commit, pushed after round 2, created this record with the F1 row and its Reply URL, logged Copilot's overview remark as a summary, and validated at 0 failures. F1's thread was replied to before the audit commit.
- Current-tree verification: this record has F1 and F2 rows with detail entries; `validate-audit-record.py --pr-number 2436 --base torrust/develop` reports 0 failures.
- Resolution reference: `docs(pr-reviews): [#2435] audit the review of #2436`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2436#discussion_r4183507273>

## Processing Log

- 2026-10-05 08:01 UTC - Copilot review 5411559700 posted an overview without inline findings. Its remark that the specification bypasses the semantic bug workflow is not a separate row: human review 5411949365 assessed it and recorded the wording concern as F1.
- 2026-10-05 08:37 UTC - Human review 5411949365 (da2ce7, round 1) approved with one Nit, F1.
- 2026-10-05 09:09 UTC - Human review 5412275418 (da2ce7, round 2, at `docs(issues): [#2435] record the task classification as outside the bug rule`) confirmed F1 fixed and added F2; the audit push at 11:29 dismissed it.
- 2026-10-05 09:13 UTC - Rebased onto `develop`, fixed F1, pushed after the pre-push suite passed, and replied on its thread before recording it here.
- 2026-10-05 11:33 UTC - Replied on F2 after its fix was already pushed, then recorded F2 here.

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
