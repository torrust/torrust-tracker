---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2471-1488-fix-http-environment-drop-path/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2483 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2483>.

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

Copilot review 5453173755 (07:27 UTC) supplied the finding IDs `F1` to `F6` with their severities;
they are kept. The review body summarizes those six threads and adds no other assertion. F4 and F6
ask for the same change as F3 and F5 in a different file, so they are original findings, not
re-raises.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2483-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2483-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2483-f3` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2483-f4` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2483-f5` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2483-f6` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The #2471 reproduction does not record its symptom and hypothesis

- PR number: 2483
- Source review ID: 5453173755
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216170247>
- Concern: The fix-bug evidence contract requires the original symptom and the current hypothesis, but V1 gave only the goal and initial state, so the evidence did not preserve which explanation was tested.
- Solution: V1 now records the original symptom (found by source review: a plain `CancellationToken` and no `DropGuard`, compared with PR #2459 F1) and the hypothesis (dropping the token does not cancel it, so the server keeps running and its address stays bound).
- Current-tree verification: `grep -n` finds `- Original symptom` at line 48 and `- Hypothesis` at line 52 of `docs/issues/open/2471-1488-fix-http-environment-drop-path/manual-verification-evidence.md`; the pre-commit gate passed.
- Resolution reference: docs(issues): record the #2471 reproduction's symptom and hypothesis
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216259688>

### F2 - The #2471 spec skips the post-green design-review checkpoint

- PR number: 2483
- Source review ID: 5453173755
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216170200>
- Concern: The fix changes asynchronous cleanup of a reusable test fixture and adds a regression test, so the issue workflow requires a recorded design review after the first passing slice and before maintainer review or commit. The spec said there was no checkpoint beyond maintainer review.
- Solution: The Design and Ownership Review checkpoint now requires a design review after the regression test turns green (T2), recording the prose-first Arrange-Act-Assert review and checking `stop()` and the drop path. The T1-T2 commit point and a new workflow checkpoint require it too.
- Current-tree verification: In `docs/issues/open/2471-1488-fix-http-environment-drop-path/ISSUE.md`, the checkpoint is at line 112, the commit point at line 155, and the workflow checkpoint at line 171; `grep -c "none beyond maintainer review"` returns 0.
- Resolution reference: docs(issues): restore the #2471 post-green design-review checkpoint
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216259982>

### F3 - SI-23 claims a drop leaves no detached task

- PR number: 2483
- Source review ID: 5453173755
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216170122>
- Concern: Dropping the environment drops its `JoinHandle`s, which detaches those tasks; a `DropGuard` requests cancellation but cannot join them, as the UDP environment documents. The invariant should promise shutdown, not no detachment.
- Solution: The SI-23 drop-path bullet now says a drop can only request cancellation: it drops the task handles without joining them, so the tasks end shortly after the drop rather than before it returns.
- Current-tree verification: `grep -n "A drop can only"` matches line 123 of `docs/issues/open/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md`; `grep -c "No new detached"` returns 0.
- Resolution reference: docs(issues): describe the SI-23 and SI-24 drop path as cancellation only
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216260212>

### F4 - SI-24 claims a drop leaves no detached task

- PR number: 2483
- Source review ID: 5453173755
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216170166>
- Concern: The same concern as F3, in the SI-24 spec: the drop-path bullet claimed no new detached task, but a drop cannot join the tasks it cancels.
- Solution: The SI-24 drop-path bullet now uses the same cancellation-only wording as SI-23, in the same commit.
- Current-tree verification: `grep -n "A drop can only"` matches line 128 of `docs/issues/open/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md`; `grep -c "No new detached"` returns 0.
- Resolution reference: docs(issues): describe the SI-23 and SI-24 drop path as cancellation only
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216260430>

### F5 - SI-18's update timestamp predates its roadmap edit

- PR number: 2483
- Source review ID: 5453173755
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216169994>
- Concern: The SI-18 draft's roadmap position changed on 2026-10-07, but its `last-updated-utc` still said `2026-10-06 11:34`.
- Solution: `last-updated-utc` is now `2026-10-08 07:36`.
- Current-tree verification: `grep -n last-updated-utc docs/issues/drafts/1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md` shows `"2026-10-08 07:36"` at line 12.
- Resolution reference: docs(issues): refresh the SI-18 and SI-19 update timestamps
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216260783>

### F6 - SI-19's update timestamp predates its roadmap edit

- PR number: 2483
- Source review ID: 5453173755
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216170067>
- Concern: The same concern as F5, for the SI-19 draft.
- Solution: `last-updated-utc` is now `2026-10-08 07:36`, in the same commit as F5.
- Current-tree verification: `grep -n last-updated-utc docs/issues/drafts/1488-si-19-remove-legacy-shutdown-api/ISSUE.md` shows `"2026-10-08 07:36"` at line 12.
- Resolution reference: docs(issues): refresh the SI-18 and SI-19 update timestamps
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2483#discussion_r4216261045>

## Processing Log

- 2026-10-08 07:30 UTC - Started audit. Fetched the six review threads with GraphQL (all
  unresolved, none outdated) and review 5453173755; normalized them into F1 to F6.
- 2026-10-08 07:38 UTC - Committed the four fixes after the pre-commit gate passed, pushed them
  after the pre-push gate passed, re-checked each claim against the pushed tree, and replied on
  all six threads.

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
