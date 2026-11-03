---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2703 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2703>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the review was processed before the merge)

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

Round 1 is the Copilot review `5757009267` (https://github.com/torrust/torrust-tracker/pull/2703#pullrequestreview-5757009267), submitted 2026-11-03 17:49:54 UTC. Its body holds a warning that Copilot did not run its full agentic review, and an overview of two open issues that link to the two inline threads below; it has no other actionable assertion. Copilot marked both issues "Medium severity" without a `[Severity]` bracket, so the severity is inferred as Minor because both are wording and labelling fixes in documents.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2703-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2703-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Stale sentence says scheduled runs 2 and 3 are pending

- PR number: 2703
- Source review ID: 5757009267
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2703#discussion_r4458442379
- Concern: The introduction of the "Scheduled Runs and Reboot" section in `manual-verification-evidence.md` said runs 2 and 3 were pending, while the same section records runs 2 to 7 as completed, so the section contradicted itself.
- Solution: Rewrote the sentence as a historical statement. It says that runs 2 and 3 were pending when the section was first written, names which agent session observed each run, and refers to the blocks below.
- Current-tree verification: `grep -c "are pending" docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/manual-verification-evidence.md` printed 0, and `grep -n "pending"` on that file listed two lines: the new historical sentence and the `reboot-required` file name in the reboot block. A search of ISSUE.md, the retrospective, and the third report for another stale pending-runs statement found none (ISSUE.md line 81 is a scope note about the pending reboot, which is still true).
- Resolution reference: docs(self-hosted-runner): [#2673] fix the stale pending-runs sentence in V6
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2703#discussion_r4458681232

### F2 - Build cache freed column has no unit or source

- PR number: 2703
- Source review ID: 5757009267
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2703#discussion_r4458442724
- Concern: The "Build cache freed" column header in the third report's scheduled-runs table had no unit, while the report says that Docker prints decimal GB and `df` and `du` print binary GiB, so a reader could compare the figures with GiB disk figures.
- Solution: Renamed the header to "Build cache freed (Docker GB)" and added "(Docker GB)" to the sentence below the table that cites the older journal figures.
- Current-tree verification: `grep -n "Docker GB" docs/self-hosted-runner-reports/20261103-171940-runner-report.md` listed the table header and that sentence. The other Docker-printed figures in the report are in the Disk Space table, whose row labels name the source (`docker system df`) and which sits under a note about the unit difference; they were left unchanged.
- Resolution reference: docs(self-hosted-runner): label the build cache figures in the third report as Docker GB
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2703#discussion_r4458681581

## Processing Log

- 2026-11-03 18:08 UTC - On the maintainer's request to address Copilot's review, fetched the threads with `github-review-threads fetch` and found review `5757009267` (submitted 17:49:54 UTC) with two unresolved inline threads, F1 and F2. Both were checked against the current tree and both held.
- 2026-11-03 18:09 UTC - Committed `docs(self-hosted-runner): [#2673] fix the stale pending-runs sentence in V6` (commit time 18:09:30 UTC, from `git log --format=%cI`).
- 2026-11-03 18:10 UTC - Committed `docs(self-hosted-runner): label the build cache figures in the third report as Docker GB` (commit time 18:10:34 UTC, from `git log --format=%cI`).
- 2026-11-03 18:12 UTC - Pushed both commits to the fork branch; the push command exited 0 before any reply was posted.
- 2026-11-03 18:13 UTC - Replied on both threads (reply `created_at` 18:13:01 UTC for F1 and 18:13:03 UTC for F2, GitHub). My first draft of the F1 reply said the only remaining "pending" in the file was the `reboot-required` name; the check showed the new historical sentence also contains "pending", so I rewrote the draft before posting it, and the posted reply states both lines.
- 2026-11-03 18:13 UTC - `reply-status --login josecelano` exited 0, and the resolve mutation returned `isResolved: true` for both threads (the command's own date line printed 18:13:12 UTC).

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
