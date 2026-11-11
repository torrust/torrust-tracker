---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2817 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2817.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`, `maintainability`, `security`, `other`

## Findings

All four findings are in the body of review 5856630973: a rename without changes has no diff line to anchor on, so there are no review threads and every row is `NON_RESOLVABLE`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2817-f1 | Human | Major | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F2 | review-finding:pr-2817-f2 | Human | Major | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F3 | review-finding:pr-2817-f3 | Human | Minor | link-integrity | ORIGINAL | FIXED | NON_RESOLVABLE |
| F4 | review-finding:pr-2817-f4 | Human | Nit | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The archived spec still reads as open

- PR number: 2817
- Source review ID: 5856630973
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2817#pullrequestreview-5856630973
- Concern: `ISSUE.md` kept `status: in-progress`, the open `spec-path`, `related-pr: null`, unticked checkpoints, `Not yet assessed` and no close-out log entry.
- Solution: The spec now has `status: done`, the closed `spec-path`, `related-pr: 2815`, ticked checkpoints, a completion review and a log entry.
- Current-tree verification: `grep -n "^status:\|^spec-path:\|^related-pr:" ISSUE.md` shows the new values.
- Resolution reference: docs(issues): [#2714] close the archived spec: status, paths, checkpoints and log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2817#issuecomment-6792409028

### F2 - EPIC row 19 stays TODO and names the deleted path

- PR number: 2817
- Source review ID: 5856630973
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2817#pullrequestreview-5856630973
- Concern: Row 19 of `EPIC.md` read `TODO` with the open path and there was no progress-log entry.
- Solution: Row 19 is `DONE`, names the closed path and the fix PR #2815, and the progress log has an entry.
- Current-tree verification: `grep -n "^| 19 " EPIC.md` shows DONE and the closed path.
- Resolution reference: docs(issues): [#2714] mark EPIC row 19 done and point it at the closed spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2817#issuecomment-6792409028

### F3 - The evidence file names the open spec path

- PR number: 2817
- Source review ID: 5856630973
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2817#pullrequestreview-5856630973
- Concern: `issue-spec` in `manual-verification-evidence.md` still named the open path.
- Solution: `issue-spec` now names the closed path.
- Current-tree verification: `grep -n "^issue-spec:" manual-verification-evidence.md` shows the closed path.
- Resolution reference: docs(issues): [#2714] point the evidence file at the closed spec path
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2817#issuecomment-6792409028

### F4 - The issue reference in the title is reserved for a PR that closes the issue

- PR number: 2817
- Source review ID: 5856630973
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2817#pullrequestreview-5856630973
- Concern: The title used `[#2714]` and a subject the archive step does not prescribe.
- Solution: The title is now `chore(issues): archive closed issue #2714 spec to docs/issues/closed`.
- Current-tree verification: `gh-bot pr view 2817 --json title` shows the new title.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2817#issuecomment-6792409028
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2817#issuecomment-6792409028

## Processing Log

- 2026-11-11 09:38 UTC - AI assistant (Claude Code, Claude Sonnet 5.5) - Read review 5856630973 at head `dce968aa8`: my first commit held only the renames and the edits were never committed. Fixed in three commits and a title change; the three commits were pushed before the conversation reply (comment created at 2026-11-11T09:38:44Z) that answers all four findings.
